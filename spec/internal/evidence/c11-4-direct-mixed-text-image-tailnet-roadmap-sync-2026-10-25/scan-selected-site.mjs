import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c114-direct-mixed-tailnet-selected-site-20261025');
const manifestPath = path.join(evidenceDirectory, 'selected-build-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite selected build manifest');
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected Rspress site output is required');

const routes = [];
function visit(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const absolute = path.join(directory, entry.name);
    const stat = fs.lstatSync(absolute);
    assert.ok(!stat.isSymbolicLink(), 'symlink in selected output: ' + absolute);
    if (stat.isDirectory()) visit(absolute);
    else {
      assert.ok(stat.isFile(), 'unsupported selected output entry: ' + absolute);
      const relative = path.relative(sourceRoot, absolute).split(path.sep).join('/');
      const bytes = fs.readFileSync(absolute);
      routes.push({ path: relative, bytes: bytes.byteLength, sha256: sha256(bytes) });
    }
  }
}
visit(sourceRoot);
assert.ok(routes.length > 0, 'selected site output is empty');
const manifest = {
  selectedOutputRoot: path.relative(repositoryRoot, sourceRoot),
  selectedRouteCount: routes.length,
  selectedRoutes: routes,
  sourceMergeCommit: '25ecb4f2ced6442f85e09bcdbc33106c02beaa3b',
  officialCompletionCount: '72/213',
  noDeletes: true,
};
fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({ selectedOutputRoot: manifest.selectedOutputRoot, selectedRouteCount: routes.length, noDeletes: true }, null, 2) + '\n');
