<script lang="ts" setup>
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api";
import { sep } from "@tauri-apps/api/path";
import { IDeckResponse } from "../../types";
import { ArrowIcon } from "@components/icons";
import { Modal } from "@components/modals";
import DeckSection from "./deck-section.vue";
import CreateDeckForm from "../components/create-deck-form.vue";

interface Props {
  workspaceName?: string;
}

const props = defineProps<Props>();

const decks = ref<IDeckResponse[]>([]);

watch(
  () => props.workspaceName,
  () => {
    if (!props.workspaceName) return;
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

function createDeck(info: { pathFile?: string; name?: string }) {
  const newDeckName = info.pathFile?.split(sep).pop() ?? info.name;

  if (!newDeckName) {
    return;
  }

  if (decks.value.some(({ name }) => name === newDeckName)) {
    //TODO: Warn to the user about this mistake
    return;
  }

  if (info.pathFile) {
    invoke("import_deck_handler", {
      workspaceName: props.workspaceName,
      deckPath: info.pathFile,
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
  } else if (info.name) {
    invoke("create_deck_handler", {
      workspaceName: props.workspaceName,
      deckName: newDeckName,
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
    <CreateDeckForm :workspace-name="workspaceName" @deck-info="createDeck" />
  </Modal>
</template>

<style scoped>
div.deck-card {
  max-width: 150px;
  width: max-content;
}
</style>
