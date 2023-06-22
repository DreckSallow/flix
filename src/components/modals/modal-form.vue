<script lang="ts" setup>
import { ref, watchEffect } from "vue";
import Modal from "./modal.vue";
import { vFocus } from "../../directives";

interface IEvents {
  (e: "accept", input: string): void;
  (e: "cancel", input: string): void;
}

defineEmits<IEvents>();

interface IProps {
  input: string;
  show: boolean;
  titleLabel: string;
}

const props = defineProps<IProps>();
const modalInput = ref(props.input);

watchEffect(() => {
  modalInput.value = props.input;
  if (!props.show) modalInput.value = "";
});
</script>

<template>
  <Modal :show="show">
    <div class="flex flex-col gap-2">
      <header>{{ titleLabel }}</header>
      <input
        v-focus
        class="border border-strong border-solid p-1 rounded-md"
        type="text"
        tabindex="1"
        v-model="modalInput"
        @keyup.enter="$emit('accept', modalInput)"
      />
      <div class="flex flex-row gap-4 justify-end">
        <button
          class="bg-strong text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          tabindex="2"
          @click="$emit('accept', modalInput)"
        >
          Ok
        </button>
        <button
          tabindex="3"
          class="bg-strong text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          @click="$emit('cancel', modalInput)"
        >
          Cancel
        </button>
      </div>
    </div>
  </Modal>
</template>
