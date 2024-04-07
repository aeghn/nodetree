import useResizeObserver from "use-resize-observer";
import { NTTree } from "./components/tree";
import { NTEditor } from "./components/editor";
import { useRef, useState } from "react";
import { NTNode } from "./model";
import { saveNode } from "./helpers/data-agent";
import { SimpleTree } from "react-arborist";
import { useThrottleEffect } from "./hooks/use-throttle-effect";

function App() {
  const { ref, height } = useResizeObserver<HTMLDivElement>({});
  const [inNode, setInNode] = useState<NTNode>();
  const [outNode, setOutNode] = useState<NTNode>();
  const treeRef = useRef<SimpleTree<NTNode> | null>(null);

  useThrottleEffect(
    (outNode) => {
      console.log("begin to save node");
      if (outNode) {
        saveNode(outNode);
        if (treeRef.current) {
          treeRef.current.update({
            id: outNode.id,
            changes: {
              ...outNode,
            },
          });
        }
      }
    },
    [outNode],
    3000
  );

  return (
    <div className="h-screen p-2 shadow">
      <div
        className="flex flex-row border-solid border rounded-md border-gray-300 m-0 h-full"
        ref={ref}
      >
        <div className="w-5/12 h-full bg-[#f0f0f0]">
          <NTTree height={height} setActivate={setInNode} treeRef={treeRef} />
        </div>

        <div className="w-7/12 h-full">
          {inNode ? (
            <NTEditor
              height={height}
              inNode={inNode}
              setOutNode={setOutNode}
              treeRef={treeRef}
            />
          ) : (
            <div>No node is selected</div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
