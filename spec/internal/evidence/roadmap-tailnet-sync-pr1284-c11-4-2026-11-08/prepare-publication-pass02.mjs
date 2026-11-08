import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { createHash } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previousEvidence = path.join(repo, 'spec/internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08');
const previousPublication = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-publication-manifest-pass03.json'), 'utf8'));
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1284-c11-4-next-2026-11-08-pass02');
const root = path.join(buildBase, 'root');
const site = path.join(buildBase, 'site');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const backupRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/pass02');
const manifestPath = path.join(evidence, 'selected-publication-manifest-pass02.json');
const reportPath = path.join(evidence, 'publication-preflight-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const directory of [root, site, previewRoot]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe directory: ${directory}`);
}
assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root must not be a symlink');
for (const target of [manifestPath, reportPath, backupRoot]) assert.ok(!fs.existsSync(target), `preserve existing path: ${target}`);

function listFiles(directory, prefix = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const absolute = path.join(directory, relative);
    assert.ok(!entry.isSymbolicLink(), `symlink rejected: ${relative}`);
    if (entry.isDirectory()) files.push(...listFiles(directory, relative));
    else if (entry.isFile()) files.push(relative);
    else throw new Error(`unsupported entry: ${relative}`);
  }
  return files;
}

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `unsafe route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${route}`);
}

function inspectTarget(route) {
  safeRoute(route);
  let current = previewRoot;
  const parts = route.split('/');
  for (const [index, part] of parts.entries()) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `target symlink rejected: ${route}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `target parent is not a directory: ${route}`);
      else {
        assert.ok(stat.isFile(), `target is not a regular file: ${route}`);
        const bytes = fs.readFileSync(current);
        return { exists: true, bytes, sha256: sha256(bytes) };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

function openPids(target) {
  const result = spawnSync('lsof', ['-nP', '-t', '--', target], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, `lsof failed: ${result.stderr}`);
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}

const replaceAllowlist = new Set([
  ...previousPublication.entries.filter((entry) => entry.kind === 'rspress-generated').map((entry) => entry.route),
  'roadmap.html',
  'public/roadmap.html',
  'spec/public/roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'spec/internal/0059-c11-grid-layout.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-root-html-repeated-intrinsic-auto-rows.html',
  'plan/c11-4-root-html-repeated-intrinsic-tail-permutations.html',
  'plan/c11-4-root-html-repeated-intrinsic-tail-permutations-plan-review-20.html',
  'internal/evidence/c11-4-root-html-repeated-intrinsic-auto-rows-runtime-2026-11-08/README.html',
  'spec/internal/evidence/c11-4-root-html-repeated-intrinsic-auto-rows-runtime-2026-11-08/README.html',
  'internal/evidence/c11-4-root-html-repeated-intrinsic-tail-permutations-precomparison-2026-11-08/README.html',
  'spec/internal/evidence/c11-4-root-html-repeated-intrinsic-tail-permutations-precomparison-2026-11-08/README.html',
  'internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/README.html',
  'spec/internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/README.html',
  'internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/roadmap-status-review-20.html',
  'spec/internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/roadmap-status-review-20.html',
  'internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/roadmap-pr-diff-review-30.html',
  'spec/internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/roadmap-pr-diff-review-30.html',
]);

const selected = new Map();
function add(route, sourceRoot, kind, sourcePath = route) {
  safeRoute(route);
  const source = path.join(sourceRoot, sourcePath);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `source is not a regular file: ${source}`);
  const bytes = fs.readFileSync(source);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `selected route source collision: ${route}`);
    return;
  }
  selected.set(route, {
    route,
    source: path.relative(repo, source).split(path.sep).join('/'),
    sourcePath,
    kind,
    bytes: bytes.length,
    sha256: digest,
  });
}

for (const route of listFiles(site)) {
  if (route === 'index.html' || route === '404.html') continue;
  if (route.endsWith('.html') || route.startsWith('static/') || ['icon.svg', 'logo.svg', 'roadmap.svg'].includes(route)) {
    add(route, site, 'rspress-generated');
  }
}
for (const route of listFiles(root)) {
  const extension = path.extname(route).toLowerCase();
  if (extension === '.md' || ['.cjs', '.js', '.jsx', '.mjs', '.ts', '.tsx'].includes(extension)) continue;
  if (['public/roadmap.html', 'spec/public/roadmap.html', 'public/icon.svg', 'spec/public/icon.svg', 'public/logo.svg', 'spec/public/logo.svg', 'STATUS.md', 'spec/STATUS.md'].includes(route)) continue;
  if (route !== 'roadmap.html' && !['', '.html', '.svg', '.png', '.jpg', '.jpeg', '.webp', '.json', '.txt', '.log', '.sh', '.yaml', '.yml', '.sha256'].includes(extension)) continue;
  add(route, root, route === 'roadmap.html' ? 'raw-roadmap' : 'selected-static-source');
}

const entries = [];
for (const item of [...selected.values()].sort((a, b) => a.route.localeCompare(b.route))) {
  const current = inspectTarget(item.route);
  const state = !current.exists ? 'add' : current.sha256 === item.sha256 ? 'same' : 'replace';
  const handles = state === 'replace' ? openPids(path.join(previewRoot, item.route)) : [];
  if (state === 'replace') assert.ok(replaceAllowlist.has(item.route), `replacement not allowlisted: ${item.route}`);
  entries.push({ ...item, targetExists: current.exists, targetSha256: current.sha256 ?? null, state, openHandlePids: handles });
}

const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale must be Running');
const listenerText = execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' });
const listenerPids = [...new Set(listenerText.trim().split(/\s+/).filter(Boolean).map(Number))];
assert.equal(listenerPids.length, 1, `expected one preview listener: ${listenerPids.join(',')}`);
const serverPid = listenerPids[0];
const serverArgs = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
assert.ok(serverArgs.includes('tailnet-preview-server.mjs'), `unexpected preview server: ${serverArgs}`);
const cwd = execFileSync('lsof', ['-a', '-p', String(serverPid), '-d', 'cwd', '-Fn'], { encoding: 'utf8' })
  .split('\n').find((line) => line.startsWith('n'))?.slice(1);
assert.equal(cwd, previewRoot, `preview root mismatch: ${cwd}`);

const manifest = {
  previewRoot,
  backupRoot,
  buildBase: path.relative(repo, buildBase).split(path.sep).join('/'),
  publicationFileCount: entries.length,
  publicationBytes: entries.reduce((sum, item) => sum + item.bytes, 0),
  replaceAllowlist: [...replaceAllowlist].sort(),
  noDeletes: true,
  serverRestarted: false,
  entries,
};
const report = {
  tailscaleBackendState: tailscale.BackendState,
  previewServer: { pid: serverPid, args: serverArgs, cwd },
  fileCount: entries.length,
  bytes: manifest.publicationBytes,
  states: Object.fromEntries(['add', 'same', 'replace'].map((state) => [state, entries.filter((item) => item.state === state).length])),
  replaces: entries.filter((item) => item.state === 'replace').map(({ route, targetSha256, sha256: sourceSha256, openHandlePids }) => ({ route, targetSha256, sourceSha256, openHandlePids })),
  noDeletes: true,
  serverRestarted: false,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
