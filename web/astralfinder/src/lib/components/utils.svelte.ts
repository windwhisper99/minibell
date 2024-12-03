import type { ActionReturn } from "svelte/action";
import type { Manager } from "hammerjs";

export interface CreateTransformationProps<T> {
  input: () => T;
  update?: (value: T) => void;
}

export function createTransformation<T>(config: CreateTransformationProps<T>) {
  const output = $derived.by(() => {
    return config.input();
  });

  return {
    get value() {
      return output;
    },
    set value(value: T) {
      if (config.update) config.update(value);
    },
  };
}

export function idArena() {
  let idCounter = 0;
  return {
    next() {
      return idCounter++;
    },
    remove(id: number) {
      idCounter = Math.max(idCounter - 1, id);
    },
  };
}

export function hammer(node: HTMLElement, ontap: () => void): ActionReturn {
  let hammer: any | null = null;
  import("hammerjs").then((Hammer) => {
    hammer = new Hammer.default(node);
    hammer.on("tap", (e: any) => {
      ontap();
    });
  });

  return {
    destroy() {
      hammer?.destroy();
    },
  };
}
