import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const siteRoot = path.join(repositoryRoot, 'build/spinon/c11-4-anonymous-grid-border-overflow-tailnet-site-pass01');
const files = [];
function walk(directory, prefix = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const fullPath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected site: ${relative}`);
    if (entry.isDirectory()) walk(fullPath, relative);
    else {
      assert.ok(entry.isFile(), `unsupported selected site entry: ${relative}`);
      const bytes = fs.readFileSync(fullPath);
      files.push({ path: relative, bytes: bytes.byteLength, sha256: createHash('sha256').update(bytes).digest('hex') });
    }
  }
}
walk(siteRoot);
const requiredRoutes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-anonymous-grid-border-overflow.html',
  'plan/c11-4-anonymous-grid-border-overflow-implementation-review-20.html',
  'plan/c11-4-anonymous-grid-border-overflow-pr-diff-review-20.html',
  'internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/README.html',
  'internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/android-avd-c114-anonymous-grid-border-overflow-android-avd-20261021T070322769Z-ef7ddc0e/anonymous-runs-two-images-forward.png',
  'internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/ios-simulator-c114-anonymous-grid-border-overflow-ios-simulator-20261021T070603043Z-8ea2e515/anonymous-runs-two-images-forward.png',
];
for (const route of requiredRoutes) assert.ok(files.some((file) => file.path === route), `missing route: ${route}`);
const roadmap = fs.readFileSync(path.join(siteRoot, 'roadmap.html'), 'utf8');
const status = fs.readFileSync(path.join(siteRoot, 'STATUS.html'), 'utf8');
const plan = fs.readFileSync(path.join(siteRoot, 'plan/c11-4-anonymous-grid-border-overflow.html'), 'utf8');
assert.ok(roadmap.includes('PR #431') && roadmap.includes('C11.4 anonymous Grid border + image overflow'));
assert.ok(roadmap.includes('c11-4-anonymous-grid-border-overflow-2026-10-21/android-avd-c114-anonymous-grid-border-overflow'));
assert.ok(roadmap.includes('c11-4-anonymous-grid-border-overflow-2026-10-21/ios-simulator-c114-anonymous-grid-border-overflow'));
assert.ok(status.includes('pull/431') && status.includes('420be2c40cb0a9f68023121f66ff122158494ffa') && status.includes('72/213'));
assert.ok(plan.includes('420be2c40cb0a9f68023121f66ff122158494ffa') && plan.includes('Tailnet preview'));
const treeHash = createHash('sha256');
for (const file of files) {
  treeHash.update(file.path);
  treeHash.update('\0');
  treeHash.update(file.sha256);
  treeHash.update('\n');
}
const output = {
  pass: 'pass01',
  rspressVersion: '2.0.22',
  sourceHead: '135b241e05fbf4f4e6e41e9f29c4f145a160f11d',
  sourceTree: 'c92d41f8da22f5f42c83637a92dddf7ae711252e',
  selectedRoot: 'build/spinon/c11-4-anonymous-grid-border-overflow-tailnet-root-pass01-selected',
  siteRoot: path.relative(repositoryRoot, siteRoot),
  fileCount: files.length,
  bytes: files.reduce((sum, file) => sum + file.bytes, 0),
  siteTreeSha256: treeHash.digest('hex'),
  requiredRoutes,
  noSymlinks: true,
  roadmapNotePresent: true,
  statusMergeRecordPresent: true,
  officialCompletionCountStill72of213: true,
  buildLog: 'selected-build-pass01.log',
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-build-summary-pass01.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
