import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const rootManifestPath = path.join(evidenceDir, 'selected-root.json');
const rootManifest = JSON.parse(fs.readFileSync(rootManifestPath, 'utf8'));
const selectedRoot = path.join(repoRoot, rootManifest.selectedRoot);
const outputRoot = path.join(repoRoot, 'build/spinon/c11-4-direct-text-minmax-roadmap-site-20261030');
const reportPath = path.join(evidenceDir, 'selected-build-verification.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(fs.lstatSync(selectedRoot).isDirectory(), 'selected root missing');
assert.ok(fs.lstatSync(outputRoot).isDirectory(), 'RSPress output missing');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite build report');

const copiedStatic = [];
for (const entry of rootManifest.entries) {
  if (entry.path.endsWith('.md')) continue;
  const sourcePath = path.join(selectedRoot, entry.path);
  const sourceStat = fs.lstatSync(sourcePath);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), 'unsafe selected asset: ' + entry.path);
  const source = fs.readFileSync(sourcePath);
  assert.equal(source.byteLength, entry.bytes, 'selected asset size changed: ' + entry.path);
  assert.equal(sha256(source), entry.sha256, 'selected asset hash changed: ' + entry.path);
  const destination = path.join(outputRoot, entry.path);
  assert.ok(destination.startsWith(outputRoot + path.sep), 'asset route escaped output: ' + entry.path);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  if (fs.existsSync(destination)) {
    const existing = fs.readFileSync(destination);
    assert.equal(sha256(existing), entry.sha256, 'RSPress asset differs from selected source: ' + entry.path);
  } else {
    fs.writeFileSync(destination, source, { flag: 'wx' });
  }
  copiedStatic.push({ path: entry.path, bytes: source.byteLength, sha256: sha256(source) });
}

const expectedRoutes = [
  'roadmap.html',
  'roadmap.svg',
  'icon.svg',
  'STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-4-direct-text-aspect-ratio-min-max.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/README.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/README.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/minmax-implementation-runtime-review-20.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/pr-diff-review-30.html',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/android-avd/c114-text-ratio-mvut1ldd-e32f3c4c/grid-text-aspect-ratio-minmax-min-width-45.png',
  'internal/evidence/c11-4-direct-text-aspect-ratio-runtime-2026-10-30/ios-simulator/c114-text-ratio-mvut4y8z-74dc1d74/grid-text-aspect-ratio-minmax-max-height-17-75-min-controls.png',
  'internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/expected-values.json',
];
for (const route of expectedRoutes) {
  const fullPath = path.join(outputRoot, route);
  assert.ok(fs.lstatSync(fullPath).isFile(), 'required rendered route missing: ' + route);
}

const roadmap = fs.readFileSync(path.join(outputRoot, 'roadmap.html'), 'utf8');
for (const marker of ['#850', '4959332936a6ad599e1355d18161b90a9fea05bf', '72/213', '42/42', '129/129']) {
  assert.ok(roadmap.includes(marker), 'roadmap marker missing: ' + marker);
}
const status = fs.readFileSync(path.join(outputRoot, 'STATUS.html'), 'utf8');
for (const marker of ['#850', '4959332936a6ad599e1355d18161b90a9fea05bf', '72/213']) {
  assert.ok(status.includes(marker), 'rendered status marker missing: ' + marker);
}

function walk(directory, prefix = '') {
  const result = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (relative === '404.html') continue;
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), 'symlink in output: ' + relative);
    if (entry.isDirectory()) result.push(...walk(fullPath, relative));
    else if (entry.isFile()) {
      const bytes = fs.readFileSync(fullPath);
      result.push({ path: relative, bytes: bytes.byteLength, sha256: sha256(bytes) });
    } else throw new Error('unsupported build output entry: ' + relative);
  }
  return result;
}

const files = walk(outputRoot);
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  selectedRoot: rootManifest.selectedRoot,
  selectedOutputRoot: path.relative(repoRoot, outputRoot),
  fileCount: files.length,
  totalBytes: files.reduce((sum, entry) => sum + entry.bytes, 0),
  copiedStaticCount: copiedStatic.length,
  expectedRoutes,
  excludedExistingPreviewRoute: '404.html',
  noDeletes: true,
  copiedStatic,
  files,
};
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...report, files: undefined, copiedStatic: undefined }, null, 2) + '\n');
