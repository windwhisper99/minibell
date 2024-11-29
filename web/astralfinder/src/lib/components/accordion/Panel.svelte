<script lang="ts">
  import type { SvelteHTMLElements } from "svelte/elements";
  import { getAccordionState } from "./context.svelte";

  export type PanelProps = SvelteHTMLElements["div"] & {
    id: string;
  };

  let { children, id, ...rest }: PanelProps = $props();

  const state = getAccordionState();
</script>

<div
  role="region"
  id={`accordionpanel:${id}`}
  aria-labelledby={`accordionheader:${id}`}
  aria-hidden={state.selected !== id}
  hidden={state.selected !== id}
  {...rest}
>
  {#if state.selected === id}
    {@render children?.()}
  {/if}
</div>
