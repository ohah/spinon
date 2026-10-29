import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const taskBuildRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-c11-4-zoom-2026-10-29');
const selectedRoot = path.join(taskBuildRoot, 'selected-root-pass02');
const manifestPath = path.join(taskBuildRoot, 'selected-input-manifest-pass02.json');
assert.ok(!fs.existsSync(selectedRoot), `refusing to reuse selected root: ${selectedRoot}`);
assert.ok(!fs.existsSync(manifestPath), `refusing to replace manifest: ${manifestPath}`);
fs.mkdirSync(taskBuildRoot, { recursive: true });
fs.mkdirSync(selectedRoot, { recursive: false });
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const files = [];
function copyFile(relativeSource, relativeTarget) {
  const source = path.join(repositoryRoot, relativeSource);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe source file: ${relativeSource}`);
  const bytes = fs.readFileSync(source);
  const target = path.join(selectedRoot, relativeTarget);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
  files.push({ source: relativeSource, target: relativeTarget.split(path.sep).join('/'), bytes: bytes.length, sha256: sha256(bytes) });
}
function copyDirectory(relativeSource, relativeTarget) {
  const source = path.join(repositoryRoot, relativeSource);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe source directory: ${relativeSource}`);
  for (const entry of fs.readdirSync(source, { withFileTypes: true })) {
    const sourceChild = path.join(source, entry.name);
    const sourceChildRelative = path.join(relativeSource, entry.name);
    const targetChildRelative = path.join(relativeTarget, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in selected source: ${sourceChildRelative}`);
    if (entry.isDirectory()) copyDirectory(sourceChildRelative, targetChildRelative);
    else if (entry.isFile()) copyFile(sourceChildRelative, targetChildRelative);
    else throw new Error(`unsupported selected source entry: ${sourceChildRelative}`);
  }
}

for (const [source, target] of [
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['spec/public/roadmap.html', 'spec/public/roadmap.html'],
  ['spec/public/icon.svg', 'public/icon.svg'],
  ['spec/public/logo.svg', 'public/logo.svg'],
  ['spec/public/roadmap.svg', 'public/roadmap.svg'],
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-4-fit-content-zoom-wpt.md', 'plan/c11-4-fit-content-zoom-wpt.md'],
  ['plan/c11-4-fit-content-zoom-wpt-plan-review-20.md', 'plan/c11-4-fit-content-zoom-wpt-plan-review-20.md'],
]) copyFile(source, target);
for (const directory of [
  ['spec/internal/evidence/c11-4-fit-content-zoom-precomparison-2026-10-29', 'internal/evidence/c11-4-fit-content-zoom-precomparison-2026-10-29'],
  ['spec/internal/evidence/c11-4-fit-content-zoom-runtime-2026-10-29', 'internal/evidence/c11-4-fit-content-zoom-runtime-2026-10-29'],
]) copyDirectory(...directory);
assert.equal(new Set(files.map((entry) => entry.target)).size, files.length, 'duplicate selected route');
const manifest = {
  sourceHead: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(),
  sourceTree: execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(),
  selectedRoot,
  fileCount: files.length,
  totalBytes: files.reduce((sum, entry) => sum + entry.bytes, 0),
  files,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ selectedRoot, manifestPath, fileCount: manifest.fileCount, totalBytes: manifest.totalBytes }, null, 2)}\n`);
