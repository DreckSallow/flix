<script setup lang="ts">
import { Ref, inject, ref, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api";
import { workspaceKeyProv, type TWorkspaceProvide } from "./provider";
import { useEventListener } from "../../hooks";
import { NotifyState } from "../../state";
import SectionDeck from "./section-deck.vue";
import SectionDoc from "./section-doc.vue";
import SidebarOptions from "./components/sidebar-options.vue";
import { sep } from "@tauri-apps/api/path";

import { deckQuery, docQuery } from "./utils";

interface INoteInfo {
  id: number;
  title: string;
}

const workspaceData = inject<Ref<TWorkspaceProvide>>(
  workspaceKeyProv
) as Ref<TWorkspaceProvide>;

const decks = ref<string[]>([]);

const docs = ref<INoteInfo[]>([]);

const showSidebar = ref(true);

useEventListener(window, "keydown", (e: KeyboardEvent) => {
  if (e.code === "KeyB" && e.ctrlKey) {
    showSidebar.value = !showSidebar.value;
  }
});

type TRenderPage = "docs" | "decks";

const renderPage = ref<{
  type: TRenderPage;
  key: string | null | number;
}>({
  type: "decks",
  key: null,
});

watchEffect(() => {
  if (!workspaceData.value) return (decks.value = []);
  invoke<string[]>("get_decks_handler", {
    workspaceName: workspaceData.value.name,
  })
    .then((d) => (decks.value = d))
    .catch((e) => {
      NotifyState.notify({
        title: "Decks Error",
        content: "Error getting the decks of " + workspaceData.value?.name,
        type: "error",
      });
    });
});

watchEffect(() => {
  if (!workspaceData.value) return (docs.value = []);
  invoke<{ [k: number]: string }>("get_notes_info", {
    workspaceName: workspaceData.value.name,
  })
    .then((data) => {
      docs.value = Object.entries(data).map(([k, v]) => ({
        id: Number(k),
        title: v,
      }));
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Notes info",
        content: "Error getting the notes",
        type: "error",
      });
    });
});

function resetRenderPage() {
  renderPage.value = {
    key: null,
    type: "decks",
  };
}

type TInfoOpt = {
  id: string | number;
  name: string;
  type: "decks" | "docs";
};

function selectOptView(info: TInfoOpt) {
  renderPage.value.type = info.type;
  // Parse the id to number if its docs!
  renderPage.value.key = info.type === "docs" ? Number(info.id) : info.id;
}

function removeOpt(info: TInfoOpt) {
  if (info.type === "decks") {
    const query = {
      workspaceName: workspaceData.value?.name as string,
      deckName: info.name,
    };
    deckQuery.remove_deck(query, () => {
      decks.value = decks.value.filter((deck) => deck !== info.name);
      if (decks.value.length === 0 && renderPage.value.type === "decks") {
        renderPage.value.key = null;
      }
      if (renderPage.value.key === info.name) {
        renderPage.value.key = null;
        renderPage.value.type = "decks";
      }
    });
  } else if (info.type === "docs") {
    const query = {
      workspaceName: workspaceData.value?.name as string,
      id: Number(info.id),
    };
    docQuery.remove_doc(query, () => {
      docs.value = docs.value.filter(({ id }) => id !== Number(info.id));
      if (renderPage.value.key === Number(info.id)) {
        renderPage.value.key = null;
        renderPage.value.type = "docs";
      }
    });
  }
}

function updateOpt(info: TInfoOpt & { newName: string }) {
  if (info.type === "docs") {
    const query = {
      workspaceName: workspaceData.value?.name as string,
      id: Number(info.id),
      title: info.newName,
    };
    docQuery.update_doc(query, () => {
      const index = docs.value.findIndex(({ id }) => id === Number(info.id));
      if (index >= 0) {
        docs.value[index] = {
          id: Number(info.id),
          title: info.newName,
        };
        if (renderPage.value.key === Number(info.id)) {
          renderPage.value.key = Number(info.id);
          renderPage.value.type = "docs";
        }
      }
    });
  }
  if (info.type === "decks") {
    const query = {
      workspaceName: workspaceData.value?.name as string,
      deckName: info.name,
      newDeckName: info.newName,
    };
    deckQuery.update_deck(query, () => {
      const index = decks.value.findIndex((d) => d === info.name);
      if (index >= 0) {
        decks.value[index] = info.newName;
      }
      if (renderPage.value.key === info.name) {
        renderPage.value.key = info.newName;
        renderPage.value.type = "decks";
      }
    });
  }
}

function createDeck(info: {
  isImport: boolean;
  pathFile?: string;
  name?: string;
}) {
  const newDeckName = info.pathFile?.split(sep).pop() ?? info.name;
  if (!newDeckName || !workspaceData.value) return;

  if (decks.value.some((name) => name === newDeckName)) {
    NotifyState.notify({
      title: "Rename Deck",
      content: "The deck '" + newDeckName + "' already exists",
      type: "alert",
    });
    return;
  }

  if (info.pathFile) {
    deckQuery.import_deck(
      {
        workspaceName: workspaceData.value.name,
        filePath: info.pathFile,
      },
      (deck) => {
        decks.value.push(deck.name);
        renderPage.value.key = deck.name;
        renderPage.value.type = "decks";
      }
    );
  } else if (info.name) {
    deckQuery.create_deck(
      {
        workspaceName: workspaceData.value?.name,
        deckName: newDeckName,
      },
      (deck) => {
        decks.value.push(deck.name);
        renderPage.value.key = deck.name;
        renderPage.value.type = "decks";
      }
    );
  }
}

function createDoc(name: string) {
  const query = {
    workspaceName: workspaceData.value?.name as string,
    title: name,
    text: "",
  };

  docQuery.create_doc(query, (doc) => {
    docs.value.push(doc);
    renderPage.value.key = doc.id;
    renderPage.value.type = "docs";
  });
}
</script>

<template>
  <div class="flex flex-row h-full w-full">
    <aside
      class="side-bar flex flex-col gap-4 h-full bg-gray-100"
      v-if="showSidebar"
    >
      <header class="text-lg font-semibold gap-4 text-center p-4">
        <h4>{{ workspaceData?.name ?? "No Have workspace" }}</h4>
      </header>
      <SidebarOptions
        v-if="workspaceData"
        :decks="decks"
        :docs="docs"
        @select-opt="selectOptView"
        @remove-opt="removeOpt"
        @update-opt="updateOpt"
        @create-deck="createDeck"
        @create-doc="createDoc"
      />
      <div class="text-sm" v-if="!workspaceData">
        <button
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          @click.stop="$emit('create-workspace')"
        >
          Create a workspace
        </button>
      </div>
    </aside>
    <section
      class="section-content h-full bg-white"
      :class="{ expanded: !showSidebar }"
      v-if="workspaceData"
    >
      <SectionDeck
        v-if="renderPage.type === 'decks' && renderPage.key"
        :workspace-name="workspaceData.name"
        :deck-name="String(renderPage.key)"
      />

      <SectionDoc
        :workspace-name="workspaceData.name"
        v-if="renderPage.type === 'docs' && renderPage.key"
        :doc-id="Number(renderPage.key)"
      />
    </section>
    <section
      v-else
      class="section-content h-full bg-white grid place-content-center"
    >
      Does not have a workspace
    </section>
  </div>
</template>

<style scoped>
aside.side-bar {
  width: 170px;
  background-color: #fdfdfd;
  border-style: solid;

  border-right-width: 0.5px;
  border-right-color: #6e6e6ed3;
}

aside.side-bar header {
  border-style: solid;
  border-bottom-width: 0.5px;
  border-bottom-color: #6e6e6ed3;
  color: #6e6e6e;
}

.sidebar-options .accordion-header {
  display: grid;
  grid-template-columns: 1fr 20px;
}

section.section-content {
  width: calc(100% - 170px);
}

section.expanded {
  width: 100%;
}
</style>
