import { useEffect, useRef } from "react";
import type { ReactNode } from "react";

type ModalFlashShellProps = {
  navbar: ReactNode;
  children: ReactNode;
  enablePopovers?: boolean;
  enableTooltips?: boolean;
};

export function ModalFlashShell({
  navbar,
  children,
  enablePopovers = false,
  enableTooltips = false,
}: ModalFlashShellProps) {
  const ajaxFlashContentRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    window.showFlashMessage = (message, category = "primary") => {
      const flashes = ajaxFlashContentRef.current;
      const modal = window.bootstrap?.Modal.getOrCreateInstance(
        "#ajax-flash-modal",
        {},
      );

      if (!flashes || !modal) {
        return;
      }

      flashes.innerHTML = `<div class="alert alert-${category} alert-dismissible mb-0" role="alert">${message}<button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button></div>`;
      modal.show();
    };

    const Bootstrap = window.bootstrap;
    const Popover = enablePopovers ? Bootstrap?.Popover : undefined;
    const Tooltip = enableTooltips ? Bootstrap?.Tooltip : undefined;

    const popovers =
      Popover == null
        ? []
        : Array.from(
            document.querySelectorAll("[data-bs-toggle='popover']"),
          ).map((element) => new Popover(element as Element));

    const tooltips =
      Tooltip == null
        ? []
        : Array.from(
            document.querySelectorAll("[data-bs-toggle='tooltip']"),
          ).map((element) => new Tooltip(element as Element));

    return () => {
      delete window.showFlashMessage;
      popovers.forEach((popover) => popover?.dispose?.());
      tooltips.forEach((tooltip) => tooltip?.dispose?.());
    };
  }, [enablePopovers, enableTooltips]);

  return (
    <>
      <div className="modal" tabIndex={-1} id="ajax-flash-modal">
        <div className="modal-dialog">
          <div className="modal-content">
            <div
              className="modal-body"
              id="ajax-flash-content"
              style={{ padding: 0 }}
              ref={ajaxFlashContentRef}
            />
          </div>
        </div>
      </div>
      {navbar}
      {children}
    </>
  );
}
