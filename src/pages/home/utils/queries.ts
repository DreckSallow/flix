import { invoke } from "@tauri-apps/api";
import { InvokeArgs } from "@tauri-apps/api/tauri";
import { NotifyState } from "../../../state";
import { IDeckResponse } from "@interfaces/index";

export type QueryFn<T> = (t: T) => void;

type TRemoveDeck = {
  workspaceName: string;
  deckName: string;
};
function remove_deck(data: TRemoveDeck, cb: QueryFn<null>) {
  invoke<null>("remove_deck_handler", data)
    .then(cb)
    .catch((e) => {
      NotifyState.notify({
        type: "error",
        title: "Error removing Deck",
        content: `An error ocurred removing a deck (${data.deckName})`,
      });
    });
}

type TUpdateDeck = {
  workspaceName: string;
  deckName: string;
  newDeckName: string;
};

function update_deck(data: TUpdateDeck, cb: QueryFn<string>) {
  invoke<string>("update_deck_handler", data)
    .then(cb)
    .catch((e) => {
      NotifyState.notify({
        type: "error",
        title: "Error updating deck",
        content: `An error ocurred updating ${data.deckName}`,
      });
    });
}

type TImportDeck = {
  workspaceName: string;
  filePath: string;
};

function import_deck(data: TImportDeck, cb: QueryFn<IDeckResponse>) {
  invoke<IDeckResponse>("import_deck_handler", data)
    .then(cb)
    .catch((e) => {
      NotifyState.notify({
        type: "error",
        title: "Error importing a deck",
        content: `An error ocurred importing ${data.filePath}`,
      });
    });
}

function create_deck(
  data: {
    workspaceName: string;
    deckName: string;
  },
  cb: QueryFn<IDeckResponse>
) {
  invoke<IDeckResponse>("create_deck_handler", data)
    .then(cb)
    .catch((e) => {
      NotifyState.notify({
        type: "error",
        title: "Error creating a deck",
        content: `An error ocurred creating the deck: ${data.deckName}`,
      });
    });
}

export const deckQuery = {
  remove_deck,
  update_deck,
  create_deck,
  import_deck,
};

interface Doc {
  id: number;
  title: string;
  text: string;
}

type TCreateDoc = {
  workspaceName: string;
  title: string;
  text: string;
};

function create_doc(data: TCreateDoc, cb: QueryFn<Doc>) {
  invoke<Doc>("create_note", data)
    .then(cb)
    .catch((e) => {
      NotifyState.notify({
        type: "error",
        title: "Error creating a Doc",
        content: `An error ocurred creating the doc: ${data.title}`,
      });
    });
}

type TRemoveDoc = {
  workspaceName: string;
  id: number;
};
function remove_doc(data: TRemoveDoc, cb: QueryFn<Doc>) {
  invoke<Doc>("delete_one_note", data)
    .then(cb)
    .catch((e) => {
      NotifyState.notify({
        type: "error",
        title: "Error removing Doc",
        content: `An error ocurred removing a doc`,
      });
    });
}

type TUpdateDoc = {
  workspaceName: string;
  id: number;
  title: string;
};
function update_doc(data: TUpdateDoc, cb: QueryFn<Doc>) {
  invoke<Doc>("update_note", data)
    .then(cb)
    .catch((e) => {
      NotifyState.notify({
        type: "error",
        title: "Error updating Doc",
        content: `An error ocurred updating ${data.title}`,
      });
    });
}

export const docQuery = {
  remove_doc,
  update_doc,
  create_doc,
};
