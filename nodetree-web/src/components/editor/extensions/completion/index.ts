import { mergeAttributes, Node } from "@tiptap/core";
import { DOMOutputSpec, Node as ProseMirrorNode } from "@tiptap/pm/model";
import Suggestion, { SuggestionOptions } from "@tiptap/suggestion";
import { createSuggestion } from "./completion-suggestion";

interface CompletionInnerOptions {
  HTMLAttributes: Record<string, unknown>;
  /** @deprecated use renderText and renderHTML instead  */
  renderLabel?: (props: {
    options: CompletionInnerOptions;
    node: ProseMirrorNode;
  }) => string;
  renderText?: (props: {
    options: CompletionInnerOptions;
    node: ProseMirrorNode;
  }) => string;
  renderHTML?: (props: {
    options: CompletionInnerOptions;
    node: ProseMirrorNode;
  }) => DOMOutputSpec;
  deleteTriggerWithBackspace: boolean;
  suggestion: Omit<SuggestionOptions, "editor">;
}

export interface CompletionOptions {
  triggerChar: string;
  pluginName: string;
  items: (query: { query: string }) => any[] | Promise<any[]>;
  selectItem: (props: any, index: number) => void | undefined;
  candidateRenderer: (item: any) => any | undefined;
}

export const createCompletion = (extend: CompletionOptions) => {
  const node = Node.create<CompletionInnerOptions>({
    name: extend.pluginName,

    addOptions() {
      return {
        HTMLAttributes: {},
        renderText({ node }) {
          return `${extend.triggerChar}${node.attrs.label ?? node.attrs.id}`;
        },
        deleteTriggerWithBackspace: false,
        renderHTML({ options, node }) {
          return [
            "span",
            mergeAttributes(this.HTMLAttributes, options.HTMLAttributes),
            `${extend.triggerChar}${node.attrs.label ?? node.attrs.id}`,
          ];
        },
        suggestion: createSuggestion(extend),
      };
    },

    group: "inline",

    inline: true,

    selectable: false,

    atom: true,

    addAttributes() {
      return {
        id: {
          default: null,
          parseHTML: (element) => element.getAttribute("data-id"),
          renderHTML: (attributes) => {
            if (!attributes.id) {
              return {};
            }

            return {
              "data-id": attributes.id,
            };
          },
        },

        label: {
          default: null,
          parseHTML: (element) => element.getAttribute("data-label"),
          renderHTML: (attributes) => {
            if (!attributes.label) {
              return {};
            }

            return {
              "data-label": attributes.label,
            };
          },
        },
      };
    },

    parseHTML() {
      return [
        {
          tag: `span[data-type="${this.name}"]`,
        },
      ];
    },

    renderHTML({ node, HTMLAttributes }) {
      if (this.options.renderLabel !== undefined) {
        console.warn(
          "renderLabel is deprecated use renderText and renderHTML instead"
        );
        return [
          "span",
          mergeAttributes(
            { "data-type": this.name },
            this.options.HTMLAttributes,
            HTMLAttributes
          ),
          this.options.renderLabel({
            options: this.options,
            node,
          }),
        ];
      }
      const mergedOptions = { ...this.options };

      mergedOptions.HTMLAttributes = mergeAttributes(
        { "data-type": this.name },
        this.options.HTMLAttributes,
        HTMLAttributes
      );
      const html = this.options.renderHTML({
        options: mergedOptions,
        node,
      });

      if (typeof html === "string") {
        return [
          "span",
          mergeAttributes(
            { "data-type": this.name },
            this.options.HTMLAttributes,
            HTMLAttributes
          ),
          html,
        ];
      }
      return html;
    },

    renderText({ node }) {
      if (this.options.renderLabel !== undefined) {
        console.warn(
          "renderLabel is deprecated use renderText and renderHTML instead"
        );
        return this.options.renderLabel({
          options: this.options,
          node,
        });
      }
      return this.options.renderText({
        options: this.options,
        node,
      });
    },

    addKeyboardShortcuts() {
      return {
        Backspace: () =>
          this.editor.commands.command(({ tr, state }) => {
            let isCompletion = false;
            const { selection } = state;
            const { empty, anchor } = selection;

            if (!empty) {
              return false;
            }

            state.doc.nodesBetween(anchor - 1, anchor, (node, pos) => {
              if (node.type.name === this.name) {
                isCompletion = true;
                tr.insertText(
                  this.options.deleteTriggerWithBackspace
                    ? ""
                    : extend.triggerChar || "",
                  pos,
                  pos + node.nodeSize
                );

                return false;
              }
            });

            return isCompletion;
          }),
      };
    },

    addProseMirrorPlugins() {
      return [
        Suggestion({
          editor: this.editor,
          ...this.options.suggestion,
        }),
      ];
    },
  });

  return node;
};
