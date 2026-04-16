# Cross-Service Extraction Candidates

Last reviewed: 2026-04-13

This inventory covers the React-migrated services that currently share the same migration pattern:

- `pushkind-auth`
- `pushkind-files`
- `pushkind-crm`
- `pushkind-emailer`
- `pushkind-todo`
- `pushkind-orders`

The goal is to identify code that is already duplicated or nearly duplicated, align low-risk drift first, and then move the stable pieces into `pushkind-common`.

## Already aligned in this pass

- `pushkind-crm/frontend/src/pages/NoAccessPage.tsx`
- `pushkind-emailer/frontend/src/pages/NoAccessPage.tsx`
- `pushkind-crm/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-emailer/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-files/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-todo/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-todo/frontend/src/pages/NoAccessPage.tsx`
- `pushkind-orders/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-orders/frontend/src/pages/NoAccessPage.tsx`
- `pushkind-crm/frontend/src/components/CrmShell.tsx`
- `pushkind-emailer/frontend/src/components/EmailerShell.tsx`
- `pushkind-orders/frontend/src/components/OrdersShell.tsx`
- `pushkind-crm/frontend/src/components/CrmShellFatalState.tsx`
- `pushkind-emailer/frontend/src/components/EmailerShellFatalState.tsx`
- `pushkind-todo/frontend/src/components/TodoShellFatalState.tsx`
- `pushkind-orders/frontend/src/components/OrdersShellFatalState.tsx`
- shared shell parsing/fetch helpers now extracted into `pushkind-common/frontend/src/shellApi.ts`
- shared low-level JSON readers now extracted into `pushkind-common/frontend/src/json.ts`
- shared shell/base frontend types now live in `pushkind-common/frontend/src/types.ts`

The CRM, Emailer, ToDo, and Orders no-access pages now use the shared no-access bootstrap hook, with CRM, ToDo, and Orders also using the shared card. Auth, CRM, Emailer, Files, ToDo, and Orders now use the shared user-menu dropdown package entrypoint. CRM, Emailer, and Orders now use the shared modal-flash shell wrapper, and the converged fatal-state components now use the shared fatal-state primitive.

## Priority 1: exact or near-exact Rust candidates

### 1. Frontend HTML file opener

Current copies:

- `pushkind-auth/src/frontend.rs`
- `pushkind-files/src/frontend.rs`
- `pushkind-crm/src/frontend.rs`
- `pushkind-emailer/src/frontend.rs`

Status:

- **Moved:** implemented in `pushkind-common/src/frontend.rs` as `open_frontend_html` and `FrontendAssetError`.
- **Services updated:** `pushkind-auth`, `pushkind-files`, `pushkind-crm`, `pushkind-emailer`, and `pushkind-todo`, `pushkind-orders` now re-export the common helper from `pushkind-common`.

Recommended target:

- `pushkind_common::frontend::open_frontend_html`
- `pushkind_common::frontend::FrontendAssetError`

Reason:

- This helper is small, stable, and already proven across multiple services.

### 2. Shell and mutation DTO building blocks

Repeated shapes:

- `CurrentUserDto`
- `NavigationItemDto`
- `IamDto`
- `NoAccessPageDto`
- `ApiFieldErrorDto`
- `ApiMutationErrorDto`
- `ApiMutationSuccessDto`

Current copies:

- `pushkind-crm/src/dto/api.rs`
- `pushkind-emailer/src/dto/api.rs`
- `pushkind-todo/src/dto/api.rs`
- `pushkind-orders/src/dto/api.rs`
- related mutation DTO pattern in `pushkind-auth/src/dto/api.rs`

Recommended target:

- `pushkind_common::dto::shell`
- `pushkind_common::dto::mutation`

Variation points:

- `NoAccessPageDto.required_role` is optional in CRM and required in ToDo and Orders.
- `ApiMutationErrorDto` now builds from a shared service-level `FormError` in Auth, CRM, Emailer, ToDo, and Orders.
- Files does not participate in this API mutation DTO set because its migrated surface and shell contract differ.

Proposed shape:

- move the common structs first
- move the common `ApiMutationErrorDto` and `ApiFieldErrorDto` structs now
- keep only service-specific success payload DTOs and resource DTOs local

#### Concrete DTO types and current locations

Below are the concrete, repeatedly-defined Rust DTO types (the minimal shared set) and where their authoritative definitions currently live. Status shows whether the type has been moved to `pushkind-common`.

- `CurrentUserDto` — defined in:
  - `pushkind-auth/src/dto/api.rs`
  - `pushkind-crm/src/dto/api.rs`
  - `pushkind-emailer/src/dto/api.rs`
  - `pushkind-files/src/dto/mod.rs`
  - `pushkind-todo/src/dto/api.rs`
  - `pushkind-orders/src/dto/api.rs`
  Status: not moved (recommend: `pushkind_common::dto::shell::CurrentUserDto`).

- `NavigationItemDto` — defined in:
  - `pushkind-auth/src/dto/api.rs`
  - `pushkind-crm/src/dto/api.rs`
  - `pushkind-emailer/src/dto/api.rs`
  - `pushkind-todo/src/dto/api.rs`
  - `pushkind-orders/src/dto/api.rs`
  Status: not moved (recommend: `pushkind_common::dto::shell::NavigationItemDto`).

- `IamDto` / `ShellDataDto` (service naming differs) — defined in:
  - `pushkind-auth/src/dto/api.rs` (`ShellDataDto`)
  - `pushkind-crm/src/dto/api.rs` (`IamDto`)
  - `pushkind-emailer/src/dto/api.rs` (`IamDto`)
  - `pushkind-todo/src/dto/api.rs` (`IamDto`)
  - `pushkind-orders/src/dto/api.rs` (`IamDto`)
  - `pushkind-files/src/dto/mod.rs` (`FilesShellDto`) — files uses a smaller shell shape
  Status: not moved (recommend: consolidate as `pushkind_common::dto::shell::ShellDataDto` with minimal variation hooks).

- `NoAccessPageDto` — defined in:
  - `pushkind-crm/src/dto/api.rs`
  - `pushkind-emailer/src/dto/api.rs`
  - `pushkind-files/src/dto/mod.rs`
  - `pushkind-todo/src/dto/api.rs`
  - `pushkind-orders/src/dto/api.rs`
  Status: not moved — note: `required_role` semantics differ (`Option<String>` vs `&'static str`).

- `ApiFieldErrorDto` — defined in:
  - `pushkind-auth/src/dto/api.rs`
  - `pushkind-crm/src/dto/api.rs`
  - `pushkind-emailer/src/dto/api.rs`
  - `pushkind-files/src/dto/mod.rs`
  - `pushkind-todo/src/dto/api.rs`
  - `pushkind-orders/src/dto/api.rs`
  Status: not moved (recommend: `pushkind_common::dto::mutation::ApiFieldErrorDto`).

- `ApiMutationErrorDto` — defined in:
  - `pushkind-auth/src/dto/api.rs`
  - `pushkind-crm/src/dto/api.rs`
  - `pushkind-emailer/src/dto/api.rs`
  - `pushkind-files/src/dto/mod.rs`
  - `pushkind-todo/src/dto/api.rs`
  - `pushkind-orders/src/dto/api.rs`
  Status: not moved (recommend: `pushkind_common::dto::mutation::ApiMutationErrorDto`).

- `ApiMutationSuccessDto` — defined in:
  - `pushkind-auth/src/dto/api.rs`
  - `pushkind-crm/src/dto/api.rs`
  - `pushkind-emailer/src/dto/api.rs`
  - `pushkind-files/src/dto/mod.rs`
  - `pushkind-todo/src/dto/api.rs`
  - `pushkind-orders/src/dto/api.rs`
  Status: not moved (keep service-specific success payloads local where they carry extra data; extract the common minimal success DTO).

Notes:
- `FilesShellDto` intentionally differs (smaller shape) — keep it local or provide a thin adapter.
- `required_role` differences must be reconciled (optional vs static str) during extraction; prefer optional fields for flexibility.

Recommended next step for DTOs: extract `ApiFieldErrorDto` and `ApiMutationErrorDto` first (low-risk), then `CurrentUserDto` + `NavigationItemDto` + `ShellDataDto/IamDto` with adapters for service-specific extras.

### 3. Shell data service helpers

Repeated logic:

- build current-user shell payload
- expose home URL
- expose service navigation
- expose local menu items
- build no-access payload

Current copies:

- `pushkind-crm/src/services/api.rs`
- `pushkind-emailer/src/services/api.rs`
- `pushkind-todo/src/services/api.rs`
- `pushkind-orders/src/services/api.rs`

Recommended target:

- `pushkind_common::services::shell`

Variation points:

- local navigation differs by service
- role requirements differ by page and service

Proposed extraction style:

- shared builder functions that accept service-specific navigation/menu arrays and optional `required_role`
- do not centralize role policy itself

## Priority 2: frontend TypeScript candidates

These should live in a frontend package inside the `pushkind-common` repository, not in the Rust crate itself.

### 4. User menu dropdown

Current copies:

- `pushkind-auth/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-files/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-crm/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-emailer/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-todo/frontend/src/components/UserMenuDropdown.tsx`
- `pushkind-orders/frontend/src/components/UserMenuDropdown.tsx`

Status:

- ToDo and Orders are byte-for-byte identical.
- Auth, Files, and Emailer are effectively the same component with the same extra icon mappings.
- CRM is the minimal variant of the same component.

Recommended target:

- `frontend-shell/UserMenuDropdown.tsx`

Required inputs:

- `currentUserEmail`
- `localItems`
- `fetchedItems`
- `logoutAction`
- optional icon resolver override or merged icon map

### 5. Shell hook for loading `/api/v1/iam` and auth menu items

Current copies:

- `pushkind-crm/frontend/src/lib/useCrmShell.ts`
- `pushkind-emailer/frontend/src/lib/useEmailerShell.ts`
- `pushkind-todo/frontend/src/lib/useTodoShell.ts`
- `pushkind-orders/frontend/src/lib/useOrdersShell.ts`

Status:

- same state machine
- same menu hydration fallback
- differences are service names in warning strings and type names

Recommended target:

- `frontend-shell/useServiceShell.ts`

Required inputs:

- `fetchShellData`
- `fetchHubMenuItems`
- fatal error message
- warning label for console fallback text

### 6. No-access page bootstrap flow

Current copies:

- `pushkind-crm/frontend/src/pages/NoAccessPage.tsx`
- `pushkind-emailer/frontend/src/pages/NoAccessPage.tsx`
- `pushkind-todo/frontend/src/pages/NoAccessPage.tsx`
- `pushkind-orders/frontend/src/pages/NoAccessPage.tsx`

Status:

- CRM and Emailer were aligned in this pass to the ToDo and Orders pattern.
- Files intentionally differs because its shell is not the same page architecture.

Recommended target:

- `frontend-shell/NoAccessPage.tsx`

Required inputs:

- service shell component
- shell fatal state component
- shell hook
- service label

Current state:

- the data-loading hook is already shared as `useNoAccessPageData`
- the generic card is already shared as `NoAccessCard`
- CRM, ToDo, and Orders use the shared card directly
- Emailer intentionally keeps custom copy, so it only uses the shared hook

Conclusion:

- a fully shared no-access page wrapper is still possible
- it should support either a generic card path or a render prop for custom service copy

### 7. Frontend API shell helpers

Repeated helpers:

- `isRecord`
- `readString`
- `readNumber`
- `readStringArray`
- nullable and optional readers
- `browserLocation`
- `isJsonResponse`
- redirect-aware JSON reading
- `fetchShellData`
- `fetchNoAccessData`
- `fetchHubMenuItems`
- `toFieldErrorMap`
- API mutation error guards

Current copies:

- `pushkind-crm/frontend/src/lib/api.ts`
- `pushkind-emailer/frontend/src/lib/api.ts`
- `pushkind-todo/frontend/src/lib/api.ts`
- `pushkind-orders/frontend/src/lib/api.ts`
- related subset in `pushkind-auth/frontend/src/lib/api.ts`

Recommended target:

- `frontend-shell/api.ts`
- `frontend-shell/json.ts`

Variation points:

- shape of `NoAccessData`
- richer mutation response types in some services
- service-specific request helpers beyond the shell contract

Proposed extraction style:

- extract only the JSON readers, redirect handling, shell fetchers, and mutation error helpers first
- leave resource-specific parsers inside each service

Current state:

- the shell-focused part of this extraction is now done in:
  - `pushkind-common/frontend/src/json.ts`
  - `pushkind-common/frontend/src/shellApi.ts`
- CRM, Emailer, ToDo, and Orders source files have been switched to those shared shell helpers
- the mutation-helper layer is still duplicated and should be extracted next

Recommended split:

- `frontend/src/mutations.ts` for `ApiMutationError`, `ApiMutationSuccess`, guards, field-error mapping, and redirect-aware mutation helpers
- keep service resource parsers local

### 7a. Shared frontend shell and mutation types

Current copies:

- `pushkind-crm/frontend/src/lib/models.ts`
- `pushkind-emailer/frontend/src/lib/models.ts`
- `pushkind-todo/frontend/src/lib/models.ts`
- `pushkind-orders/frontend/src/lib/models.ts`
- related shell and mutation types in `pushkind-auth/frontend/src/lib/api.ts`

Repeated shapes:

- `NavigationItem`
- `UserMenuItem`
- `CurrentUser`
- `ShellData`
- `NoAccessData`
- `ApiFieldError`
- `ApiMutationSuccess`
- `ApiMutationError`

Current state:

- `pushkind-common/frontend/src/types.ts` now defines the common shell types and shared camel-case mutation types
- CRM, Emailer, ToDo, and Orders now alias those shell types instead of redefining them locally
- snake-case raw mutation transport types still remain local in Auth, CRM, Emailer, and Orders

Recommended target:

- extend `frontend/src/types.ts`
- re-export stable aliases from service-local model files during the transition if needed

### 7b. Shared shell fatal-state component

Current copies:

- `pushkind-crm/frontend/src/components/CrmShellFatalState.tsx`
- `pushkind-emailer/frontend/src/components/EmailerShellFatalState.tsx`
- `pushkind-todo/frontend/src/components/TodoShellFatalState.tsx`
- `pushkind-orders/frontend/src/components/OrdersShellFatalState.tsx`

Status:

- extracted as `pushkind-common/frontend/src/ShellFatalState.tsx`
- CRM, Emailer, ToDo, and Orders now use it

Recommended target:

- `frontend/src/ShellFatalState.tsx`

Recommended inputs:

- `message`
- optional `serviceLabel`
- optional `className`

### 7c. Shared shell wrapper component

Current copies:

- `pushkind-crm/frontend/src/components/CrmShell.tsx`
- `pushkind-emailer/frontend/src/components/EmailerShell.tsx`
- `pushkind-todo/frontend/src/components/TodoShell.tsx`
- `pushkind-orders/frontend/src/components/OrdersShell.tsx`

Shared responsibilities:

- render navbar
- expose `window.showFlashMessage`
- manage Bootstrap flash modal or alert stack
- optionally initialize popovers/tooltips
- render children

Current state:

- the modal-flash shell variant is now extracted as `pushkind-common/frontend/src/ModalFlashShell.tsx`
- CRM, Emailer, and Orders now use it
- ToDo still intentionally differs because it uses an inline stacked alert model instead of modal-hosted flash rendering

Conclusion:

- only the ToDo-style inline alert shell remains as a separate variant

### 7d. Shared shell navbar component

Current copies:

- `pushkind-crm/frontend/src/components/CrmNavbar.tsx`
- `pushkind-todo/frontend/src/components/TodoNavbar.tsx`
- `pushkind-orders/frontend/src/components/OrdersNavbar.tsx`

Observed drift:

- CRM adds `pt-2`, a `crm-navbar` class, and wraps the search slot in `crm-navbar-search`
- ToDo uses the simplest form with `pt-2` and no extra wrappers
- Orders omits `pt-2`, uses a different user-menu wrapper, and renders a built-in fallback search form when `search` is absent

Assessment:

- extracted as `pushkind-common/frontend/src/ServiceNavbar.tsx`
- CRM, ToDo, Orders, and Emailer source files have been switched to it
- the remaining work here is only to refresh consuming repos against the pushed package revision and verify UI parity

Recommended target:

- `frontend/src/ServiceNavbar.tsx`

Recommended inputs:

- `brandLabel`
- `collapseId`
- `navigation`
- `currentUserEmail`
- `homeUrl`
- `localMenuItems`
- `fetchedMenuItems`
- `search`
- optional `navbarClassName`
- optional `outerContainerClassName`
- optional `searchWrapperClassName`
- optional `fallbackSearch`

Extraction prerequisite:

- align the three implementations to the same DOM shape first so the shared component preserves Tera-era UI parity

### 7e. Shared dropdown multi-select

Current copies:

- `pushkind-orders/frontend/src/components/DropdownMultiSelect.tsx`
- `pushkind-auth/frontend/src/components/DropdownMultiSelect.tsx`

Status:

- this appears to be converging into the same component API
- once Auth and Orders settle on the same props and rendering, it should move into the shared frontend package

Recommended target:

- `frontend/src/DropdownMultiSelect.tsx`

## Priority 3: follow-up Rust candidates

### 8. API error response helpers

Repeated behavior:

- turn validation and service errors into `{ message, field_errors }` JSON
- standardize status code mapping for mutation endpoints

Current copies:

- `pushkind-auth/src/dto/api.rs`
- `pushkind-crm/src/routes/mod.rs`
- `pushkind-todo/src/routes/mod.rs`
- `pushkind-orders/src/routes/api.rs`
- `pushkind-emailer` route-level JSON mutation handling

Why not move immediately:

- status code policies still differ by endpoint family
- responder entry points still differ between route modules even though the validation DTO input has converged

Recommended direction:

- first converge on one small helper signature per service
- then extract a generic responder into `pushkind-common`

### 9. Current-user conversions from `AuthenticatedUser`

Repeated behavior:

- map auth identity into service shell DTOs

Current copies:

- `pushkind-crm/src/dto/api.rs`
- `pushkind-emailer/src/dto/api.rs`
- `pushkind-todo/src/dto/api.rs`
- `pushkind-orders/src/dto/api.rs`

Recommended target:

- `pushkind_common::dto::shell::CurrentUserDto`

## Not good candidates yet

### Service navigation arrays

Reason:

- these are intentionally service-local and role-sensitive

### Resource parsers for each React page

Reason:

- shape drift is business-domain specific, not infrastructure duplication

### Files browser runtime

Reason:

- `pushkind-files` has a different shell contract and embedded browser use case

## Suggested move order

1. Rust `frontend::open_frontend_html`
2. Rust shell DTOs and mutation DTO structs
3. Rust shell builder helpers
4. Shared frontend dropdown component
5. Shared frontend shell hook
6. Shared frontend no-access bootstrap
7. Shared frontend JSON and redirect helpers
8. Route-level mutation responder helpers after status-policy convergence
