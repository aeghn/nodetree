import useResizeObserver from "use-resize-observer";
import NTTree from "@/components/tree";
import { useEffect, useState } from "react";
import { NTNode } from "@/model";
import { fetchAllNodes } from "@/helpers/data-agent";
import { LuTreePine } from "react-icons/lu";
import Loading from "../element/loading";
import FullEditor from "../editor/full-editor";

function Explorer() {
  const { ref: heightRef, height } = useResizeObserver<HTMLDivElement>({});

  const [treeDataList, setTreeDataList] = useState<NTNode[]>();


  useEffect(() => {
    try {
      fetchAllNodes().then((nodes) => {
        setTreeDataList(nodes);
        console.log("Loaded all nodes");
      });
    } catch (error) {
      console.error(`Unable get all nodes ${error}`);
    }
  }, []);

  return (
    <div className="h-screen p-2 shadow bg-[#f5f5f5]">
      <div className="flex flex-row m-0 h-full content-center" ref={heightRef}>
        <div className="w-3/12 m-0 h-full pr-4">
          {treeDataList ? (
            <NTTree
              height={height}
              treeDataList={treeDataList}
            />
          ) : (
            <Loading
              customIcon={<LuTreePine size={128} strokeWidth={1} />}
              message="Loading tree"
            />
          )}
        </div>

        <div className="w-9/12 h-full border-solid border rounded-md border-gray-300 bg-white shadow-lg">
          <FullEditor
            height={height}
          />
        </div>
      </div>
    </div>
  );
}

export default Explorer;
