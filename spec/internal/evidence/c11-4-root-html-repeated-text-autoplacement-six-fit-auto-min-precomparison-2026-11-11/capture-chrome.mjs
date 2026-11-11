import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, join } from 'node:path';

const session = 'spinon-c114-six-fit-autoplacement-precomparison-20261111';
const here = dirname(fileURLToPath(import.meta.url));
const fixture = join(here, 'oracle-fixture.html');
const output = join(here, 'chrome157-observation.ndjson');
const screenshot = join(here, 'chrome157-six-fit-auto-min-sample.png');
const batchSize = 120;
const run = (args) => execFileSync('agent-browser', ['--session', session, ...args], {
  encoding: 'utf8', maxBuffer: 64 * 1024 * 1024,
}).trim();
const parse = (value) => {
  const decoded = JSON.parse(value);
  return typeof decoded === 'string' ? JSON.parse(decoded) : decoded;
};
let opened = false;
let closed = false;
const captured = [];
try {
  opened = true;
  run(['open', pathToFileURL(fixture).href]);
  run(['set', 'viewport', '1280', '900']);
  const count = Number(run(['eval', 'window.__c114SixFitScenes.length']));
  if (count !== 5760) throw new Error(`expected 5760 scenes, got ${count}`);
  for (let start = 0; start < count; start += batchSize) {
    const size = Math.min(batchSize, count - start);
    const batch = parse(run(['eval', `JSON.stringify(window.runC114SixFitBatch(${start}, ${size}))`]));
    if (batch.start !== start || batch.end !== start + size || batch.total !== count) {
      throw new Error(`batch ${start} bounds mismatch`);
    }
    captured.push(...batch.results);
    process.stdout.write(`CAPTURED ${captured.length}/${count}\n`);
  }
  if (captured.length !== count) throw new Error(`captured ${captured.length}`);
  if (!captured.every((record, index) => record.index === index)) throw new Error('non-contiguous scene order');
  const trackLists = new Set(captured.map(({ scene }) => scene.trackList));
  if (trackLists.size !== 720) throw new Error(`expected 720 track lists, got ${trackLists.size}`);
  const fixtureSha256 = createHash('sha256').update(readFileSync(fixture)).digest('hex');
  const metadata = {
    record: 'metadata', schema: 'spinon.c11-4.repeated-text-autoplacement-six-fit-auto-min-precomparison/v1',
    capturedAtUtc: new Date().toISOString(), browserCli: execFileSync('agent-browser', ['--version'], { encoding: 'utf8' }).trim(),
    userAgent: captured[0].userAgent, platform: captured[0].platform,
    devicePixelRatio: captured[0].devicePixelRatio, viewportCssPx: [1280, 900],
    fixturePath: 'oracle-fixture.html', fixtureSha256, sceneCount: count, uniqueTrackListCount: trackLists.size,
    fitPositions: [1, 2, 3, 4, 5, 6], orderedCapAssignments: 720,
    capValues: ['0px', '10px', '20%', '30px', '40px', '50px'],
    alignments: ['start', 'stretch'], domOrders: ['forward', 'reverse'], minimumModes: ['0px', 'auto'],
  };
  writeFileSync(output, `${JSON.stringify(metadata)}\n${captured.map((record) => JSON.stringify({ record: 'scene', ...record })).join('\n')}\n`);
  const sampleIndex = captured.findIndex(({ scene }) => scene.fitCaps.join(',') === '0px,10px,20%,30px,40px,50px'
    && scene.alignContent === 'start' && scene.domOrder === 'forward' && scene.minHeight === 'auto');
  if (sampleIndex < 0) throw new Error('representative sample missing');
  run(['eval', `JSON.stringify(window.showC114SixFitScene(${sampleIndex}))`]);
  run(['screenshot', screenshot]);
  process.stdout.write(`OBSERVATION ${output}\nSCREENSHOT ${screenshot}\nFIXTURE_SHA256 ${fixtureSha256}\n`);
} finally {
  if (opened) {
    try { run(['close']); closed = true; } catch (error) { process.stderr.write(`CLOSE_ERROR ${error.message}\n`); }
  }
  process.stdout.write(`BROWSER_CLOSED ${closed}\n`);
}
