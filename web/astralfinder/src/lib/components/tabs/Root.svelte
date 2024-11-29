<script lang="ts">
  import { type Snippet } from "svelte";
  import { setTabState } from "./context.svelte";

  let {
    value = $bindable(),
    selectOnFocus = false,
    children,
  }: {
    value: string;
    selectOnFocus?: boolean;
    children?: Snippet;
  } = $props();

  const tabs = setTabState({
    input: () => value,
    update: (next) => {
      value = next ?? "";
    },
    selectOnFocus: () => selectOnFocus,
  });

  export function selectTab(value: string) {
    tabs.selectTab(value);
  }

  export function selectNext() {
    tabs.selectNextTab();
  }

  export function selectPrevious() {
    tabs.selectPreviousTab();
  }
</script>

{@render children?.()}
