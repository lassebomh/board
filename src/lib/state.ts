export type Player = {
  color: string;
  name: string;
  id: string;
  held: Piece[];
};

export type Sprite = {
  content: string;
  id: string;
};

export type Piece = {
  sprite_id: string;
  x: number;
  y: number;
};

export type State = {
  sprites: Sprite[];
  players: Player[];
  ground: Piece[];
};

export type Camera = {
  x: number;
  y: number;
  zoom: number;
};

export type Viewer = {
  player_id: string;
  camera: Camera;
  state: State;
};
