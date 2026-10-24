import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c114-cross-axis-merge-roadmap-20261025-selected-root-pass02');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pass02.json');
const prefix = 'spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25';
const sourceFiles = [
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/public/icon.svg', 'public/icon.svg'],
  ['spec/public/logo.svg', 'public/logo.svg'],
  ['plan/c11-4-cross-axis-fixed-auto-repeat-areas.md', 'plan/c11-4-cross-axis-fixed-auto-repeat-areas.md'],
  ['plan/c11-4-cross-axis-fixed-auto-repeat-areas-plan-review-20.md', 'plan/c11-4-cross-axis-fixed-auto-repeat-areas-plan-review-20.md'],
  ['plan/c11-4-cross-axis-fixed-auto-repeat-areas-plan-pr-diff-review-20.md', 'plan/c11-4-cross-axis-fixed-auto-repeat-areas-plan-pr-diff-review-20.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/README.md', 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/README.md'],
  ['spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/chrome-agent-browser.png', 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/chrome-agent-browser.png'],
  [`${prefix}/README.md`, 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/README.md'],
  [`${prefix}/implementation-runtime-review-20.md`, 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/implementation-runtime-review-20.md'],
  [`${prefix}/pr-diff-review-30.md`, 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/pr-diff-review-30.md'],
  ['spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/post-merge-roadmap-pr-diff-review-30.md', 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/post-merge-roadmap-pr-diff-review-30.md'],
  [`${prefix}/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png`, 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png'],
  [`${prefix}/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png`, 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fit-rows-empty-area.png'],
  [`${prefix}/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png`, 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/android-avd-c114-cross-axis-fixed-auto-area-android-avd-20261024T184030545Z-2f33a429/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png'],
  [`${prefix}/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png`, 'internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/ios-simulator-c114-cross-axis-fixed-auto-area-ios-simulator-20261024T184055107Z-572dbc3b/template-area-repeat-cross-axis-fixed-cols-fill-rows-area-adds-two-rows.png'],
];
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace selected root');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite selected root manifest');
fs.mkdirSync(selectedRoot, { recursive: true });
const selected = [];
for (const [sourceRelative, pathRelative] of sourceFiles) {
  const source = path.join(repositoryRoot, sourceRelative);
  const sourceStat = fs.lstatSync(source);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `unsafe source: ${sourceRelative}`);
  const bytes = fs.readFileSync(source);
  const destination = path.join(selectedRoot, pathRelative);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes, { flag: 'wx' });
  const copied = fs.readFileSync(destination);
  assert.equal(sha256(copied), sha256(bytes), `selected root hash mismatch: ${pathRelative}`);
  selected.push({ source: sourceRelative, path: pathRelative, bytes: copied.byteLength, sha256: sha256(copied) });
}
const manifest = { selectedRoot: path.relative(repositoryRoot, selectedRoot), sourceFileCount: selected.length, selected, noDeletes: true };
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ selectedRoot: manifest.selectedRoot, sourceFileCount: selected.length, noDeletes: true }, null, 2)}\n`);
