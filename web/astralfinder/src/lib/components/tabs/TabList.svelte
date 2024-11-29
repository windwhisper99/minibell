<script lang="ts">
  import type { SvelteHTMLElements } from "svelte/elements";
  import { getTabState } from "./context.svelte";
  import type { Snippet } from "svelte";

  let {
    children,
    ...rest
  }: SvelteHTMLElements["div"] & {
    children: Snippet;
  } = $props();
</script>

<div role="tablist" {...rest}>
  {@render children?.()}
</div>
