import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import React from "react";
import 'tiptap-extension-resizable-image/styles.css';
import { Highlight } from "@tiptap/extension-highlight";
import { Typography } from "@tiptap/extension-typography";
import { ResizableImage } from "./extensions/ResizableImage/ResizableImage";
import { uploadImage } from "../../helpers/dataAgent";

export const NTEditor: React.FC<{
  height: number | undefined;
}> = ({ height }) => {
  const editor = useEditor(
    {
      extensions: [
        StarterKit,
        ResizableImage.configure({
          allowBase64: true
        }),
        Highlight,
        Typography,
      ],
      editorProps: {
        attributes: {
          spellcheck: 'false',
        },
        handlePaste: (view, event) => {
          const items = event.clipboardData?.files;
          if (!items) {
            return false;
          }

          const images = Array
            .from(items)
            .filter(file => (/image/i).test(file.type));

          if (images.length === 0) {
            return false;
          }

          event.preventDefault()
          const { schema } = view.state

          images.forEach(async image => {
            const node = schema.nodes.image.create({
              src: "http://chinslt.com:3011/api/download/" + (await uploadImage(image)).id,
            });
            const transaction = view.state.tr.replaceSelectionWith(node);
            view.dispatch(transaction)
          })
          return true;
        },
      },
    }
  );
  const style = {
    flex: 1,
    width: "100%",
    height: "100%",
  };

  return <EditorContent editor={editor} height={height} style={style} id="tiptap-editor" />;
};
