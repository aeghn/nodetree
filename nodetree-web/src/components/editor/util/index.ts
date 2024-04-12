export const findTableAncestor = (node: Node | null): HTMLTableElement | null => {
    while (node !== null && node.nodeName !== "TABLE") {
      node = node.parentNode;
    }
    return node as HTMLTableElement;
  };