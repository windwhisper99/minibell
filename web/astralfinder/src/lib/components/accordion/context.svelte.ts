import { getContext, setContext } from "svelte";
import {
  createTransformation,
  type CreateTransformationProps,
} from "../utils.svelte";

const ctxKey = Symbol("accordian-context");

export type CreateAccordianProps = CreateTransformationProps<string | null> & {
  selectOnFocus: () => boolean;
};

function createAccordion(props: CreateAccordianProps) {
  const selected = createTransformation<string | null>(props);
  const itemLists: string[] = [];
  const itemDoms: Record<string, HTMLElement> = {};
  const itemDisabled: Record<string, boolean> = {};
  let focusedIndex: number | null = null;

  function focusAccordion(value: string) {
    const dom = itemDoms[value];
    dom?.focus();

    if (props.selectOnFocus()) selected.value = value;
  }

  function nextIndex(index: number) {
    return index + 1 < itemLists.length ? index + 1 : 0;
  }

  function previousIndex(index: number) {
    return index - 1 >= 0 ? index - 1 : itemLists.length - 1;
  }

  return {
    get selected() {
      return selected.value;
    },
    change(value: string) {
      selected.value = value;
    },

    addItem(id: string, dom: HTMLElement) {
      itemLists.push(id);
      itemDoms[id] = dom;
    },

    removeItem(tab: string) {
      const index = itemLists.indexOf(tab);
      if (index === -1) return;

      itemLists.splice(index, 1);
      delete itemDoms[tab];
    },

    disabledItem(tab: string, disabled: boolean) {
      itemDisabled[tab] = disabled;
    },

    selectItem(value: string) {
      if (itemLists.includes(value)) {
        if (selected.value === value) {
          selected.value = "";
        } else {
          selected.value = value;
        }
      }
    },

    selectNextItem() {
      if (selected.value === null) return;

      const index = itemLists.indexOf(selected.value);
      selected.value = itemLists[nextIndex(index)];
    },

    selectPreviousItem() {
      if (selected.value === null) return;

      const index = itemLists.indexOf(selected.value);
      selected.value = itemLists[previousIndex(index)];
    },

    onFocusIn(value: string) {
      const index = itemLists.indexOf(value);
      if (index === -1) return;
      focusedIndex = index;
    },

    onFocusOut(value: string) {
      if (selected.value === value) {
        focusedIndex = null;
      }
    },

    focusItem(value: string) {
      focusAccordion(value);
    },

    focusNextItem() {
      if (focusedIndex === null) return;
      focusAccordion(itemLists[nextIndex(focusedIndex)]);
    },

    focusPreviousItem() {
      if (focusedIndex === null) return;
      focusAccordion(itemLists[previousIndex(focusedIndex)]);
    },

    focusFirstItem() {
      focusAccordion(itemLists[0]);
    },

    focusLastItem() {
      focusAccordion(itemLists[itemLists.length - 1]);
    },
  };
}

export function setAccordionState(props: CreateAccordianProps) {
  return setContext(ctxKey, createAccordion(props));
}

export function getAccordionState() {
  return getContext<ReturnType<typeof setAccordionState>>(ctxKey);
}
