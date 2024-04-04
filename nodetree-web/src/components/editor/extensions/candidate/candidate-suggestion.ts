import { ReactRenderer } from "@tiptap/react";
import tippy, { Instance, Props, GetReferenceClientRect } from "tippy.js";
import { Editor } from "@tiptap/react";

import { MentionList } from "./candidate-list";

export default {
    items: ({ query }: { query: string }) => {
        return [
            "Lea Thompson",
            "Cyndi Lauper"
        ]
            .filter((item) => item.toLowerCase().startsWith(query.toLowerCase()))
            .slice(0, 5);
    },

    render: () => {
        let reactRenderer: ReactRenderer;
        let popup: Instance<Props>[];

        return {
            onStart: (props: {
                editor: Editor;
                clientRect: GetReferenceClientRect;
            }) => {
                reactRenderer = new ReactRenderer(MentionList, {
                    props,
                    editor: props.editor
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
                    placement: "bottom-start"
                });
            },

            onUpdate(props: Record<string, any>) {
                reactRenderer.updateProps(props);

                if (!props.clientRect) {
                    return;
                }

                popup[0].setProps({
                    getReferenceClientRect: props.clientRect
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
            }
        };
    }
};
