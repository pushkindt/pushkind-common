import type { HTMLAttributes } from "react";
import { marked } from "marked";

export function renderMarkdownToHtml(source: string) {
  const trimmedSource = source.trim();
  if (!trimmedSource) {
    return "";
  }

  const rendered = marked.parse(source, { async: false });
  return typeof rendered === "string" ? rendered : source;
}

export type MarkdownPreviewProps = Omit<
  HTMLAttributes<HTMLDivElement>,
  "children" | "dangerouslySetInnerHTML"
> & {
  source: string;
};

export function MarkdownPreview({ source, ...props }: MarkdownPreviewProps) {
  return (
    <div
      {...props}
      dangerouslySetInnerHTML={{ __html: renderMarkdownToHtml(source) }}
    />
  );
}
