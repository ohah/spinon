import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1060-2026-11-02/pass03');
const root = path.join(base, 'root');
const manifestPath = path.join(evidence, 'selected-root-pass03.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const expectedHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repo, encoding: 'utf8' }).trim();
const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim();
assert.equal(sourceHead, expectedHead, 'selected root은 최신 origin/main에서 준비해야 함');
assert.equal(execFileSync('git', ['status', '--porcelain=v1', '--untracked-files=no'], { cwd: repo, encoding: 'utf8' }).trim(), '', 'tracked source 변경이 없어야 함');
assert.ok(!fs.existsSync(base), `기존 build 경로를 보존함: ${base}`);
assert.ok(!fs.existsSync(manifestPath), `기존 manifest를 보존함: ${manifestPath}`);
fs.mkdirSync(base, { recursive: true });
fs.mkdirSync(root, { recursive: false });

const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 route: ${route}`);
}
function assertNoSymlink(relative) {
  let current = repo;
  for (const part of relative.split('/')) {
    current = path.join(current, part);
    assert.ok(!fs.lstatSync(current).isSymbolicLink(), `source symlink 거부: ${relative}`);
  }
}
function add(sourceRelative, route) {
  safeRoute(route);
  assertNoSymlink(sourceRelative);
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile(), `source 일반 파일이 아님: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `route source 충돌: ${route}`);
    return;
  }
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `selected root hash 불일치: ${route}`);
  selected.set(route, { route, source: sourceRelative, bytes: bytes.length, sha256: digest });
}

add('spec/STATUS.md', 'STATUS.md');
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg']) add(`spec/public/${name}`, `public/${name}`);
for (const file of [
  'plan/c11-root-html-grid-column-flow.md',
  'plan/c11-root-html-grid-column-flow-implicit-columns.md',
  'plan/c11-root-html-grid-column-flow-plan-review-20.md',
  'plan/c11-root-html-grid-column-flow-plan-pr-diff-review-30.md',
  'plan/c11-root-html-grid-column-flow-implicit-columns-review-20.md',
  'plan/c11-root-html-grid-column-flow-implicit-columns-plan-pr-diff-review-30.md',
  'spec/internal/0059-c11-grid-layout.md',
]) add(file, file.startsWith('spec/') ? file.slice('spec/'.length) : file);

const prefixes = [
  'spec/internal/evidence/c11-root-html-grid-column-flow-precomparison-2026-11-02',
  'spec/internal/evidence/c11-root-html-grid-column-flow-runtime-2026-11-02',
  'spec/internal/evidence/roadmap-c11-column-flow-plan-sync-2026-11-02',
  'spec/internal/evidence/c11-root-html-grid-column-flow-implicit-columns-precomparison-2026-11-02',
  'spec/internal/evidence/c11-root-html-grid-column-flow-implicit-columns-runtime-2026-11-02',
];
for (const prefix of prefixes) {
  const files = execFileSync('git', ['ls-files', '-z', '--', prefix], { cwd: repo }).toString('utf8').split('\0').filter(Boolean);
  assert.ok(files.length > 0, `tracked evidence가 비어 있음: ${prefix}`);
  for (const source of files) {
    const extension = path.extname(source).toLowerCase();
    if (extension === '.md' || ['.png', '.jpg', '.jpeg', '.svg', '.webp'].includes(extension)) {
      add(source, source.slice('spec/'.length));
    }
  }
}

const rows = [...selected.values()].sort((a, b) => a.route.localeCompare(b.route));
const manifest = {
  sourceHead,
  sourceTree: execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repo, encoding: 'utf8' }).trim(),
  root: path.relative(repo, root).split(path.sep).join('/'),
  selectedFileCount: rows.length,
  totalBytes: rows.reduce((sum, item) => sum + item.bytes, 0),
  rows,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, rows: undefined }, null, 2)}\n`);
