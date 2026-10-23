import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-percentage-neighbors-roadmap-sync-20261024-root-pass02-selected');
const sourceRelative = 'spec/internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/README.md';
const sourcePath = path.join(repositoryRoot, sourceRelative);
const manifestPath = path.join(evidenceDirectory, 'selected-root-manifest-pass02.json');
const finalManifestPath = path.join(evidenceDirectory, 'selected-root-manifest-pass02-final.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sourceBytes = fs.readFileSync(sourcePath);
const oldManifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
assert.ok(!fs.existsSync(finalManifestPath), 'refusing to overwrite final manifest');

const aliases = [
  sourceRelative,
  'internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/README.md',
  'public/internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/README.md',
  'public/spec/internal/evidence/c11-4-percentage-neighbors-roadmap-sync-2026-10-24/README.md',
];
const refreshed = [];
for (const relative of aliases) {
  const target = path.join(selectedRoot, relative);
  const stat = fs.lstatSync(target);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe selected page: ${relative}`);
  const prior = oldManifest.overlays.find((item) => item.source === sourceRelative && item.path === relative);
  assert.ok(prior, `manifest is missing prior overlay: ${relative}`);
  assert.equal(sha256(fs.readFileSync(target)), prior.sha256, `selected page changed outside this refresh: ${relative}`);
  const temporary = `${target}.refresh-${randomUUID()}.tmp`;
  try {
    fs.writeFileSync(temporary, sourceBytes, { flag: 'wx' });
    fs.renameSync(temporary, target);
  } catch (error) {
    if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
    throw error;
  }
  refreshed.push({ source: sourceRelative, path: relative, bytes: sourceBytes.length, sha256: sha256(sourceBytes) });
}

const finalManifest = {
  ...oldManifest,
  overlays: oldManifest.overlays.map((item) => item.source === sourceRelative
    ? { ...item, bytes: sourceBytes.length, sha256: sha256(sourceBytes) }
    : item),
  refreshedSourceFiles: refreshed,
  refreshMethod: 'verify old overlay hash; write unique sibling temp; atomic rename each selected copy',
};
fs.writeFileSync(finalManifestPath, `${JSON.stringify(finalManifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ refreshedCopies: refreshed.length, sha256: sha256(sourceBytes), finalManifest: path.basename(finalManifestPath) }, null, 2)}\n`);
