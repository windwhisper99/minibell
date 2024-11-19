import type { APIEvent } from "@solidjs/start/server";
import { setCookie } from "vinxi/http";
import { post } from "~/utils/fetch";

export async function GET(event: APIEvent) {
  const code = new URL(event.request.url).searchParams.get("code");

  const result = await post<{ token: string }>(
    { path: "/auth" },
    { code, redirectUri: "http://localhost:3000/auth/redirect" }
  );

  setCookie("token", result.token, {
    path: "/",
    sameSite: "lax",
  });
  return new Response("Sign in with Discord", {
    status: 302,
    headers: { Location: "/" },
  });
}
