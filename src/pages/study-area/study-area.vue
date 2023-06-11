<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";
import StudyCards from "./study-cards.vue";
import { CardsIcon, BookIcon } from "@components/icons";

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

type TRendePage = "documents" | "cards";

const renderPage = ref<TRendePage>("cards");

function changeRenderPage(page: TRendePage) {
  renderPage.value = page;
}
</script>

<template>
  <div class="flex flex-row h-full w-full">
    <aside
      class="side-bar flex flex-col gap-4 p-2 h-full bg-gray-100"
      v-if="showModal"
    >
      <header class="text-lg font-semibold gap-4 text-center p-4">
        <h4>{{ $route.params.area }}</h4>
      </header>
      <ul class="flex flex-col gap-4 pl-4">
        <li
          page="cards"
          class="cursor-pointer flex flex-row gap-2"
          tabindex="0"
          @click="changeRenderPage('cards')"
        >
          <CardsIcon class="h-5 w-5 fill-gray-600" />
          <span>Cards</span>
        </li>
        <li
          page="notes"
          class="cursor-pointer flex flex-row gap-2"
          tabindex="0"
          @click="changeRenderPage('documents')"
        >
          <BookIcon class="h-5 w-5 fill-gray-600" />
          <span>Docs</span>
        </li>
      </ul>
    </aside>
    <section
      class="section-content h-full bg-white"
      :class="{ expanded: !showModal }"
    >
      <StudyCards
        v-if="renderPage === 'cards'"
        :workspace-name="$route.params.area"
      />
      <div class="bg-orange-400" v-if="renderPage === 'documents'">NOTES</div>
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
