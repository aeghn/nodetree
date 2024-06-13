import { Content, Mark, mergeAttributes } from "@tiptap/core";
import Suggestion, { SuggestionProps } from "@tiptap/suggestion";
import { createSuggestionOptions } from "./suggestion/suggestion-options-builder";
import { fetchNodesLike, guessTime } from "@/helpers/data-agent";
import { KNode, Toent } from "@/model";
import { cx } from "class-variance-authority";

import { Plugin, PluginKey } from "@tiptap/pm/state";

export interface InteractiveInputOptions<E> {
  HTMLAttributes?: Record<string, any>;
  pluginName: string;
  items: (params: { query: string }) => E[] | Promise<E[]>;
  itemRenderer: (item: E) => any | undefined;
  selectItem?: (props: SuggestionProps<E>, index: number) => void | undefined;
  elemBuilder: (props: E) => Content;
  prefixChar: string;
  extendOptions?: {};
  extendPMPlugins?: Plugin[];
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

    ...options.extendOptions,
  });
};

export const Hashtag = createInteractiveInput<string>({
  prefixChar: "#",
  pluginName: "hashtag",

  items: (param: { query: string }) => {
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
    ];
  },
});

export const Reminder = createInteractiveInput<Toent>({
  prefixChar: "%",
  pluginName: "reminder",

  items: (param: { query: string }) => {
    return param ? guessTime(param.query) : [];
  },

  itemRenderer: (item: Toent) => {
    return <div>{item.event}</div>;
  },

  elemBuilder: function (item: Toent): Content {
    return [
      {
        type: "text",
        text: `${this.prefixChar}${item.event}`,
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
    ];
  },
});

export const Backlink = (idChangeCallback: (href: string) => void) =>
  createInteractiveInput<KNode>({
    prefixChar: "&",
    pluginName: "backlink",

    items: (param: { query: string }) => {
      return param ? fetchNodesLike(param.query) : [];
    },
    itemRenderer: (node: KNode) => {
      return <div>{node.name}</div>;
    },

    elemBuilder: function (item: KNode): Content {
      return [
        {
          type: "text",
          text: `${this.prefixChar}${item.name}`,

          marks: [
            {
              type: this.pluginName,
              attrs: {
                chnothref: item.id,
              },
            },
          ],
        },
        {
          type: "text",
          text: " ",
        },
      ];
    },

    extendOptions: {
      addOptions() {
        return {
          HTMLAttributes: {
            target: "_blank",
            rel: "noopener noreferrer nofollow",
            class: cx(
              "text-muted-foreground underline underline-offset-[3px] hover:text-primary transition-colors cursor-pointer"
            ),
            dataType: "backlink",
          },
        };
      },

      addAttributes() {
        return {
          chnothref: {
            default: null,
          },
        };
      },
      parseHTML() {
        return [{ tag: 'a[chnothref]:not([chnothref *= "javascript:" i])' }];
      },

      renderHTML({ HTMLAttributes }: { HTMLAttributes: Record<string, any> }) {
        // False positive; we're explicitly checking for javascript: links to ignore them
        // eslint-disable-next-line no-script-url
        if (HTMLAttributes.chnothref?.startsWith("javascript:")) {
          // strip out the href
          return [
            "a",
            // @ts-ignore
            mergeAttributes(this.options.HTMLAttributes, {
              ...HTMLAttributes,
              chnothref: "",
            }),
            0,
          ];
        }
        return [
          "a",
          // @ts-ignore
          mergeAttributes(this.options.HTMLAttributes, HTMLAttributes),
          0,
        ];
      },
    },

    extendPMPlugins: [
      new Plugin({
        key: new PluginKey("handleClickBackLink"),
        props: {
          handleClick: (view, pos, event) => {
            event.preventDefault();

            const node = view.state.doc.nodeAt(pos);

            const marks = node?.marks.filter((e) => e.type.name === "backlink");
            if (marks && marks.length > 0) {
              const nodeId = marks[0].attrs["chnothref"];
              idChangeCallback(nodeId);
            }

            return true;
          },
        },
      }),
    ],
  });
