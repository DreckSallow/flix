<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { onBeforeUnmount, onMounted, ref } from "vue";

let spaces = ref<{ title: string }[]>([]);

onMounted(() => {
  document.querySelector("#app")?.classList.add("flex", "flex-row");
  invoke("workspaces_handler")
    .then((w) => {
      spaces.value = (w as string[]).map((s) => ({ title: s }));
    })
    .catch((e) => {
      console.log("Error: ", e);
    });
});

onBeforeUnmount(() => {
  document.querySelector("#app")?.classList.remove("flex", "flex-row");
});
</script>

<template>
  <aside
    class="sidebar-main flex flex-col items-center justify-start py-6 gap-4 h-full bg-gray-100"
  >
    <h3 class="text-blue-400 font-bold text-lg">FLIX</h3>
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
  width: 80px;
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
