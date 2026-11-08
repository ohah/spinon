(async () => {
  const grid = document.querySelector("#grid");
  const round = value => Math.round(value * 100000) / 100000;
  const box = element => {
    const rect = element.getBoundingClientRect();
    return { x: round(rect.x), y: round(rect.y), width: round(rect.width), height: round(rect.height) };
  };
  const textDescriptors = [...document.querySelectorAll(".payload")].map(element => {
    const range = document.createRange();
    range.selectNodeContents(element);
    const rects = [...range.getClientRects()];
    const lineTops = [...new Set(rects.map(rect => round(rect.top)))];
    const style = getComputedStyle(element);
    return {
      id: element.id,
      text: element.textContent,
      font: style.fontFamily,
      fontSize: style.fontSize,
      lineHeight: style.lineHeight,
      whiteSpace: style.whiteSpace,
      lineCount: lineTops.length
    };
  });
  const counts = new Map([["min-content", 2], ["max-content", 2], ["auto", 1]]);
  const tails = [];
  const visit = prefix => {
    if (prefix.length === 5) {
      tails.push(prefix.join(" "));
      return;
    }
    for (const value of ["min-content", "max-content", "auto"]) {
      const count = counts.get(value);
      if (count === 0) continue;
      counts.set(value, count - 1);
      prefix.push(value);
      visit(prefix);
      prefix.pop();
      counts.set(value, count);
    }
  };
  visit([]);
  const scenes = [];
  const capture = (tail, cap, align, gridHeight) => {
    grid.style.setProperty("--tail", tail);
    grid.style.setProperty("--cap", cap);
    grid.style.setProperty("--align", align);
    grid.style.setProperty("--grid-height", `${gridHeight}px`);
    void grid.offsetHeight;
    const style = getComputedStyle(grid);
    const gridRowsCss = style.gridTemplateRows.split(/\s+/);
    const frames = [...grid.children].map(item => {
      const payload = item.querySelector(".payload");
      return { id: item.id, ...box(item), payload: payload ? box(payload) : null };
    });
    scenes.push({
      tail, cap, align, gridHeight,
      gridRowsCss,
      gridRows: gridRowsCss.map(value => round(parseFloat(value))),
      gridFrame: box(grid),
      frames
    });
  };
  for (const tail of tails) {
    for (const cap of ["20px", "20%", "0%", "120%"]) {
      for (const align of ["start", "stretch"]) capture(tail, cap, align, 220);
    }
    capture(tail, "20px", "start", 100);
  }
  return {
    ua: navigator.userAgent,
    dpr: devicePixelRatio,
    fixtureTitle: document.title,
    uniqueTailOrders: tails.length,
    textDescriptors,
    scenes
  };
})()
