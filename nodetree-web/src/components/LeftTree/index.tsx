import clsx from "clsx";
import {
  CursorProps,
  NodeApi,
  NodeRendererProps,
  Tree,
  TreeApi,
} from "react-arborist";
import * as icons from "react-icons/md";
import styles from "./tree.module.css";
import { FillFlexParent } from "../fill-flex-parent";
import { BsTree } from "react-icons/bs";
import { ComponentType, useEffect, useState } from "react";
import { fetchAllNodes } from "../../helpers/dataAgent";
import { arrangeNodes } from "../../helpers/nodeHelper";
import { NTNode } from "../../model";


export default function NTTree() {
  const [term] = useState("");

  const [tree, setTree] = useState<TreeApi<NTNode> | null | undefined>(null);

  const [isLoading, setIsLoading] = useState(true);

  const [allNodesData, setAllNodesData] = useState<NTNode[]>([]);

  useEffect(() => {
    const fetchData = async () => {
      try {
        console.info("begin to get all nodes");
        const nodes = await fetchAllNodes();
        const arrangedNodes = arrangeNodes(nodes);
        setAllNodesData(arrangedNodes);
        setIsLoading(false);
        console.log("finished load all nodes");
      } catch (error) {
        console.error(`unable get all nodes ${error}`);
      }
    };

    fetchData();
  }, []);

  return (

    isLoading ? (
      <div> Loading </div >
    ) : (
      <div className={styles.treeContainer}>
        <FillFlexParent>
          {({ width, height }) => {
            return (
              <Tree
                ref={setTree}
                initialData={allNodesData}
                width={width}
                height={height}
                rowHeight={32}
                renderCursor={Cursor}
                searchTerm={term}
                paddingBottom={32}
              >
                {Node}
              </Tree>
            );
          }}
        </FillFlexParent>
      </div>
    )
  );
}

function Node({ node, style, dragHandle }: NodeRendererProps<NTNode>) {
  const Icon = BsTree;
  return (
    <div
      ref={dragHandle}
      style={style}
      className={clsx(styles.node, node.state)}
      onClick={() => node.isInternal && node.toggle()}
    >
      <FolderArrow node={node} />
      <span>
        <Icon />
      </span>
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
