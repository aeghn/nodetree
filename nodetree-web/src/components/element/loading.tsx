import { ReactNode } from "react";

export default function Loading({
  customIcon,
  message,
}: {
  customIcon: ReactNode;
  message: string;
}) {
  return (
    <div className="top-1/2 w-full h-full flex flex-col justify-center items-center align-middle gap-1 text-muted-foreground">
      {customIcon}
      <p>{message}</p>
    </div>
  );
}
