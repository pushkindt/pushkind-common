export function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

export function readString(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (typeof value !== "string") {
    throw new Error(`Invalid API response: expected string at ${key}.`);
  }

  return value;
}

export function readOptionalString(
  record: Record<string, unknown>,
  key: string,
) {
  const value = record[key];
  if (value == null) {
    return undefined;
  }
  if (typeof value !== "string") {
    throw new Error(`Invalid API response: expected string at ${key}.`);
  }

  return value;
}

export function readNullableString(
  record: Record<string, unknown>,
  key: string,
) {
  const value = record[key];
  if (value === null) {
    return null;
  }
  if (typeof value !== "string") {
    throw new Error(`Invalid API response: expected string|null at ${key}.`);
  }

  return value;
}

export function readNumber(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (typeof value !== "number") {
    throw new Error(`Invalid API response: expected number at ${key}.`);
  }

  return value;
}

export function readOptionalNumber(
  record: Record<string, unknown>,
  key: string,
) {
  const value = record[key];
  if (value == null) {
    return undefined;
  }
  if (typeof value !== "number") {
    throw new Error(`Invalid API response: expected number at ${key}.`);
  }

  return value;
}

export function readNullableNumber(
  record: Record<string, unknown>,
  key: string,
) {
  const value = record[key];
  if (value === null) {
    return null;
  }
  if (typeof value !== "number") {
    throw new Error(`Invalid API response: expected number|null at ${key}.`);
  }

  return value;
}

export function readBoolean(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (typeof value !== "boolean") {
    throw new Error(`Invalid API response: expected boolean at ${key}.`);
  }

  return value;
}

export function readStringArray(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string")) {
    throw new Error(`Invalid API response: expected string[] at ${key}.`);
  }

  return value;
}

export function readNumberArray(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (!Array.isArray(value) || value.some((item) => typeof item !== "number")) {
    throw new Error(`Invalid API response: expected number[] at ${key}.`);
  }

  return value;
}

export function readNullableNumberArray(
  record: Record<string, unknown>,
  key: string,
) {
  const value = record[key];
  if (
    !Array.isArray(value) ||
    value.some((item) => item !== null && typeof item !== "number")
  ) {
    throw new Error(
      `Invalid API response: expected (number|null)[] at ${key}.`,
    );
  }

  return value;
}

export function readArray(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (!Array.isArray(value)) {
    throw new Error(`Invalid API response: expected array at ${key}.`);
  }

  return value;
}

export function readRecord(record: Record<string, unknown>, key: string) {
  const value = record[key];
  if (!isRecord(value)) {
    throw new Error(`Invalid API response: expected object at ${key}.`);
  }

  return value;
}

export function parseStringMap(value: unknown) {
  if (!isRecord(value)) {
    return {};
  }

  return Object.fromEntries(
    Object.entries(value).filter((entry): entry is [string, string] => {
      return typeof entry[1] === "string";
    }),
  );
}
