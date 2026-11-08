import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const evidence = path.join(
  repo,
  'spec/internal/evidence/c11-4-root-html-repeated-intrinsic-tail-permutations-precomparison-2026-11-08',
);
const fixture = path.join(
  repo,
  'tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-tail-permutations-precomparison.html',
);
assert.ok(fs.existsSync(evidence), `evidence directory must be created first: ${evidence}`);
assert.ok(!fs.existsSync(fixture), `preserve existing fixture: ${fixture}`);
assert.ok(!fs.existsSync(path.join(evidence, 'expected.json')), 'preserve existing expected.json');

function uniquePermutations(values) {
  const sorted = [...values].sort();
  const result = [];
  const next = (items) => {
    let pivot = items.length - 2;
    while (pivot >= 0 && items[pivot] >= items[pivot + 1]) pivot -= 1;
    if (pivot < 0) return false;
    let successor = items.length - 1;
    while (items[successor] <= items[pivot]) successor -= 1;
    [items[pivot], items[successor]] = [items[successor], items[pivot]];
    for (let left = pivot + 1, right = items.length - 1; left < right; left += 1, right -= 1) {
      [items[left], items[right]] = [items[right], items[left]];
    }
    return true;
  };
  do result.push([...sorted]); while (next(sorted));
  return result;
}

const tails = uniquePermutations([
  'min-content',
  'min-content',
  'max-content',
  'max-content',
  'auto',
]);
assert.equal(tails.length, 30, 'the repeated tail multiset has exactly 30 unique orders');
const payloadHeights = [20, 10, 12, 6, 6, 6, 60, 6];
const caps = ['20px', '20%'];
const alignments = ['start', 'stretch'];
const scenes = [];
const htmlScenes = [];

function format(value) {
  return Number.isInteger(value) ? String(value) : String(Number(value.toFixed(6)));
}

for (let orderIndex = 0; orderIndex < tails.length; orderIndex += 1) {
  const tail = tails[orderIndex];
  const sequence = ['fit-content', ...tail];
  for (const cap of caps) {
    for (const alignContent of alignments) {
      const id = `order-${String(orderIndex + 1).padStart(2, '0')}-${cap === '20px' ? 'px' : 'percent'}-${alignContent}`;
      const baseImplicitRows = payloadHeights.map((payload, index) => {
        const track = sequence[index % sequence.length];
        return track === 'fit-content' ? Math.min(payload, 20) : payload;
      });
      const autoIndexes = payloadHeights.flatMap((_, index) =>
        sequence[index % sequence.length] === 'auto' ? [index] : [],
      );
      const freeSpace = 100 - 10 - baseImplicitRows.reduce((sum, size) => sum + size, 0);
      assert.equal(freeSpace, 4, `${id}: independent model should leave 4px free space`);
      const implicitRows = [...baseImplicitRows];
      if (alignContent === 'stretch') {
        const autoShare = freeSpace / autoIndexes.length;
        for (const index of autoIndexes) implicitRows[index] += autoShare;
      }
      const rows = [10, ...implicitRows];
      let y = 0;
      const items = rows.map((height, index) => {
        const item = { index, x: 0, y, width: 40, height };
        y += height;
        return item;
      });
      const payloads = payloadHeights.map((height, index) => ({
        index: index + 1,
        x: 0,
        y: items[index + 1].y,
        width: 40,
        height,
      }));
      const autoRows = implicitRows.filter((_, index) => autoIndexes.includes(index));
      const expected = {
        id,
        orderIndex: orderIndex + 1,
        suffix: tail,
        sequence,
        cap,
        alignContent,
        explicitRows: [10],
        implicitRows,
        rows,
        items,
        payloads,
        autoIndexes,
        autoShare: alignContent === 'stretch' ? freeSpace / autoIndexes.length : 0,
        totalRows: rows.reduce((sum, size) => sum + size, 0),
        fitContentPayloadOverflowPastGrid: payloads[6].y + payloads[6].height - 100,
        serializedTrackList: `fit-content(${cap}) ${tail.join(' ')}`,
      };
      scenes.push(expected);

      const children = [
        '<div class="item explicit" data-item="0"></div>',
        ...payloadHeights.map(
          (height, index) =>
            `<div class="item" data-item="${index + 1}"><div class="payload" data-payload="${index + 1}" style="height:${height}px"></div></div>`,
        ),
      ].join('\n          ');
      htmlScenes.push(`      <section class="scene" data-scene="${id}" data-order="${orderIndex + 1}" data-cap="${cap}" data-align="${alignContent}">
        <div class="label">${id}</div>
        <div class="grid" style="grid-auto-rows:${expected.serializedTrackList};align-content:${alignContent}">
          ${children}
        </div>
      </section>`);
    }
  }
}
assert.equal(scenes.length, 120);

const document = `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>C11.4 repeated intrinsic tail permutations</title>
    <style>
      * { box-sizing: border-box; }
      html, body { margin: 0; padding: 0; font: 10px/12px sans-serif; }
      main { display: grid; grid-template-columns: repeat(8, 54px); gap: 6px; padding: 8px; }
      .scene { display: flex; flex-direction: column; align-items: flex-start; gap: 2px; }
      .label { width: 54px; overflow-wrap: anywhere; color: #334155; }
      .grid {
        display: grid; width: 40px; height: 100px;
        grid-template-columns: 40px; grid-template-rows: 10px;
        grid-auto-flow: row; row-gap: 0; column-gap: 0;
        align-content: start; align-items: normal;
        justify-content: start; justify-items: normal;
        overflow: visible; margin: 0; padding: 0; border: 0;
      }
      .item { display: block; width: auto; height: auto; min-height: 0; margin: 0; padding: 0; border: 0; }
      .explicit { height: 10px; background: #234e70; }
      .payload { display: block; width: 40px; margin: 0; padding: 0; border: 0; background: #f2b134; }
      .item[data-item="1"] .payload { background: #f2b134; }
      .item[data-item="2"] .payload { background: #4a90a4; }
      .item[data-item="3"] .payload { background: #d1495b; }
      .item[data-item="4"] .payload { background: #6a994e; }
      .item[data-item="5"] .payload { background: #9b5de5; }
      .item[data-item="6"] .payload { background: #f15bb5; }
      .item[data-item="7"] .payload { background: #ff8c42; }
      .item[data-item="8"] .payload { background: #00a896; }
    </style>
  </head>
  <body>
    <main>
${htmlScenes.join('\n')}
    </main>
    <script>
      window.__gridObservation = () => Array.from(document.querySelectorAll('[data-scene]')).map((scene) => {
        const grid = scene.querySelector('.grid');
        const base = grid.getBoundingClientRect();
        const box = (element) => {
          const rect = element.getBoundingClientRect();
          return { x: rect.x - base.x, y: rect.y - base.y, width: rect.width, height: rect.height };
        };
        return {
          id: scene.dataset.scene,
          order: Number(scene.dataset.order),
          cap: scene.dataset.cap,
          alignContent: getComputedStyle(grid).alignContent,
          gridAutoRows: getComputedStyle(grid).gridAutoRows,
          gridTemplateRows: getComputedStyle(grid).gridTemplateRows.split(/\\s+/).map((value) => Number.parseFloat(value)),
          grid: { width: base.width, height: base.height },
          items: Array.from(grid.querySelectorAll('[data-item]')).map((item) => ({ index: Number(item.dataset.item), ...box(item) })),
          payloads: Array.from(grid.querySelectorAll('[data-payload]')).map((payload) => ({ index: Number(payload.dataset.payload), ...box(payload) })),
        };
      });
    </script>
  </body>
</html>
`;

fs.writeFileSync(fixture, document, { flag: 'wx' });
fs.writeFileSync(path.join(evidence, 'expected.json'), `${JSON.stringify({
  normativeRule: 'CSS Grid 2 §7.6 repeated implicit track list; §12.8 stretch auto tracks',
  source: 'independent fixed-payload track model; not generated from Chrome or Spinon',
  orderCount: tails.length,
  sceneCount: scenes.length,
  payloadHeights,
  scenes,
}, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ fixture, evidence, orderCount: tails.length, sceneCount: scenes.length, uniqueOrders: tails }, null, 2)}\n`);
