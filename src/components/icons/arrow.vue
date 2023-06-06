<script lang="ts" setup>
import { computed } from "vue";

type ArrowDirection = "top" | "left" | "right" | "bottom";

interface Props {
  direction: ArrowDirection;
  class: string;
}

const props = defineProps<Props>();

const rotateX = computed(() => {
  const rotation: { [key in ArrowDirection]: string } = {
    top: "90deg",
    left: "0deg",
    right: "180deg",
    bottom: "-90deg",
  };
  return rotation[props.direction];
});
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :class="class"
    viewBox="0 0 24 24"
    :style="{
      rotate: rotateX,
    }"
  >
    <path
      d="M12.707 17.293 8.414 13H18v-2H8.414l4.293-4.293-1.414-1.414L4.586 12l6.707 6.707z"
    ></path>
  </svg>
</template>
