<script lang="ts" setup>
import { ref, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api";
import { sep } from "@tauri-apps/api/path";
import { IDeckResponse } from "../../types";
import { ArrowIcon } from "@components/icons";
import { Modal } from "@components/modals";
import DeckSection from "./deck-section.vue";
import CreateDeckForm from "../components/create-deck-form.vue";
import { NotifyState } from "../../state";

//TODO: add create card functionality

interface Props {
  workspaceName?: string;
}

const props = defineProps<Props>();

const decks = ref<string[]>([]);

watchEffect(() => {
  if (!props.workspaceName) {
    decks.value = [];
    return;
  }
  invoke<string[]>("get_decks_handler", {
    workspaceName: props.workspaceName,
  })
    .then((d) => {
      decks.value = d;
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Decks Error",
        content: "Error gettign the decks of " + props.workspaceName,
        type: "error",
      });
    });
});

const current_deck = ref<IDeckResponse | null>(null);

function loadDeck(deckName: string) {
  if (!props.workspaceName) {
    return;
  }
  invoke<IDeckResponse>("get_deck_handler", {
    workspaceName: props.workspaceName,
    deckName,
  })
    .then((d) => {
      current_deck.value = d;
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Get the deck info",
        content: "Error while get the deck info: " + deckName,
        type: "error",
      });
    });
}

function onLoadDeck(e: MouseEvent) {
  const target = e.target as HTMLElement | null;
  if (target && target.tagName === "LI") {
    const deckName = target.getAttribute("deck-name");
    if (deckName) {
      loadDeck(deckName);
    }
  }
}

const showModal = ref(false);

function createDeck(info: { pathFile?: string; name?: string }) {
  const newDeckName = info.pathFile?.split(sep).pop() ?? info.name;

  if (!newDeckName) {
    return;
  }

  if (decks.value.some((name) => name === newDeckName)) {
    NotifyState.notify({
      title: "Rename Deck",
      content: "The deck '" + newDeckName + "' already exists",
      type: "alert",
    });
    return;
  }

  if (info.pathFile) {
    invoke<IDeckResponse>("import_deck_handler", {
      workspaceName: props.workspaceName,
      filePath: info.pathFile,
    })
      .then((deck) => {
        decks.value.push(deck.name);
        current_deck.value = deck;
      })
      .catch((e) => {
        NotifyState.notify({
          title: "Import deck",
          content: "An error ocurred when importing a file",
          type: "error",
        });
      })
      .finally(() => {
        showModal.value = false;
      });
  } else if (info.name) {
    invoke<IDeckResponse>("create_deck_handler", {
      workspaceName: props.workspaceName,
      deckName: newDeckName,
    })
      .then((deck) => {
        decks.value.push(deck.name);
        current_deck.value = deck;
      })
      .catch((e) => {
        NotifyState.notify({
          title: "Create deck",
          content: "An error ocurred while create a deck",
          type: "error",
        });
      })
      .finally(() => {
        showModal.value = false;
      });
  }
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
    <ul
      class="h-full w-full flex flex-center gap-4"
      v-if="!current_deck"
      @click="onLoadDeck"
    >
      <li
        class="deck-card p-4 bg-blue-400 text-white cursor-pointer rounded-md"
        v-for="deckName in decks"
        tabindex="0"
        :deck-name="deckName"
      >
        {{ deckName }}
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
      v-if="current_deck && current_deck.cards.length > 0"
      :cards="current_deck.cards"
      :deck-path="current_deck.path"
      :index="0"
      @finish-deck="current_deck = null"
    />
    <div class="w-full h-full grid content-center" v-else-if="current_deck">
      <p class="text-center">NO HAVE CARDS</p>
    </div>
  </div>
  <Modal
    @close="showModal = false"
    className="modal-container"
    :show="showModal"
  >
    <CreateDeckForm
      :workspace-name="workspaceName"
      @deck-info="createDeck"
      @cancel="showModal = false"
    />
  </Modal>
</template>

<style scoped>
div.deck-card {
  max-width: 150px;
  width: max-content;
}
</style>
