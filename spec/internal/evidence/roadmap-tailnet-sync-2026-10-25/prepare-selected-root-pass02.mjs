import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-root-pass05-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-root-pass02-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-site-pass02-full');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pass02.json');
const configPath = path.join(evidenceDirectory, 'rspress-pass02.config.ts');
const expectedBranch = 'docs/roadmap-tailnet-sync-2026-10-25';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.equal(execFileSync('git', ['branch', '--show-current'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), expectedBranch);
assert.equal(execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim());
assert.ok(fs.lstatSync(baseRoot).isDirectory(), 'previous selected source root is required');
for (const candidate of [selectedRoot, outputRoot, manifestPath, configPath]) assert.ok(!fs.existsSync(candidate), `refusing to replace ${candidate}`);

function hardlinkTree(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const sourcePath = path.join(source, entry.name);
    const targetPath = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in base source root: ${sourcePath}`);
    if (entry.isDirectory()) hardlinkTree(sourcePath, targetPath);
    else if (entry.isFile()) fs.linkSync(sourcePath, targetPath);
    else throw new Error(`unsupported base source entry: ${sourcePath}`);
  }
}

function aliases(relative) {
  if (relative === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html', relative];
  const targets = new Set([relative]);
  if (relative === 'spec/STATUS.md') targets.add('STATUS.md');
  if (relative.startsWith('plan/')) targets.add(`spec/${relative}`);
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    targets.add(withoutSpec);
    targets.add(`public/${withoutSpec}`);
    targets.add(`public/${relative}`);
  }
  return [...targets];
}

function walkFiles(relativeDirectory) {
  const absolute = path.join(repositoryRoot, relativeDirectory);
  const stat = fs.lstatSync(absolute);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe source directory: ${relativeDirectory}`);
  const result = [];
  for (const entry of fs.readdirSync(absolute, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const childRelative = path.posix.join(relativeDirectory, entry.name);
    const childAbsolute = path.join(repositoryRoot, childRelative);
    assert.ok(!entry.isSymbolicLink(), `symlink in source evidence: ${childRelative}`);
    if (entry.isDirectory()) result.push(...walkFiles(childRelative));
    else if (entry.isFile()) result.push(childRelative);
    else throw new Error(`unsupported source evidence entry: ${childRelative}`);
  }
  return result;
}

const sourceFiles = new Set([
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'plan/c11-3-3-unknown-named-area-implicit-lines.md',
  'plan/c11-4-fixed-repeat-minmax-flex.md',
  'plan/c11-4-grid-track-sizing.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/internal/evidence/roadmap-code-status-sync-2026-10-25/pr-diff-review-30.md',
  'spec/internal/evidence/c11-4-pr615-merge-status-2026-10-25/pr-diff-review-30.md',
  'spec/internal/evidence/c11-3-3-unknown-area-implicit-lines-followup-2026-10-21/README.md',
  'spec/internal/evidence/c11-3-3-unknown-area-implicit-lines-followup-2026-10-21/pr-diff-review-30.md',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/README.md',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/README.md',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/implementation-review-20.md',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/pr-diff-review-30.md',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/artifact-boundary/README.md',
]);
for (const directory of [
  'spec/internal/evidence/roadmap-code-status-sync-2026-10-25',
  'spec/internal/evidence/c11-4-pr615-merge-status-2026-10-25',
  'spec/internal/evidence/c11-3-3-unknown-area-implicit-lines-followup-2026-10-21',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25',
  'spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25',
]) for (const file of walkFiles(directory)) sourceFiles.add(file);

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of [...sourceFiles].sort()) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe selected source: ${relative}`);
  const bytes = fs.readFileSync(source);
  for (const alias of aliases(relative)) {
    const destination = path.join(selectedRoot, alias);
    if (fs.existsSync(destination)) {
      const previous = fs.lstatSync(destination);
      assert.ok(previous.isFile() && !previous.isSymbolicLink(), `unsafe selected overlay: ${alias}`);
      fs.unlinkSync(destination);
    }
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    const copied = fs.readFileSync(destination);
    assert.equal(sha256(copied), sha256(bytes), `overlay hash mismatch: ${alias}`);
    overlays.push({ source: relative, path: alias, bytes: copied.length, sha256: sha256(copied) });
  }
}

const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-root-pass02-selected'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-20261025-site-pass02-full'),
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

const manifest = {
  mergedHead: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(),
  baseSourceRoot: path.relative(repositoryRoot, baseRoot),
  selectedSourceRoot: path.relative(repositoryRoot, selectedRoot),
  fullOutputRoot: path.relative(repositoryRoot, outputRoot),
  sourceFileCount: sourceFiles.size,
  overlays,
  storageMethod: 'hardlinked prior selected root with atomic source overlays; no deletes',
  noDeletes: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, overlays: undefined }, null, 2)}\n`);
