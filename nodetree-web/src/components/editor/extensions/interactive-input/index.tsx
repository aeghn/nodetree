import { Content, Mark, getAttributes, mergeAttributes } from "@tiptap/core";
import Suggestion, { SuggestionProps } from "@tiptap/suggestion";
import { createSuggestionOptions } from "./suggestion/suggestion-options-builder";
import { fetchNodesLike } from "@/helpers/data-agent";
import { NTNode } from "@/model";



import { Plugin, PluginKey } from '@tiptap/pm/state'


export interface InteractiveInputOptions<E> {
  HTMLAttributes?: Record<string, any>;
  pluginName: string;
  items: (params: { query: string }) => E[] | Promise<E[]>;
  itemRenderer: (item: E) => any | undefined;
  selectItem?: (props: SuggestionProps<E>, index: number) => void | undefined;
  elemBuilder: (props: E) => Content;
  prefixChar: string;
  extendOptions?: {},
  extendPMPlugins?: Plugin[]
}


const createInteractiveInput = <E,>(options: InteractiveInputOptions<E>) => {
  const pluginName = options.pluginName;

  return Mark.create<InteractiveInputOptions<E>>({
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
      const plugins = options.extendPMPlugins ?? [];
      plugins.push(
        Suggestion({
          editor: this.editor,
          ...createSuggestionOptions(options),
        })
      );
      return plugins;
    },

    ...options.extendOptions
  });
};

export const Hashtag = createInteractiveInput<string>({
  prefixChar: "#",
  pluginName: "hashtag",

  items: (param: { query: string; }) => {
    return param ? [param.query] : [];
  },
  itemRenderer: (item: string) => {
    return <div>{item}</div>;
  },
  elemBuilder: function (item: string): Content {
    return [
      {
        type: "text",
        text: `${this.prefixChar}${item}`,
        marks: [
          {
            type: this.pluginName,
          },
        ],
      },
      {
        type: "text",
        text: " ",
      },
    ]
  }
});

export const Reminder = createInteractiveInput<string>({
  prefixChar: "<",
  pluginName: "reminder",

  items: (param: { query: string; }) => {
    return param ? [param.query] : [];
  },

  itemRenderer: (item: string) => {
    return <div>{item}</div>;
  },

  elemBuilder: function (item: string): Content {
    return [
      {
        type: "text",
        text: `${this.prefixChar}${item}`,
        marks: [
          {
            type: this.pluginName,
          },
        ],
      },
      {
        type: "text",
        text: " ",
      },
    ]
  }
});

export const Backlink = createInteractiveInput<NTNode>({
  prefixChar: "&",
  pluginName: "backlink",

  items: (param: { query: string; }) => {
    return param ? fetchNodesLike(param.query) : [];
  },
  itemRenderer: (node: NTNode) => {
    return <div>{node.name}</div>;
  },

  elemBuilder: function (item: NTNode): Content {
    return [
      {
        type: "text",
        text: `${this.prefixChar}${item.name}`,

        marks: [
          {
            type: this.pluginName,
            attrs: {
              "href": item.id,
            }
          },
        ],
      },
      {
        type: "text",
        text: " ",
      },
    ]
  },

  extendOptions: {
    addAttributes() {
      return {
        href: {
          default: null,
        },
      }
    }
  }
});

