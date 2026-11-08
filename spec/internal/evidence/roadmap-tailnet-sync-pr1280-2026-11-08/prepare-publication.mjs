import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { createHash } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const [pass = 'pass04'] = process.argv.slice(2);
assert.equal(pass, 'pass04', 'usage: node prepare-publication.mjs [pass04]');
const buildBase = path.join(repo, 'build/spinon/roadmap-pr1280-postmerge-docs-pass04-2026-11-08');
const root = path.join(buildBase, 'root');
const site = path.join(buildBase, 'site');
const previewRoot = path.join(os.homedir(), 'Library/Application Support/SpinonPreview');
const backupRoot = path.join(os.homedir(), `Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1280-2026-11-08/${pass}`);
const manifestPath = path.join(evidence, `selected-publication-manifest-${pass}.json`);
const reportPath = path.join(evidence, `publication-preflight-${pass}.json`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const replaceAllowlist = new Set([
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-precomparison-2026-11-08/README.html',
  'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-precomparison-2026-11-08/plan-pr-diff-review-30.html',
  'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08/README.html',
  'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08/pr-diff-review-30.html',
  'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/README.html',
  'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-pr-diff-review-30.html',
  'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-status-review-20.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-root-html-fit-content-min-content-auto-remaining-order-plan-review-20.html',
  'plan/c11-4-root-html-fit-content-min-content-auto-remaining-order.html',
  'plan/c11-4-root-html-fit-content-min-content-auto-track-list.html',
  'plan/c11-4-root-html-fit-min-max-content-auto-rows.html',
  'plan/c11-4-root-html-fit-min-max-content-auto-rows-plan-review-20.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/README.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/plan-pr-diff-review-30.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08/README.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08/pr-diff-review-30.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08/android/final/screenshot.png',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08/ios/final/screenshot.png',
  'STATUS.html',
  'spec/STATUS.html',
]);

for (const directory of [root, site, previewRoot]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}
assert.equal(fs.realpathSync(previewRoot), previewRoot, '미리보기 경로에 symlink가 없음');
for (const target of [manifestPath, reportPath, backupRoot]) assert.ok(!fs.existsSync(target), `기존 상태를 덮어쓰지 않음: ${target}`);

function listFiles(directory, prefix = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const absolute = path.join(directory, relative);
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${relative}`);
    if (entry.isDirectory()) files.push(...listFiles(directory, relative));
    else if (entry.isFile()) files.push(relative);
    else throw new Error(`지원하지 않는 파일: ${relative}`);
  }
  return files;
}

function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}

function inspectTarget(route) {
  safeRoute(route);
  let current = previewRoot;
  for (const [index, part] of route.split('/').entries()) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `target symlink 거부: ${route}`);
      if (index < route.split('/').length - 1) assert.ok(stat.isDirectory(), `target 상위 경로가 디렉터리가 아님: ${route}`);
      else {
        assert.ok(stat.isFile(), `target가 일반 파일이 아님: ${route}`);
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

function openPids(route) {
  const target = path.join(previewRoot, route);
  if (!fs.existsSync(target)) return [];
  const result = spawnSync('lsof', ['-nP', '-t', '--', target], { encoding: 'utf8' });
  assert.ok(result.status === 0 || result.status === 1, `lsof 실패: ${result.stderr}`);
  return result.stdout.trim().split(/\s+/).filter(Boolean).map(Number);
}

const selected = new Map();
function add(route, sourcePath, sourceRoot, kind) {
  safeRoute(route);
  const source = path.join(sourceRoot, route === 'roadmap.html' && kind === 'raw-roadmap' ? 'roadmap.html' : sourcePath);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `source 일반 파일 검사 실패: ${source}`);
  const bytes = fs.readFileSync(source);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `선택 route source 충돌: ${route}`);
    return;
  }
  selected.set(route, { route, source: path.relative(repo, source).split(path.sep).join('/'), sourceRoot, sourcePath, kind, bytes: bytes.length, sha256: digest });
}

for (const route of listFiles(site)) {
  if (route === 'index.html' || route === '404.html') continue;
  if (route.endsWith('.html') || route.startsWith('static/') || ['icon.svg', 'logo.svg', 'roadmap.svg'].includes(route)) {
    add(route, route, site, 'rspress-generated');
  }
}
for (const route of listFiles(root)) {
  const extension = path.extname(route).toLowerCase();
  if (extension === '.md' || ['.cjs', '.js', '.jsx', '.mjs', '.ts', '.tsx'].includes(extension)) continue;
  if (['public/roadmap.html', 'spec/public/roadmap.html', 'public/icon.svg', 'spec/public/icon.svg', 'public/logo.svg', 'spec/public/logo.svg', 'STATUS.md', 'spec/STATUS.md'].includes(route)) continue;
  if (route !== 'roadmap.html' && !['', '.html', '.svg', '.png', '.jpg', '.jpeg', '.webp', '.json', '.txt', '.log', '.sh', '.yaml', '.yml', '.sha256'].includes(extension)) continue;
  add(route, route, root, route === 'roadmap.html' ? 'raw-roadmap' : 'selected-static-source');
}

const entries = [];
for (const item of [...selected.values()].sort((a, b) => a.route.localeCompare(b.route))) {
  const current = inspectTarget(item.route);
  const state = !current.exists ? 'add' : current.sha256 === item.sha256 ? 'same' : 'replace';
  const handles = state === 'replace' ? openPids(item.route) : [];
  if (state === 'replace') {
    assert.ok(replaceAllowlist.has(item.route), `승인된 선택 교체 경로가 아님: ${item.route}`);
  }
  entries.push({ ...item, targetExists: current.exists, targetSha256: current.sha256 ?? null, state, openHandlePids: handles });
}

const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend가 Running이어야 함');
const listenerText = execFileSync('lsof', ['-nP', '-t', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' });
const listenerPids = [...new Set(listenerText.trim().split(/\s+/).filter(Boolean).map(Number))];
assert.equal(listenerPids.length, 1, `preview listener가 하나여야 함: ${listenerPids.join(',')}`);
const serverPid = listenerPids[0];
const serverArgs = execFileSync('ps', ['-p', String(serverPid), '-o', 'args='], { encoding: 'utf8' }).trim();
assert.ok(serverArgs.includes('tailnet-preview-server.mjs'), `예상한 Preview server가 아님: ${serverArgs}`);
const cwd = execFileSync('lsof', ['-a', '-p', String(serverPid), '-d', 'cwd', '-Fn'], { encoding: 'utf8' }).split('\n').find((line) => line.startsWith('n'))?.slice(1);
assert.equal(cwd, previewRoot, `Preview server root 불일치: ${cwd}`);

const manifest = {
  previewRoot,
  backupRoot,
  buildBase: path.relative(repo, buildBase).split(path.sep).join('/'),
  publicationFileCount: entries.length,
  publicationBytes: entries.reduce((sum, item) => sum + item.bytes, 0),
  replaceAllowlist: [...replaceAllowlist],
  noDeletes: true,
  pass,
  serverRestarted: false,
  entries,
};
const report = {
  tailscaleBackendState: tailscale.BackendState,
  previewServer: { pid: serverPid, args: serverArgs, cwd },
  fileCount: entries.length,
  bytes: manifest.publicationBytes,
  states: Object.fromEntries(['add', 'same', 'replace'].map((state) => [state, entries.filter((item) => item.state === state).length])),
  replaces: entries.filter((item) => item.state === 'replace').map(({ route, targetSha256, sha256: sourceSha256 }) => ({ route, targetSha256, sourceSha256 })),
  openHandlesUseReadOnlyCreateReadStream: true,
  replacementsWillUseSameDirectoryAtomicRenameAndKeepBackups: true,
  noDeletes: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
