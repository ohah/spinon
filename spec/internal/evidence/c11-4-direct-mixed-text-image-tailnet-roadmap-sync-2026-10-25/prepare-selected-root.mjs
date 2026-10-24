import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c114-direct-mixed-tailnet-selected-root-20261025');
const manifestPath = path.join(evidenceDirectory, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace selected root');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite selected root manifest');
fs.mkdirSync(selectedRoot, { recursive: true });

const selected = [];
function copyFile(sourceRelative, destinationRelative) {
  const source = path.join(repositoryRoot, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), 'unsafe source: ' + sourceRelative);
  const bytes = fs.readFileSync(source);
  const destination = path.join(selectedRoot, destinationRelative);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  const copied = fs.readFileSync(destination);
  assert.equal(sha256(copied), sha256(bytes), 'copy hash mismatch: ' + destinationRelative);
  selected.push({ source: sourceRelative, path: destinationRelative, bytes: copied.byteLength, sha256: sha256(copied) });
}
function copyEvidenceTree(sourceRelative, destinationRelative) {
  const sourceRoot = path.join(repositoryRoot, sourceRelative);
  const rootStat = fs.lstatSync(sourceRoot);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), 'unsafe evidence root: ' + sourceRelative);
  function visit(currentSource, currentRelative) {
    for (const entry of fs.readdirSync(currentSource, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const source = path.join(currentSource, entry.name);
      const relative = path.posix.join(currentRelative, entry.name);
      const stat = fs.lstatSync(source);
      assert.ok(!stat.isSymbolicLink(), 'symlink in evidence: ' + relative);
      if (stat.isDirectory()) {
        visit(source, relative);
      } else {
        assert.ok(stat.isFile(), 'unsupported evidence entry: ' + relative);
        const extension = path.extname(entry.name).toLowerCase();
        const docFile = extension === '.md';
        const destination = path.posix.join(docFile ? '' : 'public', destinationRelative, relative);
        const bytes = fs.readFileSync(source);
        const target = path.join(selectedRoot, destination);
        fs.mkdirSync(path.dirname(target), { recursive: true });
        fs.writeFileSync(target, bytes, { flag: 'wx' });
        const copied = fs.readFileSync(target);
        assert.equal(sha256(copied), sha256(bytes), 'evidence copy hash mismatch: ' + destination);
        selected.push({ source: path.posix.join(sourceRelative, relative), path: destination, bytes: copied.byteLength, sha256: sha256(copied) });
      }
    }
  }
  visit(sourceRoot, '');
}
const sourceFiles = [
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/public/icon.svg', 'public/icon.svg'],
  ['spec/public/logo.svg', 'public/logo.svg'],
  ['plan/c11-css-grid.md', 'plan/c11-css-grid.md'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-4-mixed-direct-text-image-intrinsic.md', 'plan/c11-4-mixed-direct-text-image-intrinsic.md'],
  ['plan/c11-4-mixed-direct-text-image-intrinsic-plan-review-20.md', 'plan/c11-4-mixed-direct-text-image-intrinsic-plan-review-20.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
];
for (const [source, destination] of sourceFiles) copyFile(source, destination);
copyEvidenceTree(
  'spec/internal/evidence/c11-4-mixed-direct-text-image-intrinsic-precomparison-2026-10-25',
  'internal/evidence/c11-4-mixed-direct-text-image-intrinsic-precomparison-2026-10-25',
);
copyEvidenceTree(
  'spec/internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25',
  'internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25',
);
const manifest = {
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  sourceFileCount: selected.length,
  sourceFiles: selected,
  noDeletes: true,
};
fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ selectedRoot: manifest.selectedRoot, sourceFileCount: manifest.sourceFileCount, noDeletes: true }, null, 2) + '\n');
