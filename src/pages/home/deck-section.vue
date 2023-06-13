<script lang="ts" setup>
import { sep } from "@tauri-apps/api/path";
import { formatCard } from "@utils/card-format";
import { computed, ref } from "vue";
import { ICardResponse } from "../../types";

interface Props {
  cards: ICardResponse[];
  deckPath: string;
  index: number;
}

const props = defineProps<Props>();

const emit = defineEmits(["finishDeck"]);

const currentIndex = ref(props.index);

const currentCard = computed(() => {
  const cardRaw = props.cards[currentIndex.value];
  if (!cardRaw) {
    return null;
  }
  const goodCard = formatCard(cardRaw, props.deckPath + sep + "media");

  return goodCard;
});

const isRevealCard = ref(false);

function onRevealCard() {
  isRevealCard.value = true;
}

function onTestCard() {
  if (currentIndex.value + 1 >= props.cards.length) {
    return emit("finishDeck");
  }

  currentIndex.value += 1;
  isRevealCard.value = false;
}

const buttonStyle = "p-2 text-xs rounded-1 cursor-pointer tracking-wide";
</script>

<template>
  <div class="cards-container h-full w-full">
    <div class="flex flex-col gap-6 items-center px-6" v-if="currentCard">
      <div class="card flex flex-col gap-4 items-center">
        <p v-html="isRevealCard ? currentCard.back : currentCard.front"></p>
      </div>
    </div>
    <button
      v-if="!isRevealCard"
      @click="onRevealCard"
      :class="'bg-blue-400 text-white ' + buttonStyle"
    >
      Reveal
    </button>
    <ul class="flex flex-row gap-4">
      <button
        v-if="isRevealCard"
        @click="onTestCard"
        :class="'bg-blue-400 text-white ' + buttonStyle"
      >
        Bad
      </button>
      <button
        v-if="isRevealCard"
        @click="onTestCard"
        :class="'bg-blue-400 text-white ' + buttonStyle"
      >
        Good
      </button>
    </ul>
  </div>
</template>

<style>
div.cards-container {
  align-items: center;
  display: grid;
  justify-items: center;
  grid-template-rows: 1fr 60px;
}

div.card {
  overflow: auto;
  max-height: 400px;
  max-width: 700px;
}
div.card img {
  aspect-ratio: 1;
  object-fit: contain;
  max-height: 200px;
}
</style>
