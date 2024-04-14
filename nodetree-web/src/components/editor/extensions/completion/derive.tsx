import { fetchNodesLike } from "@/helpers/data-agent";
import { createCompletion } from ".";
import { NTNode } from "@/model";

export const Backlink = createCompletion({
  triggerChar: "&",
  pluginName: "backlink",
  items: (query: { query: string }) => {
    return fetchNodesLike(query.query);

  },
  selectItem: (props: any, index: number) => {
    const item: NTNode = props.items[index];
    console.log(props)
    if (item) {
      props.command({ id: item.id, label: item.name });
    }
  },
  completionItemRenderer: (item: NTNode) => {
    return (
      <div>
        <div>{item.name}</div>
        <br />
        <div>{item.content}</div>
      </div>
    );
  }
});

export const Hashtag = createCompletion({
  triggerChar: "#",
  pluginName: "hashtag",
  items: (query: { query: string }) => {
    console.log(query)
    return query ? [query.query] : [];
  },
  selectItem: (props: any, index: number) => {
    const item: string = props.items[index];
    console.log(props)
    if (item) {
      props.command({ id: item, label: item });
    }
  },
  completionItemRenderer: (item: string) => {
    return (
      <div>{item}</div>
    );
  }
});