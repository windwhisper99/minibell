import { Dexie, liveQuery } from "dexie";
import { createId } from "@paralleldrive/cuid2";
import { scheduling } from "./finder";

export interface Party {
  id: string;
  name: string;
  combination: string;
  created_at: Date;
  members: Record<string, Member>;

  combinations?: Combination[];
}

export interface Member {
  id: string;
  name: string;
  jobs: Record<string, number>;
}

export interface Combination {
  assigned: { id: string; job: string }[];
  score: number;
}

export const db = new Dexie("astralfinder");
db.version(1).stores({
  party: "id,name,created_at,*members",
});

db.open();

function query<T>(query: () => Promise<T>): { data: T | null } {
  let result = $state<T | null>(null);
  const sub = liveQuery<T>(query).subscribe((data) => {
    result = data;
  });

  $effect(() => {
    return () => sub.unsubscribe();
  });

  return {
    get data() {
      return result;
    },
  };
}

export function queryParties() {
  return query<Party[]>(() => db.table("party").toArray());
}

export function queryPartyById(id: string) {
  return query<Party>(() => db.table("party").get(id));
}

export async function updateParty(
  id: string,
  updater: (party: Party) => Partial<Party>
) {
  const party = await db.table<Party>("party").get(id);
  if (!party) return;

  const newParty = updater(party);
  await db.table<Party>("party").update(id, newParty);
}

export async function addMemberToParty(partyId: string, name: string) {
  const memberId = createId();
  await updateParty(partyId, (party) => ({
    members: {
      ...party.members,
      [memberId]: {
        id: memberId,
        name,
        jobs: {},
      },
    },
  }));

  return memberId;
}

export function createParty(name: string) {
  const firstMemberId = createId();
  return db.table<Party>("party").add({
    id: createId(),
    name,
    created_at: new Date(),
    combination: "standard_light",
    members: {
      [firstMemberId]: {
        id: firstMemberId,
        name: "Member 1",
        jobs: {},
      },
    },
  });
}

export async function schedule(id: string) {
  const party = await db.table<Party>("party").get(id);
  if (!party) return;

  const combinations = await scheduling(party);
  await db.table<Party>("party").update(id, { combinations });
}

export function deleteParty(id: string) {
  return db.table<Party>("party").delete(id);
}
