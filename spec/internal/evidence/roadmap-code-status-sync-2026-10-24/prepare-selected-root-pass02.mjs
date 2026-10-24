import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-root-pass01-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-root-pass02-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-site-pass02-selected');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pass02.json');
const baseHead = 'af23eded0853732379b2f46a1d72b74ba6bfe8f4';
const currentHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.equal(execFileSync('git', ['branch', '--show-current'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), 'plan/c11-4-negative-percentage-neighbor-20261024');
assert.ok(fs.lstatSync(baseRoot).isDirectory(), 'pass01 selected root is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace pass02 selected root');
assert.ok(!fs.existsSync(outputRoot), 'refusing to replace pass02 build output');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite pass02 manifest');

const sourceFiles = [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'plan/c11-4-auto-repeat-calc-neighbors.md',
  'plan/c11-4-auto-repeat-negative-percentage-neighbors.md',
  'plan/c11-4-auto-repeat-negative-percentage-neighbors-plan-review-20.md',
  'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.md',
  'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json',
  'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/validate-precomparison.py',
  'spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-full.png',
  'tests/fixtures/css/c11/grid-auto-repeat-negative-percentage-neighbors-c114-precomparison.html',
];

function hardlinkTree(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const sourcePath = path.join(source, entry.name);
    const targetPath = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in base selected root: ${sourcePath}`);
    if (entry.isDirectory()) hardlinkTree(sourcePath, targetPath);
    else if (entry.isFile()) fs.linkSync(sourcePath, targetPath);
    else throw new Error(`unsupported base selected root entry: ${sourcePath}`);
  }
}

function aliases(relative) {
  if (relative === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html'];
  const targets = new Set([relative]);
  if (relative.startsWith('plan/')) targets.add(`spec/${relative}`);
  if (relative === 'spec/STATUS.md') targets.add('STATUS.md');
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    targets.add(withoutSpec);
    targets.add(`public/${withoutSpec}`);
    targets.add(`public/${relative}`);
  }
  if (relative.startsWith('tests/')) targets.add(`public/${relative}`);
  return [...targets];
}

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of sourceFiles) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe source path: ${relative}`);
  const bytes = fs.readFileSync(source);
  for (const alias of aliases(relative)) {
    const destination = path.join(selectedRoot, alias);
    if (fs.existsSync(destination)) {
      const previous = fs.lstatSync(destination);
      assert.ok(previous.isFile() && !previous.isSymbolicLink(), `unsafe overlay target: ${alias}`);
      fs.unlinkSync(destination);
    }
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    const copied = fs.readFileSync(destination);
    assert.equal(sha256(copied), sha256(bytes), `overlay hash mismatch: ${alias}`);
    overlays.push({ source: relative, path: alias, bytes: copied.length, sha256: sha256(copied) });
  }
}

const manifest = {
  pass: 'pass02',
  baseHead,
  currentHead,
  baseSelectedRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  sourceFiles,
  overlays,
  storageMethod: 'hardlink previous selected tree; unlink overlay destinations before copying current source bytes',
  noDeletes: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });

const configPath = path.join(evidenceDirectory, 'rspress-pass02.config.ts');
assert.ok(!fs.existsSync(configPath), 'refusing to overwrite pass02 Rspress config');
const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-root-pass02-selected'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-site-pass02-selected'),
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
process.stdout.write(`${JSON.stringify({ pass: 'pass02', currentHead, sourceFileCount: sourceFiles.length, overlayCount: overlays.length, noDeletes: true }, null, 2)}\n`);
