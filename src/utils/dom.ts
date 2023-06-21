export function getAttribute(
  el: HTMLElement | EventTarget | null,
  att: string
) {
  return el instanceof HTMLElement
    ? el.getAttribute(att)
    : (el as HTMLElement | null)?.getAttribute(att);
}
