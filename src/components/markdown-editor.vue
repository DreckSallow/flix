<script lang="ts" setup>
import { defineEmits, onBeforeUnmount, onMounted, ref } from "vue";
import { EditorView, basicSetup } from "codemirror";
import { markdown } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";

interface IProps {
  content: string;
}

const props = defineProps<IProps>();

interface IEvents {
  (e: "update:content", v: string): void;
}

const emit = defineEmits<IEvents>();

const editorRef = ref<HTMLElement | null>(null);

const markdownEditor = ref<EditorView | null>(null);

onMounted(() => {
  if (!editorRef.value) {
    return;
  }
  const editorView = new EditorView({
    doc: props.content,
    extensions: [
      basicSetup,
      markdown({ codeLanguages: languages }),
      EditorView.updateListener.of((e) => {
        emit("update:content", e.state.doc.toString());
      }),
    ],
    parent: editorRef.value,
  });
  markdownEditor.value = editorView;
});

onBeforeUnmount(() => {
  if (markdownEditor.value) {
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
