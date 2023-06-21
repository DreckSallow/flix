<script lang="ts" setup>
import { computed } from "vue";

type Direction = "top" | "left" | "right" | "bottom";

interface Props {
  direction: Direction;
}

const props = defineProps<Props>();

const rotateX = computed(() => {
  const rotation: { [key in Direction]: string } = {
    top: "-180deg",
    left: "-90deg",
    right: "90deg",
    bottom: "0deg",
  };
  return rotation[props.direction];
});
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    :style="{
      rotate: rotateX,
    }"
  >
    <path
      d="M16.293 9.293 12 13.586 7.707 9.293l-1.414 1.414L12 16.414l5.707-5.707z"
    ></path>
  </svg>
</template>

<style scoped>
svg {
  transition: rotate 300ms ease;
}
</style>
