<script lang="ts">
  import type { SvelteHTMLElements } from "svelte/elements";
  import { getAccordionState } from "./context.svelte";
  import { hammer } from "../utils.svelte";

  type AccordionHeaderProps = {
    id: string;
    disabled?: boolean;
  } & SvelteHTMLElements["button"];

  let { id, children, disabled, ...rest }: AccordionHeaderProps = $props();

  const state = getAccordionState();
  let trigger: HTMLButtonElement;

  $effect(() => {
    state.addItem(id, trigger);
    return () => {
      state.removeItem(id);
    };
  });

  $effect(() => {
    state.disabledItem(id, disabled ?? false);
  });
</script>

<h3>
  <button
    id="accordianheader:{id}"
    aria-expanded={state.selected === id}
    aria-controls="accordianpanel:{id}"
    aria-disabled={disabled ? true : false}
    bind:this={trigger}
    onkeydown={(event) => {
      rest.onkeydown?.(event);
      if (event.defaultPrevented) return;

      switch (event.key) {
        case "ArrowDown":
          state.focusNextItem();
          break;
        case "ArrowUp":
          state.focusPreviousItem();
          break;
        case "Home":
          state.focusFirstItem();
          break;
        case "End":
          state.focusLastItem();
          break;
        case "Enter":
        case " ":
          state.selectItem(id);
          break;

        default:
          break;
      }
    }}
    use:hammer={() => {
      state.selectItem(id);
    }}
    onfocusin={(event) => {
      rest.onfocusin?.(event);
      if (event.defaultPrevented) return;
      state.onFocusIn(id);
    }}
    onfocusout={(event) => {
      rest.onfocusout?.(event);
      if (event.defaultPrevented) return;
      state.onFocusOut(id);
    }}
    {...rest}
  >
    {@render children?.()}
  </button>
</h3>
