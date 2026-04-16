import { readJsonResponse, ensureResponseIsNotAuthRedirect } from "./shellApi";
import { isRecord } from "./json";

export interface ApiFieldError {
  field: string;
  message: string;
}

export interface ApiMutationSuccess {
  message: string;
  redirect_to: string | null;
}

export interface ApiMutationError {
  message: string;
  field_errors: ApiFieldError[];
}

export function toFieldErrorMap(
  error: ApiMutationError,
): Record<string, string> {
  return Object.fromEntries(
    error.field_errors.map((fieldError) => [
      fieldError.field,
      fieldError.message,
    ]),
  );
}

export function isApiMutationError(error: unknown): error is ApiMutationError {
  if (!isRecord(error)) {
    return false;
  }

  return (
    typeof error.message === "string" &&
    Array.isArray(error.field_errors) &&
    error.field_errors.every((fieldError) => {
      return (
        isRecord(fieldError) &&
        typeof fieldError.field === "string" &&
        typeof fieldError.message === "string"
      );
    })
  );
}

export async function postForm(
  endpoint: string,
  body: URLSearchParams,
): Promise<ApiMutationSuccess> {
  const response = await fetch(endpoint, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8",
    },
    credentials: "include",
    body: body.toString(),
  });

  ensureResponseIsNotAuthRedirect(response);

  const payload = (await readJsonResponse(response, endpoint)) as
    | ApiMutationSuccess
    | ApiMutationError;

  if (!response.ok) {
    throw payload as ApiMutationError;
  }

  return payload as ApiMutationSuccess;
}

export async function postMultipartForm(
  endpoint: string,
  body: FormData,
): Promise<ApiMutationSuccess> {
  const response = await fetch(endpoint, {
    method: "POST",
    headers: {
      Accept: "application/json",
    },
    credentials: "include",
    body,
  });

  ensureResponseIsNotAuthRedirect(response);

  const payload = (await readJsonResponse(response, endpoint)) as
    | ApiMutationSuccess
    | ApiMutationError;

  if (!response.ok) {
    throw payload as ApiMutationError;
  }

  return payload as ApiMutationSuccess;
}

export async function postEmpty(endpoint: string): Promise<ApiMutationSuccess> {
  const response = await fetch(endpoint, {
    method: "POST",
    headers: {
      Accept: "application/json",
    },
    credentials: "include",
  });

  ensureResponseIsNotAuthRedirect(response);

  const payload = (await readJsonResponse(response, endpoint)) as
    | ApiMutationSuccess
    | ApiMutationError;

  if (!response.ok) {
    throw payload as ApiMutationError;
  }

  return payload as ApiMutationSuccess;
}
