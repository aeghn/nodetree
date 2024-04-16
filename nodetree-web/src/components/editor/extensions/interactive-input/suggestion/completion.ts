export interface CompletionOptions {
    triggerChar: string;
    pluginName: string;
    items: (query: { query: string }) => any[] | Promise<any[]>;
    selectItem: (props: any, index: number) => void | undefined;
    completionItemRenderer: (item: any) => any | undefined;
  }