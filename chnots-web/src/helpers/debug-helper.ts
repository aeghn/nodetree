import { useEffect } from "react";

export const useDebugChanged = (title: string, v: unknown) => {
  useEffect(() => {
    console.log(` ==> ${title} changed`);
  }, [v]);
};
