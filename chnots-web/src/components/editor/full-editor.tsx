import { ContentParsedInfo, KNode, NodeId } from "@/model";
import { useCallback, useEffect, useState } from "react";
import { MininalEditor } from ".";
import {
  fetchNodeContent,
  setNodeReadonly,
  updateNodeContent,
} from "@/helpers/data-agent";
import { formatDistanceToNow } from "date-fns";
import { useAtom } from "jotai";
import { LuLoader, LuLock, LuTornado, LuUnlock } from "react-icons/lu";
import {
  getNodeIdAtom,
  tocSwitchAtom,
  setContentAtom,
  setTreeNodeIdAtom,
  readonlyAtom,
  setParsedInfoAtom,
  getInitialTime,
  contentChangedAtom,
  setNodeAtom,
  activeNodeAtom,
  useGetNodeNameAtom,
} from "@/state/explorer";
import Loading from "../element/loading";
import { debounce } from "@/helpers/debouce";
import { JSONContent } from "@tiptap/core";

const topbarElemClassName =
  "border border-solid border-gray-300 rounded-lg p-1 bg-slate-100 ml-2";

function onSave(
  setSaving: (v: boolean) => void,
  setParsedInfo: (v: ContentParsedInfo) => void,
  setContentChanged: (v: boolean) => void,
  node?: KNode,
  contentChanged?: boolean
) {
  if (contentChanged && node) {
    setSaving(true);
    updateNodeContent(node.id, node.content, node.version_time)
      .then((parsed_info) => {
        setParsedInfo(parsed_info);
      })
      .finally(() => {
        setSaving(false);
        setContentChanged(false);
      });
  }
}
const deferredSave = debounce(onSave, 600);

const EditorTopbarSaver = () => {
  const [saving, setSaving] = useState(false);

  const [currentNode] = useAtom(activeNodeAtom);

  const [, setParsedInfo] = useAtom(setParsedInfoAtom);
  const [contentChanged, setContentChanged] = useAtom(contentChangedAtom);

  useEffect(
    () =>
      deferredSave(
        setSaving,
        setParsedInfo,
        setContentChanged,
        currentNode,
        contentChanged
      ),
    [currentNode, contentChanged, setParsedInfo, setContentChanged]
  );

  return (
    <div className={topbarElemClassName}>
      {saving
        ? "Saving"
        : currentNode
        ? formatDistanceToNow(currentNode.version_time)
        : "Unknown Version Time"}
    </div>
  );
};

const EditorTopbarReadonlySwicther: React.FC<{ nodeId: NodeId }> = ({
  nodeId,
}) => {
  const [readonly, setReadonly] = useAtom(readonlyAtom);

  const toggleReadOnly = useCallback(() => {
    const r = !readonly;
    setNodeReadonly(nodeId, r).then((count) => {
      if (count > 0) {
        setReadonly(r);
      }
    });
  }, [readonly, setReadonly, nodeId]);

  return (
    <button onClick={toggleReadOnly} className={topbarElemClassName}>
      {readonly ? <LuLock size={22} /> : <LuUnlock size={22} />}
    </button>
  );
};

const EditorTopbar: React.FC<{ nodeId: NodeId }> = ({ nodeId }) => {
  const [tocSwitch, setTocSwitch] = useAtom(tocSwitchAtom);

  const [initial_time] = useAtom(getInitialTime);

  const toggleTocVisable = useCallback(() => {
    setTocSwitch(!tocSwitch);
  }, [setTocSwitch, tocSwitch]);

  return (
    <div className="h-[30] m-0 p-1 text-sm border-0 border-b border-gray-300 flex flex-row items-end justify-end">
      <div className={topbarElemClassName}>
        {initial_time
          ? formatDistanceToNow(initial_time)
          : "Unknown Initial Time"}
      </div>
      <EditorTopbarSaver />
      <button onClick={toggleTocVisable} className={topbarElemClassName}>
        <LuTornado size={22} color={tocSwitch ? "#888888" : undefined} />
      </button>
      <EditorTopbarReadonlySwicther nodeId={nodeId} />
    </div>
  );
};

/* const EditorToC: React.FC<{}> = () => {
  return <></>;
}; */

const FullEditor: React.FC<{
  height: number | undefined;
}> = ({ height }) => {
  console.log("full editor", height);
  const [nodeContent, setNodeContent] = useState<string>();

  const [readonly] = useAtom(readonlyAtom);
  const [nodeId] = useAtom(getNodeIdAtom);
  const [nodeName] = useAtom(useGetNodeNameAtom());
  const [, setContent] = useAtom(setContentAtom);
  const [, setTreeId] = useAtom(setTreeNodeIdAtom);
  const [, setNode] = useAtom(setNodeAtom);

  useEffect(() => {
    setNodeContent(undefined);
    if (nodeId) {
      fetchNodeContent(nodeId).then((retNode: KNode) => {
        let text = retNode.content ?? "";
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
        setNode(retNode);
        setNodeContent(text);
      });
    }
  }, [nodeId, setNode, setNodeContent]);

  const contentChangeCallback = useCallback((nodeContent: JSONContent) => {
/*     const firstElem = nodeContent.content?.at(0);
 */    /*     if (firstElem) {
      if (firstElem.type === "heading") {
        const firstCont = firstElem.content?.at(0);
        if (
          firstCont?.type === "text" &&
          firstCont.text &&
          firstCont.text?.length > 0
        ) {
          console.log("set node content");
          setNodeTitle(firstCont.text);
        } else {
          console.log("set node content 2");
          setNodeTitle("Untitled");
        }
      }
    }
 */
    setContent(JSON.stringify(nodeContent), new Date());
  }, []);

  const idChangeCallback = useCallback(
    (id: NodeId) => {
      setTreeId(id);
    },
    [setTreeId]
  );

  const defaultWithTitle = (title: string | undefined) => {
    return {
      type: "doc",
      content: [
        {
          type: "heading",
          attrs: {
            level: 1,
          },
          content: [
            {
              type: "text",
              text: title,
            },
          ],
        },
      ],
    };
  };

  let content;
  if (nodeContent) {
    content = JSON.parse(nodeContent);
  } else {
    content = defaultWithTitle(nodeName);
  }

  return nodeId && nodeContent != undefined ? (
    <div className="h-full">
      <EditorTopbar nodeId={nodeId} />
      <MininalEditor
        nodeId={nodeId}
        readonly={readonly}
        content={content}
        height={height ? height - 50 : undefined}
        contentChangeCallback={contentChangeCallback}
        idChangeCallback={idChangeCallback}
        alwaysWithTitle={true}
      />
    </div>
  ) : (
    <Loading customIcon={<LuLoader size={128} strokeWidth={1} />} message="" />
  );
};

export default FullEditor;
