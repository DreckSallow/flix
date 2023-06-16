<script lang="ts" setup>
import { CloseIcon, ErrorIcon, AlertIcon, InfoIcon } from "./icons";
import { TypeNotification } from "../state";

interface IProps {
  keyId: number | string;
  text: string;
  title: string;
  type: TypeNotification;
}

defineProps<IProps>();
</script>

<template>
  <div class="notification relative overflow-auto p-2">
    <header class="grid gap-8px">
      <div class="flex-center">
        <AlertIcon v-if="type === 'alert'" class="fill-orange-400 h-5 w-5" />
        <ErrorIcon v-else-if="type === 'error'" class="fill-red-400 h-5 w-5" />
        <InfoIcon v-else-if="type === 'info'" class="fill-blue-400 h-5 w-5" />
      </div>
      <h4 class="font-600 text-sm">{{ title }}</h4>
      <button @click="$emit('close', keyId)" class="flex-center">
        <CloseIcon class="fill-gray-600 h-5 w-5 cursor-pointer" />
      </button>
    </header>
    <section class="text-xs p-2">
      {{ text }}
    </section>
  </div>
</template>

<style scoped>
div.notification {
  grid-template-rows: 40px 80px;
}

div.notification > header {
  grid-template-columns: 40px 1fr 40px;
  align-items: center;
}
</style>
