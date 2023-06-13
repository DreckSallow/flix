<script lang="ts" setup>
type TOption = {
  text: string;
  id?: string | number;
  options?: TOption[];
};

interface IProps {
  options: TOption[];
}

defineProps<IProps>();

const emit = defineEmits<{
  (e: "select-opt", id: string | number): void;
}>();

function selectOpt(e: MouseEvent) {
  const el = e.target as HTMLElement | null;
  if (el && el.tagName === "LI") {
    emit("select-opt", el.getAttribute("opt-id") as string);
  }
}
</script>

<template>
  <div class="menu-container absolute z-99 text-sm">
    <ul class="menu flex flex-col items-start rounded-lg" @click="selectOpt">
      <li
        class="px-2 py-1 rounded-md"
        v-for="opt in options"
        :opt-id="opt.id ?? opt.text"
      >
        {{ opt.text }}
        <ul v-if="opt.options?.length ?? 0 > 0" class="submenu">
          <li
            class="px-2 py-1 rounded-md"
            v-for="subOpt in opt.options"
            :opt-id="subOpt.id ?? subOpt.text"
          >
            {{ subOpt.text }}
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>

<style scoped>
ul {
  background-color: inherit;
}
ul li {
  background-color: inherit;
  position: relative;
  transition: filter 0.1s;
}

ul li:hover {
  cursor: pointer;
  filter: brightness(90%);
}

li:hover > ul {
  position: absolute;
  right: 0px;
}
</style>
