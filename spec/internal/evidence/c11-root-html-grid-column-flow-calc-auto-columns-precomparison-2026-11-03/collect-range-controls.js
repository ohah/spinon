const grid = document.querySelector('#grid');
const cases = [
  ['negative-result', 'calc(1px - 10%)', 240],
  ['over-100', 'calc(120% + 1px)', 240],
  ['zero-basis', 'calc(10% + 5px)', 0],
  ['mixed-fr', 'calc(10% + 5px) 1fr', 240],
];
JSON.stringify(
  cases.map(([name, track, width]) => {
    grid.style.gridAutoColumns = track;
    grid.style.width = `${width}px`;
    const style = getComputedStyle(grid);
    const rect = grid.getBoundingClientRect();
    return {
      name,
      width,
      authored: track,
      gridTemplateColumns: style.gridTemplateColumns,
      gridAutoColumns: style.gridAutoColumns,
      gridWidth: rect.width,
      firstImplicitItemX: grid.children[4].getBoundingClientRect().x,
      thirdColumnItemX: grid.children[8].getBoundingClientRect().x,
    };
  }),
);
