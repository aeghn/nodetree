/**
 * useThrottleEffect(
    () => { //callback },
    [value],
    time,
  );
 */

import React, { useEffect, useRef } from "react";

export const useThrottleEffect = <T, U extends React.DependencyList>(
  fn: (...args: U) => T,
  args: U,
  delay = 200
) => {
  const timeout = useRef<ReturnType<typeof setTimeout>>();
  const nextArgs = useRef<U>();

  useEffect(() => {
    if (timeout.current) {
      nextArgs.current = args;
    } else {
      const timeoutCallback = () => {
        if (nextArgs.current) {
          fn(...nextArgs.current);
          nextArgs.current = undefined;
        }
        timeout.current = undefined;
      };
      timeout.current = setTimeout(timeoutCallback, delay);
    }
  }, [args]);

  /*   useUnMount(() => {
    timeout.current && clearTimeout(timeout.current);
    console.log("timeout.current", timeout.current);
  }); */
};
