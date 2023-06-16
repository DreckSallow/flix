<script lang="ts" setup>
import { computed, ref } from "vue";
import styles from "./markdown.module.css";
import { micromark } from "micromark";
import { gfm, gfmHtml } from "micromark-extension-gfm";
import { ArrowIcon, EditIcon, ReadIcon } from "@components/icons";
import MarkdownEditor from "@components/markdown-editor.vue";

interface Doc {
  title: string;
  content: string;
}

const docs = ref<Doc[]>([]);

const currentDoc = ref<Doc | null>(null);

const typeRenderDoc = ref<"write" | "read">("read");

function createDoc() {
  const newDock: Doc = {
    title: "Empty Doc",
    content: "Hello\n\n```javascript\nlet x = 'y'\n```",
  };
  docs.value.push(newDock);
  currentDoc.value = { ...newDock };
}

function selectDoc(e: MouseEvent) {
  const el = e.target as HTMLElement | null;
  if (el && el.tagName === "LI") {
    //FIXME: change for Id (beacuse the doc can been duplicate)
    const dockTitle = el.getAttribute("doc-title");
    if (dockTitle) {
      currentDoc.value =
        docs.value.find(({ title }) => title === dockTitle) ?? null;
    }
  }
}

const docParsed = computed(() => {
  return currentDoc.value
    ? micromark(currentDoc.value.content, {
        extensions: [gfm()],
        htmlExtensions: [gfmHtml()],
      })
    : null;
});

function toggleRenderType() {
  const newType = typeRenderDoc.value === "read" ? "write" : "read";
  typeRenderDoc.value = newType;
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
        :doc-title="doc.title"
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
        <div class="flex flex-row">
          <h4>{{ currentDoc.title }}</h4>
          <button @click="toggleRenderType" class="cursor-pointer">
            <EditIcon
              v-if="typeRenderDoc === 'read'"
              class="h-5 w-5 fill-gray-600"
            />
            <ReadIcon
              v-if="typeRenderDoc === 'write'"
              class="h-5 w-5 fill-gray-600"
            />
          </button>
        </div>
      </header>
      <main class="h-full w-full overflow-auto">
        <MarkdownEditor
          v-model:content="currentDoc.content"
          class="w-full h-full"
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
  grid-template-columns: 1fr 80px;
}
</style>
