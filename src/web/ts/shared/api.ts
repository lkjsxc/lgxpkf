export type ApiError = Error & { status?: number };

export const apiJson = async <T>(
  path: string,
  token: string | null,
  options: RequestInit = {},
): Promise<T> => {
  const headers = new Headers(options.headers || {});
  if (token) {
    headers.set("Authorization", `Bearer ${token}`);
  }
  const response = await fetch(path, { ...options, headers });
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    const message = (data as { message?: string }).message || "Request failed";
    const error = new Error(message) as ApiError;
    error.status = response.status;
    throw error;
  }
  return data as T;
};
