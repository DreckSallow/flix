<script lang="ts" setup>
import { computed, ref, watch } from "vue";
import Chevron from "./icons/chevron.vue";

interface IProps {
  custom: boolean;
  hasContent?: boolean;
}

const props = defineProps<IProps>();

const isOpen = ref(false);

watch(
  () => props.hasContent,
  () => {
    isOpen.value = props.hasContent ?? false;
  }
);

const arrowDirection = computed(() => {
  return isOpen.value ? "top" : "bottom";
});

function setOpen() {
  if (props.hasContent) isOpen.value = !isOpen.value;
}
</script>

<template>
  <div class="accordion">
    <div @click="setOpen" class="accordion-header cursor-pointer p-2 gap-2">
      <slot name="header"></slot>
      <button class="rounded-sm hover:bg-#ececec cursor-pointer">
        <Chevron :direction="arrowDirection" class="h-6 w-6 fill-gray-400" />
      </button>
    </div>
    <div v-if="!custom && isOpen" class="p-2">
      <slot />
    </div>
    <slot name="custom" :show="isOpen" v-else></slot>
  </div>
</template>

<style scoped>
div.accordion div:first-child {
  display: grid;
  grid-template-columns: 1fr 30px;
  justify-items: center;
}
</style>
