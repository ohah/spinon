import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c114-direct-mixed-tailnet-selected-root-20261025-pass02');
const rspressOutput = path.join(repositoryRoot, 'build/spinon/c114-direct-mixed-tailnet-selected-site-20261025-pass02');
const selectedOutput = path.join(repositoryRoot, 'build/spinon/c114-direct-mixed-tailnet-selected-output-20261025-pass03');
const reportPath = path.join(evidenceDirectory, 'selected-output-preparation-pass03.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(selectedOutput), 'refusing to replace selected output');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected output preparation report');
for (const root of [selectedRoot, rspressOutput]) {
  const stat = fs.lstatSync(root);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), 'unsafe selected root: ' + root);
}
fs.mkdirSync(selectedOutput, { recursive: true });
const copied = [];
function copyOne(source, relative, sourceName) {
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'unsafe selected file: ' + sourceName);
  const bytes = fs.readFileSync(source);
  const destination = path.join(selectedOutput, relative);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  if (fs.existsSync(destination)) {
    const existing = fs.readFileSync(destination);
    assert.equal(sha256(existing), sha256(bytes), 'route collision with different bytes: ' + relative);
  } else {
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
  }
  const finalBytes = fs.readFileSync(destination);
  assert.equal(sha256(finalBytes), sha256(bytes), 'selected output hash mismatch: ' + relative);
  copied.push({ source: sourceName, path: relative, bytes: bytes.byteLength, sha256: sha256(bytes) });
}
function copyTree(sourceRoot, routePrefix, sourceLabel) {
  function visit(directory, relative) {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const absolute = path.join(directory, entry.name);
      const child = path.posix.join(relative, entry.name);
      const stat = fs.lstatSync(absolute);
      assert.ok(!stat.isSymbolicLink(), 'symlink in selected tree: ' + absolute);
      if (stat.isDirectory()) visit(absolute, child);
      else {
        assert.ok(stat.isFile(), 'unsupported selected entry: ' + absolute);
        copyOne(absolute, path.posix.join(routePrefix, child), path.posix.join(sourceLabel, child));
      }
    }
  }
  visit(sourceRoot, '');
}
copyTree(rspressOutput, '', path.relative(repositoryRoot, rspressOutput));
copyOne(path.join(selectedRoot, 'roadmap.html'), 'roadmap.html', path.relative(repositoryRoot, path.join(selectedRoot, 'roadmap.html')));
copyTree(path.join(selectedRoot, 'public'), '', path.relative(repositoryRoot, path.join(selectedRoot, 'public')));

function copyDocRootImages(directory, relative) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const absolute = path.join(directory, entry.name);
    const child = path.posix.join(relative, entry.name);
    const stat = fs.lstatSync(absolute);
    assert.ok(!stat.isSymbolicLink(), 'symlink in selected docs: ' + absolute);
    if (stat.isDirectory()) {
      if (entry.name !== 'public') copyDocRootImages(absolute, child);
    } else if (stat.isFile() && ['.png', '.jpg', '.jpeg', '.webp', '.gif'].includes(path.extname(entry.name).toLowerCase())) {
      copyOne(absolute, child, path.relative(repositoryRoot, absolute));
    }
  }
}
copyDocRootImages(selectedRoot, '');

const report = {
  selectedOutput: path.relative(repositoryRoot, selectedOutput),
  rspressOutputRelative: path.relative(repositoryRoot, rspressOutput),
  rspressOutputFileCount: copied.filter((entry) => entry.source.startsWith(path.relative(repositoryRoot, rspressOutput) + path.sep)).length,
  selectedRouteCount: copied.length,
  roadmapCopiedAsStaticHtml: true,
  linkedEvidenceAssetsCopied: true,
  noDeletes: true,
  routes: copied,
};
fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({
  selectedOutput: report.selectedOutput,
  selectedRouteCount: copied.length,
  roadmapCopiedAsStaticHtml: true,
  noDeletes: true,
}, null, 2) + '\n');
