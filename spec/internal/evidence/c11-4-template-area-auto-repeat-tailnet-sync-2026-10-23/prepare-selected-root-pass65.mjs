import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previousRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr537-tailnet-root-pass64-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr540-tailnet-root-pass65-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr540-tailnet-site-pass65-selected');
const baseHead = '31610361bedd27a7dc2b0013ae19bd05f68a5c66';
const sourceHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.equal(sourceHead, '92b447f687e751cfc2c333bd17e22c4271c36536', 'unexpected origin/main revision');
assert.ok(fs.lstatSync(previousRoot).isDirectory(), 'pass61 selected root is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace existing pass65 selected root');
assert.ok(!fs.existsSync(outputRoot), 'refusing to replace existing pass65 build output');

function hardlinkTree(sourceRoot, targetRoot) {
  fs.mkdirSync(targetRoot, { recursive: true });
  for (const entry of fs.readdirSync(sourceRoot, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const source = path.join(sourceRoot, entry.name);
    const target = path.join(targetRoot, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in prior selected root: ${source}`);
    if (entry.isDirectory()) hardlinkTree(source, target);
    else if (entry.isFile()) fs.linkSync(source, target);
    else throw new Error(`unsupported prior root entry: ${source}`);
  }
}

function sourceBytes(relative) {
  return execFileSync('git', ['show', `${sourceHead}:${relative}`], {
    cwd: repositoryRoot,
    maxBuffer: 64 * 1024 * 1024,
  });
}

function copyTo(relative, bytes, record) {
  const target = path.join(selectedRoot, relative);
  if (fs.existsSync(target)) {
    const stat = fs.lstatSync(target);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unexpected overlay target: ${relative}`);
    fs.unlinkSync(target);
  }
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes);
  const actual = fs.readFileSync(target);
  assert.equal(sha256(actual), sha256(bytes), `overlay hash mismatch: ${relative}`);
  record.push({ path: relative, bytes: actual.byteLength, sha256: sha256(actual) });
}

function targetAliases(source) {
  const targets = new Set([source]);
  if (source.startsWith('plan/')) targets.add(`spec/${source}`);
  if (source === 'spec/STATUS.md') targets.add('STATUS.md');
  if (source === 'spec/0001-conformance.md') targets.add('0001-conformance.md');
  if (source === 'spec/public/roadmap.html') {
    targets.add('roadmap.html');
    targets.add('public/roadmap.html');
  }
  if (source.startsWith('spec/internal/')) {
    const relative = source.slice('spec/'.length);
    targets.add(relative);
    targets.add(`public/${relative}`);
    targets.add(`public/spec/${relative}`);
  }
  return [...targets];
}

hardlinkTree(previousRoot, selectedRoot);
const changedText = execFileSync('git', [
  'diff', '--name-status', '--no-renames', baseHead, sourceHead, '--', 'plan', 'spec',
], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const changes = changedText.split('\n').filter(Boolean).map((row) => {
  const [status, relative] = row.split('\t');
  assert.ok(status === 'A' || status === 'M', `unexpected change type ${status}: ${relative}`);
  return { status, relative };
});
assert.ok(changes.length > 0, 'expected documentation changes since pass61');

const copied = [];
for (const change of changes) {
  let bytes = sourceBytes(change.relative);
  if (path.extname(change.relative).toLowerCase() !== '.png') {
    bytes = Buffer.from(bytes.toString('utf8')
      .replaceAll('/Users/yoonhb/Documents/workspace/spinon', '[repository]')
      .replaceAll('/Users/yoonhb/Library/Android/sdk', '[android-sdk]')
      .replaceAll('/Users/yoonhb', '[user-home]'));
  }
  for (const target of targetAliases(change.relative)) copyTo(target, bytes, copied);
}

function listFiles(directory, prefix = '') {
  const result = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const absolute = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected root: ${relative}`);
    if (entry.isDirectory()) result.push(...listFiles(absolute, relative));
    else if (entry.isFile()) result.push(relative);
    else throw new Error(`unsupported selected root entry: ${relative}`);
  }
  return result;
}

const files = listFiles(selectedRoot);
const treeHash = createHash('sha256');
let totalBytes = 0;
for (const relative of files) {
  const bytes = fs.readFileSync(path.join(selectedRoot, relative));
  totalBytes += bytes.byteLength;
  treeHash.update(relative).update('\0').update(sha256(bytes)).update('\n');
}
const manifest = {
  pass: 'pass65', baseHead, sourceHead,
  previousRoot: path.relative(repositoryRoot, previousRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  selectedFileCount: files.length, selectedBytes: totalBytes,
  selectedTreeSha256: treeHash.digest('hex'),
  gitChangedDocumentCount: changes.length,
  gitChangedDocuments: changes.map(({ status, relative }) => ({ status, path: relative })),
  overlayCount: copied.length, overlays: copied,
  ignoredUntrackedWorkspaceEvidence: true, noDeletes: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass65.json'), `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });

const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/c11-4-pr540-tailnet-root-pass65-selected'),
  outDir: path.join(repositoryRoot, 'build/spinon/c11-4-pr540-tailnet-site-pass65-selected'),
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
fs.writeFileSync(path.join(evidenceDirectory, 'rspress-pass65.config.ts'), config, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({
  pass: manifest.pass, sourceHead, gitChangedDocumentCount: changes.length,
  overlayCount: copied.length, selectedFileCount: files.length,
  selectedBytes: totalBytes, selectedTreeSha256: manifest.selectedTreeSha256,
  noDeletes: true,
}, null, 2)}\n`);
