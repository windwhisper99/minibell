<script lang="ts">
  import type { SvelteHTMLElements } from "svelte/elements";
  import { hstack } from "$lib/styled-system/patterns";
  import { cx } from "$lib/styled-system/css";

  export type BreadcrumbItem = {
    id: string;
    name: string;
  };

  export type BreadcrumbProps = SvelteHTMLElements["nav"] & {
    olProps?: SvelteHTMLElements["ol"];
  };

  let { olProps, children, ...rest }: BreadcrumbProps = $props();
</script>

<nav {...rest}>
  <ol {...olProps} class={cx(hstack(), olProps?.class)}>
    {@render children?.()}
  </ol>
</nav>
