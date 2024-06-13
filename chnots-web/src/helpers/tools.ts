import { v4 as uuidv4 } from "uuid";

export function generateId(): string {
  return uuidv4().replace("-", "");
}

export function escapeForRegEx(string: string): string {
  return string.replace(/[-/\\^$*+?.()|[\]{}]/g, "\\$&");
}
