/*
This file is part of the Notesnook project (https://notesnook.com/)

Copyright (C) 2023 Streetwriters (Private) Limited

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

/*---------------------------------------------------------
 *  Author: Benjamin R. Bray
 *  License: MIT (see LICENSE in project root for details)
 *--------------------------------------------------------*/

// prosemirror imports
import { Node as ProseNode } from "prosemirror-model";
import {
  Plugin as ProsePlugin,
  PluginKey,
  PluginSpec,
} from "prosemirror-state";
import { CompletionView } from "./completion-node-view";
import { EditorView } from "prosemirror-view";

////////////////////////////////////////////////////////////

export interface ICompletionPluginState {
  macros: { [cmd: string]: string };
  /** A list of currently active `NodeView`s, in insertion order. */
  activeNodeViews: CompletionView[];
  /**
   * Used to determine whether to place the cursor in the front- or back-most
   * position when expanding a completion node, without overriding the default arrow
   * key behavior.
   */
  prevCursorPos: number;
}

// uniquely identifies the prosemirror-completion plugin
const COMPLETION_PLUGIN_KEY = new PluginKey<ICompletionPluginState>(
  "prosemirror-completion"
);

/**
 * Returns a function suitable for passing as a field in `EditorProps.nodeViews`.
 * @param inline TRUE for block completion, FALSE for inline completion.
 * @see https://prosemirror.net/docs/ref/#view.EditorProps.nodeViews
 */
export function createCompletionView() {
  return (
    node: ProseNode,
    view: EditorView,
    getPos: boolean | (() => number | undefined)
  ): CompletionView => {
    /** @todo is this necessary?
     * Docs says that for any function proprs, the current plugin instance
     * will be bound to `this`.  However, the typings don't reflect this.
     */
    const pluginState = COMPLETION_PLUGIN_KEY.getState(view.state);
    if (!pluginState) {
      throw new Error("no completion plugin!");
    }
    const nodeViews = pluginState.activeNodeViews;

    // set up NodeView
    const nodeView = new CompletionView(
      node,
      view,
      getPos as () => number,
      {
        className: "completion",
        renderer: (text, element) => {
          console.log("text", text);
          console.log("element", element);
        },
        tagName: "span",
      },
      COMPLETION_PLUGIN_KEY
    );

    nodeViews.push(nodeView);
    return nodeView;
  };
}

const completionPluginSpec: PluginSpec<ICompletionPluginState> = {
  key: COMPLETION_PLUGIN_KEY,
  state: {
    init() {
      return {
        macros: {},
        activeNodeViews: [],
        prevCursorPos: 0,
      };
    },
    apply(_tr, value, oldState, newState) {
      // produce updated state field for this plugin
      const newPos = newState.selection.from;
      const oldPos = oldState.selection.from;

      return {
        // these values are left unchanged
        activeNodeViews: value.activeNodeViews,
        macros: value.macros,
        // update with the second-most recent cursor pos
        prevCursorPos: oldPos !== newPos ? oldPos : value.prevCursorPos,
      };
    },
    /** @todo (8/21/20) implement serialization for completion plugin */
    // toJSON(value) { },
    // fromJSON(config, value, state){ return {}; }
  },
  props: {
    nodeViews: {
      completion: createCompletionView(),
    },
  },
};

export const completionPlugin = new ProsePlugin(completionPluginSpec);
