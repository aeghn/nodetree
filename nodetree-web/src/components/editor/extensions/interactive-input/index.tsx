import { Content, Mark, getAttributes, mergeAttributes } from "@tiptap/core";
import Suggestion, { SuggestionProps } from "@tiptap/suggestion";
import { createSuggestionOptions } from "./suggestion/suggestion-options-builder";
import { fetchNodesLike } from "@/helpers/data-agent";
import { NTNode } from "@/model";
import { cx } from "class-variance-authority";



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
    addOptions() {
      return {
        HTMLAttributes: {
          target: '_blank',
          rel: 'noopener noreferrer nofollow',
          class: cx(
            "text-muted-foreground underline underline-offset-[3px] hover:text-primary transition-colors cursor-pointer",
          ),
          dataType: "backlink"
        },
      }
    },

    addAttributes() {
      return {
        href: {
          default: null,
        },
      }
    },
    parseHTML() {
      return [{ tag: 'a[href]:not([href *= "javascript:" i])' }]
    },

    renderHTML({ HTMLAttributes }: { HTMLAttributes: Record<string, any> }) {
      // False positive; we're explicitly checking for javascript: links to ignore them
      // eslint-disable-next-line no-script-url
      if (HTMLAttributes.href?.startsWith('javascript:')) {
        // strip out the href
        return ['a', mergeAttributes(this.options.HTMLAttributes, { ...HTMLAttributes, href: '' }), 0]
      }
      return ['a', mergeAttributes(this.options.HTMLAttributes, HTMLAttributes), 0]
    },
  },

  extendPMPlugins: [
    new Plugin({
      key: new PluginKey('handleClickBackLink'),
      props: {
        handleClick: (view, pos, event) => {
          if (event.button !== 0) {
            return false
          }

          let a = event.target as HTMLElement
          const els = []

          while (a.nodeName !== 'DIV') {
            els.push(a)
            a = a.parentNode as HTMLElement
            a.nextSibling
          }

          if (!els.find(value => value.nodeName === 'A')) {
            return false
          }

          const attrs = getAttributes(view.state, "href")
          const link = (event.target as HTMLLinkElement)

          const href = link?.href ?? attrs.href
          const target = link?.target ?? attrs.target

          if (link && href) {
            window.open(href, target)

            return true
          }

          return false
        },
      },
    })
  ]
});
