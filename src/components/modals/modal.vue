<script lang="ts" setup>
import { CloseIcon } from "@components/icons";
import { Transition, onBeforeUnmount, onMounted, ref } from "vue";

const props = defineProps({
  className: String,
  show: Boolean,
});

const emit = defineEmits(["close"]);

const modalContent = ref<HTMLElement | null>(null);

function onOutClick(ev: MouseEvent) {
  if (!modalContent.value) {
    return;
  }
  const target = ev.target as HTMLElement;

  if (!modalContent.value.contains(target)) {
    emit("close");
  }
}
console.log(props);

onMounted(() => {
  document.addEventListener("click", onOutClick);
});

onBeforeUnmount(() => {
  document.removeEventListener("click", onOutClick);
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
        class="modal-content bg-gray-200 relative p-4 pt-8 rounded-lg"
      >
        <button
          @click="$emit('close')"
          tabindex="0"
          class="bg-gray-300 absolute top-1 right-1 cursor-pointer rounded-0.5"
        >
          <CloseIcon class="h-5 w-5 fill-gray-700" />
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
