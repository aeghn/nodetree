import { NTNode } from "../model";
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
