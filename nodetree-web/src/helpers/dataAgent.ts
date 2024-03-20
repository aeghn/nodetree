import { NTNode } from "../model";
import requests from "./request";

export const fetchAllNodes = async (): Promise<NTNode[]> => {
  console.info("begin to fetch all nodes");
  return requests.get("api/fetch-all-nodes");
};
