<script lang="ts" setup>
import { Transition } from "vue";
import { CloseIcon } from "@components/icons";
import { useOutClick } from "../../hooks";

interface IProps {
  show: boolean;
}

defineProps<IProps>();

const emit = defineEmits(["close"]);

const modalContent = useOutClick(() => {
  emit("close");
});
</script>

<template>
  <div
    class="modal-container fixed top-0 bottom-0 left-0 right-0 flex-center z-99"
    :class="{ 'modal-show': show }"
  >
    <Transition name="mdl">
      <div
        tabindex="0"
        @keyup.esc="$emit('close')"
        v-if="show"
        ref="modalContent"
        class="modal-content relative p-4 pt-8 rounded-lg over-shadow bg-accent"
      >
        <button
          @click="$emit('close')"
          tabindex="0"
          class="absolute top-1 right-1 cursor-pointer rounded-0.5 bg-strong"
        >
          <CloseIcon class="h-5 w-5 fill-white" />
        </button>
        <slot />
      </div>
    </Transition>
  </div>
</template>

<style scoped>
div.modal-container {
  pointer-events: none;
}

div.modal-show {
  pointer-events: all;
}

.mdl-enter-active,
.mdl-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.mdl-enter-from,
.mdl-leave-to {
  transform: translateY(-30px);
  opacity: 0;
}
</style>
