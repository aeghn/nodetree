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
import {
  NTNode,
  ContentParsedInfo,
  NodeType,
  NodeId,
  strToNodeId,
} from "@/model";
import { generateId } from "@/helpers/tools";
import React from "react";

import { LuChevronRight, LuChevronDown } from "react-icons/lu";
import { useAtom } from "jotai";
import { setNodeAtom, treeNodeIdAtom } from "@/state/explorer";
import { shortDate } from "@/helpers/date-helper";

export const NTTree: React.FC<{
  height?: number;
  treeDataList: NTNode[];
}> = ({ height, treeDataList }) => {
  const [treeData, setTreeData] = useState<NTNode[]>(treeDataList);
  const tree = useMemo(() => new SimpleTree<NTNode>(treeData), [treeData]);
  let oldNode: NTNode | undefined = undefined;

  const [activeNodeId] = useAtom(treeNodeIdAtom);
  const [, setNode] = useAtom(setNodeAtom);

  const treeFind = (nodeId?: NodeId | null) => {
    if (nodeId) {
      return tree.find(nodeId);
    } else {
      return tree.root;
    }
  };

  const onMove: MoveHandler<NTNode> = async (args: {
    dragIds: string[];
    parentId: null | string;
    parentNode: NodeApi<NTNode> | null;
    index: number;
  }) => {
    for (const id of args.dragIds) {
      const index = args.index - 1;
      let prev = null;
      if (args.parentNode) {
        prev = args.parentNode?.children?.[index]?.id;
      } else {
        prev = tree.root.children?.[index]?.id;
      }

      try {
        moveNode(id, strToNodeId(args.parentId), strToNodeId(prev)).then(
          (res: any) => {
            tree.move({ id, parentId: args.parentId, index: args.index });
            setTreeData(tree.data);
          }
        );
      } catch (error) {
        console.log(error);
      }
    }
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
      domain: "",
      parent_id: strToNodeId(parentId),
      prev_sliding_id: strToNodeId(
        treeFind(parentId)?.children?.[index - 1]?.id
      ),
      version_time: new Date(),
      initial_time: new Date(),
      parsed_info: parsed_info,
      node_type: NodeType.TiptapV1,
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
        setNode(an.data);
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
          if (node.data !== oldNode) {
            setNode(node.data);
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
      {node.data.parsed_info.todo_status ?? (
        <div className="border rounded">
          {node.data.parsed_info.todo_status}
        </div>
      )}
      <div className="p-1 m-0 text-sm text-gray-500 text-pretty">
        {shortDate(node.data.initial_time)}
      </div>
      <div className="text-base">
        {node.isEditing ? <Input node={node} /> : node.data.name}
      </div>
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
