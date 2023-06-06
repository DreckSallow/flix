<script lang="ts" setup>
import { ref } from "vue";
import { Deck } from "../../types";
import { ArrowIcon } from "../../components/icons";

const props = defineProps({
  decks: Array<Deck>,
});

const current_deck = ref<Deck | null>(null);
</script>

<template>
  <div class="w-full h-full grid place-content-center relative">
    <span
      class="absolute top-3 left-2 cursor-pointer"
      @click="current_deck = null"
      v-if="current_deck"
    >
      <ArrowIcon class="h-5 w-10 fill-gray-600" />
    </span>
    <div
      class="p-4 bg-green-500 text-white cursor-pointer"
      v-for="deck in props.decks"
      v-if="!current_deck"
      @click="current_deck = deck"
    >
      {{ deck.name }}
    </div>
    <div v-if="current_deck">
      <!-- {{ current_deck.cards[0].front }} -->
      <span v-html="current_deck.cards[0].front"> </span>
    </div>
  </div>
</template>
