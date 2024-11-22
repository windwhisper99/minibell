import { getContext, setContext } from "svelte";
import {
  createTransformation,
  type CreateTransformationProps,
} from "../utils.svelte";

const ctxKey = Symbol("tab-context");

export type CreateTabsProps = CreateTransformationProps<string | null> & {
  selectOnFocus: () => boolean;
};

function createTabs(props: CreateTabsProps) {
  const selected = createTransformation<string | null>(props);
  const tabLists: string[] = [];
  const tabDoms: Record<string, HTMLElement> = {};
  const tabDisabled: Record<string, boolean> = {};
  let focusedTabIndex: number | null = null;

  function focusTab(value: string) {
    const dom = tabDoms[value];
    dom?.focus();

    if (props.selectOnFocus()) selected.value = value;
  }

  function nextIndex(index: number) {
    return index + 1 < tabLists.length ? index + 1 : 0;
  }

  function previousIndex(index: number) {
    return index - 1 >= 0 ? index - 1 : tabLists.length - 1;
  }

  return {
    get selected() {
      return selected.value;
    },
    change(value: string) {
      selected.value = value;
    },

    addTab(tab: string, dom: HTMLElement) {
      tabLists.push(tab);
      tabDoms[tab] = dom;
    },

    removeTab(tab: string) {
      const index = tabLists.indexOf(tab);
      if (index === -1) return;

      tabLists.splice(index, 1);
      delete tabDoms[tab];
    },

    disabledTab(tab: string, disabled: boolean) {
      tabDisabled[tab] = disabled;
    },

    selectTab(value: string) {
      if (tabLists.includes(value)) {
        selected.value = value;
      }
    },

    selectNextTab() {
      if (selected.value === null) return;

      const index = tabLists.indexOf(selected.value);
      selected.value = tabLists[nextIndex(index)];
    },

    selectPreviousTab() {
      if (selected.value === null) return;

      const index = tabLists.indexOf(selected.value);
      selected.value = tabLists[previousIndex(index)];
    },

    onFocusIn(value: string) {
      const index = tabLists.indexOf(value);
      if (index === -1) return;
      focusedTabIndex = index;
    },

    onFocusOut(value: string) {
      if (selected.value === value) {
        focusedTabIndex = null;
      }
    },

    focusTab(value: string) {
      focusTab(value);
    },

    focusNextTab() {
      if (focusedTabIndex === null) return;
      focusTab(tabLists[nextIndex(focusedTabIndex)]);
    },

    focusPreviousTab() {
      if (focusedTabIndex === null) return;
      focusTab(tabLists[previousIndex(focusedTabIndex)]);
    },

    focusFirstTab() {
      focusTab(tabLists[0]);
    },

    focusLastTab() {
      focusTab(tabLists[tabLists.length - 1]);
    },
  };
}

export function setTabState(props: CreateTabsProps) {
  return setContext(ctxKey, createTabs(props));
}

export function getTabState() {
  return getContext<ReturnType<typeof setTabState>>(ctxKey);
}
