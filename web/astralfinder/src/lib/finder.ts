import init, { resolve } from "astralfinder";
import type { Combination, Party } from "./db.svelte";
import { combination, jobs } from "./jobs";

export async function scheduling(party: Party) {
  await init();
  const result: { combinations: Combination[] } = resolve({
    roles: combination[party.combination].roles,
    members: Object.values(party.members),
  });

  return result.combinations;
}
