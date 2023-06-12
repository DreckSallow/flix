import { ICard, ICardResponse } from "@interfaces/index";
import { sep } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";

const mediaToken = {
  audio: "$audio",
  image: "$image",
};

export const formatCard = (card: ICardResponse, media_path: string): ICard => {
  const front = replaceTokens(card.front, card.items, media_path);
  const back = replaceTokens(
    card.back,
    {
      ...card.items,
      FrontSide: front,
    },
    media_path
  );
  return {
    id: card.id,
    back,
    front,
  };
};

export const getTokens = (text: string): string[] => {
  let tokens = text.match(/{{(.*?)}}/g);
  return tokens ?? [];
};

export const replaceTokens = (
  text: string,
  replaces: { [key: string]: string },
  media_path: string
): string => {
  const tokens = getTokens(text);
  let text_parsed = text;

  tokens.forEach((tk) => {
    let textToInsert = replaces[tk.slice(2, tk.length - 2)];
    if (!textToInsert) {
      textToInsert = " ";
    }
    if (textToInsert.startsWith(mediaToken.audio)) {
      let audio = textToInsert.split(":").pop() as string;
      textToInsert = `<audio src="${convertFileSrc(
        media_path + sep + audio
      )}" controls></audio>`;
    } else if (textToInsert.startsWith(mediaToken.image)) {
      let image = textToInsert.split(":").pop() as string;
      textToInsert = `<img src="${convertFileSrc(
        media_path + sep + image
      )}"></img>`;
    }
    text_parsed = text_parsed.replaceAll(tk, textToInsert);
  });
  return text_parsed;
};
