import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const repoRoot = process.cwd();
const outputDirectory = resolve(
  repoRoot,
  'spec/internal/evidence/c11-4-root-html-repeated-text-later-fit-content-row-span-implementation-2026-11-09/platform-smoke',
);
const matrixPath = resolve(
  repoRoot,
  'spec/internal/evidence/c11-4-root-html-repeated-text-later-fit-content-row-span-precomparison-2026-11-09/chrome157-matrix-observation.ndjson',
);
const lines = (await readFile(matrixPath, 'utf8')).trimEnd().split('\n');
const metadata = JSON.parse(lines[0]);
const scenes = lines.slice(1).map((line) => JSON.parse(line));
if (metadata.caseCount !== 1920 || scenes.length !== 1920) {
  throw new Error('Chrome matrix must contain exactly 1,920 scenes.');
}

const selectedTail = ['min-content', 'max-content', 'min-content', 'max-content', 'auto'];
const payloads = {
  A: 'A A A A A A',
  B: 'A A A A A A A A',
  C: 'A A A A A A A A A A A A',
};
const css = (trackList, align) => `
* { box-sizing: border-box; }
html, body { margin: 0; padding: 0; }
#grid { display: grid; width: 40px; height: 220px; grid-template-columns: 40px; grid-template-rows: 10px; grid-auto-rows: ${trackList}; grid-auto-flow: row; align-content: ${align}; align-items: normal; justify-content: start; justify-items: normal; direction: ltr; overflow: visible; margin: 0; padding: 0; row-gap: 0; column-gap: 0; }
.item { display: block; width: auto; height: auto; min-height: 0; margin: 0; padding: 0; overflow: visible; grid-column: 1 / 2; }
.empty { height: 10px; background-color: rgb(11, 17, 23); }
.payload { display: block; width: auto; height: auto; margin: 0; padding: 0; font-family: monospace; font-size: 10px; font-weight: 400; font-style: normal; line-height: 10px; letter-spacing: 0; word-spacing: 0; white-space: normal; direction: ltr; }
.item[data-item="A"] { background-color: rgb(31, 41, 51); }
.item[data-item="B"] { background-color: rgb(61, 71, 81); }
.item[data-item="C"] { background-color: rgb(91, 101, 111); }
.item[data-item="A"] .payload { background-color: rgb(121, 131, 141); }
.item[data-item="B"] .payload { background-color: rgb(151, 161, 171); }
.item[data-item="C"] .payload { background-color: rgb(181, 191, 201); }
`;

function sourceFor(scene, align) {
  const children = scene.items.map((item) => {
    if (item.id === 'empty-0') {
      return '<div class="empty" style="grid-row:1 / 2;grid-column:1 / 2"></div>';
    }
    return `<div class="item" data-item="${item.id}" style="grid-row:${item.rowStart} / ${item.rowEnd}"><div class="payload">${payloads[item.id]}</div></div>`;
  }).join('');
  const tracks = scene.trackList.join(' ');
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><style>${css(tracks, align)}</style></head><body><div id="grid">${children}</div></body></html>`;
}

await mkdir(outputDirectory, { recursive: true });
const manifest = [];
for (const fitPosition of [2, 3]) {
  for (const align of ['start', 'stretch']) {
    const sceneIndex = scenes.findIndex((scene) =>
      scene.fitPosition === fitPosition
      && JSON.stringify(scene.tail) === JSON.stringify(selectedTail)
      && scene.cap === '20px'
      && scene.align === align
      && scene.topology === 'span2-before-span3');
    if (sceneIndex < 0) throw new Error(`Missing sentinel scene ${fitPosition}/${align}.`);
    const scene = scenes[sceneIndex];
    const filename = `position-${fitPosition}-${align}.html`;
    const source = sourceFor(scene, align);
    await writeFile(resolve(outputDirectory, filename), source);
    manifest.push({
      filename,
      sceneIndex,
      fitPosition,
      tail: scene.tail,
      cap: scene.cap,
      align,
      topology: scene.topology,
      sha256: createHash('sha256').update(source).digest('hex'),
    });
  }
}
await writeFile(
  resolve(outputDirectory, 'manifest.json'),
  `${JSON.stringify({ matrix: matrixPath, caseCount: scenes.length, sentinels: manifest }, null, 2)}\n`,
);
