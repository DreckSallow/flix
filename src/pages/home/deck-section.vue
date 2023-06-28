<script lang="ts" setup>
import { sep } from "@tauri-apps/api/path";
import dompurify from "dompurify";
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
  //TODO: add spaced repetition functionality

  currentIndex.value += 1;
  isRevealCard.value = false;
}

function getHtml() {
  return dompurify.sanitize(
    isRevealCard.value
      ? (currentCard.value?.back as string)
      : (currentCard.value?.front as string)
  );
}

const buttonStyle = "p-2 text-xs rounded-1 cursor-pointer tracking-wide";
</script>

<template>
  <div class="cards-container h-full w-full overflow-hidden p-4">
    <div class="h-full w-full overflow-auto flex-center">
      <div class="card-content" v-if="currentCard" v-html="getHtml()" />
    </div>

    <div class="flex flex-row gap-4">
      <button
        v-if="!isRevealCard"
        @click="onRevealCard"
        :class="'bg-strong text-white ' + buttonStyle"
      >
        Reveal
      </button>
      <!-- <button
        v-if="isRevealCard"
        @click="onTestCard"
        :class="'bg-strong text-white ' + buttonStyle"
      >
        Bad
      </button>
      <button
        v-if="isRevealCard"
        @click="onTestCard"
        :class="'bg-strong text-white ' + buttonStyle"
      >
        Good
      </button> -->
      <button
        v-if="isRevealCard"
        @click="onTestCard"
        :class="'bg-strong text-white ' + buttonStyle"
      >
        Next
      </button>
    </div>
  </div>
</template>

<style scoped>
div.cards-container {
  align-items: center;
  display: grid;
  justify-items: center;
  grid-template-rows: 1fr 60px;
}

div.card img {
  aspect-ratio: 1;
  object-fit: contain;
  max-height: 200px;
}

div.card-content :deep(img) {
  object-fit: contain;
}

div.card-content :deep(*) {
  max-height: 18em !important;
  line-height: 1.6rem;
}
</style>
