import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-status-sync-pass01-site-selected');
const applicationSupportRoot = '/Users/yoonhb/Library/Application Support';
const targetRoot = path.join(applicationSupportRoot, 'SpinonPreview');
const pass = process.argv[3] ?? 'pass01';
assert.ok(['pass01', 'pass02'].includes(pass), 'pass must be pass01 or pass02');
const backupRoot = path.join(applicationSupportRoot, `SpinonPreviewBackups/c11-4-roadmap-status-sync-2026-10-24/${pass}`);
const origin = 'https://macstudio.tailed42f2.ts.net/spinon/';
const reportNames = {
  preflight: pass === 'pass01' ? 'preflight.json' : `preflight-${pass}.json`,
  delta: pass === 'pass01' ? 'delta-manifest.json' : `delta-manifest-${pass}.json`,
  copy: pass === 'pass01' ? 'copy-and-backup.json' : `copy-and-backup-${pass}.json`,
  verify: pass === 'pass01' ? 'remote-verification.json' : `remote-verification-${pass}.json`,
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const mode = process.argv[2];
assert.ok(['preflight', 'copy', 'verify'].includes(mode), 'usage: sync-tailnet.mjs preflight|copy|verify [pass01|pass02]');

const pageRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-auto-repeat-zero-percent-minimum.html',
  'plan/c11-4-auto-repeat-zero-percent-minimum-plan-review-20.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.html',
  'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/plan-pr-diff-review-20.html',
  'spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.html',
  'spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/plan-pr-diff-review-20.html',
];
const rawScreenshot = 'internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/chrome156-zero-percent-auto-repeat-agent-browser-pass02.png';
const routeSet = new Set([...pageRoutes, rawScreenshot]);
for (const relative of pageRoutes) {
  const html = fs.readFileSync(path.join(outputRoot, relative), 'utf8');
  for (const match of html.matchAll(/(?:src|href)="([^"]+)"/g)) {
    if (match[1].startsWith('/spinon/static/')) routeSet.add(match[1].slice('/spinon/'.length));
  }
}
const routes = [...routeSet].sort();

function fileStat(filePath, optional = false) {
  try {
    const stat = fs.lstatSync(filePath);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe file: ${filePath}`);
    return stat;
  } catch (error) {
    if (optional && error.code === 'ENOENT') return null;
    throw error;
  }
}

function assertSafePath(root, relative, allowMissingLeaf = false) {
  assert.ok(!path.isAbsolute(relative) && !relative.split(path.sep).includes('..'), `unsafe route: ${relative}`);
  const absolute = path.resolve(root, relative);
  assert.ok(absolute.startsWith(`${root}${path.sep}`), `route escapes root: ${relative}`);
  let current = root;
  const parts = relative.split(path.sep);
  for (const part of parts.slice(0, -1)) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${current}`);
    } catch (error) {
      if (error.code === 'ENOENT') break;
      throw error;
    }
  }
  return fileStat(absolute, allowMissingLeaf);
}

function readReport(kind) {
  return JSON.parse(fs.readFileSync(path.join(evidenceDirectory, reportNames[kind]), 'utf8'));
}

assert.ok(fs.lstatSync(targetRoot).isDirectory() && !fs.lstatSync(targetRoot).isSymbolicLink(), 'unsafe Tailnet preview root');
assert.ok(fs.lstatSync(outputRoot).isDirectory() && !fs.lstatSync(outputRoot).isSymbolicLink(), 'selected build output missing');

if (mode === 'preflight') {
  for (const name of Object.values(reportNames)) assert.ok(!fs.existsSync(path.join(evidenceDirectory, name)), `refusing to overwrite ${name}`);
  assert.ok(!fs.existsSync(backupRoot), 'refusing to reuse backup root');
  const delta = [];
  for (const relative of routes) {
    const source = path.join(outputRoot, relative);
    const sourceStat = fileStat(source);
    const sourceBytes = fs.readFileSync(source);
    const previous = assertSafePath(targetRoot, relative, true);
    if (!previous) {
      delta.push({ path: relative, operation: 'add', bytes: sourceStat.size, sha256: sha256(sourceBytes) });
      continue;
    }
    const previousBytes = fs.readFileSync(path.join(targetRoot, relative));
    if (previousBytes.length !== sourceBytes.length || sha256(previousBytes) !== sha256(sourceBytes)) {
      delta.push({ path: relative, operation: 'replace', bytes: sourceBytes.length, sha256: sha256(sourceBytes), previousBytes: previousBytes.length, previousSha256: sha256(previousBytes) });
    }
  }
  const report = {
    pass,
    selectedRoutes: routes,
    routesUnchanged: routes.length - delta.length,
    routesAdded: delta.filter((item) => item.operation === 'add').length,
    routesReplaced: delta.filter((item) => item.operation === 'replace').length,
    backupBytes: delta.reduce((sum, item) => sum + (item.previousBytes ?? 0), 0),
    noDeletes: true,
    previewRoot: 'SpinonPreview',
    backupRoot: `SpinonPreviewBackups/c11-4-roadmap-status-sync-2026-10-24/${pass}`,
  };
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.preflight), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.delta), `${JSON.stringify(delta, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
}

if (mode === 'copy') {
  const preflight = readReport('preflight');
  const delta = readReport('delta');
  assert.deepEqual(preflight.selectedRoutes, routes, 'selected route list changed after preflight');
  assert.ok(!fs.existsSync(backupRoot), 'refusing to reuse backup root');
  if (delta.length) fs.mkdirSync(backupRoot, { recursive: true });
  const outcomes = [];
  for (const item of delta) {
    const source = path.join(outputRoot, item.path);
    const sourceBytes = fs.readFileSync(source);
    assert.equal(sha256(sourceBytes), item.sha256, `source changed after preflight: ${item.path}`);
    const target = path.join(targetRoot, item.path);
    const previous = assertSafePath(targetRoot, item.path, true);
    if (item.operation === 'add') assert.equal(previous, null, `new route appeared after preflight: ${item.path}`);
    else {
      assert.ok(previous, `replacement target disappeared: ${item.path}`);
      const previousBytes = fs.readFileSync(target);
      assert.equal(sha256(previousBytes), item.previousSha256, `target changed after preflight: ${item.path}`);
      const backup = path.join(backupRoot, item.path);
      fs.mkdirSync(path.dirname(backup), { recursive: true });
      fs.writeFileSync(backup, previousBytes, { flag: 'wx' });
      assert.equal(sha256(fs.readFileSync(backup)), item.previousSha256, `backup verification failed: ${item.path}`);
    }
    fs.mkdirSync(path.dirname(target), { recursive: true });
    const temporary = path.join(path.dirname(target), `.${path.basename(target)}.${randomUUID()}.tmp`);
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
    fs.renameSync(temporary, target);
    assert.equal(sha256(fs.readFileSync(target)), item.sha256, `copy verification failed: ${item.path}`);
    outcomes.push({ path: item.path, operation: item.operation, bytes: sourceBytes.length, sha256: item.sha256, previousSha256: item.previousSha256 ?? null });
  }
  const report = {
    pass,
    added: outcomes.filter((item) => item.operation === 'add').length,
    replaced: outcomes.filter((item) => item.operation === 'replace').length,
    unchanged: routes.length - outcomes.length,
    backedUp: outcomes.filter((item) => item.operation === 'replace').length,
    files: outcomes,
    backupRoot: `SpinonPreviewBackups/c11-4-roadmap-status-sync-2026-10-24/${pass}`,
    noDeletes: true,
  };
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.copy), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ added: report.added, replaced: report.replaced, unchanged: report.unchanged, backedUp: report.backedUp, noDeletes: true }, null, 2)}\n`);
}

if (mode === 'verify') {
  const delta = readReport('delta');
  const checks = [];
  for (const relative of routes) {
    const sourceBytes = fs.readFileSync(path.join(outputRoot, relative));
    const served = fs.readFileSync(path.join(targetRoot, relative));
    assert.equal(sha256(served), sha256(sourceBytes), `local preview mismatch: ${relative}`);
    const responses = [];
    for (const base of ['http://127.0.0.1:8768/spinon/', origin]) {
      const response = await fetch(new URL(relative, base));
      const bytes = Buffer.from(await response.arrayBuffer());
      assert.equal(response.status, 200, `HTTP ${response.status}: ${new URL(relative, base)}`);
      assert.equal(sha256(bytes), sha256(sourceBytes), `served hash mismatch: ${relative} from ${base}`);
      responses.push({ url: new URL(relative, base).toString(), status: response.status, bytes: bytes.length, sha256: sha256(bytes) });
    }
    checks.push({ path: relative, bytes: served.length, sha256: sha256(served), responses });
  }
  const report = {
    pass,
    checkedRoutes: checks.length,
    addReplaceDeltaCount: delta.length,
    sha256EqualAcrossBuildLocalAndHttps: true,
    allHttp200: true,
    routes: checks,
  };
  fs.writeFileSync(path.join(evidenceDirectory, reportNames.verify), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
  process.stdout.write(`${JSON.stringify({ checkedRoutes: report.checkedRoutes, sha256EqualAcrossBuildLocalAndHttps: true, allHttp200: true }, null, 2)}\n`);
}
