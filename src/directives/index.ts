export interface Directive {
  [k: string]: (el: HTMLElement) => void;
}

export const vFocus: Directive = {
  mounted: (el) => el.focus(),
};
