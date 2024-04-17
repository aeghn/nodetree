import { ReactRenderer } from "@tiptap/react";
import tippy, { Instance, Props } from "tippy.js";

import { SuggestionList } from "./suggestion-list";
import { PluginKey } from "@tiptap/pm/state";

import { SuggestionProps } from "@tiptap/suggestion";
import { Editor } from "@tiptap/core";
import { InteractiveInputOptions } from "../mark-builder";

export const createSuggestionOptions = (options: InteractiveInputOptions) => {
  const { items, pluginName, selectItem, itemRenderer, prefixChar } = options;
  return {
    char: prefixChar,
    items: items,
    // https://blog.projectan.cn/vue/tiptap-multiple-mention-instances/
    pluginKey: new PluginKey(`sug-${pluginName}`),

    // https://github.com/ueberdosis/tiptap/issues/823
    allowSpaces: true,

    command: ({
      editor,
      range,
      props,
    }: {
      editor: Editor;
      range: any;
      props: any;
    }) => {
      // increase range.to by one when the next node is of type "text"
      // and starts with a space character
      const nodeAfter = editor.view.state.selection.$to.nodeAfter;
      const overrideSpace = nodeAfter?.text?.startsWith(" ");

      if (overrideSpace) {
        range.to += 1;
      }

      editor
        .chain()
        .focus()
        .insertContentAt(range, [
          {
            type: options.pluginName,
            attrs: props,
          },
          {
            type: "text",
            text: " ",
          },
        ])
        .run();

      window.getSelection()?.collapseToEnd();
    },

    render: () => {
      let reactRenderer: ReactRenderer;
      let popup: Instance<Props>[];

      return {
        onStart: (props: SuggestionProps) => {
          const propsExtend = {
            ...props,
            selectItem,
            itemRenderer,
          };
          reactRenderer = new ReactRenderer(SuggestionList, {
            props: propsExtend,
            editor: props.editor,
          });

          if (!props.clientRect) {
            return;
          }

          // @ts-ignore
          popup = tippy("body", {
            getReferenceClientRect: props.clientRect,
            appendTo: () => document.body,
            content: reactRenderer.element,
            showOnCreate: true,
            interactive: true,
            trigger: "manual",
            placement: "bottom-start",
          });
        },

        onUpdate(props: Record<string, any>) {
          reactRenderer.updateProps(props);

          if (!props.clientRect) {
            return;
          }

          popup[0].setProps({
            getReferenceClientRect: props.clientRect,
          });
        },

        onKeyDown(props: any) {
          if (props.event.key === "Escape") {
            popup[0].hide();

            return true;
          }

          // @ts-ignore
          return reactRenderer.ref?.onKeyDown(props);
        },

        onExit() {
          popup[0].destroy();
          reactRenderer.destroy();
        },
      };
    },
  };
};
