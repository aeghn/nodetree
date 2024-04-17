import { Mark, mergeAttributes } from "@tiptap/core";
import { createSuggestionOptions } from "./suggestion/suggestion-options-builder";
import { NTSuggestion } from "./suggestion/suggestion";

export interface InteractiveInputOptions {
  HTMLAttributes?: Record<string, any>;
  pluginName: string;
  items: (query: { query: string }) => any[] | Promise<any[]>;
  selectItem: (props: any, index: number) => void | undefined;
  itemRenderer: (item: any) => any | undefined;
  prefixChar: string;
}

const createInteractiveInput = (options: InteractiveInputOptions) => {
  const pluginName = options.pluginName;

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

    addProseMirrorPlugins() {
      return [
        NTSuggestion({
          editor: this.editor,
          ...createSuggestionOptions(options),
        }),
      ];
    },
  });
};

export const Hashtag = createInteractiveInput({
  prefixChar: "#",
  pluginName: "hashtag",
  items: (query: { query: string }) => {
    return query ? [query.query] : [];
  },
  selectItem: (props: any, index: number) => {
    const item: string = props.items[index];
    if (item) {
      props.command({ id: item, label: item });
    }
  },
  itemRenderer: (item: string) => {
    return (
      <div>
        <div>{item}</div>
      </div>
    );
  },
});

export const Reminder = createInteractiveInput({
  prefixChar: "<",
  items: (query: { query: string }) => {
    return query ? [query.query] : [];
  },
  selectItem: (props: any, index: number) => {
    const item: string = props.items[index];
    if (item) {
      props.command({ id: item, label: item });
    }
  },
  itemRenderer: (item: string) => {
    return <div>{item}</div>;
  },
  pluginName: "reminder",
});
