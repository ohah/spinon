import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const pass = process.argv[2] ?? 'pass01';
assert.ok(['pass01', 'pass02'].includes(pass), 'usage: node prepare-selected-assets.mjs pass01|pass02');
const suffix = pass === 'pass01' ? '' : '-pass02';
const variant = process.argv[3] ?? '';
assert.ok(['', '-final'].includes(variant), 'usage: node prepare-selected-assets.mjs pass01|pass02 [-final]');
const selectedRoot = path.join(repositoryRoot, `build/spinon/c11-4-percentage-neighbors-roadmap-sync-20261024-root${suffix}-selected`);
const outputRoot = path.join(repositoryRoot, `build/spinon/c11-4-percentage-neighbors-roadmap-sync-20261024-site${suffix}-selected`);
const manifestPath = path.join(evidenceDirectory, `selected-assets-manifest${suffix}${variant}.json`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(selectedRoot).isDirectory(), 'selected root is missing');
assert.ok(fs.lstatSync(outputRoot).isDirectory(), 'selected build output is missing');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite asset manifest');

const staticAssets = [
  'roadmap.html',
  'internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/android-avd-c114-percentage-neighbors-android-avd-20261023T203439598Z-425fea5b/col-fill-both.png',
  'internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/ios-simulator-c114-percentage-neighbors-ios-simulator-20261023T203501591Z-007b4f1b/col-fill-both.png',
  'internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/tailnet-roadmap-before-sync.png',
];
const records = [];
for (const relative of staticAssets) {
  const source = relative === 'roadmap.html'
    ? path.join(repositoryRoot, 'spec/public/roadmap.html')
    : path.join(repositoryRoot, 'spec/internal', relative.slice('internal/'.length));
  const publicPath = path.join(selectedRoot, 'public', relative);
  const outputPath = path.join(outputRoot, relative);
  const bytes = fs.readFileSync(source);
  for (const destination of [publicPath, outputPath]) {
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    if (fs.existsSync(destination)) {
      const stat = fs.lstatSync(destination);
      assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe existing asset: ${destination}`);
      assert.equal(sha256(fs.readFileSync(destination)), sha256(bytes), `asset collision: ${destination}`);
    } else if (destination === publicPath) {
      fs.linkSync(source, destination);
    } else {
      fs.linkSync(publicPath, destination);
    }
  }
  records.push({ path: relative, bytes: bytes.length, sha256: sha256(bytes), selectedRootPublic: true, selectedOutput: true });
}

const pageRoutes = [
  'roadmap.html',
  'STATUS.html',
  'plan/c11-4-auto-repeat-percentage-neighbors.html',
  'internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/README.html',
  'internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/README.html',
  'internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/pr-diff-review-30.html',
];
const pageHashes = [];
for (const route of pageRoutes) {
  const file = path.join(outputRoot, route);
  const stat = fs.lstatSync(file);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing built page: ${route}`);
  pageHashes.push({ path: route, bytes: stat.size, sha256: sha256(fs.readFileSync(file)) });
}

const roadmap = fs.readFileSync(path.join(outputRoot, 'roadmap.html'), 'utf8');
for (const value of ['72/213', 'PR #580', '33b00d077147ad9f2102af3220daa2c6cf74e2f9', '13/13 scene', 'C13은 미완료지만 현재 활성 작업은 아니며 진행 중 lane은 C11.4다']) {
  assert.ok(roadmap.includes(value), `roadmap is missing current state: ${value}`);
}
const neighborsCard = roadmap.match(/C11\.4 definite percentage auto-repeat neighbors[\s\S]*?<\/p>/)?.[0];
assert.ok(neighborsCard, 'C11.4 percentage-neighbors card is missing');
for (const stale of ['candidate branch는 아직 병합 전', 'runtime 미검증', '2/2 layout']) {
  assert.ok(!neighborsCard.includes(stale), `stale roadmap claim remains: ${stale}`);
}
const status = fs.readFileSync(path.join(outputRoot, 'STATUS.html'), 'utf8');
assert.ok(status.includes('72/213'), 'official status does not show 72/213');
assert.ok(status.includes('pull/580') && status.includes('33b00d077147ad9f2102af3220daa2c6cf74e2f9'), 'official status does not reference merged PR #580');
const plan = fs.readFileSync(path.join(outputRoot, 'plan/c11-4-auto-repeat-percentage-neighbors.html'), 'utf8');
assert.ok(plan.includes('C11.4 상위 체크는 미완료'), 'plan incorrectly promotes C11.4 completion');

const referencedStatic = new Set();
for (const route of pageRoutes.filter((item) => item.endsWith('.html') && item !== 'roadmap.html')) {
  const html = fs.readFileSync(path.join(outputRoot, route), 'utf8');
  for (const [, reference] of html.matchAll(/(?:src|href)="([^"]+)"/g)) {
    if (reference.startsWith('/spinon/static/')) referencedStatic.add(reference.slice('/spinon/'.length));
  }
}
const staticFiles = [];
for (const relative of [...referencedStatic].sort()) {
  const file = path.join(outputRoot, relative);
  const stat = fs.lstatSync(file);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing referenced static file: ${relative}`);
  staticFiles.push({ path: relative, bytes: stat.size, sha256: sha256(fs.readFileSync(file)) });
}

const report = {
  pass,
  variant: variant || null,
  rspressVersion: '2.0.22',
  selectedBuildExitCode: 0,
  pageRoutes: pageHashes,
  selectedStaticAssets: records,
  referencedStaticFiles: staticFiles,
  roadmapSourceSha256: sha256(fs.readFileSync(path.join(repositoryRoot, 'spec/public/roadmap.html'))),
  officialCountAndLaneVerified: true,
  staleCandidateClaimAbsent: true,
  noDeletes: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ pageRoutes: pageHashes.length, staticAssets: records.length, referencedStaticFiles: staticFiles.length, officialCountAndLaneVerified: true, staleCandidateClaimAbsent: true }, null, 2)}\n`);
