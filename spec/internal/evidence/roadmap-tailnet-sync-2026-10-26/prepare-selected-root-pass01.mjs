import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = '/Users/yoonhb/Documents/workspace/spinon/build/spinon/roadmap-tailnet-sync-20261025-root-pass08-corrected-selected';
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261026-root-pass01-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261026-site-pass01-selected');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pass01.json');
const configPath = path.join(evidenceDirectory, 'rspress-pass01-selected.config.ts');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sourceFiles = [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'spec/internal/0059-c11-grid-layout.md',
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-stretch-auto-track-stage-oracle.md',
  'plan/c11-4-stretch-auto-track-stage-plan-review-20-2026-10-26.md',
  'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/README.md',
  'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/focused-test.log',
  'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/fmt-check.log',
  'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/clippy.log',
  'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/implementation-review-20.md',
  'spec/internal/evidence/c11-4-stretch-auto-track-stage-oracle-2026-10-26/pr-diff-review-30.md',
];

const mainHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mainTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const checkoutHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const checkoutTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(checkoutTree, mainTree, 'checkout tree differs from pinned origin/main');
assert.ok(fs.lstatSync(baseRoot).isDirectory() && !fs.lstatSync(baseRoot).isSymbolicLink(), 'pass08-corrected selected root is missing or unsafe');
for (const candidate of [selectedRoot, outputRoot, manifestPath, configPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);

const trackedMainFiles = new Set(execFileSync('git', ['ls-tree', '-r', '--name-only', mainHead, '--', ...sourceFiles], { cwd: repositoryRoot, encoding: 'utf8' }).trim().split('\n').filter(Boolean));
assert.equal(trackedMainFiles.size, sourceFiles.length, 'a selected source is absent from origin/main');
for (const relative of sourceFiles) {
  const localBytes = fs.readFileSync(path.join(repositoryRoot, relative));
  const mainBytes = execFileSync('git', ['show', `${mainHead}:${relative}`], { cwd: repositoryRoot });
  assert.deepEqual(localBytes, mainBytes, `selected source differs from origin/main: ${relative}`);
}

function hardlinkTree(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const from = path.join(source, entry.name);
    const to = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in base root: ${from}`);
    if (entry.isDirectory()) hardlinkTree(from, to);
    else if (entry.isFile()) fs.linkSync(from, to);
    else throw new Error(`unsupported base entry: ${from}`);
  }
}

function aliases(relative) {
  if (relative === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html', relative];
  const routes = new Set([relative]);
  if (relative === 'spec/STATUS.md') routes.add('STATUS.md');
  if (relative.startsWith('plan/')) routes.add(`spec/${relative}`);
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    routes.add(withoutSpec);
    routes.add(`public/${withoutSpec}`);
    routes.add(`public/${relative}`);
  }
  return [...routes];
}

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of sourceFiles) {
  const bytes = fs.readFileSync(path.join(repositoryRoot, relative));
  for (const route of aliases(relative)) {
    const destination = path.join(selectedRoot, route);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    if (fs.existsSync(destination)) {
      const stat = fs.lstatSync(destination);
      assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe overlay target: ${route}`);
      fs.unlinkSync(destination);
    }
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    assert.equal(sha256(fs.readFileSync(destination)), sha256(bytes), `overlay hash mismatch: ${route}`);
    overlays.push({ source: relative, path: route, bytes: bytes.length, sha256: sha256(bytes) });
  }
}

const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261026-root-pass01-selected'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261026-site-pass01-selected'),
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
  sourceFiles,
  overlayCount: overlays.length,
  overlayBytes: overlays.reduce((sum, entry) => sum + entry.bytes, 0),
  baseStorage: '이전 pass08-corrected root의 읽기 전용 hardlink 복사본이며 overlay는 새 경로만 교체한다',
  noDeletes: true,
  overlays,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, overlays: undefined }, null, 2)}\n`);
