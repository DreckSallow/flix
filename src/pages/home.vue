<script setup lang="ts">
import { onBeforeUnmount, onMounted } from "vue";

onMounted(() => {
  document.querySelector("#app")?.classList.add("flex", "flex-row");
});

onBeforeUnmount(() => {
  document.querySelector("#app")?.classList.remove("flex", "flex-row");
});

let spaces = [
  {
    title: "Ingles",
  },
  {
    title: "Aleman",
  },
  {
    title: "Sueco",
  },
  {
    title: "Zig",
  },
  {
    title: "Rust",
  },
  {
    title: "Español",
  },
  {
    title: "Quechua",
  },
  {
    title: "Quechua-1",
  },
  {
    title: "Quechua-2",
  },
  {
    title: "Quechua-3",
  },
  {
    title: "Quechua-4",
  },
  {
    title: "Quechua-5",
  },
];
</script>

<template>
  <aside
    class="sidebar-main flex flex-col items-center justify-start py-6 gap-4 h-full bg-gray-100"
  >
    <ul class="flex flex-col gap-4 overflow-auto items-center w-full">
      <li
        v-for="space in spaces"
        class="rounded-lg w-10 h-10 bg-white grid place-content-center cursor-pointer"
        :class="{
          selected: $route.params.area === space.title,
        }"
      >
        <router-link :to="space.title" class="h-full w-full p-4">
          {{ space.title.slice(0, 2).toUpperCase() }}
        </router-link>
      </li>
    </ul>
    <router-link
      class="rounded-lg w-10 h-10 bg-white grid place-content-center cursor-pointer"
      to="new-space"
    >
      +
    </router-link>
  </aside>
  <section class="section-content h-full">
    <router-view></router-view>
  </section>
</template>

<style scoped>
aside.sidebar-main {
  /* border-right: 1px solid #90969ea1; */
  /* --bg-sidebar: 200, 13%, 95%; */
  width: 80px;
  /* background-color: hsl(var(--bg-sidebar), var(--alpha)); */
}

aside.sidebar-main > ul {
  max-height: 80%;
}

section.section-content {
  width: calc(100% - 74px);
}

li.selected {
  /* border: 2px solid red; */
}
</style>
