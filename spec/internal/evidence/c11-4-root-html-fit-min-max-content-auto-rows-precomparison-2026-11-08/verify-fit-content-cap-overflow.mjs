import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const directory = path.dirname(fileURLToPath(import.meta.url));
const observationPath = path.join(directory, 'chrome-fit-content-cap-overflow-observation.json');
const fixturePath = path.resolve(
  directory,
  '../../../../tests/fixtures/css/c11/root-html-grid-fit-min-max-content-auto-implicit-row-list-overflow-precomparison.html',
);
const screenshotPath = path.join(directory, 'chrome-fit-content-cap-overflow.png');
const outputPath = path.join(directory, 'chrome-fit-content-cap-overflow-verification.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const observation = JSON.parse(fs.readFileSync(observationPath, 'utf8'));
const fixtureSha256 = sha256(fs.readFileSync(fixturePath));
const screenshotSha256 = sha256(fs.readFileSync(screenshotPath));

assert.match(observation.userAgent, /HeadlessChrome\/157\./);
assert.deepEqual(observation.grid, [0, 0, 40, 100]);
assert.equal(observation.gridAutoRows, 'fit-content(20px) min-content max-content auto');
assert.equal(observation.resolvedRows, '10px 20px 10px 12px 6px 20px');
assert.deepEqual(
  observation.items.find((item) => item.id === 'item-5')?.rect,
  [0, 58, 40, 20],
);
assert.deepEqual(
  observation.payloads.find((payload) => payload.id === 'payload-5')?.rect,
  [0, 58, 40, 60],
);
assert.equal(observation.lastPayloadOutsideGrid, true);
assert.equal(observation.overflowVisible, true);
assert.equal(observation.pageScrollHeight, 633);
assert.equal(fixtureSha256, 'e9fcf67de7106f1c8cc9a8c6ffc6213f7e59663ab216c6f8a4bb4f315959289d');

const report = {
  oracle: 'Agent Browser HeadlessChrome 157.0.0.0',
  fixture: path.relative(path.resolve(directory, '../../../..'), fixturePath).split(path.sep).join('/'),
  fixtureSha256,
  screenshot: path.basename(screenshotPath),
  screenshotSha256,
  gridRect: observation.grid,
  resolvedRows: observation.resolvedRows,
  fifthFitContentItemRect: observation.items.find((item) => item.id === 'item-5').rect,
  fifthPayloadRect: observation.payloads.find((payload) => payload.id === 'payload-5').rect,
  gridBottom: observation.grid[1] + observation.grid[3],
  payloadBottom: observation.payloads.find((payload) => payload.id === 'payload-5').rect[1]
    + observation.payloads.find((payload) => payload.id === 'payload-5').rect[3],
  visibleSpillCssPx: observation.payloads.find((payload) => payload.id === 'payload-5').rect[1]
    + observation.payloads.find((payload) => payload.id === 'payload-5').rect[3]
    - (observation.grid[1] + observation.grid[3]),
  assertions: 'PASS',
};
assert.equal(report.visibleSpillCssPx, 18);
fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
