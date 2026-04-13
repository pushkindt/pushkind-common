import type { ReactNode } from "react";

import { UserMenuDropdown } from "./UserMenuDropdown";
import type {
  FrontendShellNavigationItem,
  FrontendShellUserMenuItem,
} from "./types";

function joinClasses(...values: Array<string | undefined>) {
  return values.filter(Boolean).join(" ");
}

export type ServiceNavbarProps<
  TNavigationItem extends FrontendShellNavigationItem =
    FrontendShellNavigationItem,
  TUserMenuItem extends FrontendShellUserMenuItem = FrontendShellUserMenuItem,
> = {
  brand: ReactNode;
  collapseId: string;
  navigation: TNavigationItem[];
  currentUserEmail: string;
  homeUrl: string;
  localMenuItems: TUserMenuItem[];
  fetchedMenuItems: TUserMenuItem[];
  logoutAction: string;
  brandHref?: string;
  search?: ReactNode;
  fallbackSearch?: ReactNode;
  outerContainerClassName?: string;
  navbarClassName?: string;
  searchWrapperClassName?: string;
  userMenuWrapperClassName?: string;
  isNavigationItemActive?: (item: TNavigationItem, pathname: string) => boolean;
};

export function ServiceNavbar<
  TNavigationItem extends FrontendShellNavigationItem =
    FrontendShellNavigationItem,
  TUserMenuItem extends FrontendShellUserMenuItem = FrontendShellUserMenuItem,
>({
  brand,
  collapseId,
  navigation,
  currentUserEmail,
  homeUrl,
  localMenuItems,
  fetchedMenuItems,
  logoutAction,
  brandHref = "/",
  search,
  fallbackSearch,
  outerContainerClassName = "container",
  navbarClassName,
  searchWrapperClassName,
  userMenuWrapperClassName = "ms-sm-2",
  isNavigationItemActive,
}: ServiceNavbarProps<TNavigationItem, TUserMenuItem>) {
  const pathname =
    typeof window === "undefined" ? "/" : window.location.pathname;
  const searchContent = search ?? fallbackSearch ?? null;

  return (
    <div className={outerContainerClassName}>
      <nav
        className={joinClasses(
          "navbar navbar-expand-sm bg-body-tertiary",
          navbarClassName,
        )}
      >
        <div className="container-fluid">
          <a className="navbar-brand" href={brandHref}>
            {brand}
          </a>
          <button
            className="navbar-toggler"
            type="button"
            data-bs-toggle="collapse"
            data-bs-target={`#${collapseId}`}
            aria-controls={collapseId}
            aria-expanded="false"
            aria-label="Toggle navigation"
          >
            <span className="navbar-toggler-icon" />
          </button>
          <div className="collapse navbar-collapse" id={collapseId}>
            <ul className="navbar-nav me-auto">
              {navigation.map((item) => {
                const isActive = isNavigationItemActive?.(item, pathname);

                return (
                  <li className="nav-item" key={item.url}>
                    <a
                      className={joinClasses(
                        "nav-link",
                        isActive ? "active" : undefined,
                      )}
                      href={item.url}
                    >
                      {item.name}
                    </a>
                  </li>
                );
              })}
            </ul>
            {searchContent ? (
              searchWrapperClassName ? (
                <div className={searchWrapperClassName}>{searchContent}</div>
              ) : (
                searchContent
              )
            ) : null}
          </div>
          <div className={userMenuWrapperClassName}>
            <UserMenuDropdown
              currentUserEmail={currentUserEmail}
              localItems={[{ name: "Домой", url: homeUrl }, ...localMenuItems]}
              fetchedItems={fetchedMenuItems}
              logoutAction={logoutAction}
            />
          </div>
        </div>
      </nav>
    </div>
  );
}
