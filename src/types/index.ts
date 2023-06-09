export interface IWorkspace {
  name: string;
  image?: string;
}

export interface ICard {
  id: number;
  front: string;
  back: string;
}

export interface IDeck {
  name: string;
  cards: ICard[];
}

export interface ICardResponse {
  front: string;
  back: string;
  id: number;
  items: {
    [k: string]: string;
  };
}

export interface IDeckResponse {
  path: string;
  name: string;
  cards: ICardResponse[];
}
