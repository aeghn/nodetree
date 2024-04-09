import useResizeObserver from "use-resize-observer";
import NTTree from "../tree";
import { Suspense, useCallback, useEffect, useRef, useState } from "react";
import { NTNode, NodeId } from "../../model";
import { saveNode } from "../../helpers/data-agent";
import { SimpleTree } from "react-arborist";
import { useThrottleEffect } from "../../hooks/use-throttle-effect";
import React from "react";

const NTEditor = React.lazy(() => import("../editor/index"));

function TreeEditor() {
  const { ref, height } = useResizeObserver<HTMLDivElement>({});

  const [activeNode, setActiveNode] = useState<NTNode>();

  const treeRef = useRef<SimpleTree<NTNode> | null>(null);
  const [activeNodeId, setActiveNodeId] = useState<NodeId>();

  const contentChangeCallback = useCallback((content: string) => {
    if (activeNode && activeNode.content !== content) {
      setActiveNode({ ...activeNode, content: content });
    }
  }, []);

  const setActiveNodeHandler = useCallback(
    (node: NTNode) => setActiveNode(node),
    []
  );

  useThrottleEffect(
    (node) => {
      if (node) {
        saveNode(node);
        if (treeRef.current) {
          treeRef.current.update({
            id: node.id,
            changes: {
              ...node,
            },
          });
        }
      }
    },
    [activeNode],
    3000
  );

  /*   const changed = (title: string, e: any) => {
    return useEffect(() => {
      console.log("changed: " + title, e);
    }, [e]);
  };

  changed("setActiveNode", setActiveNode);
  changed("activeNodeId", activeNodeId);
  changed("height", height);
  changed("treeRef", treeRef);
  changed("ref", ref);
  changed("activeNode.id", activeNode?.id);
  changed("contentChangeCallback", contentChangeCallback);
  changed("setActiveNodeId", setActiveNodeId); */

  return (
    <div className="h-screen p-2 shadow">
      <div
        className="flex flex-row border-solid border rounded-md border-gray-300 m-0 h-full"
        ref={ref}
      >
        <div className="w-5/12 h-full bg-[#f0f0f0]">
          <NTTree
            height={height}
            setActiveNode={setActiveNodeHandler}
            activeNodeId={activeNodeId}
            treeRef={treeRef}
          />
        </div>

        <div className="w-7/12 h-full">
          {activeNode?.id ? (
            <Suspense fallback={<div>Loading...</div>}>
              <NTEditor
                height={height}
                nodeId={activeNode.id}
                contentChangeCallback={contentChangeCallback}
                idChangeCallback={setActiveNodeId}
              />
            </Suspense>
          ) : (
            <div>Non node is selected</div>
          )}
        </div>
      </div>
    </div>
  );
}

export default TreeEditor;
