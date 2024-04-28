import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import "tiptap-extension-resizable-image/styles.css";
import { Highlight } from "@tiptap/extension-highlight";
import { Typography } from "@tiptap/extension-typography";

import { uploadImage } from "@/helpers/data-agent";
import { MathBlock, MathInline } from "./extensions/math";
import "katex/dist/katex.min.css";
import React from "react";
import { NodeId } from "@/model";
import { ImageExtension } from "./extensions/image";
import { startImageUpload } from "./extensions/image/upload-image";
import { findTableAncestor } from "./util";
import { ImageResizer } from "./extensions/image/image-resize";
import { TaskList } from "@tiptap/extension-task-list";
import { TaskItem } from "@tiptap/extension-task-item";
import { Link } from "@tiptap/extension-link";

import { cx } from "class-variance-authority";

import "@/styles/editor.css";
import { Backlink, Hashtag, Reminder } from "./extensions/interactive-input";
import { setShouldShowSuggestion } from "./extensions/interactive-input/suggestion/suggestion-options-builder";

const NTEditor: React.FC<{
  height: number | undefined;
  nodeId: NodeId;
  content: string;
  contentChangeCallback: (content: string, nodeId: NodeId) => void;
  idChangeCallback: (id: NodeId) => void;
}> = ({ height, nodeId, contentChangeCallback, idChangeCallback, content }) => {
  const uploadFile = async (file: File) => {
    return (
      "http://chinslt.com:3011/api/download/" + (await uploadImage(file)).id
    );
  };

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
      TaskList,
      TaskItem.configure({
        nested: true,
      }),
      Hashtag,
      Reminder,
      Backlink(idChangeCallback),
      Link.configure({
        autolink: true,
        HTMLAttributes: {
          class: cx(
            "text-muted-foreground underline underline-offset-[3px] hover:text-primary transition-colors cursor-pointer"
          ),
        },
      }),
    ],
    content: content,
    editorProps: {
      attributes: {
        spellcheck: "false",
      },
      handleDOMEvents: {
        mousedown: () => {
          setShouldShowSuggestion(false);
        },
      },
      /* handleKeyDown: () => {
        setShouldShowSuggestion(true);
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
        if (
          event.clipboardData &&
          event.clipboardData.files &&
          event.clipboardData.files[0]
        ) {
          event.preventDefault();
          const file = event.clipboardData.files[0];
          const pos = view.state.selection.from;
          startImageUpload(file, view, pos, uploadFile);
          return true;
        }
        return false;
      },
      handleDrop: (view, event, _slice, moved) => {
        if (
          !moved &&
          event.dataTransfer &&
          event.dataTransfer.files &&
          event.dataTransfer.files[0]
        ) {
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
      },
    },
    onUpdate: ({ editor }) => {
      setShouldShowSuggestion(true);
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
      {editor?.isActive("image") && editor?.isEditable && (
        <ImageResizer editor={editor} />
      )}
    </div>
  );
};

const NTEditorMemo = React.memo(NTEditor);

export default NTEditorMemo;
