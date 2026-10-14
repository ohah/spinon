import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, isAbsolute, join, resolve } from 'node:path';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryPath = join(repositoryRoot, 'tests/fixtures/css/c11/grid-template-areas-c1133-inventory.json');
const argumentsByName = new Map(process.argv.slice(2).map((argument) => {
  const separator = argument.indexOf('=');
  assert.ok(separator > 2 && argument.startsWith('--'), `잘못된 인수입니다: ${argument}`);
  return [argument.slice(2, separator), argument.slice(separator + 1)];
}));
const wptRoot = argumentsByName.get('wpt-root');
const reportPath = argumentsByName.get('report');
const outputPath = argumentsByName.get('output');
assert.ok(wptRoot && isAbsolute(wptRoot), '--wpt-root에는 고정 WPT checkout 절대 경로가 필요합니다.');
assert.ok(reportPath && isAbsolute(reportPath), '--report에는 WPT report JSON 절대 경로가 필요합니다.');
assert.ok(outputPath && isAbsolute(outputPath), '--output에는 결과 JSON 절대 경로가 필요합니다.');

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const inventoryBytes = await readFile(inventoryPath);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const reportBytes = await readFile(reportPath);
const report = JSON.parse(reportBytes.toString('utf8'));
assert.equal(report.run_info.revision, inventory.wpt.revision);
assert.equal(report.run_info.browser_version, '154.0.8037.92');
assert.equal(report.run_info.product, 'chrome');

const candidateByPath = new Map(inventory.wpt.candidates.map((candidate) => [candidate.path, candidate]));
const reportsByPath = new Map(report.results.map((result) => [result.test.replace(/^\//, ''), result]));
assert.deepEqual([...reportsByPath.keys()].sort(), [...candidateByPath.keys()].sort(),
  '실행된 WPT 후보 집합이 고정 inventory와 다릅니다.');

const tests = [];
for (const candidate of inventory.wpt.candidates) {
  const sourcePath = join(wptRoot, candidate.path);
  const sourceBytes = await readFile(sourcePath);
  assert.equal(sha256(sourceBytes), candidate.sha256, `${candidate.path}: 고정 source SHA-256 불일치`);
  const result = reportsByPath.get(candidate.path);
  assert.ok(['PASS', 'OK'].includes(result.status), `${candidate.path}: WPT unexpected status ${result.status}`);
  assert.ok((result.subtests ?? []).every(({ status }) => status === 'PASS'),
    `${candidate.path}: 실패하거나 skip된 하위 검사가 있습니다.`);
  const unsupported = candidate.classification.startsWith('Currently unsupported:');
  const subtests = (result.subtests ?? []).map((subtest, index) => ({
    index: index + 1,
    name: subtest.name,
    status: subtest.status,
    classification: unsupported ? 'currently-unsupported' : 'spinon-fixture-candidate',
    reason: candidate.subtestClassification,
  }));
  tests.push({
    path: candidate.path,
    sourceSha256: candidate.sha256,
    status: result.status,
    classification: candidate.classification,
    subtests,
  });
}

const classified = {
  schema: 'spinon-css-c11-3-3-wpt-chrome-classification/v1',
  classifier: {
    path: 'tools/css-reference/classify-c11-3-3-grid-template-areas-wpt-report.mjs',
    sha256: sha256(await readFile(fileURLToPath(import.meta.url))),
  },
  inventorySha256: sha256(inventoryBytes),
  reportEvidence: inventory.wpt.chromeReportEvidence,
  reportSha256: sha256(reportBytes),
  runner: {
    revision: report.run_info.revision,
    product: report.run_info.product,
    browserVersion: report.run_info.browser_version,
    browserChannel: report.run_info.browser_channel,
    headlessReportedByRunner: report.run_info.headless,
    os: report.run_info.os,
    osVersion: report.run_info.version,
    processor: report.run_info.processor,
  },
  outcome: {
    tests: tests.length,
    testSubtests: tests.reduce((sum, test) => sum + test.subtests.length, 0),
    skipped: 0,
    failures: 0,
    meaning: '고정 Chrome에서 실행한 WPT oracle 결과이며 Spinon runner 또는 Spinon WPT 적합성 결과가 아닙니다.',
  },
  tests,
};
await mkdir(dirname(outputPath), { recursive: true });
await writeFile(outputPath, `${JSON.stringify(classified, null, 2)}\n`, { flag: 'wx' });
console.log(`WPT subtest classification을 저장했습니다: ${outputPath}`);
console.log(`Tests: ${classified.outcome.tests}; subtests: ${classified.outcome.testSubtests}; skipped: 0; failures: 0`);
