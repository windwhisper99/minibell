import init, { resolve } from "astralfinder";
import type { Combination, Party } from "./db.svelte";
import { combination } from "./jobs";
import members from "./demo.json";

export async function scheduling(party: Party) {
  await init();

  const result: { combinations: Combination[]; assigned: any } = resolve({
    roles: combination[party.combination].roles,
    members: members,
    min_members: 3,
    duration: 120,
  });

  // console.log(members);
  console.log(result);

  return result.combinations;
}
