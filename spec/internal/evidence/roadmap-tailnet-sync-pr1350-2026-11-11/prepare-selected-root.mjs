import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildRoot = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1350-2026-11-11');
const archive = path.join(buildRoot, 'tracked-spec-and-plan.tar');
const extracted = path.join(buildRoot, 'extracted');
const root = path.join(buildRoot, 'root');
const reportPath = path.join(evidence, 'selected-root.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const scriptExtensions = new Set(['.cjs', '.js', '.jsx', '.mjs', '.sh', '.ts', '.tsx']);
const sourcePaths = [
  'spec/index.mdx',
  'spec/styles.css',
  'spec/public/icon.svg',
  'spec/public/logo.svg',
  'spec/public/roadmap.svg',
  'spec/public/roadmap.html',
  'spec/STATUS.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-min-precomparison-2026-11-11',
  'spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum-runtime-2026-11-11',
  'plan/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum.md',
  'plan/c11-4-root-html-repeated-text-autoplacement-four-fit-auto-minimum.md',
  'plan/c11-4-root-html-repeated-text-autoplacement-five-fit-auto-minimum-plan-review-20.md',
];

const sourceHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repo, encoding: 'utf8' }).trim();
const mainHead = execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repo, encoding: 'utf8' }).trim();
assert.equal(sourceHead, mainHead, '선택 root는 현재 origin/main에서만 만든다');
for (const target of [archive, extracted, root, reportPath]) assert.ok(!fs.existsSync(target), `기존 산출물 보존: ${target}`);
fs.mkdirSync(buildRoot, { recursive: true });

const trackedPaths = execFileSync('git', ['ls-tree', '-r', '--name-only', sourceHead, '--', ...sourcePaths], { cwd: repo, encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 })
  .trim().split('\n').filter(Boolean);
assert.ok(trackedPaths.length > 0, 'tracked spec/plan source가 비어 있지 않아야 함');

const archiveFd = fs.openSync(archive, 'wx');
try {
  execFileSync('git', ['archive', '--format=tar', sourceHead, ...sourcePaths], { cwd: repo, stdio: ['ignore', archiveFd, 'inherit'] });
} finally {
  fs.closeSync(archiveFd);
}
fs.mkdirSync(extracted, { recursive: false });
execFileSync('tar', ['-xf', archive, '-C', extracted], { cwd: repo, stdio: 'inherit' });
function removeSourceScripts(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const absolute = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `source archive symlink 거부: ${absolute}`);
    if (entry.isDirectory()) removeSourceScripts(absolute);
    else if (entry.isFile() && scriptExtensions.has(path.extname(entry.name).toLowerCase())) fs.unlinkSync(absolute);
  }
}
removeSourceScripts(path.join(extracted, 'spec'));
removeSourceScripts(path.join(extracted, 'plan'));
fs.mkdirSync(root, { recursive: false });
fs.cpSync(path.join(extracted, 'spec'), root, { recursive: true, dereference: false, errorOnExist: true });
fs.cpSync(path.join(extracted, 'spec'), path.join(root, 'spec'), { recursive: true, dereference: false, errorOnExist: true });
fs.cpSync(path.join(extracted, 'plan'), path.join(root, 'plan'), { recursive: true, dereference: false, errorOnExist: true });
fs.cpSync(path.join(extracted, 'plan'), path.join(root, 'spec', 'plan'), { recursive: true, dereference: false, errorOnExist: true });
for (const name of ['icon.svg', 'logo.svg', 'roadmap.svg', 'roadmap.html']) {
  fs.copyFileSync(path.join(root, 'public', name), path.join(root, name), fs.constants.COPYFILE_EXCL);
}
fs.rmSync(extracted, { recursive: true, force: false });
fs.unlinkSync(archive);

const entries = [];
function visit(directory, relative = '') {
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    const absolute = path.join(directory, child);
    assert.ok(!entry.isSymbolicLink(), `선택 root symlink 거부: ${child}`);
    if (entry.isDirectory()) visit(directory, child);
    else if (entry.isFile()) {
      const bytes = fs.readFileSync(absolute);
      entries.push({ path: child.split(path.sep).join('/'), bytes: bytes.length, sha256: sha256(bytes) });
    } else throw new Error(`지원하지 않는 root 항목: ${child}`);
  }
}
visit(root);
const report = {
  sourceHead,
  sourceTree: execFileSync('git', ['rev-parse', `${sourceHead}^{tree}`], { cwd: repo, encoding: 'utf8' }).trim(),
  selectedTrackedSourcePaths: trackedPaths.length,
  selectedRootFileCount: entries.length,
  selectedRootBytes: entries.reduce((sum, entry) => sum + entry.bytes, 0),
  includesUntrackedFiles: false,
  outputRoot: path.relative(repo, root).split(path.sep).join('/'),
  files: entries,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, files: undefined }, null, 2)}\n`);
