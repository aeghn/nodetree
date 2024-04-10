import useResizeObserver from "use-resize-observer";
import NTTree from "../tree";
import { ReactNode, useCallback, useEffect, useState } from "react";
import { NTNode, NodeId } from "../../model";
import {
  fetchAllNodes,
  fetchNodeContent,
  saveNode,
} from "../../helpers/data-agent";
import { useThrottleEffect } from "../../hooks/use-throttle-effect";
import { arrangeNodes } from "../../helpers/node-helper";
import NTEditor from "../editor/index";
import { TreePine, BookX } from "lucide-react";

function TreeEditor() {
  const { ref: heightRef, height } = useResizeObserver<HTMLDivElement>({});

  const [treeData, setTreeData] = useState<NTNode[]>();

  // tree => view
  const [activeNode, setActiveNode] = useState<NTNode>();

  // tree => editor(props)
  const [activeNodeContent, setActiveNodeContent] = useState<string>();

  // editor => tree(props)
  const [targetNodeId, setTargetNodeId] = useState<NodeId>();

  // to save nodes
  const [toSaveNodes, setToSaveNodes] = useState(new Map<NodeId, NTNode>());

  const setActiveNodeCallback = useCallback((node: NTNode) => {
    setActiveNodeContent(undefined);
    setActiveNode(node);
  }, []);

  useEffect(() => {
    if (activeNode?.id) {
      fetchNodeContent(activeNode.id).then((node: NTNode) => {
        let text = node.content;
        if (text && text.length > 0) {
          const trimedStart = text.trimStart();
          if (trimedStart.startsWith("{") || trimedStart.startsWith("[")) {
            try {
              text = JSON.parse(text);
            } catch (err) {
              console.error("unable to parse node content: ", err);
            }
          }
        }
        setActiveNodeContent(text);
      });
    }
  }, [activeNode]);

  const contentChangeCallback = useCallback(
    (content: string, nodeId: NodeId) => {
      if (
        activeNode &&
        activeNode.content !== content &&
        activeNode.id === nodeId
      ) {
        setToSaveNodes(
          new Map(toSaveNodes).set(nodeId, { ...activeNode, content })
        );
      }
    },
    [activeNode]
  );

  const idChangeCallback = useCallback((nodeId: NodeId) => {
    setTargetNodeId(nodeId);
  }, []);

  useThrottleEffect(
    (toSaveNodes) => {
      const keysToDelete: string[] = [];

      toSaveNodes.forEach((value, key) => {
        saveNode(value, false);
        keysToDelete.push(key);
      });

      for (const key of keysToDelete) {
        toSaveNodes.delete(key);
      }
    },

    [toSaveNodes],
    3000
  );

  useEffect(() => {
    try {
      fetchAllNodes().then((nodes) => {
        const arrangedNodes = arrangeNodes(nodes);
        setTreeData(arrangedNodes);
        console.log("Loaded all nodes");
      });
    } catch (error) {
      console.error(`Unable get all nodes ${error}`);
    }
  }, []);

  return (
    <div className="h-screen p-5 shadow bg-slate-300">
      <div className="flex flex-row m-0 h-full content-center" ref={heightRef}>
        <div className="w-3/12 m-0 h-full pr-4">
          {treeData ? (
            <NTTree
              height={height}
              setActiveNodeCallback={setActiveNodeCallback}
              treeData={treeData}
              activeNodeId={targetNodeId}
            />
          ) : (
            <Loading
              customIcon={<TreePine size={34} strokeWidth={1} />}
              message="Loading tree"
            />
          )}
        </div>

        <div className="w-9/12 h-full border-solid border rounded-md border-gray-300 bg-white shadow-lg">
          {activeNode?.id ? (
            activeNodeContent !== undefined ? (
              <NTEditor
                height={height}
                nodeId={activeNode.id}
                content={activeNodeContent}
                contentChangeCallback={contentChangeCallback}
                idChangeCallback={idChangeCallback}
              />
            ) : (
              <div></div>
            )
          ) : (
            <Loading
              customIcon={<BookX size={34} strokeWidth={1} />}
              message="No active node is selected."
            />
          )}
        </div>
      </div>
    </div>
  );
}

function Loading({
  customIcon,
  message,
}: {
  customIcon: ReactNode;
  message: string;
}) {
  return (
    <div className="top-1/2 w-full h-full flex flex-col justify-center items-center align-middle gap-1 text-muted-foreground">
      {customIcon}
      <p>{message}</p>
    </div>
  );
}

export default TreeEditor;
