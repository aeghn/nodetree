import { KNode, NodeId } from "@/model";

export const arrangeNodes = (nodes: KNode[]): KNode[] => {
  const nodeMap = new Map(nodes.map((node) => [node.id, node]));
  const nodeIdSet = new Set<NodeId>(nodes.map((node) => node.id));
  for (const n of nodes) {
    n.children = [];
  }

  for (const n of nodes) {
    if (!n.parent_id) continue;

    const p = nodeMap.get(n.parent_id);
    if (p != null) {
      p.children?.push(n);
      nodeIdSet.delete(n.id);
    }
  }

  const ra: KNode[] = [];
  for (const nid of nodeIdSet) {
    const p = nodeMap.get(nid);
    if (p != null) {
      ra.push(p);
    }
  }

  return ra;
};
