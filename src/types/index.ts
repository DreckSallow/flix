export interface Workspace {
  name: string;
  image?: string;
}

export interface Card {
  front: string;
  back: string;
  audio_path?: string;
  image_path?: string;
}

export interface Deck {
  name: string;
  cards: Card[];
}
