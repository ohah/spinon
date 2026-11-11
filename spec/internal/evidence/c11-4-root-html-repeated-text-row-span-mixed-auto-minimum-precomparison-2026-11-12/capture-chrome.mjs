import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { createWriteStream, existsSync, readFileSync, unlinkSync } from 'node:fs';
import { once } from 'node:events';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, join } from 'node:path';
import { createGzip } from 'node:zlib';

const session = 'spinon-c114-mixed-minimum-precompare-20261112';
const here = dirname(fileURLToPath(import.meta.url));
const fixture = join(here, '../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-mixed-auto-minimum-precomparison.html');
const output = join(here, 'chrome157-mixed-minimum-observation.ndjson.gz');
const screenshot = join(here, 'chrome157-mixed-minimum-sample.png');
const batchSize = 240;
const run = (args) => execFileSync('agent-browser', ['--session', session, ...args], {
  encoding: 'utf8', maxBuffer: 64 * 1024 * 1024,
}).trim();
const parse = (value) => {
  const decoded = JSON.parse(value);
  return typeof decoded === 'string' ? JSON.parse(decoded) : decoded;
};
const gzip = createGzip({ level: 9 });
const outputStream = createWriteStream(output);
gzip.pipe(outputStream);
const writeGzip = async (chunk) => {
  if (!gzip.write(chunk)) await once(gzip, 'drain');
};

let opened = false;
let closed = false;
let finished = false;
let capturedCount = 0;
let sampleIndex = -1;
const trackLists = new Set();

try {
  opened = true;
  run(['open', pathToFileURL(fixture).href]);
  run(['set', 'viewport', '1280', '900']);
  const count = Number(run(['eval', 'window.__c114SixFitMixedMinimumScenes.length']));
  if (count !== 138240) throw new Error(`expected 138240 scenes, got ${count}`);
  const browser = parse(run(['eval', 'JSON.stringify({ userAgent: navigator.userAgent, platform: navigator.platform, devicePixelRatio: window.devicePixelRatio })']));
  const fixtureBytes = readFileSync(fixture);
  const fixtureSha256 = createHash('sha256').update(fixtureBytes).digest('hex');
  const metadata = {
    record: 'metadata', schema: 'spinon.c11-4.repeated-text-row-span-six-fit-mixed-auto-min-precomparison/v1',
    capturedAtUtc: new Date().toISOString(), browserCli: execFileSync('agent-browser', ['--version'], { encoding: 'utf8' }).trim(),
    userAgent: browser.userAgent, platform: browser.platform,
    devicePixelRatio: browser.devicePixelRatio, viewportCssPx: [1280, 900],
    fixturePath: 'tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-mixed-auto-minimum-precomparison.html',
    fixtureBytes: fixtureBytes.length, fixtureSha256, sceneCount: count, uniqueTrackListCount: 720,
    fitPositions: [1, 2, 3, 4, 5, 6], orderedCapAssignments: 720,
    observationPath: 'chrome157-mixed-minimum-observation.ndjson.gz',
    capValues: ['0px', '10px', '20%', '30px', '40px', '50px'],
    alignments: ['start', 'stretch'], topologies: ['span2Six', 'span2Span3Six'],
    domOrders: ['forward', 'reverse'], minimumAssignments: { span2Six: 32, span2Span3Six: 16 },
    minimumMaskBitOrder: { span2Six: ['A', 'B', 'C', 'D', 'E'], span2Span3Six: ['A', 'B', 'C', 'D'] },
  };
  await writeGzip(`${JSON.stringify(metadata)}\n`);

  for (let start = 0; start < count; start += batchSize) {
    const size = Math.min(batchSize, count - start);
    const batch = parse(run(['eval', `JSON.stringify(window.runC114SixFitMixedMinimumBatch(${start}, ${size}))`]));
    if (batch.start !== start || batch.end !== start + size || batch.total !== count) {
      throw new Error(`batch ${start} bounds mismatch`);
    }
    for (const record of batch.results) {
      if (record.index !== capturedCount) throw new Error(`non-contiguous scene order at ${capturedCount}`);
      trackLists.add(record.scene.trackList);
      if (sampleIndex < 0 && record.scene.fitCaps.join(',') === '0px,10px,20%,30px,40px,50px'
        && record.scene.alignContent === 'start' && record.scene.topology === 'span2Six-forward'
        && record.scene.minimumMask === 5 && record.scene.autoItems.join(',') === 'A,C') {
        sampleIndex = record.index;
      }
      capturedCount += 1;
    }
    await writeGzip(`${batch.results.map((record) => JSON.stringify({ record: 'scene', ...record })).join('\n')}\n`);
    if (capturedCount % 11520 === 0 || capturedCount === count) {
      process.stdout.write(`CAPTURED ${capturedCount}/${count}\n`);
    }
  }

  if (capturedCount !== count) throw new Error(`captured ${capturedCount}`);
  if (trackLists.size !== 720) throw new Error(`expected 720 track lists, got ${trackLists.size}`);
  if (sampleIndex < 0) throw new Error('representative sample missing');
  gzip.end();
  await once(outputStream, 'finish');
  finished = true;

  run(['eval', `JSON.stringify(window.showC114SixFitMixedMinimumScene(${sampleIndex}))`]);
  run(['eval', "document.querySelector('#grid').style.transform = 'scale(2.5)'; document.querySelector('#grid').style.transformOrigin = 'top left';"]);
  run(['screenshot', screenshot]);
  process.stdout.write(`OBSERVATION ${output}\nSCREENSHOT ${screenshot}\nFIXTURE_SHA256 ${metadata.fixtureSha256}\n`);
} finally {
  if (opened) {
    try { run(['close']); closed = true; } catch (error) { process.stderr.write(`CLOSE_ERROR ${error.message}\n`); }
  }
  if (!finished) {
    gzip.destroy();
    outputStream.destroy();
    if (existsSync(output)) unlinkSync(output);
  }
  process.stdout.write(`BROWSER_CLOSED ${closed}\n`);
}
