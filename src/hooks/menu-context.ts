import { Ref, ref } from "vue";

export interface IMouseMenu {
  x: number;
  y: number;
}

export interface IMenuInfo<T> {
  data: T;
  mouse: IMouseMenu;
}

export type Updater<S> = S | ((s: S) => S);

export function useMenuContext<T = null>(showInit: boolean, data: T) {
  const show = ref(showInit);

  const menuInfo = ref<IMenuInfo<T>>({
    mouse: {
      x: 0,
      y: 0,
    },
    data: data,
  }) as Ref<IMenuInfo<T>>;

  function updateInfo(updater: Updater<IMenuInfo<T>>) {
    if (typeof updater === "function") {
      menuInfo.value = updater(menuInfo.value);
    } else {
      menuInfo.value = updater;
    }
  }

  function updateShow(updater: Updater<boolean>) {
    if (typeof updater === "function") {
      show.value = updater(show.value);
    } else {
      show.value = updater;
    }
  }

  return {
    info: menuInfo,
    show,
    updateInfo,
    updateShow,
    getShow() {
      return show.value;
    },
    getInfo() {
      return menuInfo.value;
    },
    style() {
      return {
        top: menuInfo.value.mouse.y + "px",
        left: menuInfo.value.mouse.x + "px",
      };
    },
  };
}
