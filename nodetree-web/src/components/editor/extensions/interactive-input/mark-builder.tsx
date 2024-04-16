import { Mark, mergeAttributes } from "@tiptap/core";
import { CompletionOptions } from "./suggestion/completion";
import { NTSuggestion } from "./suggestion/suggestion";
import { createSuggestion } from "./suggestion/completion-suggestion";
import { fetchNodesLike } from "@/helpers/data-agent";
import { NTNode } from "@/model";

export interface InteractiveInputOptions {
  HTMLAttributes?: Record<string, any>;
  prefix: string;
  suffix: string;
  completionOptions: CompletionOptions;
}

export const createInteractiveInput = (options: InteractiveInputOptions) => {
  const suggestion = createSuggestion(options.completionOptions);
  const pluginName = options.completionOptions.pluginName;

  return Mark.create<InteractiveInputOptions>({
    name: pluginName,

    parseHTML() {
      return [
        {
          tag: `span[data-type="${pluginName}"]`,
        },
      ];
    },

    renderHTML({ HTMLAttributes }) {
      return [
        "span",
        mergeAttributes(
          { "data-type": pluginName },
          { class: pluginName },
          options.HTMLAttributes ?? {},
          HTMLAttributes
        ),
        0,
      ];
    },

    /*     addProseMirrorPlugins() {
          return [
            NTSuggestion({
              editor: this.editor,
              ...suggestion,
            }),
          ];
        }, */
  });
};

export const Hashtag = createInteractiveInput({
  prefix: "#",
  suffix: "#",
  completionOptions: {
    pluginName: "backlink",
    items: (query: { query: string; }) => {
      return fetchNodesLike(query.query);

    },
    selectItem: (props: any, index: number) => {
      const item: NTNode = props.items[index];
      console.log(props);
      if (item) {
        props.command({ id: item.id, label: item.name });
      }
    },
    completionItemRenderer: (item: NTNode) => {
      return (
        <div>
          <div>{item.name}</div>
          <br />
          <div>{item.content}</div>
        </div>
      );
    },
    triggerChar: ""
  }
});

export const Reminder = createInteractiveInput({
  prefix: "<",
  suffix: ">",
  completionOptions: {
    items: (query: { query: string; }) => {
      console.log(query);
      return query ? [query.query] : [];
    },
    selectItem: (props: any, index: number) => {
      const item: string = props.items[index];
      console.log(props);
      if (item) {
        props.command({ id: item, label: item });
      }
    },
    completionItemRenderer: (item: string) => {
      return <div>{item}</div>;
    },
    triggerChar: "",
    pluginName: "reminder"
  },
});
