import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildRoot = path.join(repo, 'build/spinon/c11-4-text-indent-status-sync-2026-11-01-pass02');
const root = path.join(buildRoot, 'root');
const previousEvidence = path.join(repo, 'spec/internal/evidence/c11-4-line-break-loose-public-roadmap-2026-11-01');
const previousManifest = JSON.parse(fs.readFileSync(path.join(previousEvidence, 'selected-root.json'), 'utf8'));
const previousRoot = path.join(repo, previousManifest.selectedRoot);
const manifestPath = path.join(evidence, 'selected-root.json');
const sha256 = (value) => createHash('sha256').update(value).digest('hex');
const git = (args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8' }).trim();
const changed = git(['diff', '--name-only', 'HEAD']).split('\n').filter(Boolean).sort();
assert.deepEqual(changed, ['spec/STATUS.md'], 'STATUS.md만 tracked 변경이어야 함');
assert.equal(git(['rev-parse', 'HEAD']), git(['rev-parse', 'origin/main']), 'branch base는 origin/main이어야 함');
assert.ok(!fs.existsSync(buildRoot) && !fs.existsSync(manifestPath), '새 build root/manifest여야 함');
function listFiles(directory, prefix = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(directory, prefix), { withFileTypes: true })) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `source symlink 거부: ${relative}`);
    if (entry.isDirectory()) files.push(...listFiles(directory, relative));
    else if (entry.isFile()) files.push(relative);
    else throw new Error(`지원하지 않는 source entry: ${relative}`);
  }
  return files;
}
const previousFiles = listFiles(previousRoot);
assert.equal(previousFiles.length, previousManifest.stagedFileCount, '이전 root regular-file count 확인');
fs.mkdirSync(buildRoot, { recursive: false });
fs.cpSync(previousRoot, root, { recursive: true, errorOnExist: true, force: false });
for (const target of ['STATUS.md', 'spec/STATUS.md']) {
  const output = path.join(root, target);
  fs.mkdirSync(path.dirname(output), { recursive: true });
  const existing = fs.lstatSync(output);
  assert.ok(existing.isFile() && !existing.isSymbolicLink(), `overlay target must be a regular selected-root file: ${target}`);
  fs.copyFileSync(path.join(repo, 'spec/STATUS.md'), output);
}
const sourceHash = sha256(fs.readFileSync(path.join(repo, 'spec/STATUS.md')));
for (const target of ['STATUS.md', 'spec/STATUS.md']) {
  assert.equal(sha256(fs.readFileSync(path.join(root, target))), sourceHash, `${target} source hash match`);
}
const manifest = {
  sourceHead: git(['rev-parse', 'HEAD']),
  baseTree: git(['rev-parse', 'HEAD^{tree}']),
  sourceStatusWorkingTreeSha256: sourceHash,
  previousSelectedRoot: previousManifest.selectedRoot,
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  previousFileCount: previousManifest.stagedFileCount,
  fileCount: listFiles(root).length,
  overlays: ['STATUS.md', 'spec/STATUS.md'],
  overlaySha256: sourceHash,
  noDeletes: true,
  noSymlinksAdded: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(manifest, null, 2)}\n`);
