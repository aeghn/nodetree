import useResizeObserver from "use-resize-observer";
import { NTTree } from "./components/NTTree";
import { NTEditor } from "./components/NTEditor";
import { useState } from "react";
import { NTNode } from "./model";

function App() {
  const { ref, height } = useResizeObserver<HTMLDivElement>({});
  const [currentNode, setCurrentNode] = useState<NTNode | undefined>(undefined);

  return (
    <div className="h-screen p-2 shadow">
      <div
        className="flex flex-row border-solid border rounded-md border-gray-300 m-0 h-full"
        ref={ref}
      >
        <div className="w-5/12 h-full bg-[#f0f0f0]">
          <NTTree height={height} setActivate={setCurrentNode} />
        </div>

        <div className="w-7/12 h-full">
          {currentNode != undefined ? (
            <NTEditor height={height} node={currentNode} />
          ) : (
            <div> No node is selected!</div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
