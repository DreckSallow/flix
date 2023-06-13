<script setup lang="ts">
import { Modal } from "@components/modals";
import { invoke } from "@tauri-apps/api";
import { sep } from "@tauri-apps/api/path";
import { onBeforeUnmount, onMounted, provide, ref } from "vue";
import { InputFocused } from "@components/inputs";
import MenuContext from "@components/menu-context.vue";
import StudyAreaPage from "../study-area/study-area.vue";
import { useWorkspaceProvider, workspaceKeyProv } from "./provider";

const workspaces = ref<{ title: string }[]>([]);

const workspaceProvider = useWorkspaceProvider();

provide(workspaceKeyProv, workspaceProvider.workspaceData);

onMounted(() => {
  document.querySelector("#app")?.classList.add("flex", "flex-row");
  invoke<string[]>("workspaces_handler")
    .then((workList) => {
      workspaces.value = workList.map((s) => ({ title: s }));
      if (workList.length > 0) {
        workspaceProvider.setData({
          name: workList[0],
        });
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
  invoke("create_workspace_handler", {
    workspaceName: createWorkspaceValue.value,
  })
    .then(() => {
      workspaces.value.push({
        title: createWorkspaceValue.value,
      });
      workspaceProvider.setData({
        name: createWorkspaceValue.value,
      });
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

type TMousePos = { x: number; y: number };

const workspaceInfo = ref<{ workspace: string; mouse: TMousePos } | null>(null);

function openContextMenu(e: MouseEvent) {
  e.preventDefault();
  const el = e.currentTarget as HTMLLIElement | null;
  if (!el) return;
  const { x, width } = el.getBoundingClientRect();
  workspaceInfo.value = {
    mouse: {
      x: x + width,
      y: e.clientY,
    },
    workspace: el.getAttribute("workspace-name") as string,
  };
}

function selectContextMenu(type: "Remove" | "Rename") {
  switch (type) {
    case "Remove": {
      invoke<string>("remove_workspace_handler", {
        workspaceName: workspaceInfo.value?.workspace,
      })
        .then((p) => {
          const name = p.split(sep).pop();
          const workspaceFiltered = workspaces.value.filter(
            ({ title }) => title != name
          );
          workspaces.value = workspaceFiltered;

          if (workspaceFiltered.length > 0) {
            workspaceProvider.setData({
              name: workspaceFiltered[0].title,
            });
          } else {
            workspaceProvider.setData(null);
          }
        })
        .catch((e) => {
          console.log("ERROR REMOVING CONTEXT: ", e);
        });
    }
    case "Rename": {
    }
  }

  workspaceInfo.value = null;
}

function setCurrentWorkspace(e: MouseEvent) {
  const el = e.target as HTMLElement | null;
  if (el && el.tagName === "LI") {
    const workspaceName = el.getAttribute("workspace-name");
    workspaceProvider.setData(
      workspaceName
        ? {
            name: workspaceName,
          }
        : null
    );
  }
}
</script>

<template>
  <aside
    class="sidebar-main flex flex-col items-center justify-start py-6 gap-4 h-full bg-gray-100"
  >
    <h3 class="text-blue-400 font-bold text-lg">FLIX</h3>
    <ul
      class="flex flex-col gap-4 overflow-auto items-center w-full"
      @click="setCurrentWorkspace"
    >
      <li
        v-for="space in workspaces"
        class="rounded-lg w-10 h-10 bg-white grid place-content-center cursor-pointer"
        :class="{
          selected: workspaceProvider.workspaceData.value?.name === space.title,
        }"
        @contextmenu="openContextMenu"
        :workspace-name="space.title"
      >
        {{ space.title.slice(0, 2).toUpperCase() }}
      </li>
    </ul>
    <MenuContext
      v-if="workspaceInfo"
      :style="{
        top: workspaceInfo.mouse.y + 'px',
        left: workspaceInfo.mouse.x + 'px',
      }"
      class="rounded-md bg-gray-300 py-2 px-1"
      :options="[{ text: 'Remove' }, { text: 'Rename' }]"
      @select-opt="selectContextMenu"
    />
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
    <StudyAreaPage />
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
