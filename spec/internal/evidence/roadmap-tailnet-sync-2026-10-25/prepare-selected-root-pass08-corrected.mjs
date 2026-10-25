import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-root-pass07-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-root-pass08-corrected-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-site-pass08-corrected-full');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pass08-corrected.json');
const configPath = path.join(evidenceDirectory, 'rspress-pass08-corrected.config.ts');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

const checkoutHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const checkoutTree = execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mainHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const mainTree = execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.ok(fs.lstatSync(baseRoot).isDirectory(), 'pass07 selected source root is required');
for (const candidate of [selectedRoot, outputRoot, manifestPath, configPath]) {
  assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);
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

function walk(relativeDirectory) {
  const absolute = path.join(repositoryRoot, relativeDirectory);
  assert.ok(fs.lstatSync(absolute).isDirectory(), `source directory is missing: ${relativeDirectory}`);
  const files = [];
  for (const entry of fs.readdirSync(absolute, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = path.posix.join(relativeDirectory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in source directory: ${relative}`);
    if (entry.isDirectory()) files.push(...walk(relative));
    else if (entry.isFile()) files.push(relative);
    else throw new Error(`unsupported source entry: ${relative}`);
  }
  return files;
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

const sources = new Set([
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-cyclic-percentage-auto-repeat.md',
  'plan/c11-4-orthogonal-row-intrinsic-contribution.md',
]);
const runtimeEvidenceFiles = walk('spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25');
for (const relative of runtimeEvidenceFiles) sources.add(relative);
const trackedMainFiles = new Set(execFileSync('git', ['ls-tree', '-r', '--name-only', mainHead, '--', ...sources], { cwd: repositoryRoot, encoding: 'utf8' }).trim().split('\n').filter(Boolean));
const excludedUntrackedFiles = [...sources].filter((relative) => !trackedMainFiles.has(relative)).map((relative) => ({ path: relative, sha256: sha256(fs.readFileSync(path.join(repositoryRoot, relative))) }));
for (const relative of excludedUntrackedFiles.map((entry) => entry.path)) sources.delete(relative);
for (const relative of sources) {
  assert.ok(trackedMainFiles.has(relative), 'source is absent from origin/main: ' + relative);
  const localBytes = fs.readFileSync(path.join(repositoryRoot, relative));
  const mainBytes = execFileSync('git', ['show', mainHead + ':' + relative], { cwd: repositoryRoot });
  assert.deepEqual(localBytes, mainBytes, 'source differs from origin/main: ' + relative);
}

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of [...sources].sort()) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing/unsafe source: ${relative}`);
  const bytes = fs.readFileSync(source);
  for (const route of aliases(relative)) {
    const target = path.join(selectedRoot, route);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    if (fs.existsSync(target)) {
      const prior = fs.lstatSync(target);
      assert.ok(prior.isFile() && !prior.isSymbolicLink(), `unsafe overlay target: ${route}`);
      fs.unlinkSync(target);
    }
    fs.writeFileSync(target, bytes, { flag: 'wx' });
    assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `overlay hash mismatch: ${route}`);
    overlays.push({ source: relative, path: route, bytes: bytes.length, sha256: sha256(bytes) });
  }
}

const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-root-pass08-corrected-selected'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-site-pass08-corrected-full'),
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
  pass: 'pass08',
  sourceHead: mainHead,
  checkoutHead,
  originMainHead: mainHead,
  sourceTree: mainTree,
  checkoutTree,
  excludedUntrackedFiles,
  originMainTree: mainTree,
  baseSelectedRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  fullOutputRoot: path.relative(repositoryRoot, outputRoot),
  sourceFiles: sources.size,
  overlayCount: overlays.length,
  overlayBytes: overlays.reduce((sum, entry) => sum + entry.bytes, 0),
  storage: 'hard-linked pass07 source root plus independent current-main overlays; prior roots remain unchanged',
  noDeletes: true,
  overlays,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, overlays: undefined }, null, 2)}\n`);
