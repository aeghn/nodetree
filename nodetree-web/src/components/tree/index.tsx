import clsx from "clsx";
import {
  CreateHandler,
  CursorProps,
  DeleteHandler,
  MoveHandler,
  NodeApi,
  NodeRendererProps,
  RenameHandler,
  SimpleTree,
  Tree,
} from "react-arborist";
import styles from "./tree.module.css";
import { useEffect, useMemo, useState } from "react";
import {
  moveNode,
  saveNode,
  updateNodeName,
  deteleNode as deleteNode,
} from "@/helpers/data-agent";
import { NTNode, ContentParsedInfo, NodeId, NodeType } from "@/model";
import { generateId } from "@/helpers/tools";
import React from "react";

import { LuChevronRight, LuChevronDown } from "react-icons/lu";

export const NTTree: React.FC<{
  height?: number;
  activeNodeId?: NodeId;
  treeDataList: NTNode[];
  setActiveNodeCallback: (node: NTNode) => void;
}> = ({ height, activeNodeId, treeDataList, setActiveNodeCallback }) => {
  const [treeData, setTreeData] = useState<NTNode[]>(treeDataList);
  const tree = useMemo(() => new SimpleTree<NTNode>(treeData), [treeData]);
  let oldNode: NTNode | undefined = undefined;

  const onMove: MoveHandler<NTNode> = (args: {
    dragIds: string[];
    parentId: null | string;
    parentNode: NodeApi<NTNode> | null;
    index: number;
  }) => {
    for (const id of args.dragIds) {
      tree.move({ id, parentId: args.parentId, index: args.index });
      const index = args.index - 1;
      const prev = args.parentNode?.children?.[index]?.id;
      try {
        moveNode(id, args.parentId, prev);
      } catch (error) {
        console.log(error);
      }
    }
    setTreeData(tree.data);
  };

  const onRename: RenameHandler<NTNode> = ({ name, id }) => {
    tree.update({ id, changes: { name } as NTNode });
    try {
      updateNodeName(id, name);
    } catch (error) {
      console.log(error);
    }
    setTreeData(tree.data);
  };

  const onCreate: CreateHandler<NTNode> = ({ parentId, index, type }) => {
    const parsed_info: ContentParsedInfo = {};
    const data: NTNode = {
      id: generateId(),
      name: "untitled",
      content: "",
      user: "",
      parent_id: parentId ? parentId : undefined,
      version_time: new Date(),
      initial_time: new Date(),
      parsed_info: parsed_info,
      node_type: NodeType.TiptapV1
    };

    if (type === "internal") data.children = [];
    try {
      saveNode(data, true);
    } catch (error) {
      console.log(error);
    }

    tree.create({ parentId, index, data });
    setTreeData(tree.data);
    return data;
  };

  const onDelete: DeleteHandler<NTNode> = (args: { ids: string[] }) => {
    args.ids.forEach((id) => {
      tree.drop({ id });
      try {
        deleteNode(id);
      } catch (error) {
        console.log(error);
      }
    });

    setTreeData(tree.data);
  };

  useEffect(() => {
    if (activeNodeId) {
      const an = tree.find(activeNodeId);
      if (an && an.data != oldNode) {
        setActiveNodeCallback(an.data);
        oldNode = an.data;
      }
    }
  }, [tree, activeNodeId]);

  return (
    <div className={styles.treeContainer}>
      <Tree
        data={treeData}
        width="100%"
        height={height}
        rowHeight={32}
        renderCursor={Cursor}
        paddingBottom={32}
        selection={activeNodeId}
        onMove={onMove}
        onRename={onRename}
        onCreate={onCreate}
        onDelete={onDelete}
        openByDefault={true}
        onActivate={(node) => {
          if (node.data != oldNode) {
            setActiveNodeCallback(node.data);
            oldNode = node.data;
          }
        }}
        keybinding={{
          ArrowDown: "ActivateNext",
          ArrowUp: "ActivatePrev",
          ArrowRight: "Right",
          ArrowLeft: "Left",
          Tab: "Toggle",
          c: "CreateChild",
          s: "CreateSlibing",
          R: "Rename",
          D: "Delete",
        }}
      >
        {Node}
      </Tree>
    </div>
  );
};

function Node({ node, style, dragHandle }: NodeRendererProps<NTNode>) {
  return (
    <div
      ref={dragHandle}
      style={style}
      className={clsx(styles.node, node.state)}
      onClick={() => node.activate()}
    >
      <FolderArrow node={node} />
      <span>{node.isEditing ? <Input node={node} /> : node.data.name}</span>
    </div>
  );
}

function Input({ node }: { node: NodeApi<NTNode> }) {
  return (
    <input
      autoFocus
      type="text"
      defaultValue={node.data.name}
      onFocus={(e) => e.currentTarget.select()}
      onBlur={() => node.reset()}
      onKeyDown={(e) => {
        if (e.key === "Escape") node.reset();
        if (e.key === "Enter") node.submit(e.currentTarget.value);
      }}
    />
  );
}

function FolderArrow({ node }: { node: NodeApi<NTNode> }) {
  if (node.isLeaf || node.children?.length == 0) return <span></span>;
  return (
    <span className="pl-2">
      {node.isOpen ? (
        <LuChevronDown onClick={() => node.toggle()} />
      ) : (
        <LuChevronRight onClick={() => node.toggle()} />
      )}
    </span>
  );
}

function Cursor({ top, left }: CursorProps) {
  return <div className={styles.dropCursor} style={{ top, left }}></div>;
}

const NTTreeMemo = React.memo(NTTree);

export default NTTreeMemo;
