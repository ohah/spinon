import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1087-2026-11-03/pass01');
const root = path.join(buildBase, 'root');
const previousSourceHead = 'c82639bd1';
const previousRoot = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-4-full-size-kana-2026-11-03/pass01/root');
const manifestPath = path.join(evidence, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
const overlays = [];

for (const target of [root, manifestPath]) {
  assert.ok(!fs.existsSync(target), '기존 선택 root 산출물을 덮어쓰지 않음: ' + target);
}

function assertDirectory(directory) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), '안전하지 않은 디렉터리: ' + directory);
}

function listFiles(directory, prefix = '') {
  const paths = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? prefix + '/' + entry.name : entry.name;
    assert.ok(!entry.isSymbolicLink(), 'symlink 거부: ' + relative);
    if (entry.isDirectory()) paths.push(...listFiles(directory, relative));
    else if (entry.isFile()) paths.push(relative.split(path.sep).join('/'));
    else throw new Error('지원하지 않는 파일 형식: ' + relative);
  }
  return paths;
}

function routesFor(source) {
  if (source === 'spec/STATUS.md') return ['STATUS.md', 'spec/STATUS.md'];
  if (source === 'spec/public/roadmap.html') return ['public/roadmap.html', 'spec/public/roadmap.html'];
  if (source.startsWith('spec/')) return [source.slice('spec/'.length), source];
  return [source];
}

function replaceFile(sourceRelative, route) {
  const source = path.join(repo, sourceRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'source가 일반 파일이 아님: ' + sourceRelative);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
  const bytes = fs.readFileSync(target);
  assert.equal(sha256(bytes), sha256(fs.readFileSync(source)), 'overlay hash 불일치: ' + route);
  overlays.push({ source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) });
}

assertDirectory(previousRoot);
assertDirectory(path.join(repo, 'build'));
assertDirectory(path.join(repo, 'build/spinon'));
listFiles(previousRoot);
fs.mkdirSync(buildBase, { recursive: true });
fs.cpSync(previousRoot, root, { recursive: true, errorOnExist: true, force: false });

const changedSources = git(['diff', '--name-only', '--diff-filter=ACMR', previousSourceHead + '..HEAD', '--', 'plan/', 'spec/'])
  .split(/\r?\n/).filter(Boolean);
assert.ok(changedSources.length > 0, '이전 공개 source 이후 변경 문서가 있어야 함');
for (const source of changedSources) {
  for (const route of routesFor(source)) replaceFile(source, route);
}

const staged = listFiles(root).map((route) => {
  const bytes = fs.readFileSync(path.join(root, route));
  return { route, bytes: bytes.length, sha256: sha256(bytes) };
});
const trackedDiff = execFileSync('git', ['diff', '--binary', previousSourceHead + '..HEAD', '--', 'plan/', 'spec/'], {
  cwd: repo,
  maxBuffer: 32 * 1024 * 1024,
});
const manifest = {
  sourceHead: git(['rev-parse', 'HEAD']),
  sourceTree: git(['rev-parse', 'HEAD^{tree}']),
  previousSourceHead,
  trackedDiffSha256: sha256(trackedDiff),
  previousRoot: path.relative(repo, previousRoot).split(path.sep).join('/'),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  baseFileCount: listFiles(previousRoot).length,
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, file) => sum + file.bytes, 0),
  changedSourceCount: changedSources.length,
  overlayCount: overlays.length,
  noDeletes: true,
  overlays,
  staged,
};
fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ ...manifest, staged: undefined, overlays: undefined }, null, 2) + '\n');
