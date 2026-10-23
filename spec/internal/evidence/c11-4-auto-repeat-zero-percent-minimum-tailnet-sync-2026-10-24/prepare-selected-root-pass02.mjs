import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const baseRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-root-pass01-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-zero-percent-auto-repeat-minimum-root-pass02-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-zero-percent-auto-repeat-minimum-site-pass02-selected');
const manifestPath = path.join(evidenceDirectory, 'selected-root-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const originMain = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.ok(fs.lstatSync(baseRoot).isDirectory(), 'existing pass01 selected root is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to reuse selected root');
assert.ok(!fs.existsSync(outputRoot), 'refusing to reuse selected build output');
assert.ok(!fs.existsSync(manifestPath), 'refusing to overwrite selected-root manifest');

function filesUnder(directory) {
  const result = [];
  function visit(current) {
    for (const entry of fs.readdirSync(current, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const fullPath = path.join(current, entry.name);
      assert.ok(!entry.isSymbolicLink(), 'symlinks are not accepted in selected sources: ' + fullPath);
      if (entry.isDirectory()) visit(fullPath);
      else if (entry.isFile()) result.push(path.relative(repositoryRoot, fullPath).split(path.sep).join('/'));
      else throw new Error('unsupported source entry: ' + fullPath);
    }
  }
  visit(directory);
  return result;
}

const evidenceRoots = [
  path.join(repositoryRoot, 'spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24'),
  path.join(repositoryRoot, 'spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24'),
];
const fixedFiles = [
  'spec/STATUS.md',
  'spec/public/roadmap.html',
  'spec/internal/0059-c11-grid-layout.md',
  'plan/c11-4-auto-repeat-zero-percent-minimum.md',
];
const sourceFiles = [...new Set([...fixedFiles, ...evidenceRoots.flatMap(filesUnder)])].sort();

function hardlinkTree(source, target) {
  fs.mkdirSync(target, { recursive: true });
  for (const entry of fs.readdirSync(source, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const sourcePath = path.join(source, entry.name);
    const targetPath = path.join(target, entry.name);
    assert.ok(!entry.isSymbolicLink(), 'symlink in selected base: ' + sourcePath);
    if (entry.isDirectory()) hardlinkTree(sourcePath, targetPath);
    else if (entry.isFile()) fs.linkSync(sourcePath, targetPath);
    else throw new Error('unsupported selected-base entry: ' + sourcePath);
  }
}

function aliases(relative) {
  if (relative === 'spec/public/roadmap.html') return ['roadmap.html', 'public/roadmap.html'];
  const targets = new Set([relative]);
  if (relative.startsWith('plan/')) targets.add('spec/' + relative);
  if (relative === 'spec/STATUS.md') targets.add('STATUS.md');
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    targets.add(withoutSpec);
    targets.add('public/' + withoutSpec);
    targets.add('public/' + relative);
  }
  return [...targets];
}

function overlay(relative, bytes, records) {
  for (const alias of aliases(relative)) {
    const destination = path.join(selectedRoot, alias);
    if (fs.existsSync(destination)) {
      const stat = fs.lstatSync(destination);
      assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'unsafe overlay target: ' + alias);
      fs.unlinkSync(destination);
    }
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes, { flag: 'wx' });
    records.push({ source: relative, path: alias, bytes: bytes.length, sha256: sha256(bytes) });
  }
}

hardlinkTree(baseRoot, selectedRoot);
const overlays = [];
for (const relative of sourceFiles) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), 'unsafe source file: ' + relative);
  overlay(relative, fs.readFileSync(source), overlays);
}

const manifest = {
  pass: 'pass02',
  baseSelectedRoot: path.relative(repositoryRoot, baseRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  outputRoot: path.relative(repositoryRoot, outputRoot),
  head,
  originMain,
  sourceFileCount: sourceFiles.length,
  sourceFiles,
  overlays,
  storageMethod: 'hardlink the prior selected root; unlink each overlay target before writing current source bytes',
  noDeletes: true,
};
fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });

const configPath = path.join(evidenceDirectory, 'rspress-pass02.config.ts');
assert.ok(!fs.existsSync(configPath), 'refusing to overwrite Rspress config');
const config = "import baseConfig from '../../../../packages/docs/rspress.config.ts';\n"
  + "import path from 'node:path';\n"
  + "import { fileURLToPath } from 'node:url';\n\n"
  + "const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');\n\n"
  + "export default {\n"
  + "  ...baseConfig,\n"
  + "  root: path.join(repositoryRoot, 'build/spinon/c11-4-zero-percent-auto-repeat-minimum-root-pass02-selected'),\n"
  + "  outDir: path.join(repositoryRoot, 'build/spinon/c11-4-zero-percent-auto-repeat-minimum-site-pass02-selected'),\n"
  + "  builderConfig: {\n"
  + "    ...baseConfig.builderConfig,\n"
  + "    server: { ...baseConfig.builderConfig?.server, publicDir: false },\n"
  + "    output: { ...baseConfig.builderConfig?.output, cleanDistPath: true },\n"
  + "  },\n"
  + "  markdown: {\n"
  + "    ...baseConfig.markdown,\n"
  + "    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },\n"
  + "    image: { ...baseConfig.markdown?.image, checkDeadImages: false },\n"
  + "  },\n"
  + "};\n";
fs.writeFileSync(configPath, config, { flag: 'wx' });
process.stdout.write(JSON.stringify({ pass: manifest.pass, head, originMain, sourceFileCount: sourceFiles.length, overlayCopies: overlays.length, noDeletes: true }, null, 2) + '\n');
