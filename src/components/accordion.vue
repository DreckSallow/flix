<script lang="ts" setup>
import { computed, ref } from "vue";
import Chevron from "./icons/chevron.vue";

interface IProps {
  custom: boolean;
}

defineProps<IProps>();

const isOpen = ref(false);

const arrowDirection = computed(() => {
  return isOpen.value ? "top" : "bottom";
});
</script>

<template>
  <div class="accordion">
    <div @click="isOpen = !isOpen" class="cursor-pointer p-2 gap-2">
      <slot name="header"></slot>
      <Chevron :direction="arrowDirection" class="h-6 w-6 fill-gray-400" />
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
