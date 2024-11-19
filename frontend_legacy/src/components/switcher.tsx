import { createEffect, createSignal, JSX } from "solid-js";

export function Switcher(props: {
  class?: string;
  children: JSX.Element;
  name: string;
  onChange?: (toggle: boolean) => void;
  checked?: boolean;
}) {
  const [state, setState] = createSignal(props.checked || false);

  createEffect(() => {
    if (props.onChange) props.onChange(state());
  });

  return (
    <div
      class={state() ? "switcher active" : "switcher"}
      onClick={() => setState((e) => !e)}
    >
      <input
        id={props.name}
        type="checkbox"
        name={props.name}
        class="hidden"
        checked={state()}
      />

      <div class="switcher-btn">
        <span class="switcher-dot"></span>
      </div>

      <div class="ml-4 select-none">{props.children}</div>
    </div>
  );
}
