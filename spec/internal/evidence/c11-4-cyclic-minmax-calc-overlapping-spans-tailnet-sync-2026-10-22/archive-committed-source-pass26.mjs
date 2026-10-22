import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const pass = process.env.SPINON_TAILNET_PASS;
assert.ok(['pass26-initial', 'pass26-final'].includes(pass ?? ''), 'set pass26-initial or pass26-final');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-spans-tailnet-root-' + pass);
const outputPath = path.join(evidenceDirectory, process.argv[2] || ('source-archive-' + pass + '.json'));
assert.ok(!fs.existsSync(sourceRoot), 'refusing to replace existing archive root');

const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceTree = execFileSync('git', ['rev-parse', sourceCommit + '^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const sourceBranch = execFileSync('git', ['branch', '--show-current'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const archive = execFileSync('git', ['archive', '--format=tar', sourceCommit], {
  cwd: repositoryRoot,
  maxBuffer: 2 * 1024 * 1024 * 1024,
});
fs.mkdirSync(sourceRoot, { recursive: false });
execFileSync('tar', ['-xf', '-', '-C', sourceRoot], {
  cwd: repositoryRoot,
  input: archive,
  maxBuffer: 2 * 1024 * 1024 * 1024,
});

function inventory(directory) {
  let files = 0;
  let bytes = 0;
  const symlinks = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) symlinks.push(path.relative(sourceRoot, fullPath).split(path.sep).join('/'));
    else if (entry.isDirectory()) {
      const child = inventory(fullPath);
      files += child.files;
      bytes += child.bytes;
      symlinks.push(...child.symlinks);
    } else if (entry.isFile()) {
      files += 1;
      bytes += fs.statSync(fullPath).size;
    } else {
      throw new Error('unsupported archive entry: ' + fullPath);
    }
  }
  return { files, bytes, symlinks };
}

const contents = inventory(sourceRoot);
assert.deepEqual(contents.symlinks, [], 'committed archive contains symlinks');
const output = {
  schemaVersion: 1,
  pass,
  sourceCommit,
  sourceTree,
  sourceBranch,
  archiveRoot: path.relative(repositoryRoot, sourceRoot).split(path.sep).join('/'),
  archiveBytes: archive.byteLength,
  archivedFiles: contents.files,
  archivedBytes: contents.bytes,
  noSymlinks: true,
  sourceKind: 'git archive of committed docs-sync branch; excludes untracked checkout files',
};
fs.writeFileSync(outputPath, JSON.stringify(output, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify(output, null, 2) + '\n');
