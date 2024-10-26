<script lang="ts">
  import king from "$lib/assets/sprite/king.svg?raw";

  let ground: SVGGElement;
  let lift: SVGGElement;

  function hoverAction(el: SVGUseElement) {
    function mouseleave(e: MouseEvent) {
      el.setAttribute("filter", "");
    }

    function mouseenter(e: MouseEvent) {
      el.setAttribute("filter", "url(#filter_lifted)");
      el.addEventListener("mouseleave", mouseleave);
    }

    el.addEventListener("mouseenter", mouseenter);

    return {
      destroy() {
        el.removeEventListener("mouseenter", mouseenter);
        el.removeEventListener("mouseleave", mouseleave);
      },
    };
  }
</script>

<h1>Welcome to SvelteKit</h1>
<p>
  Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the
  documentation
</p>

<svg width="200" version="1.1" height="200" xmlns="http://www.w3.org/2000/svg">
  <g id="sprites">
    <symbol id="sprite_king">
      {@html king}
    </symbol>
  </g>

  <g bind:this={ground}>
    <use href="#sprite_king" use:hoverAction></use>
  </g>

  <g bind:this={lift}></g>

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
