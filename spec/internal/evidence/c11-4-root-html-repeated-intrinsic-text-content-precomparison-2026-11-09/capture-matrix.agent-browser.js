(async () => {
  const grid = document.querySelector("#grid");
  const contentCases = [
    {
      id: "legacy-1290",
      payloads: [
        "A A A A A A",
        "AAAA AAAA A",
        "AA AA AA AA",
        "A A",
        "AAA AAA",
        "AAAA",
        "A A A A A A A A",
        "AA"
      ]
    },
    {
      id: "varied-ascii",
      payloads: [
        "B C D E F G H",
        "BCDEFG AB",
        "ABCDEF ABCDEF",
        "AB CD EF GH",
        "B AB C ABCD",
        "CDEF A B",
        "ABCDE F BC",
        "B A BC D E F"
      ]
    },
    {
      id: "length-boundary-32",
      payloads: [
        "ABCDEF ABCDEF ABCDEF ABCDEF ABCD",
        "A B C D E F G H I J K L M N O P",
        "ABCDEF AB CDEF AB CDEF",
        "ABCD EFGHIJ KLMN OP",
        "AAA BBB CCC DDD",
        "ABCDEF",
        "AB CD EF GH IJ KL",
        "A BCDEF ABCDEF ABCDEF"
      ]
    }
  ];
  const round = value => Math.round(value * 100000) / 100000;
  const box = element => {
    const rect = element.getBoundingClientRect();
    return {
      x: round(rect.x),
      y: round(rect.y),
      width: round(rect.width),
      height: round(rect.height)
    };
  };
  const descriptorsFor = (contentCase, payloads) => {
    payloads.forEach((text, index) => {
      document.querySelector(`#text-${index + 1}`).textContent = text;
    });
    void grid.offsetHeight;
    return payloads.map((text, index) => {
      const element = document.querySelector(`#text-${index + 1}`);
      const range = document.createRange();
      range.selectNodeContents(element);
      const lineTops = [
        ...new Set([...range.getClientRects()].map(rect => round(rect.top)))
      ];
      const style = getComputedStyle(element);
      return {
        contentCase,
        id: element.id,
        text,
        textLength: [...text].length,
        font: style.fontFamily,
        fontSize: style.fontSize,
        lineHeight: style.lineHeight,
        whiteSpace: style.whiteSpace,
        lineCount: lineTops.length
      };
    });
  };
  const counts = new Map([
    ["min-content", 2],
    ["max-content", 2],
    ["auto", 1]
  ]);
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
  const textDescriptors = [];
  const scenes = [];
  const capture = (contentCase, tail, cap, align, gridHeight) => {
    grid.style.setProperty("--tail", tail);
    grid.style.setProperty("--cap", cap);
    grid.style.setProperty("--align", align);
    grid.style.setProperty("--grid-height", `${gridHeight}px`);
    void grid.offsetHeight;
    const gridRowsCss = getComputedStyle(grid).gridTemplateRows.split(/\s+/);
    const frames = [...grid.children].map(item => {
      const payload = item.querySelector(".payload");
      return {
        id: item.id,
        ...box(item),
        payload: payload ? box(payload) : null
      };
    });
    scenes.push({
      contentCase,
      tail,
      cap,
      align,
      gridHeight,
      gridRowsCss,
      gridRows: gridRowsCss.map(value => round(parseFloat(value))),
      gridFrame: box(grid),
      frames
    });
  };
  for (const contentCase of contentCases) {
    textDescriptors.push(...descriptorsFor(contentCase.id, contentCase.payloads));
    for (const tail of tails) {
      for (const cap of ["20px", "20%", "0%", "120%"])
        for (const align of ["start", "stretch"])
          capture(contentCase.id, tail, cap, align, 220);
      capture(contentCase.id, tail, "20px", "start", 100);
    }
  }
  return {
    ua: navigator.userAgent,
    dpr: devicePixelRatio,
    fixtureTitle: document.title,
    uniqueTailOrders: tails.length,
    contentCases,
    textDescriptors,
    scenes
  };
})()
