import { useEffect } from "react";

export const useDebugChanged = (title: string, v: any) => {
  useEffect(() => {
    console.log(` ==> ${title} changed`);
  }, [v]);
};
