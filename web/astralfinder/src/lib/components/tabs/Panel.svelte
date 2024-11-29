<script lang="ts">
  import type { SvelteHTMLElements } from "svelte/elements";
  import { getTabState } from "./context.svelte";

  export type PanelProps = SvelteHTMLElements["div"] & {
    id: string;
  };

  let { children, id, ...rest }: PanelProps = $props();

  const state = getTabState();
</script>

<div
  role="tabpanel"
  id={`tabpanel:${id}`}
  aria-labelledby={`tab:${id}`}
  aria-hidden={state.selected !== id}
  {...rest}
>
  {#if state.selected === id}
    {@render children?.()}
  {/if}
</div>
