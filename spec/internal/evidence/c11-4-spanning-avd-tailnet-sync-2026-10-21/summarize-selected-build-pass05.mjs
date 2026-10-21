import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const preparation = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass05.json'), 'utf8'));
const siteRoot = path.join(repositoryRoot, 'build/spinon/c11-4-spanning-avd-tailnet-site-pass05');
assert.equal(preparation.sourceHead, execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim());
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
const evidence = 'internal/evidence/c11-4-spanning-auto-min-flex-avd-recheck-2026-10-21';
const run = `${evidence}/android-avd-c114-spanning-auto-min-flex-avd-recheck-android-avd-20261021T091026603Z-21ddb7ba`;
const requiredRoutes = [
  'roadmap.html', 'STATUS.html', 'spec/STATUS.html', 'plan/c11-4-grid-track-sizing.html',
  `${evidence}/README.html`, `${evidence}/pr-diff-review-30.html`, `${evidence}/pr438-image-agent-browser.json`,
  `${run}/spanning-auto-min-flex-tracks.png`,
];
for (const route of requiredRoutes) assert.ok(files.some((file) => file.path === route), `missing generated route: ${route}`);
const read = (route) => fs.readFileSync(path.join(siteRoot, route), 'utf8');
const roadmap = read('roadmap.html');
const status = read('spec/STATUS.html');
const plan = read('plan/c11-4-grid-track-sizing.html');
const report = read(`${evidence}/README.html`);
assert.ok(roadmap.includes('Android API 37 AVD 보충 검증') && roadmap.includes('C11.4 Ahem spanning auto-minimum flex text'));
assert.ok(roadmap.includes('/spinon/internal/evidence/c11-4-spanning-auto-min-flex-avd-recheck-2026-10-21/README.html'));
assert.ok(status.includes('72/213') && status.includes('C11.4'));
assert.ok(plan.includes('Android API 37 AVD'));
assert.ok(report.includes('Ahem 문자의 glyph paint'));
const treeHash = createHash('sha256');
for (const file of files) { treeHash.update(file.path); treeHash.update('\0'); treeHash.update(file.sha256); treeHash.update('\n'); }
const output = {
  pass: 'pass05', rspressVersion: '2.0.22', sourceHead: preparation.sourceHead, sourceTree: preparation.sourceTree,
  selectedRoot: preparation.selectedRoot, siteRoot: path.relative(repositoryRoot, siteRoot), fileCount: files.length,
  bytesTotal: files.reduce((sum, file) => sum + file.bytes, 0), siteTreeSha256: treeHash.digest('hex'),
  requiredRoutes, roadmapNotePresent: true, statusStill72of213: true, c114RemainsIncomplete: true,
  ahemGlyphAndOverflowPixelNotClaimed: true, noSymlinks: true, fullRepositoryDeadLinkCheck: false,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-build-summary-pass05.json'), `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
