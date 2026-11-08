import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previousManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-manifest.json'), 'utf8'));
const buildBase = path.join(repo, 'build/spinon/roadmap-c11-4-pr1277-status-docs-pass03-2026-11-08');
const previousRoot = path.join(repo, previousManifest.selectedRoot);
const root = path.join(buildBase, 'root');
const manifestPath = path.join(evidence, 'selected-root-manifest-pass03.json');
const stageReportPath = path.join(evidence, 'stage-pass03.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
for (const target of [buildBase, root, manifestPath, stageReportPath]) assert.ok(!fs.existsSync(target), `기존 pass03 산출물을 보존함: ${target}`);

function walk(directory, relative = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${child}`);
    const absolute = path.join(directory, child);
    if (entry.isDirectory()) files.push(...walk(directory, child));
    else if (entry.isFile()) files.push(child);
    else throw new Error(`지원하지 않는 항목: ${child}`);
  }
  return files;
}

const previousFiles = walk(previousRoot);
assert.equal(previousFiles.length, previousManifest.stagedFileCount, '이전 selected root 파일 수가 바뀜');
fs.mkdirSync(buildBase, { recursive: false });
fs.cpSync(previousRoot, root, { recursive: true, errorOnExist: true, force: false, preserveTimestamps: true });

const updatedRoutes = new Set([
  'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/README.md',
  'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-status-review-20.md',
  'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-pr-diff-review-30.md',
]);
for (const route of updatedRoutes) {
  const sourceRelative = `spec/${route}`;
  const source = path.join(repo, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source 파일이 아님: ${sourceRelative}`);
  const target = path.join(root, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
}

const staged = [];
for (const route of walk(root)) {
  const sourceRelative = updatedRoutes.has(route)
    ? `spec/${route}`
    : previousManifest.staged.find((item) => item.route === route)?.source;
  assert.ok(sourceRelative, `기존 manifest에 없는 staged 파일: ${route}`);
  const sourceBytes = fs.readFileSync(path.join(repo, sourceRelative));
  const stagedBytes = fs.readFileSync(path.join(root, route));
  assert.equal(sha256(stagedBytes), sha256(sourceBytes), `staged source hash 불일치: ${route}`);
  staged.push({ source: sourceRelative, route, bytes: stagedBytes.length, sha256: sha256(stagedBytes) });
}
for (const route of updatedRoutes) assert.ok(staged.some((item) => item.route === route), `updated route 누락: ${route}`);
const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['진행 중: C11.4 Grid 트랙 크기', "task.code + ' · 진행 중'", '78/214']) assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);

const report = {
  buildBase: path.relative(repo, buildBase).split(path.sep).join('/'),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  copiedFromPreviousSelectedRoot: previousManifest.selectedRoot,
  updatedRoutes: [...updatedRoutes],
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  allFilesMatchCurrentSources: true,
  noSymlinks: true,
  fullDocsDeadLinkCheck: false,
  staged,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(stageReportPath, `${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
