<script lang="ts">
  import { viewer } from "../test_state.svelte";
  import { type Piece } from "$lib/state";

  let ground: SVGGElement;
  let groups: Record<string, SVGGElement> = $state({});

  type GroupData = {
    count: number;
    pressed: boolean;
    x: number;
    y: number;
    dragElement: SVGGElement | null;
  };
  let groupData: Record<string, GroupData> = {};

  function groupAction(el: SVGGElement, player_id: string) {
    const group = {
      count: 0,
      pressed: true,
      x: 0,
      y: 0,
      dragElement: null,
    };

    groupData[player_id] = group;

    el.addEventListener("pointerdown", (ev: PointerEvent) => {});

    el.addEventListener("pointermove", (ev: PointerEvent) => {});

    function pointerup(ev: PointerEvent) {
      const samePos = ev.offsetX === group.x && ev.offsetY === group.y;
      if (!samePos || group.dragElement === ev.target) {
        ev.stopPropagation();
      } else {
        // group.dragElement = null;
      }
    }

    el.addEventListener("pointerup", pointerup, { capture: true });
  }

  function pieceAction(el: SVGGElement, piece: Piece) {
    let toggle = false;
    el.addEventListener("pointerdown", (ev: PointerEvent) => {
      if (piece.player_id === null) {
        toggle = true;

        const player_id = viewer.player_id;
        piece.player_id = player_id;
        el.setAttribute("filter", `url(#filter_group_${player_id})`);
        groups[player_id].append(el);

        const group = groupData[player_id];

        group.count += 1;
        group.pressed = true;
        group.x = ev.offsetX;
        group.y = ev.offsetY;
        group.dragElement = el;
      }
    });

    el.addEventListener("pointerup", () => {
      console.log(piece);

      if (piece.player_id !== null) {
        piece.player_id = null;
        el.removeAttribute("filter");
        ground.append(el);
      } else {
        // ground.append(el);
        piece.player_id = null;
        el.removeAttribute("filter");
      }
    });
  }

  $inspect(groupData);
</script>

<svg
  width={window.innerWidth}
  height={window.innerHeight}
  version="1.1"
  xmlns="http://www.w3.org/2000/svg"
>
  <g id="sprites">
    {#each viewer.state.sprites as sprite}
      <symbol id="sprite_{sprite.id}">
        {@html sprite.content}
      </symbol>
    {/each}
  </g>

  <g bind:this={ground}>
    {#each viewer.state.ground as piece}
      <use
        href="#sprite_{piece.sprite_id}"
        transform="translate({piece.x} {piece.y})"
        use:pieceAction={piece}
      >
      </use>
    {/each}

    {#each viewer.state.players as player (player.id)}
      <g bind:this={groups[player.id]} use:groupAction={player.id}></g>

      <filter width="150%" height="150%" id="filter_group_{player.id}">
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
    {/each}
  </g>

  <g id="filters">
    <filter width="150%" height="150%" id="filter_hover">
      <feDropShadow dx="3" dy="5" stdDeviation="3" flood-opacity="0.3">
      </feDropShadow>
    </filter>
    <!-- <filter width="150%" height="150%" id="filter_piece_mutual">
      <feDropShadow dx="1" dy="1" stdDeviation="1"></feDropShadow>
    </filter>
    <filter width="150%" height="150%" id="filter_stack_mutual">
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
    </filter> -->
  </g>
</svg>
