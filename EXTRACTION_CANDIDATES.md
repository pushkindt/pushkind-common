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

The CRM, Emailer, ToDo, and Orders no-access pages now use the shared no-access bootstrap hook, with CRM, ToDo, and Orders also using the shared card. Auth, CRM, Emailer, Files, ToDo, and Orders now use the shared user-menu dropdown package entrypoint.

## Priority 1: exact or near-exact Rust candidates

### 1. Frontend HTML file opener

Current copies:

- `pushkind-auth/src/frontend.rs`
- `pushkind-files/src/frontend.rs`
- `pushkind-crm/src/frontend.rs`
- `pushkind-emailer/src/frontend.rs`

Status:

- `pushkind-files`, `pushkind-crm`, and `pushkind-emailer` are byte-for-byte identical.
- `pushkind-auth` is the same helper with different documentation wording.

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

- this is still the largest remaining frontend duplication
- `pushkind-auth/frontend/src/lib/api.ts`
- `pushkind-crm/frontend/src/lib/api.ts`
- `pushkind-emailer/frontend/src/lib/api.ts`
- `pushkind-todo/frontend/src/lib/api.ts`
- `pushkind-orders/frontend/src/lib/api.ts`
  all still carry overlapping redirect-aware JSON parsing and mutation error handling logic

Recommended split:

- `frontend/src/json.ts` for low-level payload readers
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

- `pushkind-common/frontend/src/types.ts` already defines the common shell types
- the services still duplicate local aliases or copies instead of importing those shared types
- mutation response types are still duplicated and should be added to the shared package next

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

- these are near-identical components that differ mostly in service labels or wording

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

Current blockers:

- flash rendering still differs between modal-based shells and inline-stack shells
- Bootstrap helper initialization differs slightly by service

Conclusion:

- this is a valid extraction target, but only after one more alignment pass on flash delivery and optional Bootstrap feature flags

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

- these components should be nearly identical
- the current differences are mostly migration drift, not fundamental service requirements
- the only potentially legitimate variation is Orders' fallback search form, and even that should be prop-driven rather than implemented in a separate navbar

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
