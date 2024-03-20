import { NTNode, NodeId } from "../model";

export const arrangeNodes = (nodes: NTNode[]): NTNode[] => {
  const nodeMap = new Map(nodes.map((node) => [node.id, node]));
  const nodeIdSet = new Set<NodeId>(nodes.map((node) => node.id));
  for (let n of nodes) {
    let p = nodeMap.get(n.parent_id);
    if (p != null) {
      if (!p.children) {
        p.children = [];
      }
      p.children.push(n);
      nodeIdSet.delete(n.id);
    }
  }

  let ra: NTNode[] = [];
  for (let nid of nodeIdSet) {
    const p = nodeMap.get(nid);
    if (p != null) {
      ra.push(p);
    }
  }

  console.info(ra);

  return ra;
};
