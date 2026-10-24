import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c114-cross-axis-merge-roadmap-20261025-selected-site-pass03');
const staticRoot = path.join(sourceRoot, 'static');
const manifestPath = path.join(evidenceDirectory, 'selected-static-assets-pass03.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(staticRoot).isDirectory(), 'selected Rspress static output is required');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite static asset manifest');

const routes = [];
function visit(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))) {
    const absolute = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in static output: ${absolute}`);
    if (entry.isDirectory()) visit(absolute);
    else {
      assert.ok(entry.isFile(), `unsupported static output entry: ${absolute}`);
      const relative = path.relative(sourceRoot, absolute).split(path.sep).join('/');
      const bytes = fs.readFileSync(absolute);
      routes.push({ path: relative, bytes: bytes.byteLength, sha256: sha256(bytes) });
    }
  }
}
visit(staticRoot);
assert.ok(routes.length > 0, 'selected Rspress static output is empty');
const manifest = {
  sourceRoot: path.relative(repositoryRoot, sourceRoot),
  staticRouteCount: routes.length,
  routes,
  noDeletes: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ sourceRoot: manifest.sourceRoot, staticRouteCount: manifest.staticRouteCount, noDeletes: true }, null, 2)}\n`);
