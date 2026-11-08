import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previousRoot = path.join(repo, 'build/spinon/roadmap-c11-4-pr1277-status-docs-pass03-2026-11-08/root');
const [pass = 'pass02'] = process.argv.slice(2);
assert.ok(['pass02', 'pass03', 'pass04'].includes(pass), `지원하지 않는 pass: ${pass}`);
const buildBase = path.join(repo, `build/spinon/roadmap-pr1280-postmerge-docs-${pass}-2026-11-08`);
const root = path.join(buildBase, 'root');
const manifest = path.join(evidence, `staging-manifest-${pass}.json`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const staged = [];

for (const target of [buildBase, root, manifest]) {
  assert.ok(!fs.existsSync(target), `기존 산출물을 보존함: ${target}`);
}
for (const directory of [path.join(repo, 'build'), path.join(repo, 'build/spinon'), previousRoot]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}
fs.mkdirSync(root, { recursive: true });

function safeFile(file) {
  const stat = fs.lstatSync(file);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `일반 파일이 아님: ${file}`);
}

function copyTree(source, target, { currentEvidence = false } = {}) {
  const stat = fs.lstatSync(source);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 source tree: ${source}`);
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const from = path.join(source, entry.name);
    const to = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${from}`);
    if (entry.isDirectory()) {
      if (currentEvidence && entry.name === 'bootstrap-original') continue;
      copyTree(from, to, { currentEvidence });
    }
    else if (entry.isFile()) {
      if (currentEvidence && (/\.tar\.gz$/i.test(entry.name) || ['.cjs', '.js', '.jsx', '.mjs', '.ts', '.tsx'].includes(path.extname(entry.name).toLowerCase()))) continue;
      safeFile(from);
      fs.copyFileSync(from, to);
    } else throw new Error(`지원하지 않는 source 항목: ${from}`);
  }
}

copyTree(previousRoot, root);

function stageFile(sourceRelative, routes) {
  const source = path.join(repo, sourceRelative);
  safeFile(source);
  const bytes = fs.readFileSync(source);
  for (const route of routes) {
    assert.ok(!route.startsWith('/') && !route.split('/').includes('..'), `잘못된 route: ${route}`);
    const target = path.join(root, route);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.writeFileSync(target, bytes);
    assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `staging hash 불일치: ${route}`);
    staged.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
  }
}

stageFile('spec/public/roadmap.html', ['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html']);
stageFile('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
stageFile('plan/c11-4-root-html-fit-min-max-content-auto-rows.md', ['plan/c11-4-root-html-fit-min-max-content-auto-rows.md']);
stageFile('plan/c11-4-root-html-fit-min-max-content-auto-rows-plan-review-20.md', ['plan/c11-4-root-html-fit-min-max-content-auto-rows-plan-review-20.md']);
stageFile('plan/c11-4-grid-track-sizing.md', ['plan/c11-4-grid-track-sizing.md']);
stageFile('spec/internal/0059-c11-grid-layout.md', ['internal/0059-c11-grid-layout.md']);
for (const directory of [
  'c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08',
  'c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08',
]) {
  copyTree(path.join(repo, 'spec/internal/evidence', directory), path.join(root, 'internal/evidence', directory), { currentEvidence: true });
}

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['진행 중: C11.4 Grid implicit 행 목록', '2026-11-08 C11.4 parsed Root HTML', '#1280', 'b1b27b02491f1614ded260fd3b66f50184c5bb80', '78/214']) {
  assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
}
assert.ok(!roadmap.includes('PR diff 적대 검토 30개가 남아 있다.'), '이전 PR-review 상태가 남음');

const allFiles = [];
function verifyTree(directory, prefix = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const absolute = path.join(directory, entry.name);
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `staged symlink 거부: ${relative}`);
    if (entry.isDirectory()) verifyTree(absolute, relative);
    else if (entry.isFile()) {
      const bytes = fs.readFileSync(absolute);
      allFiles.push({ route: relative, bytes: bytes.length, sha256: sha256(bytes) });
    } else throw new Error(`지원하지 않는 staged 항목: ${relative}`);
  }
}
verifyTree(root);
const report = {
  buildBase: path.relative(repo, buildBase).split(path.sep).join('/'),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  copiedPreviousSelectedRoot: path.relative(repo, previousRoot).split(path.sep).join('/'),
  stagedCount: allFiles.length,
  stagedBytes: allFiles.reduce((sum, item) => sum + item.bytes, 0),
  updatedSources: staged,
  selectedStaticFiles: allFiles,
  noSymlinks: true,
  fullDocsDeadLinkCheck: false,
};
fs.writeFileSync(manifest, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, selectedStaticFiles: undefined }, null, 2)}\n`);
