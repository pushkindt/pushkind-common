import type {
  FrontendNoAccessData,
  FrontendShellCurrentUser,
  FrontendShellData,
  FrontendShellNavigationItem,
  FrontendShellUserMenuItem,
} from "./types";
import {
  isRecord,
  readNumber,
  readOptionalString,
  readString,
  readStringArray,
} from "./json";

export const browserLocation = {
  assign(url: string) {
    window.location.assign(url);
  },
};

function currentVisiblePath(): string {
  const path = `${window.location.pathname}${window.location.search}`;
  return path || "/";
}

export function normalizeAuthRedirectUrl(url: string): string {
  const redirectUrl = new URL(url, window.location.href);

  if (redirectUrl.searchParams.has("next")) {
    redirectUrl.searchParams.set("next", currentVisiblePath());
  }

  return redirectUrl.toString();
}

export function isJsonResponse(response: Response): boolean {
  return (
    response.headers.get("content-type")?.includes("application/json") ?? false
  );
}

export function handleAuthRedirectResponse(response: Response): never {
  browserLocation.assign(normalizeAuthRedirectUrl(response.url));
  throw new Error("Сессия истекла. Выполняется переход на страницу входа.");
}

export function ensureResponseIsNotAuthRedirect(response: Response) {
  if (response.redirected && !isJsonResponse(response)) {
    handleAuthRedirectResponse(response);
  }
}

export async function readJsonResponse<T>(
  response: Response,
  endpoint: string,
) {
  if (!isJsonResponse(response)) {
    throw new Error(
      `Expected JSON response from ${endpoint} with status ${response.status}.`,
    );
  }

  return (await response.json()) as T;
}

export async function fetchJson(
  url: string,
  options?: {
    unauthorizedMessage?: string;
    notFoundMessage?: string;
  },
) {
  const response = await fetch(url, {
    headers: {
      Accept: "application/json",
    },
    cache: "no-store",
    credentials: "include",
  });

  if (!response.ok) {
    if (response.status === 401 && options?.unauthorizedMessage) {
      throw new Error(options.unauthorizedMessage);
    }

    if (response.status === 404 && options?.notFoundMessage) {
      throw new Error(options.notFoundMessage);
    }

    throw new Error(`Request failed with status ${response.status}.`);
  }

  ensureResponseIsNotAuthRedirect(response);
  return readJsonResponse(response, url);
}

export function parseNavigationItems<
  TItem extends FrontendShellNavigationItem = FrontendShellNavigationItem,
>(payload: unknown): TItem[] {
  if (!Array.isArray(payload)) {
    throw new Error("Invalid navigation payload.");
  }

  return payload.map((item) => {
    if (!isRecord(item)) {
      throw new Error("Invalid navigation item payload.");
    }

    return {
      name: readString(item, "name"),
      url: readString(item, "url"),
    } as TItem;
  });
}

export function parseMenuItems<
  TItem extends FrontendShellUserMenuItem = FrontendShellUserMenuItem,
>(payload: unknown): TItem[] {
  return parseNavigationItems<TItem>(payload);
}

export function parseCurrentUser<
  TUser extends FrontendShellCurrentUser = FrontendShellCurrentUser,
>(payload: unknown): TUser {
  if (!isRecord(payload)) {
    throw new Error("Invalid current user payload.");
  }

  return {
    email: readString(payload, "email"),
    name: readString(payload, "name"),
    hubId: readNumber(payload, "hub_id"),
    roles: readStringArray(payload, "roles"),
  } as TUser;
}

export function parseShellData<
  TShell extends FrontendShellData = FrontendShellData,
  TUser extends FrontendShellCurrentUser = FrontendShellCurrentUser,
  TNavigationItem extends FrontendShellNavigationItem =
    FrontendShellNavigationItem,
  TMenuItem extends FrontendShellUserMenuItem = FrontendShellUserMenuItem,
>(payload: unknown): TShell {
  if (!isRecord(payload)) {
    throw new Error("Invalid shell payload.");
  }

  return {
    currentUser: parseCurrentUser<TUser>(payload.current_user),
    homeUrl: readString(payload, "home_url"),
    navigation: parseNavigationItems<TNavigationItem>(payload.navigation),
    localMenuItems: parseMenuItems<TMenuItem>(payload.local_menu_items),
  } as unknown as TShell;
}

export function parseNoAccessData<
  TData extends FrontendNoAccessData = FrontendNoAccessData,
  TUser extends FrontendShellCurrentUser = FrontendShellCurrentUser,
>(payload: unknown): TData {
  if (!isRecord(payload)) {
    throw new Error("Invalid no-access payload.");
  }

  return {
    currentUser: parseCurrentUser<TUser>(payload.current_user),
    homeUrl: readString(payload, "home_url"),
    requiredRole: readOptionalString(payload, "required_role"),
  } as unknown as TData;
}

export async function fetchShellData<
  TShell extends FrontendShellData = FrontendShellData,
>(endpoint: string, unauthorizedMessage?: string): Promise<TShell> {
  const payload = await fetchJson(endpoint, { unauthorizedMessage });
  return parseShellData<TShell>(payload);
}

export async function fetchNoAccessData<
  TData extends FrontendNoAccessData = FrontendNoAccessData,
>(endpoint: string, unauthorizedMessage?: string): Promise<TData> {
  const payload = await fetchJson(endpoint, { unauthorizedMessage });
  return parseNoAccessData<TData>(payload);
}

export async function fetchHubMenuItems<
  TItem extends FrontendShellUserMenuItem = FrontendShellUserMenuItem,
>(endpoint: string, unauthorizedMessage?: string): Promise<TItem[]> {
  const payload = await fetchJson(endpoint, { unauthorizedMessage });
  return parseMenuItems<TItem>(payload);
}
