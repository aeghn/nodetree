import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import "tiptap-extension-resizable-image/styles.css";
import { Highlight } from "@tiptap/extension-highlight";
import { Typography } from "@tiptap/extension-typography";

import { uploadImage } from "@/helpers/data-agent";
import { MathBlock, MathInline } from "./extensions/math";
import "katex/dist/katex.min.css";
import { BackLink } from "./extensions/candidate/backlink";
import React from "react";
import { NodeId } from "@/model";
import { ImageExtension } from "./extensions/image";
import { startImageUpload } from "./extensions/image/upload-image";
import { findTableAncestor } from "./util";
import { ImageResizer } from "./extensions/image/image-resize";

const NTEditor: React.FC<{
  height: number | undefined;
  nodeId: NodeId;
  content: string;
  contentChangeCallback: (content: string, nodeId: NodeId) => void;
  idChangeCallback: (id: NodeId) => void;
}> = ({ height, nodeId, contentChangeCallback, idChangeCallback, content }) => {
  const uploadFile = async (file: File) => {
    return "http://chinslt.com:3011/api/download/" +
      (await (uploadImage(file))).id
  }

  const editor = useEditor({
    extensions: [
      StarterKit,
      ImageExtension().configure({
        HTMLAttributes: {
          class: "rounded-lg border border-custom-border-300",
        },
      }),
      Highlight,
      Typography,
      MathInline,
      MathBlock,

      BackLink()
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
      /*       handlePaste: (view, event) => {
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
            }, */
      handlePaste: (view, event) => {
        if (typeof window !== "undefined") {
          const selection: any = window?.getSelection();
          if (selection.rangeCount !== 0) {
            const range = selection.getRangeAt(0);
            if (findTableAncestor(range.startContainer)) {
              return;
            }
          }
        }
        if (event.clipboardData && event.clipboardData.files && event.clipboardData.files[0]) {
          event.preventDefault();
          const file = event.clipboardData.files[0];
          const pos = view.state.selection.from;
          startImageUpload(file, view, pos, uploadFile);
          return true;
        }
        return false;
      },
      handleDrop: (view, event, _slice, moved) => {
        if (!moved && event.dataTransfer && event.dataTransfer.files && event.dataTransfer.files[0]) {
          event.preventDefault();
          const file = event.dataTransfer.files[0];
          const coordinates = view.posAtCoords({
            left: event.clientX,
            top: event.clientY,
          });
          if (coordinates) {
            startImageUpload(file, view, coordinates.pos - 1, uploadFile);
          }
          return true;
        }
        return false;
      },
      transformPastedHTML(html) {
        return html.replace(/<img.*?>/g, "");
      }
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
    <div className="h-full w-full">
      <EditorContent
        editor={editor}
        height={height}
        style={style}
        id="tiptap-editor"
      />
      {editor?.isActive("image") && editor?.isEditable && <ImageResizer editor={editor} />}
    </div>
  );
};

const NTEditorMemo = React.memo(NTEditor);

export default NTEditorMemo;
