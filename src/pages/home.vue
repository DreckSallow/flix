<script setup lang="ts">
import { useRouter } from "vue-router";

let cards = [
  {
    title: "Ingles",
    description: "Learn Ingles today",
  },
  {
    title: "Aleman",
    description: "Learn aleman today using FLIX!",
  },
  {
    title: "Sueco",
    description: "Learning Sueco",
  },
];
const router = useRouter();
</script>

<template>
  <main class="flex flex-col gap-4">
    <header class="text-center">OPTIONS</header>
    <ul
      class="max-w-300 max-h-100 max-h-100 border border-orange-400 flex flex-row gap-4 flex-wrap"
    >
      <li
        v-for="card in cards"
        class="p-4 bg-soft rounded-lg w-40 h-40 flex-center flex-col gap-4 cursor-pointer"
        :redirect="card.title"
      >
        <h4 class="text-lg font-semibold">
          <router-link :to="card.title">
            {{ card.title }}
          </router-link>
        </h4>
        <p>{{ card.description }}</p>
      </li>
    </ul>
  </main>
</template>

<style scoped></style>
