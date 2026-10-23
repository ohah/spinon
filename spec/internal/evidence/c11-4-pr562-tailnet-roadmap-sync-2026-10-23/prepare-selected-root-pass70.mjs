import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const previousRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr550-tailnet-root-pass69-selected');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-root-pass70-selected');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-site-pass70-selected');
const baseHead = '82f7514dc51bedd4160f27180145a13bc3df8750';
const sourceHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const branchHead = '10ae04a09f9f0b1b97056693dad0438a2c022b62';
const expectedHead = '22eafb6cad66b6be8facaf264299e7bd6073d0f5';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sanitize = (bytes, relative) => {
  if (relative.toLowerCase().endsWith('.png')) return bytes;
  return Buffer.from(bytes.toString('utf8')
    .replaceAll('/Users/yoonhb/Documents/workspace/spinon', '[repository]')
    .replaceAll('/Users/yoonhb/Library/Android/sdk', '[android-sdk]')
    .replaceAll('/Users/yoonhb', '[user-home]'));
};

assert.equal(sourceHead, expectedHead, 'unexpected origin/main revision');
execFileSync('git', ['cat-file', '-e', `${branchHead}^{commit}`], { cwd: repositoryRoot, stdio: 'ignore' });
const branchChangeText = execFileSync('git', ['diff', '--name-status', '--no-renames', sourceHead, branchHead, '--', 'plan', 'spec'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
assert.equal(branchChangeText, 'M\tspec/public/roadmap.html', 'only the verified public roadmap route correction may overlay origin/main');
assert.ok(fs.lstatSync(previousRoot).isDirectory(), 'pass69 selected root is required');
assert.ok(!fs.existsSync(selectedRoot), 'refusing to replace existing pass70 selected root');
assert.ok(!fs.existsSync(outputRoot), 'refusing to replace existing pass70 build output');

function hardlinkTree(sourceRoot, targetRoot) {
  fs.mkdirSync(targetRoot, { recursive: true });
  for (const entry of fs.readdirSync(sourceRoot, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const source = path.join(sourceRoot, entry.name);
    const target = path.join(targetRoot, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in prior selected root: ${source}`);
    if (entry.isDirectory()) hardlinkTree(source, target);
    else if (entry.isFile()) fs.linkSync(source, target);
    else throw new Error(`unsupported prior selected root entry: ${source}`);
  }
}

function copyTo(relative, originalBytes, record) {
  const bytes = sanitize(originalBytes, relative);
  const target = path.join(selectedRoot, relative);
  if (fs.existsSync(target)) {
    const stat = fs.lstatSync(target);
    assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unexpected overlay target: ${relative}`);
    fs.unlinkSync(target);
  }
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  const actual = fs.readFileSync(target);
  assert.equal(sha256(actual), sha256(bytes), `overlay hash mismatch: ${relative}`);
  record.push({ path: relative, bytes: actual.byteLength, sha256: sha256(actual) });
}

function aliases(relative) {
  const targets = new Set([relative]);
  if (relative.startsWith('plan/')) targets.add(`spec/${relative}`);
  if (relative === 'spec/STATUS.md') targets.add('STATUS.md');
  if (relative === 'spec/0001-conformance.md') targets.add('0001-conformance.md');
  if (relative === 'spec/public/roadmap.html') {
    targets.add('roadmap.html');
    targets.add('public/roadmap.html');
  }
  if (relative.startsWith('spec/internal/')) {
    const withoutSpec = relative.slice('spec/'.length);
    targets.add(withoutSpec);
    targets.add(`public/${withoutSpec}`);
    targets.add(`public/${relative}`);
  }
  return [...targets];
}

hardlinkTree(previousRoot, selectedRoot);
const changeText = execFileSync('git', ['diff', '--name-status', '--no-renames', baseHead, sourceHead, '--', 'plan', 'spec'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const changes = changeText.split('\n').filter(Boolean).map((row) => {
  const [status, relative] = row.split('\t');
  assert.ok(status === 'A' || status === 'M', `unexpected change type ${status}: ${relative}`);
  return { status, relative };
});
assert.ok(changes.length > 0, 'expected documentation changes since pass69');

const copied = [];
for (const change of changes) {
  const bytes = execFileSync('git', ['show', `${sourceHead}:${change.relative}`], { cwd: repositoryRoot, maxBuffer: 512 * 1024 * 1024 });
  for (const target of aliases(change.relative)) copyTo(target, bytes, copied);
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
  pass: 'pass70', baseHead, sourceHead, branchHead,
  branchOnlyDocuments: [{ status: 'M', path: 'spec/public/roadmap.html' }],
  previousRoot: path.relative(repositoryRoot, previousRoot),
  selectedRoot: path.relative(repositoryRoot, selectedRoot),
  selectedFileCount: files.length, selectedBytes: totalBytes,
  selectedTreeSha256: treeHash.digest('hex'),
  gitChangedDocumentCount: changes.length,
  gitChangedDocuments: changes.map(({ status, relative }) => ({ status, path: relative })),
  overlayCount: copied.length,
  overlays: copied,
  noDeletes: true,
};
fs.writeFileSync(path.join(evidenceDirectory, 'selected-root-preparation-pass70.json'), `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });

const config = `import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-root-pass70-selected'),
  outDir: path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-site-pass70-selected'),
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
fs.writeFileSync(path.join(evidenceDirectory, 'rspress-pass70.config.ts'), config, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({
  pass: manifest.pass, sourceHead, gitChangedDocumentCount: changes.length,
  overlayCount: copied.length,
  selectedFileCount: files.length, selectedBytes: totalBytes,
  selectedTreeSha256: manifest.selectedTreeSha256, noDeletes: true,
}, null, 2)}\n`);
