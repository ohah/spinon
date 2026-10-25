import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-root-pass02-selected');
const reportPath = path.join(evidenceDirectory, 'selected-root-repair-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(selectedRoot).isDirectory(), 'initial selected root is required');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite repair manifest');

const sources = [
  'spec/internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-21/android-avd-c1133-matrix-20261020T150911869Z-7abbeeff/unknown-area-overflow-pixelcopy.png',
  'spec/internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-21/android-physical-c1133-matrix-20261020T152039508Z-5886c27a/unknown-area-overflow-pixelcopy.png',
  'spec/internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-21/ios-simulator-c1133-ios-matrix-20261020T151033155Z-4d6bf988/ios-unknown-area-overflow-screen.png',
];

function aliases(relative) {
  const withoutSpec = relative.slice('spec/'.length);
  return [relative, withoutSpec, `public/${withoutSpec}`, `public/${relative}`];
}

const entries = [];
for (const relative of sources) {
  const source = path.join(repositoryRoot, relative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `unsafe image source: ${relative}`);
  const sourceBytes = fs.readFileSync(source);
  for (const alias of aliases(relative)) {
    const destination = path.join(selectedRoot, alias);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    if (fs.existsSync(destination)) {
      const currentStat = fs.lstatSync(destination);
      assert.ok(currentStat.isFile() && !currentStat.isSymbolicLink(), `unsafe existing route: ${alias}`);
      assert.equal(sha256(fs.readFileSync(destination)), sha256(sourceBytes), `existing route has different content: ${alias}`);
    } else {
      fs.linkSync(source, destination);
    }
    entries.push({ source: relative, route: alias, bytes: sourceBytes.length, sha256: sha256(sourceBytes), linked: true });
  }
}

const report = { reason: 'Rspress image imports from the C11.3.3 evidence page require three sibling PNG assets', entries, noDeletes: true };
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ entryCount: entries.length, noDeletes: true }, null, 2)}\n`);
