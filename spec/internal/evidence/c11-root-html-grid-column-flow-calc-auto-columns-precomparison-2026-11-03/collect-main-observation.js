const grid = document.querySelector('#grid');
const rect = (element) => {
  const value = element.getBoundingClientRect();
  return {
    x: value.x,
    y: value.y,
    width: value.width,
    height: value.height,
    right: value.right,
    bottom: value.bottom,
  };
};
JSON.stringify({
  userAgent: navigator.userAgent,
  viewport: { width: innerWidth, height: innerHeight, dpr: devicePixelRatio },
  grid: {
    computed: {
      width: getComputedStyle(grid).width,
      height: getComputedStyle(grid).height,
      gridTemplateColumns: getComputedStyle(grid).gridTemplateColumns,
      gridTemplateRows: getComputedStyle(grid).gridTemplateRows,
      gridAutoColumns: getComputedStyle(grid).gridAutoColumns,
      columnGap: getComputedStyle(grid).columnGap,
      rowGap: getComputedStyle(grid).rowGap,
      direction: getComputedStyle(grid).direction,
    },
    rect: rect(grid),
  },
  items: [...grid.children].map((item, index) => ({ index, rect: rect(item) })),
});
