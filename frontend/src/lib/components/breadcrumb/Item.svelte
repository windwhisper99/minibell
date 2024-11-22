<script lang="ts">
  import { hstack } from "$lib/styled-system/patterns";
  import type { SvelteHTMLElements } from "svelte/elements";
  import RightCaret from "virtual:icons/mingcute/right-fill";
  import { css } from "$lib/styled-system/css";

  export type BreadcrumbItemProps = SvelteHTMLElements["a"] & {
    href: string;
    home?: boolean;
    current?: boolean;
    iconClass?: string;
  };

  let {
    href,
    children,
    current = false,
    home = false,
    iconClass,
    ...rest
  }: BreadcrumbItemProps = $props();
</script>

<li class={hstack()}>
  {#if !home}
    <RightCaret class={css({ w: "4", h: "4", color: "slate.500" })} />
  {/if}

  <a {...rest} {href} aria-current={current ? "page" : undefined}>
    {@render children?.()}
  </a>
</li>
