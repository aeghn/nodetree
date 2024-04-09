import useResizeObserver from "use-resize-observer";
import NTTree from "../tree";
import { Suspense, useCallback, useEffect, useRef, useState } from "react";
import { NTNode, NodeId } from "../../model";
import { fetchNodeContent, saveNode } from "../../helpers/data-agent";
import { SimpleTree } from "react-arborist";
import { useThrottleEffect } from "../../hooks/use-throttle-effect";
import React from "react";

const NTEditor = React.lazy(() => import("../editor/index"));

function TreeEditor() {
  const { ref, height } = useResizeObserver<HTMLDivElement>({});

  const treeRef = useRef<SimpleTree<NTNode> | null>(null);

  // tree => view
  const [activeNode, setActiveNode] = useState<NTNode>();

  // tree => editor(props)
  const [activeNodeContent, setActiveNodeContent] = useState<string>();

  // editor => tree(props)
  const [targetNodeId, setTargetNodeId] = useState<NodeId>();

  // to save nodes
  const [toSaveNodes, setToSaveNodes] = useState(new Map<NodeId, NTNode>());

  const setActiveNodeCallback = useCallback(
    (node: NTNode) => {
      if (activeNode !== node) {
        setActiveNodeContent(undefined);
        setActiveNode(node);
      }
    },
    [activeNode]
  );

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

  return (
    <div className="h-screen p-2 shadow">
      <div
        className="flex flex-row border-solid border rounded-md border-gray-300 m-0 h-full"
        ref={ref}
      >
        <div className="w-5/12 h-full bg-[#f0f0f0]">
          <NTTree
            height={height}
            setActiveNodeCallback={setActiveNodeCallback}
            activeNodeId={targetNodeId}
            treeRef={treeRef}
          />
        </div>

        <div className="w-7/12 h-full">
          {activeNode?.id ? (
            activeNodeContent !== undefined ? (
              <Suspense fallback={Loading()}>
                <NTEditor
                  height={height}
                  nodeId={activeNode.id}
                  content={activeNodeContent}
                  contentChangeCallback={contentChangeCallback}
                  idChangeCallback={setTargetNodeId}
                />
              </Suspense>
            ) : (
              Loading()
            )
          ) : (
            <div>Non node is selected</div>
          )}
        </div>
      </div>
    </div>
  );
}

function Loading() {
  return <div>Loading...</div>;
}

export default TreeEditor;
