import { Mention } from "@tiptap/extension-mention";
import { suggestion, CandidateConfig } from "./candidate-suggestion";


export const Candidate = (config: CandidateConfig) => {
  const { items, name, prefix, selectItem, candidateRenderer } = config;

  return Mention.extend({
    name: name,
  }).configure({
    HTMLAttributes: {
      class: name,
    },
    // @ts-ignore
    suggestion: suggestion(items, prefix, name, selectItem, candidateRenderer),
  });
};
