import { browser } from "$app/environment";
import { createParty } from "$lib/db.svelte";
import { redirect } from "@sveltejs/kit";

export async function load() {
  if (browser) {
    const id = await createParty("New Party");
    throw redirect(302, `/${id}`);
  }

  throw redirect(302, "/");
}
