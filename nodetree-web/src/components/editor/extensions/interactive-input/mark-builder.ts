import { Mark, mergeAttributes } from "@tiptap/core";

export interface InteractiveInputOptions {
  HTMLAttributes?: Record<string, any>;
  pluginName: string;
  prefix: string;
  suffix: string;
}

export const createInteractiveInput = (options: InteractiveInputOptions) => {
  return Mark.create<InteractiveInputOptions>({
    name: options.pluginName,

    parseHTML() {
      return [
        {
          tag: `span[data-type="${options.pluginName}"]`,
        },
      ];
    },

    renderHTML({ HTMLAttributes }) {
      return [
        "span",
        mergeAttributes(
          { "data-type": options.pluginName },
          { class: options.pluginName },
          options.HTMLAttributes ?? {},
          HTMLAttributes
        ),
        0,
      ];
    },
  });
};

export const Hashtag = createInteractiveInput({
  pluginName: "hashtag",
  prefix: "#",
  suffix: "#",
});

export const Reminder = createInteractiveInput({
  pluginName: "reminder",
  prefix: "<",
  suffix: ">",
});
