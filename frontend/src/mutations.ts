import {
  readJsonResponse,
  ensureResponseIsNotAuthRedirect,
  isJsonResponse,
} from "./shellApi";
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

function statusMutationError(response: Response): ApiMutationError {
  if (response.status === 401) {
    return {
      message: "Сессия истекла. Войдите снова и повторите действие.",
      field_errors: [],
    };
  }

  if (response.status === 403) {
    return {
      message: "Недостаточно прав для выполнения действия.",
      field_errors: [],
    };
  }

  return {
    message: `Запрос не выполнен. Статус: ${response.status}.`,
    field_errors: [],
  };
}

async function readMutationResponse(
  response: Response,
  endpoint: string,
): Promise<ApiMutationSuccess> {
  ensureResponseIsNotAuthRedirect(response);

  if (!response.ok && !isJsonResponse(response)) {
    throw statusMutationError(response);
  }

  const payload = await readJsonResponse<ApiMutationSuccess | ApiMutationError>(
    response,
    endpoint,
  );

  if (!response.ok) {
    if (isApiMutationError(payload)) {
      throw payload;
    }

    throw statusMutationError(response);
  }

  return payload as ApiMutationSuccess;
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

  return readMutationResponse(response, endpoint);
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

  return readMutationResponse(response, endpoint);
}

export async function postEmpty(endpoint: string): Promise<ApiMutationSuccess> {
  const response = await fetch(endpoint, {
    method: "POST",
    headers: {
      Accept: "application/json",
    },
    credentials: "include",
  });

  return readMutationResponse(response, endpoint);
}
