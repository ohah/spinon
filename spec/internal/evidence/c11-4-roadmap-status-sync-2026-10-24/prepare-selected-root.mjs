import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-root-pass70-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-status-sync-pass01-root-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-status-sync-pass01-site-selected');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const remoteMain = execFileSync('git', ['rev-parse', 'FETCH_HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();

assert.equal(head, remoteMain, 'selected preview base must match fetched main');
assert.ok(fs.lstatSync(baseRoot).isDirectory(), 'known-good selected base root is missing');
for (const target of [selectedRoot, outputRoot, path.join(evidenceDirectory, 'selected-root.json')]) {
  assert.ok(!fs.existsSync(target), `refusing to reuse ${target}`);
}

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
  const targets = new Set([relative]);
  if (relative.startsWith('plan/')) targets.add(`spec/${relative}`);
  if (relative === 'spec/STATUS.md') targets.add('STATUS.md');
  if (relative === 'spec/public/roadmap.html') targets.add('roadmap.html');
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

const files = [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-auto-repeat-zero-percent-minimum.md',
  'plan/c11-4-auto-repeat-zero-percent-minimum-plan-review-20.md',
];
const evidenceRelative = 'spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24';
const evidenceAbsolute = path.join(repositoryRoot, evidenceRelative);
for (const entry of fs.readdirSync(evidenceAbsolute, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
  assert.ok(entry.isFile() && !entry.isSymbolicLink(), `unexpected evidence entry: ${entry.name}`);
  files.push(`${evidenceRelative}/${entry.name}`);
}

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of files) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe source file: ${relative}`);
  overlay(relative, fs.readFileSync(source), overlays);
}

const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-status-sync-pass01-root-selected'),
  outDir: path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-status-sync-pass01-site-selected'),
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
const configPath = path.join(evidenceDirectory, 'rspress.config.ts');
fs.writeFileSync(configPath, config, { flag: 'wx' });

const report = {
  pass: 'pass01',
  head,
  fetchedMain: remoteMain,
  baseRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  overlaySourceFiles: files,
  overlays,
  storageMethod: 'hardlink the known-good selected root; unlink each overlay destination before writing its isolated copy',
  deadLinkChecks: 'disabled only for this isolated preview build because repository-wide existing dead links are documented',
  noDeletes: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root.json'), `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ pass: report.pass, head, overlaySourceFiles: files.length, overlayCopies: overlays.length, noDeletes: true }, null, 2)}\n`);
