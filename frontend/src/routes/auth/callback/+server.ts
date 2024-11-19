import { redirect } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
import { post } from "$lib/fetch";

export const GET: RequestHandler = async (evt) => {
  const code = evt.url.searchParams.get("code");
  if (!code) return redirect(302, "/");
  console.log(code);

  const token = await post<{ token: string }>(
    evt,
    { path: "/auth" },
    { code, redirectUri: "http://localhost:5173/auth/callback" }
  );
  evt.cookies.set("token", token.token, { path: "/" });

  return redirect(302, "/");
};
