import { get } from "$lib/fetch";
import type { ServerLoad } from "@sveltejs/kit";

export interface Member {
  id: number;
  name: string;
  avatar: string;
}

export const load: ServerLoad = async (evt) => {
  const result = await get<{ authUrl: string; member?: Member }>(evt, {
    path: "/auth",
    query: { redirect_uri: "http://localhost:5173/auth/callback" },
  });

  return {
    auth: result,
  };
};
