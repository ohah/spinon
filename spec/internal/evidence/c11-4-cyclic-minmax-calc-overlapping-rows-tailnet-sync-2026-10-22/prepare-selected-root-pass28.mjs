import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidenceDir, '../../../..');
const previousRoot = path.join(repo, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-rows-tailnet-root-pass27-selected');
const selectedRoot = path.join(repo, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-rows-tailnet-root-pass28-selected');
const previousMetadataPath = path.join(repo, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-rows-tailnet-sync-pass27-evidence/selected-docs-root-pass27.json');
const outputPath = path.join(evidenceDir, 'selected-root-preparation-pass28.json');
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', `${sourceCommit}^{tree}`], { cwd: repo, encoding: 'utf8' }).trim();
const previousMetadata = JSON.parse(fs.readFileSync(previousMetadataPath, 'utf8'));
assert.equal(sourceCommit, '530691ee889b03f40e79e216bab042efd5427e09');
assert.equal(previousMetadata.sourceCommit, 'b4fa4725500b3a081f93174be547257659c76f3e');
assert.ok(fs.statSync(previousRoot).isDirectory(), 'pass27 selected root is missing');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to overwrite pass28 selected root');
assert.ok(!fs.existsSync(outputPath), 'refusing to overwrite pass28 metadata');

const changedPaths = execFileSync('git', ['diff', '--name-only', previousMetadata.sourceCommit, sourceCommit], { cwd: repo, encoding: 'utf8' })
  .trim().split('\n').filter(Boolean);
const statusPath = 'spec/STATUS.md';
const roadmapPath = 'spec/public/roadmap.html';
const planPath = 'plan/c11-4-cyclic-minmax-calc-overlapping-rows.md';
const evidenceDirectory = 'spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-rows-runtime-2026-10-22';
assert.ok(changedPaths.includes(statusPath));
assert.ok(changedPaths.includes(roadmapPath));
assert.ok(changedPaths.includes(planPath));
assert.ok(changedPaths.some((file) => file.startsWith(`${evidenceDirectory}/`)));
const plan = execFileSync('git', ['show', `${sourceCommit}:${planPath}`], { cwd: repo, encoding: 'utf8' });
const status = execFileSync('git', ['show', `${sourceCommit}:${statusPath}`], { cwd: repo, encoding: 'utf8' });
const roadmap = execFileSync('git', ['show', `${sourceCommit}:${roadmapPath}`], { cwd: repo, encoding: 'utf8' });
for (const source of [plan, status, roadmap]) {
  assert.ok(source.includes('36acdd4f48f23499835530704fa8aef20590a6e5'));
  assert.ok(source.includes('77/213'));
}

fs.cpSync(previousRoot, selectedRoot, { recursive: true, errorOnExist: true });
const copied = [];
function copyFromCommit(source, target) {
  assert.ok(!path.isAbsolute(target) && !target.split(path.sep).includes('..'));
  const bytes = execFileSync('git', ['show', `${sourceCommit}:${source}`], { cwd: repo, maxBuffer: 32 * 1024 * 1024 });
  const destination = path.join(selectedRoot, target);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes);
  copied.push({ source, target, bytes: bytes.byteLength, sha256: createHash('sha256').update(bytes).digest('hex') });
}

copyFromCommit(planPath, planPath);
copyFromCommit(statusPath, 'STATUS.md');
copyFromCommit(statusPath, 'spec/STATUS.md');
copyFromCommit(roadmapPath, 'roadmap.html');
copyFromCommit(roadmapPath, 'public/roadmap.html');
const aliases = ['internal/evidence', 'spec/internal/evidence', 'public/internal/evidence', 'public/spec/internal/evidence'];
const evidenceFiles = execFileSync('git', ['ls-tree', '-r', '--name-only', sourceCommit, '--', evidenceDirectory], { cwd: repo, encoding: 'utf8' })
  .trim().split('\n').filter(Boolean);
assert.ok(evidenceFiles.length >= 50, 'runtime evidence directory is incomplete');
for (const source of evidenceFiles) {
  const suffix = source.slice('spec/internal/evidence/'.length);
  for (const alias of aliases) copyFromCommit(source, `${alias}/${suffix}`);
}

let selectedFiles = 0;
let selectedBytes = 0;
const symlinks = [];
function scan(directory, prefix = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const full = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) symlinks.push(relative);
    else if (entry.isDirectory()) scan(full, relative);
    else if (entry.isFile()) { selectedFiles += 1; selectedBytes += fs.statSync(full).size; }
    else throw new Error(`unsupported entry: ${relative}`);
  }
}
scan(selectedRoot);
assert.deepEqual(symlinks, []);
const result = {
  schemaVersion: 1,
  sourceCommit,
  sourceTree,
  previousSourceCommit: previousMetadata.sourceCommit,
  previousSelectedRoot: path.relative(repo, previousRoot),
  selectedRoot: path.relative(repo, selectedRoot),
  changedPathCountSincePass27: changedPaths.length,
  copiedPaths: copied.length,
  copiedRuntimeEvidenceFiles: evidenceFiles.length,
  selectedFiles,
  selectedBytes,
  symlinkCount: symlinks.length,
  ignoredUntrackedWorkspaceEvidence: true,
  noPreviewWrites: true,
  copies: copied,
};
fs.writeFileSync(outputPath, `${JSON.stringify(result, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...result, copies: copied.length }, null, 2)}\n`);
