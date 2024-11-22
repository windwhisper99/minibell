import { getContext, setContext } from "svelte";
import { idArena } from "../utils.svelte";

export function createBreadcrumb() {
  const idAlloc = idArena();
  const items: number[] = $state([]);

  return {
    get items() {
      return items;
    },
    add() {
      const id = idAlloc.next();
      items.push(id);
      return id;
    },
    remove(id: number) {
      idAlloc.remove(id);
      const idx = items.indexOf(id);
      if (idx !== -1) {
        items.splice(idx, 1);
      }
    },
    isHome(id: number) {
      return items[0] === id;
    },
    isCurrent(id: number) {
      return items[items.length - 1] === id;
    },
  };
}

const ctxKey = Symbol("breadcrumb-context");

export function setBreadcrumbContext() {
  return setContext(ctxKey, createBreadcrumb());
}

export function getBreadcrumbContext() {
  return getContext<ReturnType<typeof setBreadcrumbContext>>(ctxKey);
}
