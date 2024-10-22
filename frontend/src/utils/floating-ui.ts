// import {} from '@floating-ui/dom'

import {
  computePosition,
  ComputePositionConfig,
  FloatingElement,
  flip,
  shift,
  ShiftOptions,
  offset,
  autoUpdate,
  Placement,
} from "@floating-ui/dom";
import { createEffect, createSignal, JSX, onCleanup } from "solid-js";

export interface UseFloatingConfig {
  placement?: ComputePositionConfig["placement"];
  shift?: ShiftOptions;
  offset?: number;
}

export function useFloating(config?: UseFloatingConfig) {
  const [ref, setRef] = createSignal<FloatingElement>();
  const [floating, setFloating] = createSignal<FloatingElement>();
  const [floatingStyle, setFloatingStyle] = createSignal<JSX.CSSProperties>();
  const [placement, setPlacement] = createSignal<Placement>();
  const [open, setOpen] = createSignal(false);

  let cleanup: () => void;

  createEffect(() => {
    cleanup && cleanup();

    const refEle = ref();
    const floatingEle = floating();

    if (refEle && floatingEle) {
      cleanup = autoUpdate(refEle, floatingEle, () => {
        computePosition(refEle, floatingEle, {
          placement: config?.placement,
          middleware: [
            flip({ padding: { top: 64 } }),
            shift(config?.shift),
            config?.offset ? offset(config.offset) : undefined,
          ],
        }).then((pos) => {
          setPlacement(pos.placement);
          setFloatingStyle({
            left: `${pos.x}px`,
            top: `${pos.y}px`,
          });
        });
      });
    }
  });

  onCleanup(() => {
    cleanup && cleanup();
  });

  return {
    open,
    placement,
    setOpen,
    toggle() {
      setOpen(!open());
    },

    floatingStyle,

    // Reference
    refs: {
      root: setRef,
      floating: setFloating,
    },
  };
}
