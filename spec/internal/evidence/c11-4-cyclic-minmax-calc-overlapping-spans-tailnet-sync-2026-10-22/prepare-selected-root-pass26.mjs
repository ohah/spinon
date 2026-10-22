import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const pass = process.env.SPINON_TAILNET_PASS;
if (!['pass26-initial', 'pass26-final'].includes(pass ?? '')) throw new Error('SPINON_TAILNET_PASS must be pass26-initial or pass26-final');
const sourceRoot = path.join(repoRoot, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-spans-tailnet-root-' + pass);
const outputPath = path.join(evidenceDir, process.argv[2] ?? ('selected-root-preparation-' + pass + '.json'));
const commit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const tree = execFileSync('git', ['rev-parse', `${commit}^{tree}`], { cwd: repoRoot, encoding: 'utf8' }).trim();
const branch = execFileSync('git', ['branch', '--show-current'], { cwd: repoRoot, encoding: 'utf8' }).trim();

if (fs.existsSync(sourceRoot)) throw new Error(`selected root already exists: ${sourceRoot}`);
fs.mkdirSync(sourceRoot, { recursive: false });
const archive = execFileSync('git', ['archive', '--format=tar', commit], {
  cwd: repoRoot,
  maxBuffer: 1024 * 1024 * 1024,
});
execFileSync('tar', ['-xf', '-', '-C', sourceRoot], { input: archive, cwd: repoRoot });

function countEntries(directory) {
  let files = 0;
  let bytes = 0;
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in selected root: ${fullPath}`);
    if (entry.isDirectory()) {
      const child = countEntries(fullPath);
      files += child.files;
      bytes += child.bytes;
    } else if (entry.isFile()) {
      files += 1;
      bytes += fs.statSync(fullPath).size;
    } else {
      throw new Error(`unsupported selected-root entry: ${fullPath}`);
    }
  }
  return { files, bytes };
}

const counts = countEntries(sourceRoot);
const output = {
  schemaVersion: 1,
  sourceCommit: commit,
  sourceTree: tree,
  sourceBranch: branch,
  archiveBytes: archive.byteLength,
  selectedRoot: path.relative(repoRoot, sourceRoot).split(path.sep).join('/'),
  pass,
  selectedFiles: counts.files,
  selectedBytes: counts.bytes,
  sourceKind: 'git archive of the committed docs-sync branch; no worktree and no untracked checkout files',
};
fs.writeFileSync(outputPath, `${JSON.stringify(output, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
