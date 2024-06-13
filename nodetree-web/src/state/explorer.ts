import { ContentParsedInfo, NTNode, NodeId } from "@/model";
import { atom } from "jotai";
import { selectAtom } from "jotai/utils";
import { useCallback } from "react";

export const tocSwitchAtom = atom(false);
export const contentChangedAtom = atom(false);

export const activeNodeAtom = atom<NTNode | undefined>(undefined);

export const treeNodeIdAtom = atom<NodeId | undefined>(undefined);

export const readonlyAtom = atom(
  (get) => {
    const node = get(activeNodeAtom);
    return node?.readonly;
  },

  (get, set, readonly: boolean) => {
    const node = get(activeNodeAtom);
    if (node) {
      set(activeNodeAtom, { ...node, readonly });
    }
  }
);

export const getNodeIdAtom = atom((get) => {
  const node = get(activeNodeAtom);
  return node?.id;
});

export const getInitialTime = atom((get) => {
  const node = get(activeNodeAtom);
  return node?.initial_time;
});

export const setNodeAtom = atom(null, (_get, set, node: NTNode) => {
  set(activeNodeAtom, node);
});

export const setContentAtom = atom(
  null,
  (get, set, content: string, version_time: Date) => {
    const activeNode = get(activeNodeAtom);
    if (activeNode && activeNode.content !== content) {
      set(activeNodeAtom, { ...activeNode, content, version_time });
      set(contentChangedAtom, true);
    }
  }
);

export const setTreeNodeIdAtom = atom(null, (_get, set, id: NodeId) => {
  set(treeNodeIdAtom, id);
});

export const setNodeNameAtom = atom(null, (get, set, name: string) => {
  const activeNode = get(activeNodeAtom);
  if (activeNode && activeNode.name !== name) {
    set(activeNodeAtom, { ...activeNode, name });
  }
});

export const useGetNodeIdNameAtom = () =>
  selectAtom(
    activeNodeAtom,
    useCallback((activeNode) => {
      return {
        id: activeNode?.id,
        name: activeNode?.name,
      };
    }, [])
  );

export const useGetNodeNameAtom = () =>
  selectAtom(
    activeNodeAtom,
    useCallback((activeNode) => activeNode?.name, [])
  );

export const getContentAtom = atom((get) => {
  const activeNode = get(activeNodeAtom);
  return activeNode?.content;
});

export const setParsedInfoAtom = atom(
  null,
  (get, set, parsed_info: ContentParsedInfo) => {
    const activeNode = get(activeNodeAtom);
    if (activeNode) {
      set(activeNodeAtom, { ...activeNode, parsed_info });
    }
  }
);
