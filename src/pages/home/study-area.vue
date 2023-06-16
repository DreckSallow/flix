<script setup lang="ts">
import { inject, onBeforeUnmount, onMounted, ref } from "vue";
import { CardsIcon, BookIcon } from "@components/icons";
import StudyCards from "./study-cards.vue";
import { workspaceKeyProv, type TWorkspaceProvide } from "./provider";
import StudyDocs from "./study-docs.vue";

const workspaceData = inject<TWorkspaceProvide>(workspaceKeyProv, null);

const showModal = ref(true);

function listenKey(e: KeyboardEvent) {
  if (e.code === "KeyB" && e.ctrlKey) {
    showModal.value = !showModal.value;
  }
}

onMounted(() => {
  window.addEventListener("keydown", listenKey);
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", listenKey);
});

const renderPage = ref<"documents" | "cards">("cards");
</script>

<template>
  <div class="flex flex-row h-full w-full">
    <aside
      class="side-bar flex flex-col gap-4 p-2 h-full bg-gray-100"
      v-if="showModal"
    >
      <header class="text-lg font-semibold gap-4 text-center p-4">
        <h4>{{ workspaceData?.name ?? "No Have workspace" }}</h4>
      </header>
      <ul class="flex flex-col gap-4 pl-4" v-if="workspaceData">
        <li
          class="cursor-pointer flex flex-row gap-2"
          tabindex="0"
          @click="renderPage = 'cards'"
        >
          <CardsIcon class="h-5 w-5 fill-gray-600" />
          <span>Cards</span>
        </li>
        <li
          class="cursor-pointer flex flex-row gap-2"
          tabindex="0"
          @click="renderPage = 'documents'"
        >
          <BookIcon class="h-5 w-5 fill-gray-600" />
          <span>Docs</span>
        </li>
      </ul>
      <div class="text-sm" v-else>
        <button
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          @click.stop="$emit('create-workspace')"
        >
          Create a workspace
        </button>
      </div>
    </aside>
    <section
      class="section-content h-full bg-white"
      :class="{ expanded: !showModal }"
      v-if="workspaceData"
    >
      <StudyCards
        v-if="renderPage === 'cards'"
        :workspace-name="workspaceData?.name"
      />
      <StudyDocs
        v-if="renderPage === 'documents'"
        :workspace-name="workspaceData?.name"
      />
    </section>
    <section
      v-else
      class="section-content h-full bg-white grid place-content-center"
    >
      Does not have a workspace
    </section>
  </div>
</template>

<style scoped>
aside.side-bar {
  width: 170px;
}
section.section-content {
  width: calc(100% - 170px);
}

section.expanded {
  width: 100%;
}
</style>
