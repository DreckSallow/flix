<script setup lang="ts">
import { Modal } from "@components/modals";
import { invoke } from "@tauri-apps/api";
import { onBeforeUnmount, onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { InputFocused } from "@components/inputs";

const workspaces = ref<{ title: string }[]>([]);

const router = useRouter();

onMounted(() => {
  document.querySelector("#app")?.classList.add("flex", "flex-row");
  invoke("workspaces_handler")
    .then((w) => {
      workspaces.value = (w as string[]).map((s) => ({ title: s }));
      if ((w as string[]).length > 0) {
        router.push((w as string[])[0]);
      }
    })
    .catch((e) => {
      console.log("Error: ", e);
    });
});

onBeforeUnmount(() => {
  document.querySelector("#app")?.classList.remove("flex", "flex-row");
});

const showModal = ref(false);

const createWorkspaceValue = ref("");

function create_workspace() {
  console.log({ value: createWorkspaceValue.value });
  invoke("create_workspace_handler", {
    workspaceName: createWorkspaceValue.value,
  })
    .then(() => {
      workspaces.value.push({
        title: createWorkspaceValue.value,
      });
      router.push(createWorkspaceValue.value);
      createWorkspaceValue.value = "";
    })
    .catch((e) => {
      console.log("ERROR CREATING WORKSPACE: ", e);
    })
    .finally(() => {
      showModal.value = false;
      console.log("ERROR");
    });
}
</script>

<template>
  <aside
    class="sidebar-main flex flex-col items-center justify-start py-6 gap-4 h-full bg-gray-100"
  >
    <h3 class="text-blue-400 font-bold text-lg">FLIX</h3>
    <ul class="flex flex-col gap-4 overflow-auto items-center w-full">
      <li
        v-for="space in workspaces"
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
    <button
      tabindex="0"
      class="rounded-lg w-10 h-10 bg-white grid place-content-center cursor-pointer"
      @click.stop="showModal = true"
      @keyup.enter="showModal = true"
    >
      +
    </button>
  </aside>
  <Modal @close="showModal = false" :show="showModal">
    <div class="content-modal flex flex-col gap-2">
      <InputFocused
        type="text"
        tabindex="1"
        v-model:value="createWorkspaceValue"
        @keyup.enter="create_workspace"
      />
      <div class="flex flex-row gap-4 justify-end">
        <button
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          tabindex="2"
          @click="create_workspace"
        >
          Ok
        </button>
        <button
          tabindex="3"
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          @click="showModal = false"
        >
          Cancel
        </button>
      </div>
    </div>
  </Modal>
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
