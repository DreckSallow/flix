import { ref, watch } from "vue";

export type OnInputFn = (input: string) => void;

export function useCheckInput(i: string) {
  const input = ref(i);
  const listeners: OnInputFn[] = [];

  function onInput(fn: OnInputFn) {
    listeners.push(fn);
  }

  function set(i: string) {
    input.value = i;
  }

  watch(input, () => {
    listeners.forEach((fn) => fn(input.value));
  });

  return {
    input,
    onInput,
    set,
  };
}
