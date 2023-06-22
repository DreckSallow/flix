<script setup lang="ts">
import { onBeforeUnmount, onMounted, provide, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { sep } from "@tauri-apps/api/path";
import { ModalForm } from "@components/modals";
import MenuContext from "@components/menu-context.vue";
import StudyArea from "./study-area.vue";
import { useWorkspaceProvider, workspaceKeyProv } from "./provider";
import { NotifyState } from "../../state";
import { useMenuContext } from "../../hooks";

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
        content: "An error occurred getting the workspaces.",
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

function createOrUpdate(input: string) {
  if (input.length === 0) {
    return NotifyState.notify({
      title: "Create workspace",
      content: "The workspace name cannot be empty.",
      type: "error",
    });
  }
  if (workspaces.value.some(({ title }) => title === input)) {
    return NotifyState.notify({
      title: "Create workspace",
      content: "The workspace name already exists.",
      type: "error",
    });
  }

  if (workspaceForm.type === "create") {
    invoke("create_workspace_handler", {
      workspaceName: input,
    })
      .then(() => {
        workspaces.value.push({
          title: input,
        });
        workspaceProvider.setData({
          name: input,
        });
      })
      .catch((e) => {
        NotifyState.notify({
          title: "Create Workspace",
          content: "Error creating workspace " + input,
          type: "error",
        });
      })
      .finally(() => {
        workspaceForm.show = false;
        workspaceForm.inputs.name = "";
      });
    return;
  }
  const selectedWorkspace = menuContext.getInfo().data;
  invoke<string>("rename_workspace_handler", {
    workspaceName: selectedWorkspace,
    newName: input,
  })
    .then(() => {
      const index = workspaces.value.findIndex(
        ({ title }) => title === selectedWorkspace
      );
      if (workspaceProvider.workspaceData.value?.name === selectedWorkspace) {
        workspaceProvider.setData({
          name: input,
        });
      }

      workspaces.value[index] = {
        title: input,
      };
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Rename Workspace",
        content: "Error renaming workspace " + selectedWorkspace,
        type: "error",
      });
    })
    .finally(() => {
      workspaceForm.show = false;
      workspaceForm.inputs.name = "";
    });
}

const menuContext = useMenuContext(false, "");

function openContextMenu(e: MouseEvent) {
  e.preventDefault();
  const el = e.currentTarget as HTMLLIElement | null;
  if (!el) return;
  const { x, width } = el.getBoundingClientRect();
  menuContext.updateInfo({
    mouse: {
      x: x + width,
      y: e.clientY,
    },
    data: el.getAttribute("workspace-name") as string,
  });
  menuContext.updateShow(true);
}

function selectContextMenu(t: string | number) {
  const type = t as "Remove" | "Rename";
  if (type === "Remove") {
    menuContext.updateShow(false);
    invoke<string>("remove_workspace_handler", {
      workspaceName: menuContext.getInfo().data,
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
          title: "Remove Workspace",
          content: "Error deleting workspace " + workspaceForm.inputs.name,
          type: "error",
        });
      });
    return;
  }
  if (type === "Rename") {
    workspaceForm.show = true;
    workspaceForm.type = "update";
    workspaceForm.inputs.name = menuContext.getInfo().data as string;
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
    class="sidebar-main flex flex-col items-center justify-start gap-4 h-full bg-accent"
  >
    <h3 class="font-bold text-lg h-60px flex-center pt-1.2">FLIX</h3>
    <ul
      class="flex flex-col gap-4 overflow-auto items-center w-full"
      @click="setCurrentWorkspace"
    >
      <li
        v-for="space in workspaces"
        class="rounded-lg w-10 h-10 bg-white grid place-content-center cursor-pointer"
        :class="{
          selected: workspaceProvider.workspaceData.value?.name === space.title,
          'border-strong':
            workspaceProvider.workspaceData.value?.name === space.title,
        }"
        @contextmenu="openContextMenu"
        :workspace-name="space.title"
      >
        {{ space.title.slice(0, 2).toUpperCase() }}
      </li>
    </ul>
    <MenuContext
      v-if="menuContext.show.value"
      :style="menuContext.style()"
      @close="menuContext.updateShow(false)"
      class="rounded-md bg-accent py-2 px-1 border-strong border-solid border-1px over-shadow"
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
  <ModalForm
    :show="workspaceForm.show"
    :input="workspaceForm.inputs.name"
    @accept="createOrUpdate"
    @cancel="workspaceForm.show = false"
    @close="workspaceForm.show = false"
  />
  <section class="section-content h-full">
    <StudyArea @create-workspace="workspaceForm.show = true" />
  </section>
</template>

<style scoped>
aside.sidebar-main {
  width: 80px;
  border-style: solid;
  border-right-width: 0.5px;
  border-right-color: #6e6e6e5f;
}

aside.sidebar-main > ul {
  max-height: 80%;
}

aside.sidebar-main > ul li.selected {
  border-style: solid;
  border-width: 2px;
}

section.section-content {
  width: calc(100% - 74px);
}
</style>
