<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { IDeckResponse } from "../../types";
import { ArrowIcon } from "@components/icons";
import DeckSection from "./deck-section.vue";
import { invoke } from "@tauri-apps/api";

import { useRoute } from "vue-router";

const decks = ref<IDeckResponse[]>([]);
const route = useRoute();

onMounted(() => {
  invoke("get_decks_handler", {
    workspaceName: route.params.area,
  })
    .then((d) => {
      let data = d as Array<IDeckResponse>;
      // data.forEach((d) => {
      //   // Mutate the paths
      //   d.path = d.path + sep + "media";
      // });
      // console.log(data);
      decks.value = data;
    })
    .catch((e) => {
      console.error("study-area: ", e);
    });
});

const current_deck = ref<IDeckResponse | null>(null);

function onFinishDeck() {
  current_deck.value = null;
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
    <div class="h-full w-full flex flex-center" v-if="!current_deck">
      <div
        class="deck-card p-4 bg-blue-400 text-white cursor-pointer rounded-md mt-10"
        v-for="deck in decks"
        @click="current_deck = deck"
      >
        {{ deck.name }}
      </div>
    </div>
    <DeckSection
      v-if="current_deck"
      :cards="current_deck.cards"
      :deck-path="current_deck.path"
      :index="0"
      @finish-deck="onFinishDeck"
    />
  </div>
</template>

<style scoped>
div.deck-card {
  max-width: 150px;
  width: max-content;
}
</style>
