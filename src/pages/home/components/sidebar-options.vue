<script lang="ts" setup>
import { reactive } from "vue";
import Accordion from "@components/accordion.vue";
import MenuContext from "@components/menu-context.vue";
import { Modal, ModalForm } from "@components/modals";
import { getAttribute } from "@utils/dom";
import { useMenuContext } from "../../../hooks";
import CreateDeckForm from "../../components/create-deck-form.vue";

interface IProps {
  decks: string[];
  docs: { id: number; title: string }[];
  selectedOpt: number | string | null;
}

defineProps<IProps>();

type TInfoOpt = {
  id: string | number;
  name: string;
  type: "decks" | "docs";
};

interface IEvents {
  (e: "select-opt", info: TInfoOpt): void;
  (e: "remove-opt", info: TInfoOpt): void;
  (e: "update-opt", info: TInfoOpt & { newName: string }): void;
  (e: "create-doc", name: string): void;
  (
    e: "create-deck",
    info: { isImport: boolean; pathFile?: string; name?: string }
  ): void;
}

const emit = defineEmits<IEvents>();

function selectOption(e: MouseEvent) {
  const typeView = getAttribute(e.currentTarget, "view") as
    | TInfoOpt["type"]
    | undefined;
  if (!typeView) return;
  if (!e.target || (e.target as HTMLElement).tagName !== "LI") return;
  const optId = getAttribute(e.target, "item-id");
  const optName = getAttribute(e.target, "item-title");
  if (!optId || !optName) throw new Error("The option id or name not exist");
  emit("select-opt", {
    id: optId,
    name: optName,
    type: typeView,
  });
}
type TModalUpdate = {
  show: boolean;
  input: string;
  titleLabel: "Rename Doc:" | "Rename Deck:";
};

const showModal = reactive<TModalUpdate>({
  show: false,
  input: "",
  titleLabel: "Rename Doc:",
});

type TMenuInfo = {
  type: "decks" | "docs";
  key?: string | number;
  name?: string;
};

const menuContextState = useMenuContext<TMenuInfo>(false, {
  type: "decks",
});

function menuOpenHandler(type: "decks" | "docs", e: MouseEvent) {
  if (!e.currentTarget) return;
  const { x, width } = (e.currentTarget as HTMLElement).getBoundingClientRect();
  if ((e.target as HTMLElement).tagName !== "LI") return;
  const key = getAttribute(e.target, "item-id") as string;
  const name = getAttribute(e.target, "item-title") as string;
  menuContextState.updateInfo(() => ({
    mouse: {
      x: x + (width - 10),
      y: e.clientY,
    },
    data: {
      type,
      key,
      name,
    },
  }));
  menuContextState.updateShow(true);
}

function menuSelectItem(key: string | number) {
  const typeAction = key as "Remove" | "Rename";

  const typeAccordion = menuContextState.info.value.data.type;

  if (typeAction === "Rename") {
    showModal.input = menuContextState.info.value.data.name as string;
    showModal.titleLabel =
      typeAccordion === "decks" ? "Rename Deck:" : "Rename Doc:";
    showModal.show = true;
    return;
  }

  if (typeAction === "Remove") {
    const selectInfo: TInfoOpt = {
      id:
        typeAccordion === "decks"
          ? (menuContextState.info.value.data.name as string)
          : Number(menuContextState.info.value.data.key),
      name: menuContextState.info.value.data.name as string,
      type: typeAccordion,
    };
    emit("remove-opt", selectInfo);
    menuContextState.updateShow(false);
  }
}

function updateItem(input: string) {
  const { data } = menuContextState.getInfo();

  const updateInfo = {
    id: (data.type === "decks" ? data.key : Number(data.key)) as string,
    name: data.name as string,
    type: data.type,
    newName: input,
  };

  emit("update-opt", updateInfo);
  showModal.show = false;
}

const modalDoc = reactive({ show: false });

function openModalDoc() {
  modalDoc.show = true;
}

function createDoc(input: string) {
  emit("create-doc", input);
  modalDoc.show = false;
}

const showDeckModal = reactive({ show: false });

function createDeck(info: { pathFile?: string; name?: string }) {
  emit("create-deck", {
    isImport: typeof info.pathFile === "string",
    pathFile: info.pathFile,
    name: info.name,
  });
  showDeckModal.show = false;
}
</script>

<template>
  <div class="sidebar-options p-1">
    <Accordion custom :has-content="decks.length > 0">
      <template #header>
        <div class="accordion-header w-full">
          <span> decks </span>
          <button
            class="text-gray-400 font-semibold flex-center cursor-pointer rounded-sm hover:bg-#ececec"
            @click.stop="showDeckModal.show = true"
          >
            +
          </button>
        </div>
      </template>
      <template #custom="{ show }" v-if="decks.length > 0">
        <ul
          class="accordion-list flex flex-col cursor-pointer text-sm gap-0.5"
          v-if="show"
          view="decks"
          @click="selectOption"
          @contextmenu.prevent="menuOpenHandler('decks', $event)"
        >
          <li
            v-for="deck in decks"
            :item-id="deck"
            :item-title="deck"
            :title="deck"
            class="text-ellipsis overflow-hidden"
            :class="{
              selected: selectedOpt === deck,
            }"
          >
            {{ deck }}
          </li>
        </ul>
      </template>
    </Accordion>
    <Accordion custom :has-content="docs.length > 0" class="mt-2">
      <template #header>
        <div class="accordion-header w-full">
          <span> docs </span>
          <button
            class="text-gray-400 font-semibold flex-center cursor-pointer rounded-sm hover:bg-#ececec"
            @click.stop="openModalDoc"
          >
            +
          </button>
        </div>
      </template>
      <template #custom="{ show }" v-if="docs.length > 0">
        <ul
          class="accordion-list flex flex-col cursor-pointer text-sm gap-0.5"
          view="docs"
          v-if="show"
          @click="selectOption"
          @contextmenu.prevent="menuOpenHandler('docs', $event)"
        >
          <li
            v-for="doc in docs"
            :item-id="doc.id"
            :item-title="doc.title"
            :title="doc.title"
            class="text-ellipsis overflow-hidden"
            :class="{
              selected: selectedOpt === doc.id,
            }"
          >
            {{ doc.title }}
          </li>
        </ul>
      </template>
    </Accordion>
    <MenuContext
      v-if="menuContextState.show.value"
      :style="{ ...menuContextState.style() }"
      class="rounded-md py-2 px-1 bg-accent border-strong border-solid border-1px over-shadow"
      @close="menuContextState.updateShow(false)"
      :options="[{ text: 'Remove' }, { text: 'Rename' }]"
      @select-opt="menuSelectItem"
    />
    <ModalForm
      :title-label="showModal.titleLabel"
      @close="showModal.show = false"
      @cancel="showModal.show = false"
      @accept="updateItem"
      :show="showModal.show"
      :input="showModal.input"
    />
    <ModalForm
      title-label="Create Doc:"
      @close="modalDoc.show = false"
      @cancel="modalDoc.show = false"
      @accept="createDoc"
      :show="modalDoc.show"
      input=""
    />
    <Modal
      @close="showDeckModal.show = false"
      className="modal-container"
      :show="showDeckModal.show"
    >
      <CreateDeckForm
        @deck-info="createDeck"
        @cancel="showDeckModal.show = false"
      />
    </Modal>
  </div>
</template>

<style scoped>
.accordion-header {
  display: grid;
  grid-template-columns: 1fr 25px;
}

:deep(.accordion-header:hover) {
  background-color: #ececec;
}

:deep(.accordion-header > button:hover) {
  background-color: #e3e2e2;
}

.accordion-list {
  overflow: auto;
  max-height: 200px;
  padding: 0.2em;
  padding-left: 0.7em;
}

.accordion-list::-webkit-scrollbar {
  height: 2px;
}

.accordion-list > li {
  overflow: hidden;
  padding: 0.3em;
  border-radius: 5px;
  transition: background-color 200ms ease;
}

.accordion-list > li:hover,
.accordion-list > li.selected {
  background-color: #ececec;
}
</style>
