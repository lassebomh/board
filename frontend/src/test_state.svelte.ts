import king from "$lib/assets/sprite/king.svg?raw";
import type { Viewer } from "$lib/state";

export let viewer = $state<Viewer>({
  player_id: "lasse",
  camera: {
    x: 0,
    y: 0,
    zoom: 0,
  },
  state: {
    players: [
      {
        color: "red",
        id: "lasse",
        name: "Lasse",
      },
    ],

    sprites: [
      {
        content: king,
        id: "king",
      },
    ],
    ground: [
      {
        player_id: null,
        sprite_id: "king",
        x: 0,
        y: 0,
      },
      {
        player_id: null,
        sprite_id: "king",
        x: 100,
        y: 100,
      },
    ],
  },
});
