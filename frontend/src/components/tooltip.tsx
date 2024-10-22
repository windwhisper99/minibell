import { JSX, Setter, Show } from "solid-js";
import { useFloating } from "~/utils/floating-ui";

export function Tooltip(props: {
  text: JSX.Element;
  children: (ctx: {
    ref: Setter<HTMLElement | undefined>;
    setOpen: (open: boolean) => void;
  }) => JSX.Element;
}) {
  const { open, refs, floatingStyle, setOpen } = useFloating({
    placement: "top",
    offset: 2,
  });

  return (
    <>
      {props.children({ ref: refs.root, setOpen })}
      <Show when={open()}>
        <span ref={refs.floating} style={floatingStyle()} class="tooltip">
          {props.text}
        </span>
      </Show>
    </>
  );
}
