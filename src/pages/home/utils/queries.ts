import { invoke } from "@tauri-apps/api";
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
        title: "Remove Deck",
        content: `An error occurred while removing the deck: ${data.deckName}`,
        type: "error",
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
        title: "Update Deck",
        content: `Failed to update the deck: ${data.deckName}`,
        type: "error",
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
        title: "Import Deck",
        content: `An error occurred while importing: ${data.filePath}`,
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
        title: "Create Deck",
        content: `An error occurred while creating the deck: ${data.deckName}`,
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
        title: "Create Doc",
        content: `Failed to create the doc: ${data.title}`,
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
        title: "Remove Doc",
        content: `Failed to remove the doc`,
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
        title: "Update Doc",
        content: `Error updating doc: ${data.title}`,
      });
    });
}

export const docQuery = {
  remove_doc,
  update_doc,
  create_doc,
};
