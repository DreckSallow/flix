<script lang="ts" setup>
import { IDeckResponse } from "@interfaces/index";
import { invoke } from "@tauri-apps/api";
import { NotifyState } from "../../state";
import { ref, watchEffect } from "vue";
import DeckSection from "./deck-section.vue";

interface IProps {
  workspaceName: string;
  deckName: string;
}

const props = defineProps<IProps>();

const deckState = ref<IDeckResponse | null>(null);

watchEffect(() => {
  invoke<IDeckResponse>("get_deck_handler", {
    workspaceName: props.workspaceName,
    deckName: props.deckName,
  })
    .then((d) => {
      deckState.value = d;
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Deck Info",
        content: "Error getting card deck information:  " + props.deckName,
        type: "error",
      });
    });
});
</script>

<template>
  <div class="w-full h-full">
    <DeckSection
      v-if="deckState && deckState.cards.length > 0"
      :cards="deckState.cards"
      :deck-path="deckState.path"
      :index="0"
      @finish-deck="deckState = null"
    />
    <div class="w-full h-full grid content-center" v-else-if="deckState">
      <p class="text-center">NO HAVE CARDS</p>
    </div>
  </div>
</template>
