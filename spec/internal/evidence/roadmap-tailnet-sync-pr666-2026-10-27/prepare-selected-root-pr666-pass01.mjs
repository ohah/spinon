import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const expectedHead = 'fb3f1e5a307c9678d6b70af017858fe7e03f435a';
const baseRoot = '/Users/yoonhb/Documents/workspace/spinon/build/spinon/roadmap-tailnet-sync-20261025-root-pass08-corrected-selected';
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr666-20261027-root-pass01-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr666-20261027-site-pass01-selected');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pr666-pass01.json');
const configPath = path.join(evidenceDirectory, 'rspress-pr666-pass01-selected.config.ts');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sourcePaths = [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'spec/internal/0059-c11-grid-layout.md',
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-fit-content-auto-neighbor-runtime.md',
  'plan/c11-4-fit-content-auto-neighbor-wpt.md',
  'plan/c11-4-fit-content-auto-neighbor-runtime-plan-review-20.md',
  'plan/c11-4-fit-content-auto-neighbor-runtime-plan-pr-diff-review-30.md',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-wpt-2026-10-26',
  'spec/internal/evidence/c11-4-pr664-public-roadmap-update-2026-10-27',
  'spec/internal/evidence/c11-4-rspress-relative-image-path-fix-2026-10-27',
];

const mainHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mainTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const checkoutHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const checkoutTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(mainHead, expectedHead, 'origin/main is not the merged PR #666 source revision');
assert.equal(checkoutHead, mainHead, 'checkout HEAD differs from origin/main');
assert.equal(checkoutTree, mainTree, 'checkout tree differs from origin/main');
assert.ok(fs.lstatSync(baseRoot).isDirectory() && !fs.lstatSync(baseRoot).isSymbolicLink(), 'previous selected root is missing or unsafe');
for (const candidate of [selectedRoot, outputRoot, manifestPath, configPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);

const trackedFiles = execFileSync('git', ['ls-tree', '-r', '--name-only', mainHead, '--', ...sourcePaths], { cwd: repositoryRoot, encoding: 'utf8' }).trim().split('\n').filter(Boolean);
for (const required of [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/README.md',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/android-avd-c114-fit-content-auto-neighbor-android-avd-20261026T190540661Z-27b72972/fit-content-auto-neighbor-wpt.png',
  'spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/ios-simulator-c114-fit-content-auto-neighbor-ios-simulator-20261026T190530681Z-85dd0569/fit-content-auto-neighbor-wpt.png',
  'spec/internal/evidence/c11-4-rspress-relative-image-path-fix-2026-10-27/pr-diff-review-30.md',
]) assert.ok(trackedFiles.includes(required), `required merged source is missing: ${required}`);

function hardlinkTree(source, destination) {
  fs.mkdirSync(destination, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const from = path.join(source, entry.name);
    const to = path.join(destination, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected-site base: ${from}`);
    if (entry.isDirectory()) hardlinkTree(from, to);
    else if (entry.isFile()) fs.linkSync(from, to);
    else throw new Error(`unsupported entry in selected-site base: ${from}`);
  }
}

function aliases(relative) {
  if (relative === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html', relative];
  if (relative === 'spec/STATUS.md') return ['STATUS.md', relative];
  if (relative === 'spec/internal/0059-c11-grid-layout.md') return [relative, 'internal/0059-c11-grid-layout.md', 'public/internal/0059-c11-grid-layout.md', `public/${relative}`];
  if (relative.startsWith('plan/')) return [relative, `spec/${relative}`];
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    return [...new Set([relative, withoutSpec, `public/${withoutSpec}`, `public/${relative}`])];
  }
  throw new Error(`no selected-root alias rule for ${relative}`);
}

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of trackedFiles) {
  const localPath = path.join(repositoryRoot, relative);
  const localBytes = fs.readFileSync(localPath);
  const mainBytes = execFileSync('git', ['show', `${mainHead}:${relative}`], { cwd: repositoryRoot });
  assert.deepEqual(localBytes, mainBytes, `working source differs from merged main: ${relative}`);
  for (const route of aliases(relative)) {
    const destination = path.join(selectedRoot, route);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    if (fs.existsSync(destination)) {
      const stat = fs.lstatSync(destination);
      assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe selected-root overlay target: ${route}`);
      fs.unlinkSync(destination);
    }
    fs.writeFileSync(destination, localBytes, { flag: 'wx' });
    const copied = fs.readFileSync(destination);
    assert.equal(sha256(copied), sha256(localBytes), `selected-root overlay hash mismatch: ${route}`);
    overlays.push({ source: relative, path: route, bytes: copied.length, sha256: sha256(copied) });
  }
}

const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr666-20261027-root-pass01-selected'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr666-20261027-site-pass01-selected'),
  builderConfig: {
    ...baseConfig.builderConfig,
    server: { ...baseConfig.builderConfig?.server, publicDir: false },
    output: { ...baseConfig.builderConfig?.output, cleanDistPath: true },
  },
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
`;
fs.writeFileSync(configPath, config, { flag: 'wx' });

const report = {
  pass: 'pass01',
  sourceHead: mainHead,
  sourceTree: mainTree,
  checkoutHead,
  checkoutTree,
  baseRoot,
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  sourceDirectories: sourcePaths,
  trackedSourceCount: trackedFiles.length,
  overlayCount: overlays.length,
  overlayBytes: overlays.reduce((sum, entry) => sum + entry.bytes, 0),
  hardlinkBasePolicy: '이전 selected root는 읽기 전용 입력이다. 새 selected root에서 교체 전 overlay 파일은 unlink 후 재작성하므로 base inode 내용은 바뀌지 않는다.',
  noDeletes: true,
  overlays,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, overlays: undefined }, null, 2)}\n`);
