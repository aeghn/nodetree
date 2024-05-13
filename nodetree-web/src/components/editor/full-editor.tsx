import { NTNode, NodeId } from "@/model";
import { useCallback, useEffect, useState } from "react";
import { MininalEditor } from ".";
import {
  fetchNodeContent,
  setNodeReadonly,
  updateNodeContent,
} from "@/helpers/data-agent";
import { formatDistanceToNow } from "date-fns";
import { useAtom } from "jotai";
import {
  LuLoader,
  LuLock,
  LuTornado,
  LuUnlock,
} from "react-icons/lu";
import {
  getNodeIdAtom,
  tocSwitchAtom,
  setContentAtom,
  setTreeNodeIdAtom,
  readonlyAtom,
  getVersionTimeAtom,
  setParsedInfoAtom,
  getInitialTime,
  contentChangedAtom,
  getContentAtom,
  setNodeAtom,
} from "@/state/explorer";
import Loading from "../element/loading";
import { useThrottleEffect } from "@/hooks/use-throttle-effect";

const topbarElemClassName =
  "border border-solid border-gray-300 rounded-lg p-1 bg-slate-100 ml-2";

const EditorTopbarSaver: React.FC<{ nodeId: NodeId }> = ({ nodeId }) => {
  const [saving, setSaving] = useState(false);
  const [version_time] = useAtom(getVersionTimeAtom);
  const [content] = useAtom(getContentAtom);
  const [, setParsedInfo] = useAtom(setParsedInfoAtom);
  const [contentChanged, setContentChanged] = useAtom(contentChangedAtom);

  useThrottleEffect(
    (version_time?: Date, content?: string, contentChanged?: boolean) => {
      if (contentChanged && nodeId && content !== undefined && version_time) {
        setSaving(true);
        updateNodeContent(nodeId, content, version_time)
          .then((parsed_info) => {
            setParsedInfo(parsed_info);
          })
          .finally(() => {
            setSaving(false);
            setContentChanged(false);
          });
      }
    },
    [version_time, content, contentChanged],
    1000
  );

  return (
    <div className={topbarElemClassName}>
      {saving
        ? "Saving"
        : version_time
        ? formatDistanceToNow(version_time)
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
      <EditorTopbarSaver nodeId={nodeId} />
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
  const [nodeContent, setNodeContent] = useState<string>();

  const [readonly] = useAtom(readonlyAtom);
  const [nodeId] = useAtom(getNodeIdAtom);
  const [, setContent] = useAtom(setContentAtom);
  const [, setTreeId] = useAtom(setTreeNodeIdAtom);
  const [, setNode] = useAtom(setNodeAtom);

  useEffect(() => {
    setNodeContent(undefined);
    if (nodeId) {
      fetchNodeContent(nodeId).then((retNode: NTNode) => {
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

  const contentChangeCallback = useCallback((nodeContent: string) => {
    setContent(nodeContent, new Date());
  }, []);

  const idChangeCallback = useCallback(
    (id: NodeId) => {
      setTreeId(id);
    },
    [setTreeId]
  );

  return nodeId && nodeContent != undefined ? (
    <div className="h-full">
      <EditorTopbar nodeId={nodeId} />
      <MininalEditor
        nodeId={nodeId}
        readonly={readonly}
        content={nodeContent ?? ""}
        height={height ? height - 50 : undefined}
        contentChangeCallback={contentChangeCallback}
        idChangeCallback={idChangeCallback}
      />
    </div>
  ) : (
    <Loading customIcon={<LuLoader size={128} strokeWidth={1} />} message="" />
  );
};

export default FullEditor;
