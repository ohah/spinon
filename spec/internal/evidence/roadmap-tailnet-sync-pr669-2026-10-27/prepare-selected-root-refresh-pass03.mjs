import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr669-20261027-root-refresh-pass03');
const manifestPath = path.join(evidenceDirectory, 'selected-source-refresh-pass03.json');
const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const treeHash = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(selectedRoot), `refusing to replace selected root: ${selectedRoot}`);
assert.ok(!fs.existsSync(manifestPath), `refusing to replace ${manifestPath}`);

const mappings = [
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['spec/public/icon.svg', 'public/icon.svg'],
  ['spec/public/logo.svg', 'public/logo.svg'],
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['plan/c11-4-align-content-normal-runtime.md', 'plan/c11-4-align-content-normal-runtime.md'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md'],
  ['tests/fixtures/css/c11/grid-track-sizing-c114-align-content-normal-runtime.html', 'tests/fixtures/css/c11/grid-track-sizing-c114-align-content-normal-runtime.html'],
  ['spec/internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27', 'internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27'],
  ['spec/internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27', 'spec/internal/evidence/c11-4-align-content-normal-runtime-precomparison-2026-10-27'],
  ['spec/internal/evidence/c11-4-align-content-normal-runtime-2026-10-27', 'internal/evidence/c11-4-align-content-normal-runtime-2026-10-27'],
  ['spec/internal/evidence/c11-4-align-content-normal-runtime-2026-10-27', 'spec/internal/evidence/c11-4-align-content-normal-runtime-2026-10-27'],
  ['spec/internal/evidence/roadmap-tailnet-sync-pr669-2026-10-27/README.md', 'internal/evidence/roadmap-tailnet-sync-pr669-2026-10-27/README.md'],
  ['spec/internal/evidence/roadmap-tailnet-sync-pr669-2026-10-27/pr-diff-review-30.md', 'internal/evidence/roadmap-tailnet-sync-pr669-2026-10-27/pr-diff-review-30.md'],
];

const copied = [];
for (const [sourceRelative, targetRelative] of mappings) {
  const source = path.join(repositoryRoot, sourceRelative);
  const target = path.join(selectedRoot, targetRelative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() || stat.isDirectory(), `unsupported source: ${sourceRelative}`);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.cpSync(source, target, { recursive: true, errorOnExist: true, force: false, dereference: false });
  const files = stat.isDirectory()
    ? listFiles(target, selectedRoot)
    : [{ path: targetRelative, source: sourceRelative, sha256: sha256(fs.readFileSync(source)), bytes: stat.size }];
  if (stat.isDirectory()) {
    for (const record of files) {
      const relativeInMappedRoot = path.relative(target, path.join(selectedRoot, record.path));
      const mappedSource = path.join(source, relativeInMappedRoot);
      const bytes = fs.readFileSync(mappedSource);
      record.source = path.join(sourceRelative, relativeInMappedRoot).split(path.sep).join('/');
      record.sha256 = sha256(bytes);
      record.bytes = bytes.length;
    }
  }
  copied.push(...files);
}

function listFiles(directory, base) {
  const records = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const full = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `refusing symlink in selected root: ${path.relative(base, full)}`);
    if (entry.isDirectory()) records.push(...listFiles(full, base));
    else if (entry.isFile()) records.push({ path: path.relative(base, full).split(path.sep).join('/') });
  }
  return records;
}

const manifest = {
  sourceHead,
  sourceTree: treeHash,
  selectedRoot: path.relative(repositoryRoot, selectedRoot).split(path.sep).join('/'),
  mappingCount: mappings.length,
  fileCount: copied.length,
  totalBytes: copied.reduce((sum, item) => sum + item.bytes, 0),
  noSymlinks: true,
  copied,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, copied: undefined }, null, 2)}\n`);
