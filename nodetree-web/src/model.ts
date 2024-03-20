export type NodeId = string;

export type TodoStatus = string;

export interface MagicConstants {
  root_id: NodeId;
}

export interface NTTag {
  name: string;
  create_time: Date;
}

export interface NTNode {
  id: NodeId;
  version: number;

  is_current: boolean;
  delete_time?: Date;

  name: string;
  content: string;

  user: string;
  todo_status?: TodoStatus;

  tags?: NTTag[];

  parent_id: NodeId;
  prev_sliding_id?: NodeId;

  create_time: Date;
  first_version_time: Date;

  children?: NTNode[];
}
