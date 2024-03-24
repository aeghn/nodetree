import { Color } from "@tiptap/extension-color";
import ListItem from "@tiptap/extension-list-item";
import TextStyle from "@tiptap/extension-text-style";
import Image from "@tiptap/extension-image";
import TextAlign from "@tiptap/extension-text-align";
import { EditorContent, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";


export default function NTEditor() {
  const editor = useEditor({
    extensions: [
      Image.configure({
        HTMLAttributes: {
          class: "mx-auto w-full object-cover max-h-[300px]",
        },
      }),
      TextAlign.configure({
        types: ["heading", "paragraph"],
        defaultAlignment: "right",
      }),
      Color.configure({ types: [TextStyle.name, ListItem.name] }),
      //@ts-ignore
      TextStyle.configure({ types: [ListItem.name] }),
      StarterKit.configure({
        bulletList: {
          keepMarks: true,
          keepAttributes: false,
        },
        orderedList: {
          keepMarks: true,
          keepAttributes: false,
        },
      }),
    ],
  });

  return (

    <div className="rounded-xl h-full">
      <EditorContent editor={editor} />
    </div>

  );
};