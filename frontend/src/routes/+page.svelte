<script lang="ts">
  import king from "$lib/assets/sprite/king.svg?raw";
  import { onMount } from "svelte";

  import { viewer } from "../test_state.svelte";
  import { type Piece } from "$lib/state";

  let ground: SVGGElement;
  let root: SVGElement;

  onMount(() => {});

  let playerLayer: SVGGElement[] = $state([]);

  let players = [0];
</script>

{#snippet pieceElement(piece: Piece)}
  <use href="#sprite_{piece.sprite_id}" transform="translate({piece.x} {piece.y})"></use>
{/snippet}

<svg bind:this={root} version="1.1" xmlns="http://www.w3.org/2000/svg">
  <g id="sprites">
    {#each viewer.state.sprites as sprite}
      <symbol id="sprite_{sprite.id}">
        {@html sprite.content}
      </symbol>
    {/each}
  </g>

  <g bind:this={ground}>
    {#each viewer.state.ground as piece}
      {@render pieceElement(piece)}
    {/each}

    {#each viewer.state.players as player, i}
      <g id="player_{player.id}">
        {#each player.held as piece}
          {@render pieceElement(piece)}
        {/each}
      </g>
    {/each}
  </g>

  <g id="filters">
    <filter width="150%" height="150%" id="filter_piece_mutual">
      <feDropShadow dx="1" dy="1" stdDeviation="1"></feDropShadow>
    </filter>
    <filter width="150%" height="150%" id="filter_stack_mutual">
      <feDropShadow dx="1" dy="1" stdDeviation="1"></feDropShadow>
    </filter>
    <filter width="150%" height="150%" id="filter_lifted">
      <feDropShadow dx="3" dy="5" stdDeviation="3" flood-opacity="0.3"
      ></feDropShadow>
    </filter>
    <filter width="150%" height="150%" id="filter_piece_player_0">
      <feMorphology
        in="SourceGraphic"
        result="DILATED"
        operator="dilate"
        radius="1"
      ></feMorphology>
      <feFlood flood-color="red" flood-opacity="1" result="COLOR"></feFlood>
      <feComposite in="COLOR" in2="DILATED" operator="in" result="OUTLINE"
      ></feComposite>
      <feMerge>
        <feMergeNode in="OUTLINE"></feMergeNode>
        <feMergeNode in="SourceGraphic"></feMergeNode>
      </feMerge>
      <feDropShadow dx="1" dy="1" stdDeviation="1"></feDropShadow>
    </filter>
    <filter width="150%" height="150%" id="filter_stack_player_0">
      <feMorphology
        in="SourceGraphic"
        result="DILATED"
        operator="dilate"
        radius="1"
      >
      </feMorphology>
      <feFlood flood-color="red" flood-opacity="1" result="COLOR"></feFlood>
      <feComposite in="COLOR" in2="DILATED" operator="in" result="OUTLINE">
      </feComposite>
      <feMerge>
        <feMergeNode in="OUTLINE"></feMergeNode>
        <feMergeNode in="SourceGraphic"></feMergeNode>
      </feMerge>
      <feDropShadow dx="1" dy="1" stdDeviation="1"></feDropShadow>
    </filter>
  </g>
</svg>
