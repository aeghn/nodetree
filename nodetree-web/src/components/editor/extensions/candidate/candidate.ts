import { Mention } from "@tiptap/extension-mention";
import { suggestions } from "./candidate-suggestion";


interface CandidateConfig {
    items: ((query: {query: string}) => any[] | Promise<any[]>);
    name: string;
    prefix: string;
}

export const Candidate = (config: CandidateConfig) => {
    const { items, name, prefix } = config;

    return Mention.extend({
        name: name,
    }).configure({
        HTMLAttributes: {
            class: name,
        },
        // @ts-ignore
        suggestion: suggestions(items, prefix),
    })
}