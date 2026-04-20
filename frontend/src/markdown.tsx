import { useEffect, useId, useRef, useState } from "react";
import type {
  ChangeEvent,
  CSSProperties,
  HTMLAttributes,
  ReactNode,
} from "react";
import { marked } from "marked";

declare global {
  interface Window {
    mountFileBrowser?: (
      target: string | Element,
      initialPath?: string,
      options?: {
        baseUrl?: string;
        historyMode?: "managed" | "disabled";
      },
    ) => unknown;
  }
}

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

export type MarkdownComposerMode = "editor" | "preview" | "files";

export type MarkdownComposerFileBrowserConfig = {
  baseUrl: string;
  initialPath?: string;
  historyMode?: "managed" | "disabled";
  scriptPath?: string;
  rootClassName?: string;
  rootStyle?: CSSProperties;
  panelClassName?: string;
  helpText?: ReactNode;
  loadingLabel?: ReactNode;
  unavailableLabel?: ReactNode;
};

export type MarkdownComposerProps = {
  id?: string;
  value: string;
  onChange: (value: string) => void;
  className?: string;
  rows?: number;
  required?: boolean;
  disabled?: boolean;
  autoFocus?: boolean;
  textareaName?: string;
  textareaClassName?: string;
  previewClassName?: string;
  placeholder?: string;
  editorLabel?: ReactNode;
  previewLabel?: ReactNode;
  fileBrowserLabel?: ReactNode;
  emptyPreviewLabel?: ReactNode;
  fileBrowser?: MarkdownComposerFileBrowserConfig;
  initialMode?: MarkdownComposerMode;
};

type FileBrowserStatus = "idle" | "loading" | "ready" | "error";

const DEFAULT_FILE_BROWSER_SCRIPT_PATH = "/assets/filebrowser.js";
const DEFAULT_FILE_BROWSER_ROOT_STYLE: CSSProperties = {
  maxHeight: "50vh",
  overflowY: "auto",
};

function joinClasses(...classes: Array<string | false | null | undefined>) {
  return classes.filter(Boolean).join(" ");
}

function buildFileBrowserScriptUrl(baseUrl: string, scriptPath: string) {
  const normalizedBaseUrl = baseUrl.replace(/\/+$/, "");
  const normalizedScriptPath = scriptPath.startsWith("/")
    ? scriptPath
    : `/${scriptPath}`;
  return `${normalizedBaseUrl}${normalizedScriptPath}`;
}

export function MarkdownComposer({
  id,
  value,
  onChange,
  className,
  rows = 8,
  required = false,
  disabled = false,
  autoFocus = false,
  textareaName,
  textareaClassName,
  previewClassName,
  placeholder,
  editorLabel = "Editor",
  previewLabel = "Preview",
  fileBrowserLabel = "Files",
  emptyPreviewLabel = "Nothing to preview yet.",
  fileBrowser,
  initialMode = "editor",
}: MarkdownComposerProps) {
  const generatedId = useId().replaceAll(":", "");
  const composerId = id ?? `markdown-composer-${generatedId}`;
  const fileBrowserRootRef = useRef<HTMLDivElement | null>(null);
  const mountedFileBrowserKeyRef = useRef<string | null>(null);
  const [activeMode, setActiveMode] = useState<MarkdownComposerMode>(() =>
    initialMode === "files" && fileBrowser == null ? "editor" : initialMode,
  );
  const [fileBrowserStatus, setFileBrowserStatus] =
    useState<FileBrowserStatus>("idle");

  const fileBrowserScriptPath =
    fileBrowser?.scriptPath ?? DEFAULT_FILE_BROWSER_SCRIPT_PATH;
  const fileBrowserHistoryMode = fileBrowser?.historyMode ?? "disabled";
  const fileBrowserInitialPath = fileBrowser?.initialPath ?? "";
  const fileBrowserKey =
    fileBrowser == null
      ? null
      : [
          fileBrowser.baseUrl,
          fileBrowserInitialPath,
          fileBrowserHistoryMode,
          fileBrowserScriptPath,
        ].join("|");

  useEffect(() => {
    if (fileBrowserKey == null) {
      mountedFileBrowserKeyRef.current = null;
      setFileBrowserStatus("idle");
      if (activeMode === "files") {
        setActiveMode("editor");
      }
      return;
    }

    if (mountedFileBrowserKeyRef.current !== fileBrowserKey) {
      mountedFileBrowserKeyRef.current = null;
      setFileBrowserStatus("idle");
    }
  }, [activeMode, fileBrowserKey]);

  useEffect(() => {
    if (fileBrowser == null || activeMode !== "files") {
      return;
    }

    const target = fileBrowserRootRef.current;
    if (target == null || mountedFileBrowserKeyRef.current === fileBrowserKey) {
      return;
    }

    let cancelled = false;
    let scriptElement: HTMLScriptElement | null = null;

    const mountFileBrowser = () => {
      if (cancelled) {
        return;
      }

      if (window.mountFileBrowser == null) {
        setFileBrowserStatus("error");
        return;
      }

      target.replaceChildren();
      window.mountFileBrowser(target, fileBrowserInitialPath, {
        baseUrl: fileBrowser.baseUrl,
        historyMode: fileBrowserHistoryMode,
      });
      mountedFileBrowserKeyRef.current = fileBrowserKey;
      setFileBrowserStatus("ready");
    };

    if (window.mountFileBrowser != null) {
      mountFileBrowser();
      return;
    }

    setFileBrowserStatus("loading");
    scriptElement = document.createElement("script");
    scriptElement.async = true;
    scriptElement.src = buildFileBrowserScriptUrl(
      fileBrowser.baseUrl,
      fileBrowserScriptPath,
    );
    scriptElement.onload = mountFileBrowser;
    scriptElement.onerror = () => {
      if (!cancelled) {
        setFileBrowserStatus("error");
      }
    };
    document.body.appendChild(scriptElement);

    return () => {
      cancelled = true;
      scriptElement?.remove();
    };
  }, [
    activeMode,
    fileBrowser,
    fileBrowserHistoryMode,
    fileBrowserInitialPath,
    fileBrowserKey,
    fileBrowserScriptPath,
  ]);

  const changeValue = (event: ChangeEvent<HTMLTextAreaElement>) => {
    onChange(event.currentTarget.value);
  };

  const showFiles = fileBrowser != null;
  const previewIsEmpty = value.trim() === "";

  return (
    <div className={joinClasses("shell-markdown-composer", className)}>
      <div className="nav nav-tabs" role="tablist">
        <button
          type="button"
          className={joinClasses(
            "nav-link",
            activeMode === "editor" && "active",
          )}
          id={`${composerId}-editor-tab`}
          aria-controls={`${composerId}-editor-panel`}
          aria-selected={activeMode === "editor"}
          role="tab"
          onClick={() => setActiveMode("editor")}
        >
          {editorLabel}
        </button>
        <button
          type="button"
          className={joinClasses(
            "nav-link",
            activeMode === "preview" && "active",
          )}
          id={`${composerId}-preview-tab`}
          aria-controls={`${composerId}-preview-panel`}
          aria-selected={activeMode === "preview"}
          role="tab"
          onClick={() => setActiveMode("preview")}
        >
          {previewLabel}
        </button>
        {showFiles ? (
          <button
            type="button"
            className={joinClasses(
              "nav-link",
              activeMode === "files" && "active",
            )}
            id={`${composerId}-files-tab`}
            aria-controls={`${composerId}-files-panel`}
            aria-selected={activeMode === "files"}
            role="tab"
            onClick={() => setActiveMode("files")}
          >
            {fileBrowserLabel}
          </button>
        ) : null}
      </div>

      <div
        id={`${composerId}-editor-panel`}
        aria-labelledby={`${composerId}-editor-tab`}
        hidden={activeMode !== "editor"}
        role="tabpanel"
      >
        <textarea
          id={composerId}
          name={textareaName}
          className={joinClasses(
            "form-control border-top-0 rounded-top-0",
            textareaClassName,
          )}
          rows={rows}
          required={required}
          disabled={disabled}
          autoFocus={autoFocus}
          placeholder={placeholder}
          value={value}
          onChange={changeValue}
        />
      </div>

      <div
        id={`${composerId}-preview-panel`}
        aria-labelledby={`${composerId}-preview-tab`}
        hidden={activeMode !== "preview"}
        role="tabpanel"
      >
        {previewIsEmpty ? (
          <div
            className={joinClasses(
              "border border-top-0 rounded rounded-top-0 p-3 text-muted",
              previewClassName,
            )}
          >
            {emptyPreviewLabel}
          </div>
        ) : (
          <MarkdownPreview
            className={joinClasses(
              "border border-top-0 rounded rounded-top-0 p-3",
              previewClassName,
            )}
            source={value}
          />
        )}
      </div>

      {showFiles ? (
        <div
          id={`${composerId}-files-panel`}
          aria-labelledby={`${composerId}-files-tab`}
          hidden={activeMode !== "files"}
          role="tabpanel"
        >
          <div
            className={joinClasses(
              "border border-top-0 rounded rounded-top-0 overflow-hidden",
              fileBrowser.panelClassName,
            )}
          >
            {fileBrowser.helpText !== null ? (
              <div className="border-bottom bg-body-tertiary px-3 py-2 small text-muted">
                {fileBrowser.helpText ??
                  "Browse or upload files, copy a file URL, then paste it into the markdown editor as a link or image."}
              </div>
            ) : null}
            <div
              className={joinClasses(
                "shell-markdown-composer-file-browser",
                fileBrowser.rootClassName,
              )}
              style={{
                ...DEFAULT_FILE_BROWSER_ROOT_STYLE,
                ...fileBrowser.rootStyle,
              }}
              ref={fileBrowserRootRef}
            />
            {fileBrowserStatus === "loading" ? (
              <div className="px-3 py-2 small text-muted">
                {fileBrowser.loadingLabel ?? "Loading file browser..."}
              </div>
            ) : null}
            {fileBrowserStatus === "error" ? (
              <div className="px-3 py-2 small text-danger">
                {fileBrowser.unavailableLabel ?? "File browser is unavailable."}
              </div>
            ) : null}
          </div>
        </div>
      ) : null}
    </div>
  );
}
