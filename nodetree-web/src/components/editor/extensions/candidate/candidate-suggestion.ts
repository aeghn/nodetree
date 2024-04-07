import { ReactRenderer } from "@tiptap/react";
import tippy, { Instance, Props, GetReferenceClientRect } from "tippy.js";
import { Editor } from "@tiptap/react";

import { MentionList } from "./candidate-list";
import { PluginKey } from "@tiptap/pm/state";

export interface CandidateConfig {
  items: (query: { query: string }) => any[] | Promise<any[]>;
  name: string;
  prefix: string;
  selectItem: (props: any, index: number) => void | undefined;
  candidateRenderer: (item: any) => any | undefined;
}

export const suggestion = (candidate_config: CandidateConfig) => {
  const { items, name, prefix, selectItem, candidateRenderer } =
    candidate_config;
  return {
    char: prefix,
    items: items,
    // https://blog.projectan.cn/vue/tiptap-multiple-mention-instances/
    pluginKey: new PluginKey(name),

    // https://github.com/ueberdosis/tiptap/issues/823
    allowSpaces: true,

    render: () => {
      let reactRenderer: ReactRenderer;
      let popup: Instance<Props>[];

      return {
        onStart: (props: {
          editor: Editor;
          clientRect: GetReferenceClientRect;
        }) => {
          const propsExtend = {
            ...props,
            selectItem,
            candidateRenderer,
          };
          reactRenderer = new ReactRenderer(MentionList, {
            props: propsExtend,
            editor: props.editor,
          });

          if (!props.clientRect) {
            return;
          }

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
