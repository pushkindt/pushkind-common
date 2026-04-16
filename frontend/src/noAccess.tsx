import { useEffect, useState } from "react";

import type { ReactNode, ComponentType } from "react";
import type {
  FrontendNoAccessData,
  FrontendNoAccessState,
  FrontendShellData,
  FrontendShellNavigationItem,
  FrontendShellUserMenuItem,
} from "./types";
import { useServiceShell } from "./useServiceShell";

export type UseNoAccessPageDataOptions<
  TData extends FrontendNoAccessData = FrontendNoAccessData,
> = {
  errorMessage: string;
  fetchNoAccessData: () => Promise<TData>;
};

export function useNoAccessPageData<
  TData extends FrontendNoAccessData = FrontendNoAccessData,
>({
  errorMessage,
  fetchNoAccessData,
}: UseNoAccessPageDataOptions<TData>): FrontendNoAccessState<TData> {
  const [state, setState] = useState<FrontendNoAccessState<TData>>({
    status: "loading",
  });

  useEffect(() => {
    let active = true;

    void fetchNoAccessData()
      .then((data) => {
        if (!active) {
          return;
        }

        setState({ status: "ready", data });
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
  }, [errorMessage, fetchNoAccessData]);

  return state;
}

export type NoAccessCardProps = {
  serviceLabel: string;
  currentUserName: string;
  currentUserEmail: string;
  homeUrl: string;
  requiredRole?: string | null;
  logoutAction: string;
  className?: string;
};

export function NoAccessCard({
  serviceLabel,
  currentUserName,
  currentUserEmail,
  homeUrl,
  requiredRole,
  logoutAction,
  className,
}: NoAccessCardProps) {
  return (
    <main className={className ?? "container py-5"}>
      <div className="card shadow-sm">
        <div className="card-body p-4">
          <p className="text-uppercase text-secondary small mb-2">
            {serviceLabel}
          </p>
          <h1 className="h3 mb-3">Недостаточно прав для доступа к сервису</h1>
          <p className="text-secondary mb-3">
            Пользователь <strong>{currentUserName}</strong> не имеет{" "}
            {requiredRole ? (
              <>
                роли <code>{requiredRole}</code>.
              </>
            ) : (
              "необходимой роли."
            )}
          </p>
          <p className="text-secondary mb-4">
            Текущий email: <strong>{currentUserEmail}</strong>
          </p>
          <div className="d-flex flex-column flex-sm-row gap-2">
            <a className="btn btn-primary" href={homeUrl}>
              Домой
            </a>
            <form method="POST" action={logoutAction}>
              <button className="btn btn-outline-secondary" type="submit">
                Выйти
              </button>
            </form>
          </div>
        </div>
      </div>
    </main>
  );
}

export type ServiceNoAccessPageProps<
  TNoAccessData extends FrontendNoAccessData,
  TShellData extends FrontendShellData,
  TMenuItem extends FrontendShellUserMenuItem,
> = {
  fetchShellData: () => Promise<TShellData>;
  fetchHubMenuItems: (homeUrl: string, hubId: number) => Promise<TMenuItem[]>;
  fetchNoAccessData: () => Promise<TNoAccessData>;
  serviceLabel: string;
  logoutAction?: string;
  menuLoadWarning?: string;
  noAccessCardClassName?: string;
  ShellComponent: ComponentType<{
    navigation: FrontendShellNavigationItem[];
    currentUserEmail: string;
    homeUrl: string;
    localMenuItems: FrontendShellUserMenuItem[];
    fetchedMenuItems: TMenuItem[];
    children: ReactNode;
  }>;
  FatalStateComponent: ComponentType<{ message: string }>;
};

export function ServiceNoAccessPage<
  TNoAccessData extends FrontendNoAccessData,
  TShellData extends FrontendShellData,
  TMenuItem extends FrontendShellUserMenuItem,
>({
  fetchShellData,
  fetchHubMenuItems,
  fetchNoAccessData,
  serviceLabel,
  logoutAction = "/logout",
  menuLoadWarning = "Failed to load auth navigation menu.",
  noAccessCardClassName,
  ShellComponent,
  FatalStateComponent,
}: ServiceNoAccessPageProps<TNoAccessData, TShellData, TMenuItem>) {
  const shellState = useServiceShell<TShellData, TMenuItem>({
    errorMessage: `Не удалось загрузить оболочку ${serviceLabel}.`,
    menuLoadWarning,
    fetchShellData,
    fetchHubMenuItems,
  });

  const noAccessState = useNoAccessPageData<TNoAccessData>({
    errorMessage: "Не удалось загрузить страницу.",
    fetchNoAccessData,
  });

  if (shellState.status === "error") {
    return <FatalStateComponent message={shellState.message} />;
  }

  if (shellState.status === "loading" || noAccessState.status === "loading") {
    return null;
  }

  if (noAccessState.status === "error") {
    return <FatalStateComponent message={noAccessState.message} />;
  }

  return (
    <ShellComponent
      navigation={shellState.shell.navigation}
      currentUserEmail={shellState.shell.currentUser.email}
      homeUrl={shellState.shell.homeUrl}
      localMenuItems={shellState.shell.localMenuItems}
      fetchedMenuItems={shellState.authMenuItems}
    >
      <NoAccessCard
        className={noAccessCardClassName}
        serviceLabel={serviceLabel}
        currentUserName={noAccessState.data.currentUser.name}
        currentUserEmail={noAccessState.data.currentUser.email}
        homeUrl={noAccessState.data.homeUrl}
        requiredRole={noAccessState.data.requiredRole}
        logoutAction={logoutAction}
      />
    </ShellComponent>
  );
}
