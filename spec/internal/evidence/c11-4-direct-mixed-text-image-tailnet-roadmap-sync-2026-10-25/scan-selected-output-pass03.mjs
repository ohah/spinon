import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c114-direct-mixed-tailnet-selected-output-20261025-pass03');
const manifestPath = path.join(evidenceDirectory, 'selected-output-pass03.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite final selected output manifest');
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'final selected output is required');
const routes = [];
function visit(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const absolute = path.join(directory, entry.name);
    const stat = fs.lstatSync(absolute);
    assert.ok(!stat.isSymbolicLink(), 'symlink in final output: ' + absolute);
    if (stat.isDirectory()) visit(absolute);
    else {
      assert.ok(stat.isFile(), 'unsupported final output entry: ' + absolute);
      const relative = path.relative(sourceRoot, absolute).split(path.sep).join('/');
      const bytes = fs.readFileSync(absolute);
      routes.push({ path: relative, bytes: bytes.byteLength, sha256: sha256(bytes) });
    }
  }
}
visit(sourceRoot);
assert.ok(routes.length > 0, 'final selected output is empty');
for (const required of [
  'roadmap.html',
  'STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-4-mixed-direct-text-image-intrinsic.html',
  'internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/README.html',
  'internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/runs/c114-direct-mixed-android-avd-20261024T214623162Z-49f60916/android-direct-max-content-forward.png',
  'internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/runs/c114-direct-mixed-ios-simulator-20261024T214638568Z-954b507d/ios-direct-max-content-forward.png',
  'internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/post-merge-main/runtime-focused.log',
  'internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/post-merge-main/ffi-focused.log',
]) assert.ok(routes.some((entry) => entry.path === required), 'missing required route: ' + required);
const manifest = {
  selectedOutputRoot: path.relative(repositoryRoot, sourceRoot),
  selectedRouteCount: routes.length,
  selectedRoutes: routes,
  sourceMergeCommit: '25ecb4f2ced6442f85e09bcdbc33106c02beaa3b',
  implementationMergeCommit: '2c4c278f0facd31ac3e0cc60dce1f27575628b35',
  officialCompletionCount: '72/213',
  noDeletes: true,
};
fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({
  selectedOutputRoot: manifest.selectedOutputRoot,
  selectedRouteCount: routes.length,
  requiredRouteCount: 9,
  noDeletes: true,
}, null, 2) + '\n');
