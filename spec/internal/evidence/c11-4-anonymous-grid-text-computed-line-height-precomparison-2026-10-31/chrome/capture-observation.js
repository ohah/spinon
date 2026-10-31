(async () => {
  await document.fonts.ready;
  const browserData = navigator.userAgentData
    ? await navigator.userAgentData.getHighEntropyValues(["fullVersionList", "platformVersion"])
    : null;
  const cases = [...document.querySelectorAll(".grid")].map((grid) => {
    const computed = getComputedStyle(grid);
    const rect = grid.getBoundingClientRect();
    const label = grid.parentElement.querySelector(".label");
    const input = label.textContent;
    label.textContent = `${input} → resolved ${computed.lineHeight}; track ${computed.gridTemplateRows}; root ${rect.height}px`;
    const range = document.createRange();
    range.selectNodeContents(grid);
    return {
      id: grid.id,
      input,
      resolvedLineHeight: computed.lineHeight,
      gridTemplateRows: computed.gridTemplateRows,
      rootHeight: rect.height,
      rangeRects: [...range.getClientRects()].map((item) => ({
        x: item.x,
        y: item.y,
        width: item.width,
        height: item.height,
      })),
    };
  });
  const observation = {
    title: document.title,
    url: location.href,
    userAgent: navigator.userAgent,
    browserData,
    language: navigator.language,
    viewport: { width: innerWidth, height: innerHeight, devicePixelRatio },
    fontLoaded: document.fonts.check("10px Ahem"),
    cases,
  };
  window.__lineHeightObservation = observation;
  return observation;
})();
