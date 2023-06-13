<script lang="ts" setup>
import { open } from "@tauri-apps/api/dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { InputFocused } from "@components/inputs";
import { ref } from "vue";

const emit = defineEmits<{
  (
    e: "deck-info",
    d: {
      pathFile?: string;
      name?: string;
    }
  ): void;
}>();

const wayToCreate = ref<"import" | "create">("import");
const newDeckName = ref<string | null>(null);
const importPathDeck = ref<string | null>(null);

function createDeck() {
  emit("deck-info", {
    pathFile: importPathDeck.value as string,
    name: newDeckName.value as string,
  });
}

async function getFileImport() {
  try {
    const pathfile = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          extensions: ["apkg"],
          name: "",
        },
      ],
      defaultPath: await downloadDir(),
    });
    if (pathfile && pathfile.length > 0) {
      importPathDeck.value = pathfile as string;
    }
  } catch (e) {
    console.log("ERROR ", e);
  }
}
</script>

<template>
  <section class="flex flex-col">
    <h4 class="font-semibold mb-3">Add a new Deck</h4>
    <nav class="mb-2">
      <ul class="flex flex-row gap-2 text-sm" role="tablist">
        <li
          class="cursor-pointer"
          role="tab"
          @click="wayToCreate = 'import'"
          :class="{ selected: wayToCreate === 'import' }"
        >
          import
        </li>
        <li
          class="cursor-pointer"
          role="tab"
          @click="wayToCreate = 'create'"
          :class="{ selected: wayToCreate === 'create' }"
        >
          Create
        </li>
        <li class="slider" role="presentation" />
      </ul>
    </nav>
    <main class="text-sm min-w-36">
      <div v-if="wayToCreate === 'create'">
        <InputFocused
          class="border border-gray-600/80 border-solid rounded-md p-1"
          type="text"
          tabindex="1"
          v-model:value="newDeckName"
          @keyup.enter="createDeck"
        />
      </div>
      <div v-if="wayToCreate === 'import'" class="m-2">
        <span
          class="deck-import p-2 text-xs text-center block"
          @click="getFileImport"
        >
          Select a file.
          <br />
          .APKG
        </span>
        <span class="block" v-if="importPathDeck">
          {{ importPathDeck.split("\\").pop() || "nothing" }}
        </span>
      </div>

      <div class="flex flex-row gap-4 justify-end mt-4">
        <button
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
          tabindex="2"
          @click="createDeck"
        >
          Ok
        </button>
        <button
          tabindex="3"
          class="bg-blue-400 text-white px-2 py-1 text-xs rounded-1 cursor-pointer"
        >
          <!-- @click="showModal = false" -->
          Cancel
        </button>
      </div>
    </main>
  </section>
</template>

<style scoped>
nav li {
  border-style: solid;
  border-bottom-width: 2px;
  border-color: transparent;
  padding-bottom: 2px;
}
nav li.selected {
  border-color: rgb(96, 165, 250);
}

span.deck-import {
  border: 2px rgba(174, 172, 172, 0.589) dashed;
  cursor: pointer;
}
</style>