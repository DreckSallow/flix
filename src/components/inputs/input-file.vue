<script lang="ts" setup>
interface IProps {
  text: string;
  file: string;
}

defineProps<IProps>();
</script>

<template>
  <div class="m-2 flex flex-col gap-2">
    <span
      class="input-file p-2 text-xs text-center block"
      @click="$emit('open')"
    >
      {{ text }}
    </span>
    <span class="block">
      {{ file }}
    </span>
  </div>
</template>

<style scoped>
span.input-file {
  border: 2px rgba(174, 172, 172, 0.589) dashed;
  cursor: pointer;
}
</style>
