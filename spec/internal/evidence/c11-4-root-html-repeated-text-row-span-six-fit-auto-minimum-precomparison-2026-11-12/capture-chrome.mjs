import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, join } from 'node:path';
import { gzipSync } from 'node:zlib';

const session = 'spinon-c114-six-fit-row-span-precompare-20261112';
const here = dirname(fileURLToPath(import.meta.url));
const fixture = join(here, '../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-six-fit-auto-minimum-precomparison.html');
const output = join(here, 'chrome157-observation.ndjson.gz');
const screenshot = join(here, 'chrome157-six-fit-row-span-sample.png');
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
  const count = Number(run(['eval', 'window.__c114SixFitSpanScenes.length']));
  if (count !== 11520) throw new Error(`expected 11520 scenes, got ${count}`);
  for (let start = 0; start < count; start += batchSize) {
    const size = Math.min(batchSize, count - start);
    const batch = parse(run(['eval', `JSON.stringify(window.runC114SixFitSpanBatch(${start}, ${size}))`]));
    if (batch.start !== start || batch.end !== start + size || batch.total !== count) {
      throw new Error(`batch ${start} bounds mismatch`);
    }
    captured.push(...batch.results);
    if (captured.length % 1440 === 0 || captured.length === count) {
      process.stdout.write(`CAPTURED ${captured.length}/${count}\n`);
    }
  }
  if (captured.length !== count) throw new Error(`captured ${captured.length}`);
  if (!captured.every((record, index) => record.index === index)) throw new Error('non-contiguous scene order');
  const trackLists = new Set(captured.map(({ scene }) => scene.trackList));
  if (trackLists.size !== 720) throw new Error(`expected 720 track lists, got ${trackLists.size}`);
  const fixtureBytes = readFileSync(fixture);
  const fixtureSha256 = createHash('sha256').update(fixtureBytes).digest('hex');
  const metadata = {
    record: 'metadata', schema: 'spinon.c11-4.repeated-text-row-span-six-fit-auto-min-precomparison/v1',
    capturedAtUtc: new Date().toISOString(), browserCli: execFileSync('agent-browser', ['--version'], { encoding: 'utf8' }).trim(),
    userAgent: captured[0].userAgent, platform: captured[0].platform,
    devicePixelRatio: captured[0].devicePixelRatio, viewportCssPx: [1280, 900],
    fixturePath: 'tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-six-fit-auto-minimum-precomparison.html',
    fixtureBytes: fixtureBytes.length, fixtureSha256, sceneCount: count, uniqueTrackListCount: trackLists.size,
    fitPositions: [1, 2, 3, 4, 5, 6], orderedCapAssignments: 720, observationPath: 'chrome157-observation.ndjson.gz',
    capValues: ['0px', '10px', '20%', '30px', '40px', '50px'],
    alignments: ['start', 'stretch'], topologies: ['span2Six', 'span2Span3Six'],
    domOrders: ['forward', 'reverse'], minimumModes: ['0px', 'auto'],
  };
  writeFileSync(output, gzipSync(`${JSON.stringify(metadata)}\n${captured.map((record) => JSON.stringify({ record: 'scene', ...record })).join('\n')}\n`, { level: 9 }));
  const sampleIndex = captured.findIndex(({ scene }) => scene.fitCaps.join(',') === '0px,10px,20%,30px,40px,50px'
    && scene.alignContent === 'start' && scene.topology === 'span2Six-forward' && scene.minimum === 'auto');
  if (sampleIndex < 0) throw new Error('representative sample missing');
  run(['eval', `JSON.stringify(window.showC114SixFitSpanScene(${sampleIndex}))`]);
  run(['eval', "document.querySelector('#grid').style.transform = 'scale(2.5)'; document.querySelector('#grid').style.transformOrigin = 'top left';"]);
  run(['screenshot', screenshot]);
  process.stdout.write(`OBSERVATION ${output}\nSCREENSHOT ${screenshot}\nFIXTURE_SHA256 ${fixtureSha256}\n`);
} finally {
  if (opened) {
    try { run(['close']); closed = true; } catch (error) { process.stderr.write(`CLOSE_ERROR ${error.message}\n`); }
  }
  process.stdout.write(`BROWSER_CLOSED ${closed}\n`);
}
