import useResizeObserver from "use-resize-observer";
import NTSidebar from "./components/NTSidebar";
import { NTTree } from "./components/NTTree";
import { NTEditor } from "./components/NTEditor";

function App() {
  const { ref, height } = useResizeObserver<HTMLDivElement>({});

  return (
    <div className="h-screen p-5">
      <div
        className="flex flex-row border-solid border-2 rounded-md border-gray-300 m-0 h-full"
        ref={ref}
      >
        <div className="w-[80px] h-full bg-[#c0c0c0] rounded-l">
          <NTSidebar />
        </div>
        <div className="w-[calc(50%-80px)] h-full bg-[#e0e0e0]">
          <NTTree height={height} />
        </div>

        <div className="w-6/12 h-full">
          <NTEditor height={height}/>
        </div>
      </div>
    </div>
  );
}

export default App;
