<script lang="ts" setup>
import { ref, watch } from "vue";
import { IDeckResponse } from "../../types";
import { ArrowIcon } from "@components/icons";
import DeckSection from "./deck-section.vue";
import { invoke } from "@tauri-apps/api";

import { Modal } from "@components/modals";

interface Props {
  workspaceName: string;
}

const props = defineProps<Props>();

const decks = ref<IDeckResponse[]>([]);

watch(
  () => props.workspaceName,
  () => {
    invoke("get_decks_handler", {
      workspaceName: props.workspaceName,
    })
      .then((d) => {
        decks.value = d as Array<IDeckResponse>;
      })
      .catch((e) => {
        console.error("study-area: ", e);
      });
  },
  {
    immediate: true,
  }
);

const current_deck = ref<IDeckResponse | null>(null);

const showModal = ref(false);

const createDeckValue = ref("");

function createDeck() {
  if (createDeckValue.value.length == 0) {
    return;
  }

  if (decks.value.some(({ name }) => name === createDeckValue.value)) {
    return;
  }

  invoke("create_deck_handler", {
    workspaceName: props.workspaceName,
    deckName: createDeckValue.value,
  })
    .then((deck) => {
      decks.value.push(deck as IDeckResponse);
    })
    .catch((e) => {
      console.log("CREATE DECK ERROR: ", e);
    })
    .finally(() => {
      showModal.value = false;
    });
}
</script>

<template>
  <div class="deck-container w-full h-full relative">
    <span
      class="absolute top-3 left-2 cursor-pointer"
      @click="current_deck = null"
      v-if="current_deck"
    >
      <ArrowIcon class="h-5 w-10 fill-gray-600" direction="left" />
    </span>
    <ul class="h-full w-full flex flex-center gap-4" v-if="!current_deck">
      <li
        class="deck-card p-4 bg-blue-400 text-white cursor-pointer rounded-md"
        v-for="deck in decks"
        tabindex="0"
        @click="current_deck = deck"
      >
        {{ deck.name }}
      </li>
      <li
        class="deck-card p-4 bg-blue-400 text-white cursor-pointer rounded-md"
        tabindex="0"
        @click.stop="showModal = true"
        @keyup.enter="showModal = true"
      >
        +
      </li>
    </ul>
    <DeckSection
      v-if="current_deck"
      :cards="current_deck.cards"
      :deck-path="current_deck.path"
      :index="0"
      @finish-deck="current_deck = null"
    />
  </div>
  <Modal
    @close="showModal = false"
    className="modal-container"
    :show="showModal"
  >
    <div class="flex flex-col gap-2">
      <input
        type="text"
        class="border-b border-gray-600/80 border-solid p-1"
        tabindex="1"
        v-model="createDeckValue"
        @keypress.enter="createDeck"
      />
      <div class="flex flex-row gap-4 justify-end">
        <button
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          tabindex="2"
          @click="createDeck"
        >
          Ok
        </button>
        <button
          tabindex="3"
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          @click="showModal = false"
        >
          Cancel
        </button>
      </div>
    </div>
  </Modal>
</template>

<style scoped>
div.deck-card {
  max-width: 150px;
  width: max-content;
}

:deep(.modal-fuera) {
  height: 30px;
  width: 30px;
  background-color: blueviolet;
}

:deep(.modal-container) {
  border: 10px solid red !important;
  background-color: dodgerblue !important;
}
</style>
