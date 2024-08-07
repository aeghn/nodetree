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
import { useEffect, useMemo, useRef, useState } from "react";
import {
  moveNode,
  saveNode,
  updateNodeName,
  deteleNode as deleteNode,
} from "@/helpers/data-agent";
import {
  KNode,
  ContentParsedInfo,
  NodeType,
  NodeId,
  strToNodeId,
} from "@/model";
import { generateId } from "@/helpers/tools";
import React from "react";

import { LuChevronRight, LuChevronDown } from "react-icons/lu";
import { useAtom } from "jotai";
import {
  useGetNodeIdNameAtom,
  setNodeAtom,
  setTreeNodeIdAtom,
  treeNodeIdAtom,
} from "@/state/explorer";
import { shortDate } from "@/helpers/date-helper";

export const KTree: React.FC<{
  height?: number;
  treeDataList: KNode[];
}> = ({ height, treeDataList }) => {
  console.log("tree render");

  const [treeData, setTreeData] = useState<KNode[]>(treeDataList);
  const tree = useMemo(() => {
    console.log("set tree ========================");
    return new SimpleTree<KNode>(treeData);
  }, [treeData]);
  const oldNode = useRef<KNode | undefined>();

  const [activeNodeId] = useAtom(treeNodeIdAtom);
  const [activeNodeIdName] = useAtom(useGetNodeIdNameAtom());
  const [, setTreeNodeId] = useAtom(setTreeNodeIdAtom);
  const [, setNode] = useAtom(setNodeAtom);

  const setNodeNameById = (
    id: string,
    name: string,
    tree: SimpleTree<KNode>
  ) => {
    tree.update({ id, changes: { name } as KNode });
    try {
      updateNodeName(id, name);
    } catch (error) {
      console.log(error);
    }
    setTreeData(tree.data);
  };

  const treeFind = (nodeId?: NodeId | null) => {
    if (nodeId) {
      return tree.find(nodeId);
    } else {
      return tree.root;
    }
  };

  const onMove: MoveHandler<KNode> = async (args: {
    dragIds: string[];
    parentId: null | string;
    parentNode: NodeApi<KNode> | null;
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
        moveNode(id, strToNodeId(args.parentId), strToNodeId(prev)).then(() => {
          tree.move({ id, parentId: args.parentId, index: args.index });
          setTreeData(tree.data);
        });
      } catch (error) {
        console.log(error);
      }
    }
  };

  const onRename: RenameHandler<KNode> = ({ name, id }) => {
    setNodeNameById(id, name, tree);
  };

  const onCreate: CreateHandler<KNode> = ({ parentId, index, type }) => {
    const parsed_info: ContentParsedInfo = {};
    const data: KNode = {
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
      readonly: false,
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

  const onDelete: DeleteHandler<KNode> = (args: { ids: string[] }) => {
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
      if (an && an.data != oldNode.current) {
        setNode(an.data);
        oldNode.current = an.data;
      }
    }
  }, [activeNodeId, tree]);

  useEffect(() => {
    if (
      activeNodeIdName.id &&
      activeNodeIdName.name &&
      activeNodeIdName.name !== tree.find(activeNodeIdName.id)?.data.name
    ) {
      const id = activeNodeIdName.id;
      const name = activeNodeIdName.name;
      setNodeNameById(id, name, tree);
    }
  }, [activeNodeIdName, tree]);

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
        openByDefault={false}
        onActivate={(node) => {
          if (node.data !== oldNode.current) {
            console.log("new node", node.data);
            console.log("old node", oldNode.current);
            setNode(node.data);
            oldNode.current = node.data;
            setTreeNodeId(node.id);
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

function Node({ node, style, dragHandle }: NodeRendererProps<KNode>) {
  return (
    <div
      ref={dragHandle}
      style={style}
      className={clsx(styles.node, node.state)}
      onClick={() => node.activate()}
    >
      <FolderArrow node={node} />
      {node.data.parsed_info.todo_status != undefined ?? (
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

function Input({ node }: { node: NodeApi<KNode> }) {
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

function FolderArrow({ node }: { node: NodeApi<KNode> }) {
  if (node.isLeaf || node.children?.length == 0) return <span></span>;
  return (
    <span className={styles.arrow}>
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

const KTreeMemo = React.memo(KTree);

export default KTreeMemo;
