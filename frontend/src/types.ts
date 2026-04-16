export type FrontendShellNavigationItem = {
  name: string;
  url: string;
};

export type FrontendShellUserMenuItem = {
  name: string;
  url: string;
  iconClass?: string;
};

export type FrontendShellCurrentUser = {
  email: string;
  name: string;
  hubId: number;
  roles: string[];
};

export type FrontendShellData = {
  currentUser: FrontendShellCurrentUser;
  homeUrl: string;
  navigation: FrontendShellNavigationItem[];
  localMenuItems: FrontendShellUserMenuItem[];
};

export type FrontendShellLoadingState = {
  status: "loading";
};

export type FrontendShellErrorState = {
  status: "error";
  message: string;
};

export type FrontendShellReadyState<
  TShell extends FrontendShellData = FrontendShellData,
  TMenuItem extends FrontendShellUserMenuItem = FrontendShellUserMenuItem,
> = {
  status: "ready";
  shell: TShell;
  authMenuItems: TMenuItem[];
  authMenuLoaded: boolean;
};

export type FrontendShellState<
  TShell extends FrontendShellData = FrontendShellData,
  TMenuItem extends FrontendShellUserMenuItem = FrontendShellUserMenuItem,
> =
  | FrontendShellLoadingState
  | FrontendShellReadyState<TShell, TMenuItem>
  | FrontendShellErrorState;

export type FrontendNoAccessData<
  TUser extends FrontendShellCurrentUser = FrontendShellCurrentUser,
> = {
  currentUser: TUser;
  homeUrl: string;
  requiredRole?: string | null;
};

export type FrontendApiFieldError = {
  field: string;
  message: string;
};

export type FrontendApiMutationSuccess = {
  message: string;
  redirectTo?: string;
};

export type FrontendApiMutationError = {
  message: string;
  fieldErrors: FrontendApiFieldError[];
};

export type FrontendNoAccessState<
  TData extends FrontendNoAccessData = FrontendNoAccessData,
> =
  | { status: "loading" }
  | { status: "ready"; data: TData }
  | { status: "error"; message: string };

declare global {
  interface Window {
    showFlashMessage?: (message: string, category?: string) => void;
    bootstrap?: {
      Modal: {
        getOrCreateInstance: (
          element: string | Element,
          options?: object,
        ) => {
          hide: () => void;
          show: () => void;
          dispose?: () => void;
        };
      };
      Popover?: new (element: Element) => { dispose?: () => void };
      Tooltip?: new (element: Element) => { dispose?: () => void };
    };
  }
}
