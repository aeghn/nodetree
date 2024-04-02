import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import React, { useEffect } from "react";
import "tiptap-extension-resizable-image/styles.css";
import { Highlight } from "@tiptap/extension-highlight";
import { Typography } from "@tiptap/extension-typography";
import { ResizableImage } from "./extensions/ResizableImage/ResizableImage";
import { uploadImage } from "../../helpers/dataAgent";
import { NTNode } from "../../model";

export const NTEditor: React.FC<{
  height: number | undefined;
  inNode: NTNode;
  setOutNode: Function;
}> = ({ height, inNode, setOutNode }) => {
  const editor = useEditor({
    extensions: [
      StarterKit,
      ResizableImage.configure({
        allowBase64: true,
      }),
      Highlight,
      Typography,
    ],
    editorProps: {
      attributes: {
        spellcheck: "false",
      },
      handlePaste: (view, event) => {
        const items = event.clipboardData?.files;
        if (!items) {
          return false;
        }

        const images = Array.from(items).filter((file) =>
          /image/i.test(file.type)
        );

        if (images.length === 0) {
          return false;
        }

        event.preventDefault();
        const { schema } = view.state;

        images.forEach(async (image) => {
          const node = schema.nodes.image.create({
            src:
              "http://chinslt.com:3011/api/download/" +
              (await uploadImage(image)).id,
          });
          const transaction = view.state.tr.replaceSelectionWith(node);
          view.dispatch(transaction);
        });
        return true;
      },
    },
    onUpdate: ({ editor }) => {
      const json = editor.getJSON();
      if (json) {
        const node = { ...inNode, content: JSON.stringify(json) };
        setOutNode(node)
      }
    },
  });

  useEffect(() => {
    let text = inNode.content;
    if (
      text &&
      text.length > 0
    ) {
      const trimedStart = text.trimStart();
      if (trimedStart.startsWith("{") || trimedStart.startsWith("[")) {
        try {
          text = JSON.parse(text);
        } catch (err) {
          console.error("unable to parse node content: ", err);
        }
      }
    }
    // WAIT Dirty, wait Tiptap to fix this. https://github.com/ueberdosis/tiptap/issues/3764#issuecomment-1546629928
    setTimeout(() => {
      editor?.commands.setContent(text);
    });
  }, [inNode]);

  const style = {
    flex: 1,
    width: "100%",
    height: "100%",
  };

  return (
    <EditorContent
      editor={editor}
      height={height}
      style={style}
      id="tiptap-editor"
    />
  );
};
