import { escapeForRegEx } from "@/helpers/tools";
import { InputRule, Mark, Node } from "@tiptap/core";

import { Hashtag, Reminder } from "./mark-builder";

export interface NTMarkType {
  type: Mark;
  prefix: string;
  suffix: string;
}

function emptyBody(markType: NTMarkType) {
  return markType.prefix + markType.suffix;
}

function toEscapedRegex(markType: NTMarkType) {
  return escapeForRegEx(emptyBody(markType));
}

export function interactiveInputRule(config: {
  types: NTMarkType[];
}): InputRule {
  const regex = new RegExp(
    `${config.types.map(toEscapedRegex).join("|")}$`,
    "gm"
  );

  return new InputRule({
    find: regex,
    handler: ({ state, range, match, commands }) => {
      const { tr } = state;

      for (const type of config.types) {
        const empty = emptyBody(type);
        const res = match.find((value) => {
          return value === empty;
        });
        if (res) {
          tr.deleteRange(range.from, range.to);
          commands.insertContentAt(range.from, " ");
          commands.focus(range.from);
          commands.setMark(type.type.name);
        }
      }
    },
  });
}

export interface NTRuleOptions {}

export const NTRules = Node.create<NTRuleOptions>({
  name: "NTRules",

  addInputRules() {
    return [
      interactiveInputRule({
        types: [
          {
            type: Hashtag,
            prefix: "#",
            suffix: "#",
          },
          {
            type: Reminder,
            prefix: "<",
            suffix: ">",
          },
        ],
      }),
    ];
  },
});
