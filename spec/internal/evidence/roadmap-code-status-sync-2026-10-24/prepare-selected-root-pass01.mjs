import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-root-pass70-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-root-pass01-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-site-pass01-selected');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const originMain = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.ok(fs.lstatSync(baseRoot).isDirectory(), 'existing pass70 selected root is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to reuse selected root');
assert.ok(!fs.existsSync(outputRoot), 'refusing to reuse build output');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite manifest');

const plans = [
  'c11-4-auto-fit-named-line-implicit-fallback.md',
  'c11-4-auto-repeat-named-line-implicit-fallback.md',
  'c11-4-auto-repeat-named-lines.md',
  'c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups.md',
  'c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md',
  'c11-4-cyclic-minmax-calc-cross-axis-span-groups.md',
  'c11-4-cyclic-minmax-calc-minimum-span.md',
  'c11-4-cyclic-minmax-calc-overlapping-rows.md',
  'c11-4-cyclic-minmax-calc-overlapping-spans.md',
  'c11-4-cyclic-minmax-calc-two-axis-spanning-text.md',
  'c11-4-cyclic-minmax-max-content-mixed-text-image-span.md',
  'c11-4-cyclic-minmax-mixed-text-image-span.md',
  'c11-4-cyclic-minmax-replaced-image-span.md',
  'c11-4-two-axis-template-areas.md',
].map((name) => `plan/${name}`);
const files = [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  ...plans,
  'spec/internal/evidence/roadmap-code-status-sync-2026-10-24/README.md',
  'spec/internal/evidence/roadmap-code-status-sync-2026-10-24/pr-diff-review-20.md',
];

function hardlinkTree(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const sourcePath = path.join(source, entry.name);
    const targetPath = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in base root: ${sourcePath}`);
    if (entry.isDirectory()) hardlinkTree(sourcePath, targetPath);
    else if (entry.isFile()) fs.linkSync(sourcePath, targetPath);
    else throw new Error(`unsupported base entry: ${sourcePath}`);
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
  return [...targets];
}

function overlay(relative, bytes, records) {
  for (const alias of aliases(relative)) {
    const destination = path.join(selectedRoot, alias);
    if (fs.existsSync(destination)) {
      const stat = fs.lstatSync(destination);
      assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe overlay target: ${alias}`);
      fs.unlinkSync(destination);
    }
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    records.push({ source: relative, path: alias, bytes: bytes.length, sha256: sha256(bytes) });
  }
}

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of files) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe source file: ${relative}`);
  overlay(relative, fs.readFileSync(source), overlays);
}

const manifest = {
  pass: 'pass01',
  baseSelectedRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  head,
  originMain,
  sourceFiles: files,
  overlays,
  storageMethod: 'hardlink the previous selected root; unlink each overlay target before writing the current source',
  noDeletes: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });

const configPath = path.join(evidenceDirectory, 'rspress-pass01.config.ts');
assert.ok(!fs.existsSync(configPath), 'refusing to overwrite Rspress config');
const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-root-pass01-selected'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-site-pass01-selected'),
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
process.stdout.write(`${JSON.stringify({ pass: manifest.pass, head, originMain, sourceFiles: files.length, overlayCopies: overlays.length, noDeletes: true }, null, 2)}\n`);
