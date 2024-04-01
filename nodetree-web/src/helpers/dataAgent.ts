import { Asset, NTNode } from "../model";
import requests from "./request";

export const fetchAllNodes = async (): Promise<NTNode[]> => {
  console.info("begin to fetch all nodes");
  return requests.get("api/fetch-all-nodes");
};

export const moveNode = async (
  id: string,
  parentId: string | null,
  prevSlidingId: string | undefined
): Promise<NTNode[]> => {
  console.info("move node");
  return requests.post("api/move-node", {
    id: id,
    parent_id: parentId,
    prev_sliding_id: prevSlidingId,
  });
};

export const uploadImage = async (file: File): Promise<Asset> => {
  const data = new FormData();
  data.append("file", file);

  const assets: Asset[] = await requests.post("api/upload", data);
  return assets[0];
};

export const saveNode = async (node: NTNode, move: boolean = false): Promise<NTNode> => {
  if (move) {
    const parsedNode: NTNode = await requests.post("api/insert-node", node);
    return parsedNode;
  } else {
    const parsedNode: NTNode = await requests.post("api/insert-node-only", node);
    return parsedNode;
  }
};
