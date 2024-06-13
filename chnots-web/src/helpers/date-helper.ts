import { format } from "date-fns";

export const shortDate = (date: Date) => {
  return format(date, "yyMM-dd");
};
