import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-mixed-span-plan-docs-root-pass04');
const preparationPath = path.join(evidenceDirectory, 'selected-root-preparation-pass04.json');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace the selected docs root');
assert.ok(!fs.existsSync(preparationPath), 'refusing to replace the preparation record');

const copies = [
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-4-cyclic-minmax-calc-cross-axis-span-groups.md', 'plan/c11-4-cyclic-minmax-calc-cross-axis-span-groups.md'],
  ['plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md', 'plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md'],
  ['plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-plan-review-20.md', 'plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-plan-review-20.md'],
  ['spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/plan-pr-diff-review-30.md', 'internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/plan-pr-diff-review-30.md'],
  ['plan/c11-4-grid-track-sizing.md', 'spec/plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-4-cyclic-minmax-calc-cross-axis-span-groups.md', 'spec/plan/c11-4-cyclic-minmax-calc-cross-axis-span-groups.md'],
  ['plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md', 'spec/plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md'],
  ['plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-plan-review-20.md', 'spec/plan/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-plan-review-20.md'],
  ['spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/plan-pr-diff-review-30.md', 'spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/plan-pr-diff-review-30.md'],
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['spec/public/roadmap.html', 'spec/public/roadmap.html'],
  ['spec/public/icon.svg', 'public/icon.svg'],
  ['spec/public/logo.svg', 'public/logo.svg'],
  ['spec/public/roadmap.svg', 'public/roadmap.svg'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md'],
  ['spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/README.md', 'internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/README.md'],
  ['spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/README.md', 'spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/README.md'],
  ['spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/chrome156-mixed-span-groups.png', 'internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/chrome156-mixed-span-groups.png'],
  ['spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/chrome156-mixed-span-groups.png', 'spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/chrome156-mixed-span-groups.png'],
  ['spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/chrome156-observation.json', 'internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-precomparison-2026-10-22/chrome156-observation.json'],
  ['spec/internal/evidence/c11-4-cross-axis-span-groups-runtime-2026-10-22/README.md', 'spec/internal/evidence/c11-4-cross-axis-span-groups-runtime-2026-10-22/README.md'],
  ['spec/internal/evidence/c11-4-cross-axis-span-groups-runtime-2026-10-22/android-avd-c114-cross-axis-span-groups-android-avd-20261022T094504492Z-be93d82d/definite-minmax-calc-cross-axis-span-groups-forward.png', 'spec/internal/evidence/c11-4-cross-axis-span-groups-runtime-2026-10-22/android-avd-c114-cross-axis-span-groups-android-avd-20261022T094504492Z-be93d82d/definite-minmax-calc-cross-axis-span-groups-forward.png'],
  ['spec/internal/evidence/c11-4-cross-axis-span-groups-runtime-2026-10-22/ios-simulator-c114-cross-axis-span-groups-ios-simulator-20261022T094650728Z-f1702aac/definite-minmax-calc-cross-axis-span-groups-forward.png', 'spec/internal/evidence/c11-4-cross-axis-span-groups-runtime-2026-10-22/ios-simulator-c114-cross-axis-span-groups-ios-simulator-20261022T094650728Z-f1702aac/definite-minmax-calc-cross-axis-span-groups-forward.png'],
];

const copiedFiles = [];
for (const [sourceRelative, targetRelative] of copies) {
  const sourcePath = path.join(repositoryRoot, sourceRelative);
  const targetPath = path.join(selectedRoot, targetRelative);
  assert.ok(fs.statSync(sourcePath).isFile(), `source file is missing: ${sourceRelative}`);
  fs.mkdirSync(path.dirname(targetPath), { recursive: true });
  fs.copyFileSync(sourcePath, targetPath, fs.constants.COPYFILE_EXCL);
  copiedFiles.push({
    source: sourceRelative,
    target: targetRelative,
    sha256: crypto.createHash('sha256').update(fs.readFileSync(sourcePath)).digest('hex'),
  });
}

const preparation = {
  purpose: 'Isolated RSPress compile for the mixed-span plan pages and roadmap.',
  sourceHead: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  copiedFiles,
  omittedRepositoryContent: 'Other Markdown and untracked evidence are excluded to avoid unrelated Shiki, asset, and legacy dead-link failures.',
};
fs.writeFileSync(preparationPath, `${JSON.stringify(preparation, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(preparation, null, 2)}\n`);
