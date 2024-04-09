import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import "tiptap-extension-resizable-image/styles.css";
import { Highlight } from "@tiptap/extension-highlight";
import { Typography } from "@tiptap/extension-typography";

import { ResizableImage } from "./extensions/resizable-image/ResizableImage";
import { fetchNodeContent, uploadImage } from "../../helpers/data-agent";
import { MathBlock, MathInline } from "./extensions/math";
import "katex/dist/katex.min.css";
import { BackLink } from "./extensions/candidate/backlink";
import { useEffect } from "react";
import React from "react";
import { NodeId } from "../../model";

const NTEditor: React.FC<{
  height: number | undefined;
  nodeId: NodeId;
  contentChangeCallback: (content: string) => void;
  idChangeCallback: (content: NodeId) => void;
}> = ({ height, nodeId, contentChangeCallback, idChangeCallback }) => {
  console.log("draw editor", new Date().toDateString());

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
    editorProps: {
      attributes: {
        spellcheck: "false",
      },
      handleClickOn: (view, pos, node) => {
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
        contentChangeCallback(JSON.stringify(json));
      }
    },
  });

  useEffect(() => {
    fetchNodeContent(nodeId).then((node) => {
      let text = node.content;
      if (text && text.length > 0) {
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
    });
  }, [nodeId]);

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
