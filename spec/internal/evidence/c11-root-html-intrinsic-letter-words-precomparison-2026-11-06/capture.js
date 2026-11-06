(() => {
const canvas = document.createElement('canvas');
const context = canvas.getContext('2d');
context.font = '10px monospace';
const cases = Array.from(document.querySelectorAll('.case')).map(section => {
  const grid = section.querySelector('.grid');
  const items = Array.from(grid.children);
  const word = section.querySelector('.word-text');
  const range = document.createRange();
  range.selectNodeContents(word);
  const rect = node => {
    const r = node.getBoundingClientRect();
    return [r.x, r.y, r.width, r.height];
  };
  const lineCount = node => {
    const selected = document.createRange();
    selected.selectNodeContents(node);
    return selected.getClientRects().length;
  };
  return {
    id: section.dataset.case,
    sourceOrder: section.dataset.order || 'forward',
    columnsMode: grid.dataset.columns,
    source: word.textContent,
    usedColumns: getComputedStyle(grid).gridTemplateColumns,
    usedRows: getComputedStyle(grid).gridTemplateRows,
    gridFrame: rect(grid),
    itemFrames: items.map(rect),
    textLineRects: Array.from(range.getClientRects()).map(r => [r.x, r.y, r.width, r.height]),
    shortLineCount: lineCount(section.querySelector('.short-text'))
  };
});
const wordTokens = Array.from(new Set(cases.flatMap(item => item.source.split(/\s+/))));
return JSON.stringify({
  userAgent: navigator.userAgent,
  viewport: [innerWidth, innerHeight, devicePixelRatio],
  canvasAdvanceCssPx: {
    A: context.measureText('A').width,
    B: context.measureText('B').width,
    space: context.measureText(' ').width,
    AA: context.measureText('AA').width,
    BBB: context.measureText('BBB').width,
    ABC: context.measureText('ABC').width,
    a: context.measureText('a').width,
    ccc: context.measureText('ccc').width
  },
  wordAdvancesCssPx: Object.fromEntries(wordTokens.map(word => [word, context.measureText(word).width])),
  cases
}, null, 2);
})();
