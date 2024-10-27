function hoverAction(el: SVGUseElement) {
  const selectLayer = playerLayer[0];

  let clientX = 0;
  let clientY = 0;

  // let offsetX = 0;
  // let offsetY = 0;

  function pointermove(e: PointerEvent) {
    console.log(e);
    selectLayer.setAttribute(
      "transform",
      `translate(${e.clientX - clientX} ${e.clientY - clientY})`
    );
  }

  function pointerup(e: PointerEvent) {
    el.removeEventListener("pointermove", pointermove);
    selectLayer.removeAttribute("transform");
    el.setAttribute(
      "transform",
      el.getAttribute("transform") +
        ` translate(${e.clientX - clientX} ${e.clientY - clientY})`
    );
    ground.append(el);
  }

  function pointerdown(e: MouseEvent) {
    el.setAttribute("filter", "url(#filter_piece_player_0)");

    clientX = e.clientX;
    clientY = e.clientY;

    selectLayer.append(el);

    console.log(e);

    el.addEventListener("pointermove", pointermove);
    el.addEventListener("pointerup", pointerup, { once: true });
  }

  function mouseleave() {
    el.removeAttribute("filter");
  }

  function mouseenter() {
    el.setAttribute("filter", "url(#filter_lifted)");
    el.addEventListener("mouseleave", mouseleave, { once: true });

    el.addEventListener("pointerdown", pointerdown);
  }

  el.addEventListener("mouseenter", mouseenter);

  return {
    destroy() {
      el.removeEventListener("mouseenter", mouseenter);
      el.removeEventListener("mouseleave", mouseleave);
    },
  };
}
