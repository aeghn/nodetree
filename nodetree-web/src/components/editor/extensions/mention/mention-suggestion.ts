import { ReactRenderer } from "@tiptap/react";
import tippy, { Instance, Props, GetReferenceClientRect } from "tippy.js";
import { Editor } from "@tiptap/react";

import { MentionList } from "./mention-list";

export default {
    items: ({ query }: { query: string }) => {
        return [
            "Lea Thompson",
            "Cyndi Lauper",
            "Tom Cruise",
            "Madonna",
            "Jerry Hall",
            "Joan Collins",
            "Winona Ryder",
            "Christina Applegate",
            "Alyssa Milano",
            "Molly Ringwald",
            "Ally Sheedy",
            "Debbie Harry",
            "Olivia Newton-John",
            "Elton John",
            "Michael J. Fox",
            "Axl Rose",
            "Emilio Estevez",
            "Ralph Macchio",
            "Rob Lowe",
            "Jennifer Grey",
            "Mickey Rourke",
            "John Cusack",
            "Matthew Broderick",
            "Justine Bateman",
            "Lisa Bonet"
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

                return reactRenderer.ref?.onKeyDown(props);
            },

            onExit() {
                popup[0].destroy();
                reactRenderer.destroy();
            }
        };
    }
};
