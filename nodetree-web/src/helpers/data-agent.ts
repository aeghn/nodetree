import { Asset, FullTimestampType, NTNode, NodeId } from "@/model";
import requests from "./request";

export const fetchAllNodes = async (): Promise<NTNode[]> => {
  return requests.get("api/fetch-all-nodes");
};

export const fetchNodesLike = async (query: string): Promise<NTNode[]> => {
  return requests.post("api/fetch-nodes", {
    selection: ["cont", "lim"],
    filter: {
      filter: "like",
      value: query,
    },
  });
};

export const fetchNodeContent = async (id: string): Promise<NTNode> => {
  return requests
    .post<NTNode[]>("api/fetch-nodes", {
      selection: ["cont"],
      filter: {
        filter: "id",
        value: id,
      },
    })
    .then((nodes) => {
      return nodes[0];
    });
};

export const moveNode = async (
  id: string,
  parentId: string | null,
  prevSlidingId: string | undefined
): Promise<NTNode[]> => {
  console.info("move node", id, parentId, prevSlidingId);
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

export const saveNode = async (
  node: NTNode,
  move: boolean = false
): Promise<NTNode> => {
  if (move) {
    const parsedNode: NTNode = await requests.post("api/insert-node", node);
    return parsedNode;
  } else {
    const parsedNode: NTNode = await requests.post(
      "api/insert-node-only",
      node
    );
    return parsedNode;
  }
};

export const updateNodeName = async (
  id: NodeId,
  name: string
): Promise<number> => {
  return await requests.post("api/update-node-name", { id, name });
};

export const deteleNode = async (id: NodeId): Promise<undefined> => {
  return await requests.post("api/delete-node", { id });
};

export const guessTime = async (
  input: string
): Promise<FullTimestampType[]> => {
  return await requests.post("api/guess-time", { input });
};
