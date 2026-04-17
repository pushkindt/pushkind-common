export type {
  FrontendApiFieldError,
  FrontendApiMutationError,
  FrontendApiMutationSuccess,
  FrontendNoAccessData,
  FrontendNoAccessState,
  FrontendShellCurrentUser,
  FrontendShellData,
  FrontendShellNavigationItem,
  FrontendShellReadyState,
  FrontendShellState,
  FrontendShellUserMenuItem,
} from "./types";

export { ModalFlashShell } from "./ModalFlashShell";

export {
  NoAccessCard,
  ServiceNoAccessPage,
  useNoAccessPageData,
  type NoAccessCardProps,
  type ServiceNoAccessPageProps,
  type UseNoAccessPageDataOptions,
} from "./noAccess";

export { ShellFatalState } from "./ShellFatalState";

export {
  UserMenuDropdown,
  type UserMenuDropdownProps,
} from "./UserMenuDropdown";

export {
  DropdownMultiSelect,
  type DropdownMultiSelectProps,
  type DropdownMultiSelectOption,
} from "./DropdownMultiSelect";

export { ServiceNavbar, type ServiceNavbarProps } from "./ServiceNavbar";

export {
  browserLocation,
  ensureResponseIsNotAuthRedirect,
  fetchHubMenuItems,
  fetchJson,
  fetchNoAccessData,
  fetchShellData,
  handleAuthRedirectResponse,
  isJsonResponse,
  normalizeAuthRedirectUrl,
  parseCurrentUser,
  parseMenuItems,
  parseNavigationItems,
  parseNoAccessData,
  parseShellData,
  readJsonResponse,
} from "./shellApi";

export {
  isRecord,
  parseStringMap,
  readArray,
  readBoolean,
  readNullableNumber,
  readNullableNumberArray,
  readNullableString,
  readNumber,
  readNumberArray,
  readOptionalNumber,
  readOptionalString,
  readRecord,
  readString,
  readStringArray,
} from "./json";

export {
  useServiceShell,
  type UseServiceShellOptions,
} from "./useServiceShell";
