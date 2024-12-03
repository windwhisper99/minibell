<script lang="ts">
  import type { SvelteHTMLElements } from "svelte/elements";
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
