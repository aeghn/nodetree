import { forwardRef, useEffect, useImperativeHandle, useState } from "react";

import { SuggestionKeyDownProps } from "@tiptap/suggestion";

export const SuggestionList = forwardRef<any, any>((props, ref) => {
  const [selectedIndex, setSelectedIndex] = useState(0);

  const selectItem = props.selectItem
    ? (index: number) => {
      props.selectItem(props, index);
    }
    : (index: number) => {
      const item = props.items[index];

      if (item) {
        props.command({ id: item });
      }
    };

  const upHandler = () => {
    setSelectedIndex(
      (selectedIndex + props.items.length - 1) % props.items.length
    );
  };

  const downHandler = () => {
    setSelectedIndex((selectedIndex + 1) % props.items.length);
  };

  const enterHandler = () => {
    selectItem(selectedIndex);
  };

  useEffect(() => setSelectedIndex(0), [props.items]);

  useImperativeHandle(ref, () => ({
    onKeyDown: ({ event }: SuggestionKeyDownProps) => {
      if (event.key === "ArrowUp") {
        upHandler();
        return true;
      }

      if (event.key === "ArrowDown") {
        downHandler();
        return true;
      }

      if (event.key === "Enter") {
        enterHandler();
        return true;
      }

      return false;
    },
  }));

  return (
    <div className="items">
      {props.items.length ? (
        props.items.map((item: any, index: any) => (
          <button
            className={`item ${index === selectedIndex ? "is-selected" : ""}`}
            key={index}
            onClick={() => selectItem(index)}
          >
            {props.itemRenderer ? props.itemRenderer(item) : item}
          </button>
        ))
      ) : (
        <div className="item">No result</div>
      )}
    </div>
  );
});
