import type { RequestEvent } from "@sveltejs/kit";

export interface FetchOptions {
  path: string;
  excludeToken?: boolean;
  query?: Record<string, string>;
}

export function get<T>(event: RequestEvent, options: FetchOptions) {
  return fetchApi<T>(event, "GET", options);
}

export function post<T>(event: RequestEvent, options: FetchOptions, body: any) {
  return fetchApi<T>(event, "POST", options, body);
}

function fetchApi<T>(
  event: RequestEvent,
  method: string,
  options: FetchOptions,
  body?: any
) {
  // const token = getCookie("token");
  const url = new URL(options.path, "http://localhost:8080");
  if (options.query) {
    for (const key in options.query) {
      url.searchParams.append(key, options.query[key]);
    }
  }

  const headers: Record<string, string> = body
    ? { "Content-Type": "application/json" }
    : {};

  if (!options.excludeToken) {
    const token = event.cookies.get("token");
    if (token) headers.Authorization = `Bearer ${token}`;
  }

  return event
    .fetch(url.toString(), {
      method,
      headers,
      body: JSON.stringify(body),
    })
    .then((res) => res.json() as Promise<T>);
}
