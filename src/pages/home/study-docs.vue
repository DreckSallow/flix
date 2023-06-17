<script lang="ts" setup>
import { computed, ref, watchEffect, reactive } from "vue";
import { invoke } from "@tauri-apps/api";
import styles from "./markdown.module.css";
import { micromark } from "micromark";
import { gfm, gfmHtml } from "micromark-extension-gfm";
import {
  ArrowIcon,
  CloseIcon,
  EditIcon,
  ReadIcon,
  CheckIcon,
} from "@components/icons";
import MarkdownEditor from "@components/markdown-editor.vue";
import { useCheckInput } from "../../hooks/index";
import { NotifyState } from "../../state";

interface IProps {
  workspaceName: string;
}

const props = defineProps<IProps>();

interface INote extends INoteInfo {
  text: string;
}

interface INoteInfo {
  id: number;
  title: string;
}

const docs = ref<INoteInfo[]>([]);

const currentDoc = ref<INote | null>(null);

const typeRenderDoc = ref<"write" | "read">("read");

const docTitleInput = useCheckInput("");
const docTextInput = useCheckInput("");

const inputsChanged = reactive({
  title: false,
  text: false,
});

docTitleInput.onInput((titleInput) => {
  if (!inputsChanged.title) {
    inputsChanged.title = titleInput !== currentDoc.value?.title;
  }
});

docTextInput.onInput((textInput) => {
  if (!inputsChanged.text) {
    inputsChanged.text = textInput !== currentDoc.value?.text;
  }
});

watchEffect(() => {
  invoke<{ [k: number]: string }>("get_notes_info", {
    workspaceName: props.workspaceName,
  })
    .then((data) => {
      const docs_data: INoteInfo[] = Object.entries(data).map(([k, v]) => ({
        id: Number(k),
        title: v,
      }));

      docs.value = docs_data;
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Notes info",
        content: "Error getting the notes",
        type: "error",
      });
    });
});

function setCurrentDoc(doc: INote | null) {
  currentDoc.value = doc;
  if (doc) {
    console.log("title: ", doc.title);
    docTitleInput.set(doc.title);
    docTextInput.set(doc.text);
  }
}

function createDoc() {
  invoke<INote>("create_note", {
    workspaceName: props.workspaceName,
    title: "Empty Doc",
    text: "",
  })
    .then((newDoc) => {
      docs.value.push(newDoc);
      setCurrentDoc({ ...newDoc });
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Create a note",
        content: "Error creating a note",
        type: "error",
      });
    });
}

function selectDoc(e: MouseEvent) {
  const el = e.target as HTMLElement | null;
  if (el && el.tagName === "LI") {
    const docId = el.getAttribute("doc-id");
    if (docId) {
      invoke<INote>("get_one_note", {
        workspaceName: props.workspaceName,
        id: Number(docId),
      })
        .then((data) => {
          console.log("SET current Doc");
          setCurrentDoc(data);
        })
        .catch((e) => {
          NotifyState.notify({
            title: "Select Note",
            content: "The note not exist",
            type: "error",
          });
        });
    }
  }
}

const docParsed = computed(() => {
  return currentDoc.value
    ? micromark(docTextInput.input.value, {
        extensions: [gfm()],
        htmlExtensions: [gfmHtml()],
      })
    : null;
});

function toggleRenderType() {
  const newType = typeRenderDoc.value === "read" ? "write" : "read";
  typeRenderDoc.value = newType;
}

function saveDoc() {
  if (!currentDoc.value) return;
  const newDataDoc: { [k: string]: string } = {};

  if (inputsChanged.text) newDataDoc.text = docTextInput.input.value;
  if (inputsChanged.title) newDataDoc.title = docTitleInput.input.value;
  if (!newDataDoc.text && !newDataDoc.title) return;

  invoke<INote>("update_note", {
    workspaceName: props.workspaceName,
    id: currentDoc.value.id,
    title: newDataDoc.title,
    text: newDataDoc.text,
  })
    .then(({ title }) => {
      const index = docs.value.findIndex(
        ({ id }) => id == currentDoc.value?.id
      );
      if (index != -1) {
        docs.value[index].title = title;
      }
      if (inputsChanged.text) inputsChanged.text = false;
      if (inputsChanged.title) inputsChanged.title = false;
    })
    .catch((e) => {
      NotifyState.notify({
        title: "Save Note",
        content: "An error ocurred saving a note",
        type: "error",
      });
    });
}
</script>

<template>
  <div
    class="w-full h-full relative"
    :class="{
      'center-content': !currentDoc,
    }"
  >
    <ul
      v-if="!currentDoc"
      class="flex flex-row gap-4 flex-wrap"
      @click="selectDoc"
    >
      <li
        class="deck-card p-4 bg-blue-400 text-white cursor-pointer rounded-md"
        v-for="doc in docs"
        :doc-id="doc.id"
      >
        {{ doc.title }}
      </li>
      <li
        class="deck-card p-4 bg-blue-400 text-white cursor-pointer rounded-md"
        tabindex="1"
        @click="createDoc"
      >
        +
      </li>
    </ul>
    <section class="doc-container h-full w-full" v-if="currentDoc">
      <header class="bg-blue-300 text-white">
        <span class="cursor-pointer flex-center" @click="currentDoc = null">
          <ArrowIcon class="h-5 w-10 fill-gray-600" direction="left" />
        </span>
        <div>
          <input
            v-model="docTitleInput.input.value"
            class="w-max"
            @blur="saveDoc"
            @keyup.enter="saveDoc"
          />
          <button @click="toggleRenderType" class="cursor-pointer w-max">
            <EditIcon
              v-if="typeRenderDoc === 'read'"
              class="h-5 w-5 fill-gray-600"
            />
            <ReadIcon
              v-if="typeRenderDoc === 'write'"
              class="h-5 w-5 fill-gray-600"
            />
          </button>
          <button
            @click="saveDoc"
            class="rounded-full border-solid border-2 w-max cursor-pointer"
            :class="{
              'border-green-600': !inputsChanged.text,
              'border-red-600': inputsChanged.text,
            }"
          >
            <CheckIcon
              v-if="!inputsChanged.text"
              class="fill-green-600 h-5 w-5"
            />
            <CloseIcon v-else class="fill-red-600 h-5 w-5" />
          </button>
        </div>
      </header>
      <main class="h-full w-full overflow-auto">
        <MarkdownEditor
          v-model:content="docTextInput.input.value"
          class="w-full h-full text-lg"
          v-if="typeRenderDoc === 'write'"
        />
        <div
          class="w-full h-full p-2"
          :class="styles['markdown']"
          v-if="typeRenderDoc === 'read'"
          v-html="docParsed"
        ></div>
      </main>
    </section>
  </div>
</template>

<style scoped>
div.center-content {
  display: grid;
  place-items: center;
}

section.doc-container header {
  align-items: center;
  display: grid;
  grid-template-columns: 80px 1fr;
  height: 50px;
}

section.doc-container main {
  height: calc(100% - 50px);
}

section.doc-container header > div {
  display: grid;
  grid-template-columns: 1fr 70px 70px;
}

/* section.doc-container header button {
  width: ;
} */
</style>
