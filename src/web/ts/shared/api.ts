import { isRecord, isString } from "./types";

export type ApiError = Error & { status?: number };

type Decoder<T> = (value: unknown) => T;

const readErrorMessage = (payload: unknown): string | null => {
  if (!isRecord(payload)) return null;
  const message = payload.message;
  return isString(message) ? message : null;
};

export const apiJson = async (
  path: string,
  token: string | null,
  options: RequestInit = {},
): Promise<unknown> => {
  const headers = new Headers(options.headers || {});
  if (token) {
    headers.set("Authorization", `Bearer ${token}`);
  }
  const response = await fetch(path, { ...options, headers });
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    const message = readErrorMessage(data) || "Request failed";
    const error = new Error(message) as ApiError;
    error.status = response.status;
    throw error;
  }
  return data as unknown;
};

export const apiJsonDecoded = async <T>(
  path: string,
  token: string | null,
  decoder: Decoder<T>,
  options: RequestInit = {},
): Promise<T> => decoder(await apiJson(path, token, options));
