import { Asset, Toent, KNode, NodeId, ContentParsedInfo } from "@/model";
import requests from "./request";

export const fetchAllNodes = async (): Promise<KNode[]> => {
  return requests.get("api/fetch-all-nodes");
};

export const fetchNodesLike = async (query: string): Promise<KNode[]> => {
  return requests.post("api/fetch-nodes", {
    selection: ["cont", "lim"],
    filter: {
      filter: "like",
      value: query,
    },
  });
};

export const fetchNodeContent = async (id: string): Promise<KNode> => {
  return requests
    .post<KNode[]>("api/fetch-nodes", {
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
  id: NodeId,
  parentId: NodeId,
  prevSlidingId: NodeId
): Promise<unknown> => {
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
  node: KNode,
  move: boolean = false
): Promise<ContentParsedInfo> => {
  if (move) {
    const parsedNode: ContentParsedInfo = await requests.post(
      "api/insert-node",
      node
    );
    return parsedNode;
  } else {
    const parsedNode: ContentParsedInfo = await requests.post(
      "api/insert-node-only",
      node
    );
    return parsedNode;
  }
};

export const updateNodeContent = async (
  id: NodeId,
  content: string,
  version_time: Date
): Promise<ContentParsedInfo> => {
  const parsedNode: ContentParsedInfo = await requests.post(
    "api/update-node-content",
    {
      id,
      content,
      version_time,
    }
  );
  return parsedNode;
};

export const updateNodeName = async (
  id: NodeId,
  name: string
): Promise<number> => {
  return await requests.post("api/update-node-name", { id, name });
};

export const setNodeReadonly = async (
  id: NodeId,
  readonly: boolean
): Promise<number> => {
  return await requests.post("api/update-node-readonly", {
    id,
    readonly,
  });
};

export const deteleNode = async (id: NodeId): Promise<undefined> => {
  return await requests.post("api/delete-node", { id });
};

export const guessTime = async (input: string): Promise<Toent[]> => {
  return await requests.post("api/guess-toent", { input });
};
