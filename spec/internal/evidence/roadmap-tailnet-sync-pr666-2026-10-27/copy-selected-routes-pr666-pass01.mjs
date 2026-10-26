import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { randomUUID } from 'node:crypto';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr666-2026-10-27/pass01';
const build = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-build-pr666-pass02.json'), 'utf8'));
const priorHandles = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'target-open-handles-pr666-pass01.json'), 'utf8'));
const reportPath = path.join(evidenceDirectory, 'tailnet-copy-pr666-pass01.json');
const logPath = path.join(evidenceDirectory, 'tailnet-copy-run-pr666-pass01.log');
for (const candidate of [reportPath, logPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.equal(fs.realpathSync(previewRoot), previewRoot, 'preview root realpath differs from configured root');
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);

const selectedRoot = path.join(repositoryRoot, build.selectedOutputRoot);
const originMain = spawnSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' });
assert.equal(originMain.status, 0);
assert.equal(originMain.stdout.trim(), build.sourceHead, 'origin/main changed after selected build');
const originTree = spawnSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' });
assert.equal(originTree.stdout.trim(), build.sourceTree, 'origin/main tree changed after selected build');

const routes = build.selectedRoutes;
const openHandleRoutes = new Set();
const targets = [];
for (const route of routes) {
  const relative = route.path;
  assert.ok(!relative.startsWith('/') && relative.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe route: ${relative}`);
  const sourcePath = path.join(selectedRoot, relative);
  const sourceStat = fs.lstatSync(sourcePath);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `unsafe selected source: ${relative}`);
  const sourceBytes = fs.readFileSync(sourcePath);
  assert.equal(sourceBytes.length, route.bytes, `selected source size changed: ${relative}`);
  assert.equal(sha256(sourceBytes), route.sha256, `selected source hash changed: ${relative}`);
  const target = path.join(previewRoot, relative);
  let currentStat = null;
  try { currentStat = fs.lstatSync(target); }
  catch (error) { if (error.code !== 'ENOENT') throw error; }
  if (currentStat) assert.ok(currentStat.isFile() && !currentStat.isSymbolicLink(), `unsafe preview target: ${relative}`);
  targets.push({ relative, sourcePath, target, sourceBytes, currentStat });
}

const existingTargetPaths = targets.filter((item) => item.currentStat).map((item) => item.target);
const currentHandles = spawnSync('lsof', ['-nP', '--', ...existingTargetPaths], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
assert.ok(currentHandles.status === 0 || currentHandles.status === 1, `lsof failed: ${currentHandles.stderr}`);
for (const line of currentHandles.stdout.trim().split('\n').slice(1).filter(Boolean)) {
  const targetPath = line.slice(line.indexOf(previewRoot));
  const relative = path.relative(previewRoot, targetPath);
  const item = targets.find((candidate) => candidate.relative === relative);
  assert.ok(item, `open file is not a selected route: ${targetPath}`);
  const currentBytes = fs.readFileSync(item.target);
  assert.equal(sha256(currentBytes), item.routeSha256 ?? sha256(item.sourceBytes), `an open target differs from the selected route: ${relative}`);
  openHandleRoutes.add(relative);
}
for (const record of priorHandles.lsofOutput.split('\n').slice(1).filter(Boolean)) {
  const targetPath = record.slice(record.indexOf(previewRoot));
  const relative = path.relative(previewRoot, targetPath);
  const item = targets.find((candidate) => candidate.relative === relative);
  assert.ok(item, `previously open target is no longer selected: ${relative}`);
  if (openHandleRoutes.has(relative)) {
    assert.equal(sha256(item.sourceBytes), JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'open-handle-route-comparison-pr666-pass01.json'), 'utf8')).records.find((entry) => entry.route === relative)?.selectedSha256, `open target evidence mismatch: ${relative}`);
  }
}

for (const target of targets) {
  const pieces = path.relative(previewRoot, target.target).split(path.sep);
  let ancestor = previewRoot;
  for (const part of pieces.slice(0, -1)) {
    ancestor = path.join(ancestor, part);
    try {
      const stat = fs.lstatSync(ancestor);
      assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe preview directory: ${ancestor}`);
    } catch (error) {
      if (error.code === 'ENOENT') break;
      throw error;
    }
  }
}

fs.mkdirSync(path.dirname(backupRoot), { recursive: true });
fs.mkdirSync(backupRoot, { recursive: false });
const created = [];
const replaced = [];
const identical = [];
const openHandleSkipped = [];
for (const item of targets) {
  const nextHash = sha256(item.sourceBytes);
  if (item.currentStat) {
    const currentBytes = fs.readFileSync(item.target);
    const currentHash = sha256(currentBytes);
    if (currentHash === nextHash) {
      identical.push({ path: item.relative, bytes: currentBytes.length, sha256: currentHash });
      if (openHandleRoutes.has(item.relative)) openHandleSkipped.push(item.relative);
      continue;
    }
    assert.ok(!openHandleRoutes.has(item.relative), `refusing to overwrite open target: ${item.relative}`);
    const backupPath = path.join(backupRoot, item.relative);
    fs.mkdirSync(path.dirname(backupPath), { recursive: true });
    fs.copyFileSync(item.target, backupPath, fs.constants.COPYFILE_EXCL);
    const backupBytes = fs.readFileSync(backupPath);
    assert.equal(sha256(backupBytes), currentHash, `backup differs from pre-sync target: ${item.relative}`);
    const tempPath = `${item.target}.codex-sync-${randomUUID()}.tmp`;
    fs.mkdirSync(path.dirname(item.target), { recursive: true });
    fs.writeFileSync(tempPath, item.sourceBytes, { flag: 'wx', mode: item.currentStat.mode & 0o777 });
    fs.chmodSync(tempPath, item.currentStat.mode & 0o777);
    fs.renameSync(tempPath, item.target);
    const written = fs.readFileSync(item.target);
    assert.equal(sha256(written), nextHash, `replacement hash mismatch: ${item.relative}`);
    replaced.push({ path: item.relative, bytes: written.length, sha256: nextHash, backupPath: path.relative(backupRoot, backupPath), backupBytes: backupBytes.length, backupSha256: currentHash });
  } else {
    fs.mkdirSync(path.dirname(item.target), { recursive: true });
    const tempPath = `${item.target}.codex-sync-${randomUUID()}.tmp`;
    fs.writeFileSync(tempPath, item.sourceBytes, { flag: 'wx' });
    fs.renameSync(tempPath, item.target);
    const written = fs.readFileSync(item.target);
    assert.equal(sha256(written), nextHash, `new route hash mismatch: ${item.relative}`);
    created.push({ path: item.relative, bytes: written.length, sha256: nextHash });
  }
}

const report = {
  sourceHead: build.sourceHead,
  sourceTree: build.sourceTree,
  previewRoot,
  backupRoot,
  selectedRouteCount: routes.length,
  selectedBytes: routes.reduce((sum, route) => sum + route.bytes, 0),
  createdCount: created.length,
  replacedCount: replaced.length,
  identicalCount: identical.length,
  openHandleCount: openHandleRoutes.size,
  openHandleSkipped,
  noDeletes: true,
  created,
  replaced,
  identical,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(logPath, `sourceHead=${build.sourceHead}\nsourceTree=${build.sourceTree}\nselectedRouteCount=${routes.length}\ncreated=${created.length}\nreplaced=${replaced.length}\nidentical=${identical.length}\nopenHandleSkipped=${openHandleSkipped.length}\nbackupRoot=${backupRoot}\nnoDeletes=true\n` , { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, created: undefined, replaced: undefined, identical: undefined }, null, 2)}\n`);
