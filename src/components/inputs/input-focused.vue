<script lang="ts" setup>
import { onMounted, ref } from "vue";

const inputRef = ref<HTMLInputElement | null>(null);

defineProps(["value"]);
defineEmits(["update:value"]);

onMounted(() => {
  inputRef.value?.focus();
});
</script>

<template>
  <input
    ref="inputRef"
    class="border-b border-gray-600/80 border-solid p-1"
    :value="value"
    @input="$emit('update:value', ($event.target as HTMLInputElement).value)"
  />
</template>
