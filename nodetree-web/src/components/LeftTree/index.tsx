import clsx from "clsx";
import {
  CursorProps,
  NodeApi,
  NodeRendererProps,
  Tree,
  TreeApi,
} from "react-arborist";
import * as icons from "react-icons/md";
import styles from "../../tree.module.css";
import { FillFlexParent } from "../fill-flex-parent";
import { BsTree } from "react-icons/bs";
import { useEffect, useState } from "react";
import { NTNode } from "../../model";
import { fetchAllNodes } from "../../helpers/dataAgent";
import { arrangeNodes } from "../../helpers/nodeHelper";

export default function NTTree() {
  const [term] = useState("");

  const [tree, setTree] = useState<TreeApi<NTNode> | null | undefined>(null);

  const [isLoading, setIsLoading] = useState(true);

  //const [allNodesData, setAllNodesData] = useState<NTNode[]>([]);
  const allNodesData: NTNode[] = [
    {
      id: "1",
      version: 0,

      is_current: true,
      delete_time: undefined,

      name: "1",
      content: "",

      user: "",
      todo_status: undefined,

      tags: undefined,

      parent_id: "",
      prev_sliding_id: undefined,

      create_time: new Date(),
      first_version_time: new Date(),

      children: undefined,
    },
  ];
  useEffect(() => {
    const fetchData = async () => {
      try {
        console.info("begin to get all nodes");
        const nodes = await fetchAllNodes();
        const arrangedNodes = arrangeNodes(nodes);
        //setAllNodesData(arrangedNodes);
        setIsLoading(false);
        console.log("finished load all nodes");
      } catch (error) {
        console.error(`unable get all nodes ${error}`);
      }
    };

    fetchData();
  }, []);

  return (
    <div className={styles.sidebar}>
      {isLoading ? (
        <div>Loading </div>
      ) : (
        <FillFlexParent>
          {({ width, height }) => {
            return <Tree initialData={allNodesData}>{Node}</Tree>;
          }}
        </FillFlexParent>
      )}
    </div>
  );
}

function Node({ node, style, dragHandle }: NodeRendererProps<NTNode>) {
  console.info(`render nodes: ${node}`);
  return (
    <div style={style} ref={dragHandle}>
      {node.isLeaf ? "🍁" : "🗀"}
      {node.data.name}
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
  if (node.isLeaf) return <span></span>;
  return (
    <span>
      {node.isOpen ? <icons.MdArrowDropDown /> : <icons.MdArrowRight />}
    </span>
  );
}

function Cursor({ top, left }: CursorProps) {
  return <div className={styles.dropCursor} style={{ top, left }}></div>;
}
