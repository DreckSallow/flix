<script lang="ts" setup>
import { defineEmits, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { EditorView, basicSetup } from "codemirror";
import { markdown } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";

interface IProps {
  content: string;
}

const props = defineProps<IProps>();

interface IEvents {
  (e: "update:content", v: string): void;
  (e: "save"): void;
}

const emit = defineEmits<IEvents>();

const editorRef = ref<HTMLElement | null>(null);

const markdownEditor = ref<EditorView | null>(null);

function onSave(e: KeyboardEvent) {
  if (e.ctrlKey && e.key.toLowerCase() === "s") {
    emit("save");
  }
}

onMounted(() => {
  if (!editorRef.value) {
    return;
  }
  const editorView = new EditorView({
    doc: props.content,
    extensions: [
      EditorView.lineWrapping,
      basicSetup,
      markdown({ codeLanguages: languages }),
      EditorView.updateListener.of((e) => {
        emit("update:content", e.state.doc.toString());
      }),
    ],
    parent: editorRef.value as Element,
  });
  markdownEditor.value = editorView;
  editorView.dom.addEventListener("keydown", onSave);
});

onBeforeUnmount(() => {
  if (markdownEditor.value) {
    markdownEditor.value.dom.removeEventListener("keydown", onSave);
    markdownEditor.value.destroy();
  }
});
</script>

<template>
  <div id="markdown-editor" ref="editorRef"></div>
</template>

<style scoped>
#markdown-editor :deep(> div) {
  outline: none !important;
}
</style>
