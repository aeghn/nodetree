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
import * as icons from "react-icons/md";
import styles from "./tree.module.css";
import { useEffect, useMemo, useState } from "react";
import { fetchAllNodes, moveNode } from "../../helpers/dataAgent";
import { arrangeNodes } from "../../helpers/nodeHelper";
import { NTNode } from "../../model";

let nextId = 0;

export const NTTree: React.FC<{
  height: number | undefined;
  setActivate: Function;
  treeRef: React.MutableRefObject<SimpleTree<NTNode> | null>;
}> = ({ height, setActivate, treeRef }) => {
  const [data, setData] = useState<NTNode[]>([]);
  const tree = useMemo(() => new SimpleTree<NTNode>(data), [data]);

  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const nodes = await fetchAllNodes();
        const arrangedNodes = arrangeNodes(nodes);
        setData(arrangedNodes);
        setIsLoading(false);
        console.log("Loaded all nodes");
      } catch (error) {
        console.error(`Unable get all nodes ${error}`);
      }
    };

    fetchData();
  }, []);

  useEffect(() => {
    treeRef.current = tree;
    return () => {
      treeRef.current = null;
    };
  }, [tree, treeRef]);

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
    setData(tree.data);
  };

  const onRename: RenameHandler<NTNode> = ({ name, id }) => {
    tree.update({ id, changes: { name } as NTNode });
    setData(tree.data);
  };

  const onCreate: CreateHandler<NTNode> = ({ parentId, index, type }) => {
    const data = { id: `simple-tree-id-${nextId++}`, name: "" } as NTNode;
    if (type === "internal") data.children = [];
    tree.create({ parentId, index, data });
    setData(tree.data);
    return data;
  };

  const onDelete: DeleteHandler<NTNode> = (args: { ids: string[] }) => {
    args.ids.forEach((id) => tree.drop({ id }));
    setData(tree.data);
  };

  return isLoading ? (
    <div> Loading </div>
  ) : (
    <div className={styles.treeContainer}>
      <Tree
        data={data}
        width="100%"
        height={height}
        rowHeight={32}
        renderCursor={Cursor}
        paddingBottom={32}
        onMove={onMove}
        onRename={onRename}
        onCreate={onCreate}
        onDelete={onDelete}
        openByDefault={true}
        onActivate={(node) => setActivate(node.data)}
        keybinding={{
          ArrowDown: "ActivateNext",
          ArrowUp: "ActivatePrev",
          ArrowRight: "Right",
          ArrowLeft: "Left",
          Tab: "Toggle",
          c: "CreateChild",
          s: "CreateSlibing",
          R: "Rename",
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
      onClick={() => node.isInternal && node.toggle()}
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
    <span>
      {node.isOpen ? <icons.MdArrowDropDown /> : <icons.MdArrowRight />}
    </span>
  );
}

function Cursor({ top, left }: CursorProps) {
  return <div className={styles.dropCursor} style={{ top, left }}></div>;
}
