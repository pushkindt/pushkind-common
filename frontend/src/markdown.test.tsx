import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { MarkdownPreview, renderMarkdownToHtml } from "./markdown";

describe("renderMarkdownToHtml", () => {
  it("renders markdown to HTML", () => {
    expect(renderMarkdownToHtml("**Hello**")).toContain(
      "<strong>Hello</strong>",
    );
  });

  it("returns an empty string for blank input", () => {
    expect(renderMarkdownToHtml("  \n ")).toBe("");
  });
});

describe("MarkdownPreview", () => {
  it("renders markdown content into a div", () => {
    const markup = renderToStaticMarkup(
      <MarkdownPreview className="preview" source="# Heading" />,
    );

    expect(markup).toContain('class="preview"');
    expect(markup).toContain("<h1>Heading</h1>");
  });
});
