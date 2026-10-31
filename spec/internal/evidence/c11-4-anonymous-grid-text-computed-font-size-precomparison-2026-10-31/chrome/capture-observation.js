(async () => {
  const cases = window.__C114_FONT_SIZE_CASES__;
  if (!cases || !window.__C114_FONT_SIZE_READY__) {
    throw new Error("C11.4 font-size fixture is not ready");
  }
  await window.__C114_FONT_SIZE_READY__;
  const ua = navigator.userAgent;
  const brands = navigator.userAgentData?.brands ?? [];
  const fullVersionList = navigator.userAgentData?.fullVersionList ?? [];
  const rows = cases.map(({ id, fontSize, track }) => {
    const grid = document.getElementById(id);
    if (!grid) throw new Error(`Missing grid ${id}`);
    const style = getComputedStyle(grid);
    const rect = grid.getBoundingClientRect();
    return {
      id,
      inputFontSizePx: fontSize,
      inputTrack: track,
      computedFontSize: style.fontSize,
      gridTemplateColumns: style.gridTemplateColumns,
      gridTemplateRows: style.gridTemplateRows,
      rootFrame: { width: rect.width, height: rect.height },
    };
  });
  return {
    browser: {
      userAgent: ua,
      brands,
      fullVersionList,
      language: navigator.language,
    },
    viewport: { width: innerWidth, height: innerHeight, dpr: devicePixelRatio },
    fontLoaded: document.fonts.check("10px Ahem"),
    cases: rows,
  };
})()
