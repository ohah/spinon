import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const repository = resolve(here, '../../../..');
const sourcePath = resolve(
  repository,
  'tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-two-fit-content-runtime.html',
);
const scenesDirectory = resolve(here, 'scenes');
const source = await readFile(sourcePath, 'utf8');

const cases = [
  { name: 'p12-start-zero-mixed', positions: [1, 2], caps: ['0px', '20px'], align: 'start' },
  { name: 'p12-stretch-over-zero', positions: [1, 2], caps: ['120%', '0%'], align: 'stretch' },
  { name: 'p25-start-mixed', positions: [2, 5], caps: ['20%', '40px'], align: 'start' },
  { name: 'p25-stretch-zero-over', positions: [2, 5], caps: ['0px', '120%'], align: 'stretch' },
  { name: 'p16-start-mixed-over', positions: [1, 6], caps: ['40px', '120%'], align: 'start' },
  { name: 'p16-stretch-mixed-zero', positions: [1, 6], caps: ['20%', '0px'], align: 'stretch' },
];

await mkdir(scenesDirectory, { recursive: true });
for (const { name, positions, caps, align } of cases) {
  const fitCaps = new Map(positions.map((position, index) => [position, caps[index]]));
  const remaining = ['min-content', 'max-content', 'auto', 'min-content'];
  const rows = [];
  for (let position = 1; position <= 6; position += 1) {
    rows.push(fitCaps.has(position)
      ? `fit-content(${fitCaps.get(position)})`
      : remaining.shift());
  }
  const html = source
    .replace('grid-auto-rows: fit-content(10px) min-content fit-content(20%) max-content auto min-content;',
      `grid-auto-rows: ${rows.join(' ')};`)
    .replace('align-content: start;', `align-content: ${align};`)
    .replace('<title>C11.4 two-fit repeated text auto-placement RuntimeSession</title>',
      `<title>C11.4 ${name}</title>`);
  if (html === source || !html.includes(`grid-auto-rows: ${rows.join(' ')};`)) {
    throw new Error(`Failed to produce isolated scene: ${name}`);
  }
  await writeFile(resolve(scenesDirectory, `${name}.html`), html);
}

console.log(`created=${cases.length} output=${scenesDirectory}`);
