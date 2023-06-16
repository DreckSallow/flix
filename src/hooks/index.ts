export * from "./input";

import { onBeforeUnmount, onMounted, ref } from "vue";

export function useEventListener<
  T extends EventTarget,
  K extends keyof HTMLElementEventMap,
  E extends Event
>(target: T | null, event: K, cb: (ev: E) => unknown) {
  onMounted(() => {
    target?.addEventListener(event, cb as EventListener);
  });

  onBeforeUnmount(() => {
    target?.removeEventListener(event, cb as EventListener);
  });
}

export function useOutClick(cb: (e: MouseEvent) => unknown) {
  const element = ref<HTMLElement | null>(null);

  function outClick(e: MouseEvent) {
    if (!element.value) {
      return;
    }
    const subTarget = e.target as HTMLElement;

    if (!element.value.contains(subTarget)) {
      cb(e);
    }
  }

  useEventListener(document, "click", outClick);
  return element;
}
