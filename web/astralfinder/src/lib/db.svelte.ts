import { Dexie, liveQuery } from "dexie";

export interface Party {
  id: number;
  name: string;
  created_at: Date;
  description?: string;
}

export const db = new Dexie("astralfinder");
db.version(1).stores({
  party: "++id,name,created_at",
});

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

export function queryParty() {
  return query<Party[]>(() => db.table("party").toArray());
}

export function createParty(name: string) {
  return db.table("party").add({ name, created_at: new Date() });
}
