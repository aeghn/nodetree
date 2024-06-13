import { useHydrateAtoms } from "jotai/utils";

export const HydrateAtoms = ({
  initialValues,
  children,
}: {
  initialValues: any;
  children: any;
}) => {
  // initialising on state with prop on render here
  useHydrateAtoms(initialValues);
  return children;
};
