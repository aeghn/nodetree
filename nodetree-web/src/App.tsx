import useResizeObserver from "use-resize-observer";
import { NTTree } from "./components/NTTree";
import { NTEditor } from "./components/NTEditor";
import { useEffect, useRef, useState } from "react";
import { NTNode } from "./model";
import { saveNode } from "./helpers/dataAgent";
import { SimpleTree } from "react-arborist";

function App() {
  const { ref, height } = useResizeObserver<HTMLDivElement>({});
  const [inNode, setInNode] = useState<NTNode>();
  const [outNode, setOutNode] = useState<NTNode>();
  const treeRef = useRef<SimpleTree<NTNode> | null>(null);

  useEffect(() => {
    if (outNode) {
      saveNode(outNode);
      if (treeRef.current) {
        treeRef.current.update({
          id: outNode.id,
          changes: {
            ...outNode
          }
        })
      }
    }
  }, [outNode])

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
          {
            inNode ?
              (<NTEditor height={height} inNode={inNode} setOutNode={setOutNode} />)
              : (<div>No node is selected</div>)
          }

        </div>
      </div>
    </div>
  );
}

export default App;
