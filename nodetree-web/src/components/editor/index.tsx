import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import "tiptap-extension-resizable-image/styles.css";
import { Highlight } from "@tiptap/extension-highlight";
import { Typography } from "@tiptap/extension-typography";

import { ResizableImage } from "./extensions/resizable-image/ResizableImage";
import { uploadImage } from "@/helpers/data-agent";
import { MathBlock, MathInline } from "./extensions/math";
import "katex/dist/katex.min.css";
import { BackLink } from "./extensions/candidate/backlink";
import React from "react";
import { NodeId } from "@/model";

const NTEditor: React.FC<{
  height: number | undefined;
  nodeId: NodeId;
  content: string;
  contentChangeCallback: (content: string, nodeId: NodeId) => void;
  idChangeCallback: (id: NodeId) => void;
}> = ({ height, nodeId, contentChangeCallback, idChangeCallback, content }) => {
  const editor = useEditor({
    extensions: [
      StarterKit,
      ResizableImage.configure({
        allowBase64: true,
      }),
      Highlight,
      Typography,
      MathInline,
      MathBlock,

      BackLink(),
    ],
    content: content,
    editorProps: {
      attributes: {
        spellcheck: "false",
      },
      handleClickOn: (_view, _pos, node) => {
        if (node.type.name === "backlink") {
          idChangeCallback(node.attrs.id);
        }
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
        contentChangeCallback(JSON.stringify(json), nodeId);
      }
    },
  });

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

const NTEditorMemo = React.memo(NTEditor);

export default NTEditorMemo;
