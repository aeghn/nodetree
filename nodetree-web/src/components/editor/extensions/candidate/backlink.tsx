import { Mention } from "@tiptap/extension-mention";
import { CandidateConfig, suggestion } from "./candidate-suggestion";
import { fetchNodesLike } from "@/helpers/data-agent";
import { NTNode } from "@/model";

export const BackLink = () => {
  const config: CandidateConfig = {
    name: "backlink",
    prefix: "&",
    items: (query: { query: string }) => {
      return fetchNodesLike(query.query);
    },
    selectItem: (props: any, index: number) => {
      const item: NTNode = props.items[index];

      if (item) {
        props.command({ id: item.id, label: item.name });
      }
    },
    candidateRenderer: (node: NTNode) => {
      return (
        <div>
          <div>{node.name}</div> <div>{node.content}</div>
        </div>
      );
    },
  };

  return Mention.extend({
    name: config.name,
  }).configure({
    HTMLAttributes: {
      class: config.name,
    },
    // @ts-ignore
    suggestion: suggestion(config),
  });
};
