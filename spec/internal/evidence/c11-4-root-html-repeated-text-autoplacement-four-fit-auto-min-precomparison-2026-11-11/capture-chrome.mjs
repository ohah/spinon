import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, join } from 'node:path';

const session = 'spinon-c114-four-fit-autoplacement-precomparison-20261111';
const here = dirname(fileURLToPath(import.meta.url));
const fixture = join(here, 'oracle-fixture.html');
const output = join(here, 'chrome157-observation.ndjson');
const screenshot = join(here, 'chrome157-four-fit-auto-min-sample.png');
const batchSize = 240;
const run = (args) => execFileSync('agent-browser', ['--session', session, ...args], {
  encoding: 'utf8', maxBuffer: 64 * 1024 * 1024,
}).trim();
const parse = (value) => {
  const decoded = JSON.parse(value);
  return typeof decoded === 'string' ? JSON.parse(decoded) : decoded;
};
let opened = false;
let closed = false;
let captured = [];
try {
  run(['open', pathToFileURL(fixture).href]);
  opened = true;
  run(['set', 'viewport', '1280', '900']);
  const count = Number(run(['eval', 'window.__c114FourFitScenes.length']));
  if (count !== 5760) throw new Error(`expected 5760 scenes, got ${count}`);
  for (let start = 0; start < count; start += batchSize) {
    const size = Math.min(batchSize, count - start);
    const batch = parse(run(['eval', `JSON.stringify(window.runC114FourFitBatch(${start}, ${size}))`]));
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
    record: 'metadata', schema: 'spinon.c11-4.repeated-text-autoplacement-four-fit-auto-min-precomparison/v1',
    capturedAtUtc: new Date().toISOString(), browserCli: 'agent-browser 0.31.1',
    userAgent: captured[0].userAgent, platform: captured[0].platform,
    devicePixelRatio: captured[0].devicePixelRatio, viewportCssPx: [1280, 900],
    fixturePath: 'oracle-fixture.html', fixtureSha256, sceneCount: count, uniqueTrackListCount: trackLists.size,
    fitPositionQuadruples: 15, tailOrders: 2, orderedCapAssignments: 24,
    alignments: ['start', 'stretch'], domOrders: ['forward', 'reverse'], minimumModes: ['0px', 'auto'],
  };
  writeFileSync(output, `${JSON.stringify(metadata)}\n${captured.map((record) => JSON.stringify({ record: 'scene', ...record })).join('\n')}\n`);
  const sampleIndex = captured.findIndex(({ scene }) => scene.fitPositions.join(',') === '1,3,5,6'
    && scene.fitCaps.join(',') === '0px,10px,20%,30px' && scene.alignContent === 'start'
    && scene.domOrder === 'forward' && scene.minHeight === 'auto');
  if (sampleIndex < 0) throw new Error('representative sample missing');
  run(['eval', `JSON.stringify(window.showC114FourFitScene(${sampleIndex}))`]);
  run(['screenshot', screenshot]);
  process.stdout.write(`OBSERVATION ${output}\nSCREENSHOT ${screenshot}\nFIXTURE_SHA256 ${fixtureSha256}\n`);
} finally {
  if (opened) {
    try { run(['close']); closed = true; } catch (error) { process.stderr.write(`CLOSE_ERROR ${error.message}\n`); }
  }
  process.stdout.write(`BROWSER_CLOSED ${closed}\n`);
}
