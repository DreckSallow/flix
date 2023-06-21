<script setup lang="ts">
import { onBeforeUnmount, onMounted, provide, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { sep } from "@tauri-apps/api/path";
import { vFocus } from "../../directives";
import { Modal } from "@components/modals";
import MenuContext from "@components/menu-context.vue";
import StudyArea from "./study-area.vue";
import { useWorkspaceProvider, workspaceKeyProv } from "./provider";
import { NotifyState } from "../../state";

const workspaces = ref<{ title: string }[]>([]);

const workspaceProvider = useWorkspaceProvider();

provide(workspaceKeyProv, workspaceProvider.workspaceData);

onMounted(() => {
  document.querySelector("#app")?.classList.add("flex", "flex-row");
  invoke<string[]>("get_workspaces_handler")
    .then((workList) => {
      workspaces.value = workList.map((s) => ({ title: s }));
      if (workList.length > 0) {
        workspaceProvider.setData({
          name: workList[0],
        });
      }
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Workspaces Error",
        content: "Error getting workspaces",
        type: "error",
      });
    });
});

onBeforeUnmount(() => {
  document.querySelector("#app")?.classList.remove("flex", "flex-row");
});

type TWorkspaceForm = {
  show: boolean;
  inputs: { name: string };
  type: "create" | "update";
};

const workspaceForm = reactive<TWorkspaceForm>({
  show: false,
  inputs: {
    name: "",
  },
  type: "create",
});

function createOrUpdate() {
  if (workspaceForm.type === "create") {
    invoke("create_workspace_handler", {
      workspaceName: workspaceForm.inputs.name,
    })
      .then(() => {
        workspaces.value.push({
          title: workspaceForm.inputs.name,
        });
        workspaceProvider.setData({
          name: workspaceForm.inputs.name,
        });
      })
      .catch((e) => {
        NotifyState.notify({
          title: "Create workspaces",
          content:
            "Error while create " + workspaceForm.inputs.name + " workspace",
          type: "error",
        });
      })
      .finally(() => {
        workspaceForm.show = false;
        workspaceForm.inputs.name = "";
      });
    return;
  }
  invoke<string>("rename_workspace_handler", {
    workspaceName: workspaceInfo.value?.workspace,
    newName: workspaceForm.inputs.name,
  })
    .then(() => {
      const index = workspaces.value.findIndex(
        ({ title }) => title === workspaceInfo.value?.workspace
      );
      workspaces.value[index] = {
        title: workspaceForm.inputs.name,
      };
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Rename workspace",
        content: "Error while rename a workspace: " + workspaceForm.inputs.name,
        type: "error",
      });
    })
    .finally(() => {
      workspaceForm.show = false;
      workspaceForm.inputs.name = "";
      workspaceInfo.value = null;
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
  if (type === "Remove") {
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
        NotifyState.notify({
          title: "Create workspaces",
          content:
            "Error while remove a workspace " + workspaceForm.inputs.name,
          type: "error",
        });
      });
    workspaceInfo.value = null;

    return;
  }
  if (type === "Rename") {
    workspaceForm.show = true;
    workspaceForm.type = "update";
    workspaceForm.inputs.name = workspaceInfo.value?.workspace as string;
  }
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
    class="sidebar-main flex flex-col items-center justify-start py-6 gap-4 h-full"
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
      @close="workspaceInfo = null"
      class="rounded-md bg-gray-300 py-2 px-1"
      :options="[{ text: 'Remove' }, { text: 'Rename' }]"
      @select-opt="selectContextMenu"
    />
    <button
      tabindex="0"
      class="rounded-lg w-10 h-10 bg-white grid place-content-center cursor-pointer"
      @click.stop="workspaceForm.show = true"
      @keyup.enter="workspaceForm.show = true"
    >
      +
    </button>
  </aside>
  <Modal @close="workspaceForm.show = false" :show="workspaceForm.show">
    <div class="content-modal flex flex-col gap-2">
      <input
        v-focus
        class="border-b border-gray-600/80 border-solid p-1"
        type="text"
        tabindex="1"
        v-model="workspaceForm.inputs.name"
        @keyup.enter="createOrUpdate"
      />
      <div class="flex flex-row gap-4 justify-end">
        <button
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          tabindex="2"
          @click="createOrUpdate"
        >
          Ok
        </button>
        <button
          tabindex="3"
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          @click="workspaceForm.show = false"
        >
          Cancel
        </button>
      </div>
    </div>
  </Modal>
  <section class="section-content h-full">
    <StudyArea @create-workspace="workspaceForm.show = true" />
  </section>
</template>

<style scoped>
aside.sidebar-main {
  width: 80px;
  background-color: #f7f6f5;
  border-style: solid;
  border-right-width: 0.5px;
  border-right-color: #6e6e6ed3;
}

aside.sidebar-main > ul {
  max-height: 80%;
}

section.section-content {
  width: calc(100% - 74px);
}
</style>
