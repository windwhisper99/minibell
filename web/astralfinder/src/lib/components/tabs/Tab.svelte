<script lang="ts">
  import type { SvelteHTMLElements } from "svelte/elements";
  import { getTabState as getTabState } from "./context.svelte";

  type TabProps = SvelteHTMLElements["button"] & {
    id: string;
    disabled?: boolean;
  };

  let { children, id, disabled, ...rest }: TabProps = $props();

  const state = getTabState();

  let trigger: HTMLButtonElement;
  $effect(() => {
    state.addTab(id, trigger);

    return () => {
      state.removeTab(id);
    };
  });

  $effect(() => {
    state.disabledTab(id, disabled ?? false);
  });
</script>

<button
  type="button"
  role="tab"
  id={`tab:${id}`}
  aria-selected={state.selected === id}
  tabindex={state.selected === id ? 0 : -1}
  aria-controls={`tabpanel:${id}`}
  aria-disabled={disabled ? true : false}
  bind:this={trigger}
  onkeydown={(event) => {
    rest.onkeydown?.(event);
    if (event.defaultPrevented) return;

    switch (event.key) {
      case "ArrowRight":
        state.focusNextTab();
        break;
      case "ArrowLeft":
        state.focusPreviousTab();
        break;
      case "Home":
        state.focusFirstTab();
        break;
      case "End":
        state.focusLastTab();
        break;
      case "Enter":
      case " ":
        state.selectTab(id);
        break;

      default:
        break;
    }
  }}
  onpointerdown={(event) => {
    rest.onpointerdown?.(event);
    if (event.defaultPrevented) return;

    if (event.button === 0) {
      state.selectTab(id);
    }
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
