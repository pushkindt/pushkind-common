import { useEffect, useState } from "react";

import type {
  FrontendShellData,
  FrontendShellState,
  FrontendShellUserMenuItem,
} from "./types";

export type UseServiceShellOptions<
  TShell extends FrontendShellData = FrontendShellData,
  TMenuItem extends FrontendShellUserMenuItem = FrontendShellUserMenuItem,
> = {
  errorMessage: string;
  menuLoadWarning: string;
  fetchShellData: () => Promise<TShell>;
  fetchHubMenuItems: (homeUrl: string, hubId: number) => Promise<TMenuItem[]>;
};

export function useServiceShell<
  TShell extends FrontendShellData = FrontendShellData,
  TMenuItem extends FrontendShellUserMenuItem = FrontendShellUserMenuItem,
>({
  errorMessage,
  menuLoadWarning,
  fetchShellData,
  fetchHubMenuItems,
}: UseServiceShellOptions<TShell, TMenuItem>): FrontendShellState<
  TShell,
  TMenuItem
> {
  const [state, setState] = useState<FrontendShellState<TShell, TMenuItem>>({
    status: "loading",
  });

  useEffect(() => {
    let active = true;

    void fetchShellData()
      .then((shell) => {
        if (!active) {
          return;
        }

        setState({
          status: "ready",
          shell,
          authMenuItems: [],
          authMenuLoaded: false,
        });
      })
      .catch((error) => {
        if (!active) {
          return;
        }

        setState({
          status: "error",
          message: error instanceof Error ? error.message : errorMessage,
        });
      });

    return () => {
      active = false;
    };
  }, [errorMessage, fetchShellData]);

  useEffect(() => {
    if (state.status !== "ready" || state.authMenuLoaded) {
      return;
    }

    let active = true;

    void fetchHubMenuItems(state.shell.homeUrl, state.shell.currentUser.hubId)
      .then((authMenuItems) => {
        if (!active) {
          return;
        }

        setState((currentState) => {
          if (currentState.status !== "ready") {
            return currentState;
          }

          return {
            status: "ready",
            shell: currentState.shell,
            authMenuItems,
            authMenuLoaded: true,
          };
        });
      })
      .catch((error) => {
        if (!active) {
          return;
        }

        console.warn(menuLoadWarning, error);

        setState((currentState) => {
          if (currentState.status !== "ready") {
            return currentState;
          }

          return {
            status: "ready",
            shell: currentState.shell,
            authMenuItems: currentState.authMenuItems,
            authMenuLoaded: true,
          };
        });
      });

    return () => {
      active = false;
    };
  }, [fetchHubMenuItems, menuLoadWarning, state]);

  return state;
}
