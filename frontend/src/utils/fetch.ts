import { getCookie } from "vinxi/http";

export interface FetchOptions {
  path: string;
  query?: Record<string, string>;
}

export function get<T>(options: FetchOptions) {
  return fetchApi<T>("GET", options);
}

export function post<T>(options: FetchOptions, body: any) {
  return fetchApi<T>("POST", options, body);
}

export function isLogin() {
  "use server";
  return !!getCookie("token");
}

function fetchApi<T>(method: string, options: FetchOptions, body?: any) {
  "use server";

  const token = getCookie("token");
  const url = new URL(options.path, "http://localhost:8080");
  if (options.query) {
    for (const key in options.query) {
      url.searchParams.append(key, options.query[key]);
    }
  }

  const headers: Record<string, string> = body
    ? { "Content-Type": "application/json" }
    : {};

  if (token) {
    headers.Authorization = `Bearer ${token}`;
  }

  return fetch(url.toString(), {
    method,
    headers,
    body: JSON.stringify(body),
  }).then((res) => res.json() as Promise<T>);
}
