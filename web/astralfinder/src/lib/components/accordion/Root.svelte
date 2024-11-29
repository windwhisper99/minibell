<script lang="ts">
  import type { Snippet } from "svelte";
  import type { SvelteHTMLElements } from "svelte/elements";
  import { setAccordionState } from "./context.svelte";

  type AccordionProps = {
    value?: string;
    selectOnFocus?: boolean;
    children?: Snippet;
  } & SvelteHTMLElements["div"];

  let {
    value = $bindable(),
    selectOnFocus = false,
    children,
    ...rest
  }: AccordionProps = $props();

  setAccordionState({
    input: () => value ?? "",
    update: (next) => {
      value = next ?? "";
    },
    selectOnFocus: () => selectOnFocus,
  });
</script>

<div {...rest}>
  {@render children?.()}
</div>
