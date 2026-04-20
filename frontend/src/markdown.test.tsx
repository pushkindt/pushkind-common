import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import {
  MarkdownComposer,
  MarkdownPreview,
  renderMarkdownToHtml,
} from "./markdown";

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

describe("MarkdownComposer", () => {
  it("renders a controlled markdown editor", () => {
    const markup = renderToStaticMarkup(
      <MarkdownComposer
        id="message"
        textareaName="message"
        value="**Hello**"
        onChange={() => undefined}
        placeholder="Write message"
      />,
    );

    expect(markup).toContain('id="message"');
    expect(markup).toContain('name="message"');
    expect(markup).toContain("Write message");
    expect(markup).toContain("**Hello**");
    expect(markup).not.toContain("Files");
  });

  it("renders markdown preview when preview mode is selected initially", () => {
    const markup = renderToStaticMarkup(
      <MarkdownComposer
        initialMode="preview"
        value="# Heading"
        onChange={() => undefined}
      />,
    );

    expect(markup).toContain("<h1>Heading</h1>");
  });

  it("renders the optional file browser panel", () => {
    const markup = renderToStaticMarkup(
      <MarkdownComposer
        initialMode="files"
        value=""
        onChange={() => undefined}
        fileBrowser={{
          baseUrl: "https://files.example.test",
          helpText: "Use copied URLs in markdown.",
        }}
      />,
    );

    expect(markup).toContain("Files");
    expect(markup).toContain("Use copied URLs in markdown.");
    expect(markup).toContain("shell-markdown-composer-file-browser");
    expect(markup).toContain("max-height:50vh");
    expect(markup).toContain("overflow-y:auto");
  });
});
