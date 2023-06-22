<script lang="ts" setup>
import styles from "./markdown.module.css";
import { computed, ref, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api";
import { NotifyState } from "../../state";
import { CheckIcon, CloseIcon, EditIcon, ReadIcon } from "@components/icons";
import { micromark } from "micromark";
import { gfm, gfmHtml } from "micromark-extension-gfm";
import MarkdownEditor from "@components/markdown-editor.vue";
import { useCheckInput } from "../../hooks";

interface IProps {
  workspaceName: string;
  docId: number;
}

interface INote {
  id: number;
  title: string;
  text: string;
}

const props = defineProps<IProps>();

const docState = ref<INote | null>(null);

const docTextInput = useCheckInput("");

const isTextChanged = ref(false);

docTextInput.onInput((textInput) => {
  if (!isTextChanged.value) {
    isTextChanged.value = textInput !== docState.value?.text;
  }
});

watchEffect(() => {
  invoke<INote>("get_one_note", {
    workspaceName: props.workspaceName,
    id: Number(props.docId),
  })
    .then((data) => {
      isTextChanged.value = false;
      docState.value = data;
      docTextInput.set(data.text);
      typePage.value = "read";
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Doc info",
        content: "Error getting doc information",
        type: "error",
      });
    });
});

function update() {
  if (!docState.value) {
    throw new Error("The doc not exist");
  }
  if (!isTextChanged) return;

  invoke<INote>("update_note", {
    workspaceName: props.workspaceName,
    id: docState.value.id,
    text: docTextInput.input.value,
  })
    .then(() => {
      if (docState.value) {
        docState.value.text = docTextInput.input.value;
      }
      isTextChanged.value = false;
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Save Doc",
        content: "An error occurred while saving a doc.",
        type: "error",
      });
    });
}

const typePage = ref<"read" | "write">("read");

function toggleRenderType() {
  typePage.value = typePage.value === "read" ? "write" : "read";
}

const docParsed = computed(() => {
  return docState.value
    ? micromark(docTextInput.input.value, {
        extensions: [gfm()],
        htmlExtensions: [gfmHtml()],
      })
    : null;
});
</script>

<template>
  <div class="w-full h-full overflow-auto">
    <section v-if="docState" class="docs-section">
      <header class="p-3 h-60px items-center sticky top-0 bg-primary z-99">
        <h4 class="w-max">
          {{ docState.title }}
        </h4>
        <button @click="toggleRenderType" class="cursor-pointer w-max">
          <EditIcon v-if="typePage === 'read'" class="h-5 w-5 fill-strong" />
          <ReadIcon v-if="typePage === 'write'" class="h-5 w-5 fill-strong" />
        </button>
        <button
          @click="update"
          class="rounded-full border-solid border-2 w-max cursor-pointer"
          :class="{
            'border-green-600': !isTextChanged,
            'border-red-600': isTextChanged,
          }"
        >
          <CheckIcon v-if="!isTextChanged" class="fill-green-600 h-5 w-5" />
          <CloseIcon v-else class="fill-red-600 h-5 w-5" />
        </button>
      </header>
      <main class="h-full w-full overflow-auto">
        <MarkdownEditor
          v-model:content="docTextInput.input.value"
          @save="update"
          class="w-full h-full text-lg"
          v-if="typePage === 'write'"
        />
        <div
          class="w-full h-full p-2"
          :class="styles['markdown']"
          v-if="typePage === 'read'"
          v-html="docParsed"
        ></div>
      </main>
    </section>
  </div>
</template>

<style scoped>
.docs-section > header {
  display: grid;
  grid-template-columns: 1fr 40px 40px;
  border-style: solid;
  border-bottom-width: 0.5px;
  border-bottom-color: #6e6e6e5f;
}
</style>
