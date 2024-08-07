import { MAGIC_EMPTY } from "./constant";

export type NodeId = string;

export const strToNodeId = (key: string | undefined | null): NodeId => {
  if (key === null || key === undefined) {
    return MAGIC_EMPTY;
  } else {
    return key;
  }
};

export type TodoStatus = string;

export interface MagicConstants {
  root_id: NodeId;
}

export interface KTag {
  name: string;
  create_time: Date;
}

export enum NodeType {
  TiptapV1 = "tiptap/v1",
}

export interface KNode {
  id: NodeId;

  delete_time?: Date;

  name: string;
  content: string;
  node_type: NodeType;

  readonly: boolean;

  domain: string;
  parsed_info: ContentParsedInfo;

  parent_id: NodeId;
  prev_sliding_id: NodeId;

  version_time: Date;
  initial_time: Date;

  children?: KNode[];
}

export interface ContentParsedInfo {
  todo_status?: string;
  tags?: KTag[];
}

export interface Asset {
  id: string;

  domain: string | undefined;
  ori_file_name: string;

  content_type: string;

  create_time: Date;
}

export interface Toent {
  id: string;
  input: string;
  event: string;
}
