import useResizeObserver from "use-resize-observer";
import { NTTree } from "./components/NTTree";
import { NTEditor } from "./components/NTEditor";
import { useEffect, useState } from "react";
import { NTNode } from "./model";
import { saveNode } from "./helpers/dataAgent";

function App() {
  const { ref, height } = useResizeObserver<HTMLDivElement>({});
  const [inContent, setInContent] = useState<string | undefined>();
  const [outContent, setOutContent] = useState<string | undefined>();
  const [currentNode, setCurrentNode] = useState<NTNode | undefined>(undefined);

  const setCurrent = (node: NTNode) => {
    setInContent(node.content);
    setCurrentNode(node)
  };

  useEffect(() => {
    if (currentNode && outContent)
      saveNode({ ...currentNode, content: outContent })
  }, [outContent])

  useEffect(() => {
    if (currentNode)
      saveNode(currentNode)
  }, [currentNode]);

  return (
    <div className="h-screen p-2 shadow">
      <div
        className="flex flex-row border-solid border rounded-md border-gray-300 m-0 h-full"
        ref={ref}
      >
        <div className="w-5/12 h-full bg-[#f0f0f0]">
          <NTTree height={height} setActivate={setCurrent} />
        </div>

        <div className="w-7/12 h-full">
          {inContent != undefined ? (
            <NTEditor height={height} content={inContent} setOutContent={setOutContent} />
          ) : (
            <div> No node is selected!</div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
