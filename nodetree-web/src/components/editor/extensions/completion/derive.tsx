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
  candidateRenderer: (item: NTNode) => {
    return (
      <div>
        <div>{item.name}</div>
        <br />
        <div>{item.content}</div>
      </div>
    );
  }
});