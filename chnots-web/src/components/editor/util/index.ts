import { Selection } from "@tiptap/pm/state";

export const findTableAncestor = (
  node: Node | null
): HTMLTableElement | null => {
  while (node !== null && node.nodeName !== "TABLE") {
    node = node.parentNode;
  }
  return node as HTMLTableElement;
};

// Helper function to find the parent node of a specific type
export function findParentNodeOfType(selection: Selection, typeName: string) {
  let depth = selection.$anchor.depth;
  while (depth > 0) {
    const node = selection.$anchor.node(depth);
    if (node.type.name === typeName) {
      return { node, pos: selection.$anchor.start(depth) - 1 };
    }
    depth--;
  }
  return null;
}

