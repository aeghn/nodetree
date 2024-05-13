import { ContentParsedInfo, NTNode, NodeId } from "@/model";
import { atom } from "jotai";

export const tocSwitchAtom = atom(false);
export const contentChangedAtom = atom(false);

export const currentNodeAtom = atom<NTNode | undefined>(undefined);

export const readonlyAtom = atom(
  (get) => {
    const node = get(currentNodeAtom);
    return node?.readonly;
  },

  (get, set, readonly: boolean) => {
    const node = get(currentNodeAtom);
    if (node) {
      set(currentNodeAtom, { ...node, readonly });
    }
  }
);

export const getNodeIdAtom = atom((get) => {
  const node = get(currentNodeAtom);
  return node?.id;
});

export const getInitialTime = atom((get) => {
  const node = get(currentNodeAtom);
  return node?.initial_time;
});

export const setNodeAtom = atom(null, (_get, set, node: NTNode) => {
  set(currentNodeAtom, node);
});

export const setContentAtom = atom(
  null,
  (get, set, content: string, version_time: Date) => {
    const node = get(currentNodeAtom);
    if (node && node.content !== content) {
      set(currentNodeAtom, { ...node, content, version_time });
      set(contentChangedAtom, true);
    }
  }
);

export const setTreeNodeIdAtom = atom(null, (_get, set, id: NodeId) => {
  set(treeNodeIdAtom, id);
});

export const getVersionTimeAtom = atom((get) => {
  return get(currentNodeAtom)?.version_time;
});

export const getIdAndContentAtom = atom((get) => {
  const node = get(currentNodeAtom);
  return {
    id: node?.id,
    content: node?.content,
    version_time: node?.version_time,
  };
});

export const getContentAtom = atom((get) => {
  const node = get(currentNodeAtom);
  return node?.content;
});

export const setParsedInfoAtom = atom(
  null,
  (get, set, parsed_info: ContentParsedInfo) => {
    const node = get(currentNodeAtom);
    if (node) {
      set(currentNodeAtom, { ...node, parsed_info });
    }
  }
);

export const treeNodeIdAtom = atom<NodeId | undefined>(undefined);
