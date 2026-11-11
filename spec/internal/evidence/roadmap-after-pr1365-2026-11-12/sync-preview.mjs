import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidenceDirectory, '../../../..');
const buildSite = path.join(repo, 'build/spinon/roadmap-after-pr1365-2026-11-12/site');
const preview = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRelative = 'backup/roadmap-after-pr1365-2026-11-12';
const backup = path.join(preview, backupRelative);
const manifestPath = path.join(evidenceDirectory, 'tailnet-copy-manifest.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const records = [];
const writes = [];

assert.ok(fs.lstatSync(preview).isDirectory(), 'preview root must be a directory');
assert.ok(!fs.lstatSync(preview).isSymbolicLink(), 'preview root must not be a symlink');
assert.ok(fs.statSync(path.join(preview, 'tailnet-preview-server.mjs')).isFile(), 'preserve the selected preview server');
assert.ok(fs.statSync(buildSite).isDirectory(), 'selected Rspress build must exist');
assert.ok(!fs.existsSync(backup), `existing backup directory is preserved: ${backup}`);
assert.ok(!fs.existsSync(manifestPath), `existing copy manifest is preserved: ${manifestPath}`);

function safeRelative(relative) {
  assert.ok(relative && !relative.startsWith('/') && !relative.includes('\\'), `unsafe relative path: ${relative}`);
  assert.ok(relative.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe path segment: ${relative}`);
}

function assertNoSymlinkComponents(base, relative) {
  safeRelative(relative);
  let current = base;
  for (const part of relative.split('/')) {
    current = path.join(current, part);
    if (fs.existsSync(current)) assert.ok(!fs.lstatSync(current).isSymbolicLink(), `refusing symlink path: ${current}`);
  }
}

function install(sourcePath, sourceRelative, destinationRelative) {
  safeRelative(sourceRelative);
  safeRelative(destinationRelative);
  assertNoSymlinkComponents(repo, sourceRelative);
  assertNoSymlinkComponents(preview, destinationRelative);
  const sourceStat = fs.lstatSync(sourcePath);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `source must be a regular file: ${sourceRelative}`);
  const bytes = fs.readFileSync(sourcePath);
  const next = sha256(bytes);
  const destinationPath = path.join(preview, destinationRelative);
  let prior = null;
  let backupPath = null;
  if (fs.existsSync(destinationPath)) {
    const oldStat = fs.lstatSync(destinationPath);
    assert.ok(oldStat.isFile() && !oldStat.isSymbolicLink(), `existing destination must be regular: ${destinationRelative}`);
    const oldBytes = fs.readFileSync(destinationPath);
    prior = { bytes: oldBytes.length, sha256: sha256(oldBytes) };
    if (prior.sha256 === next) {
      records.push({ source: sourceRelative, destination: destinationRelative, bytes: bytes.length, sha256: next, before: prior, backup: null, action: 'unchanged' });
      return;
    }
    backupPath = path.join(backup, destinationRelative);
    assert.ok(!fs.existsSync(backupPath), `existing backup is preserved: ${backupPath}`);
    fs.mkdirSync(path.dirname(backupPath), { recursive: true });
    fs.writeFileSync(backupPath, oldBytes, { flag: 'wx' });
    assert.equal(sha256(fs.readFileSync(backupPath)), prior.sha256, `backup hash mismatch: ${destinationRelative}`);
  }
  fs.mkdirSync(path.dirname(destinationPath), { recursive: true });
  const temporary = `${destinationPath}.codex-roadmap-after-pr1365.tmp`;
  assert.ok(!fs.existsSync(temporary), `existing temporary file is preserved: ${temporary}`);
  fs.writeFileSync(temporary, bytes, { flag: 'wx' });
  fs.renameSync(temporary, destinationPath);
  assert.equal(sha256(fs.readFileSync(destinationPath)), next, `installed hash mismatch: ${destinationRelative}`);
  const record = {
    source: sourceRelative,
    destination: destinationRelative,
    bytes: bytes.length,
    sha256: next,
    before: prior,
    backup: backupPath ? path.relative(preview, backupPath).split(path.sep).join('/') : null,
    action: prior ? 'replaced' : 'added',
  };
  records.push(record);
  writes.push(record);
}

function installRepoFile(sourceRelative, destinationRelative) {
  install(path.join(repo, sourceRelative), sourceRelative, destinationRelative);
}

function installBuiltFile(route) {
  const sourceRelative = path.relative(repo, path.join(buildSite, route)).split(path.sep).join('/');
  install(path.join(buildSite, route), sourceRelative, route);
}

function walkFiles(absolute, sourceRelative, destinationPrefix) {
  const stat = fs.lstatSync(absolute);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `source tree must be a regular directory: ${absolute}`);
  for (const entry of fs.readdirSync(absolute, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = `${sourceRelative}/${entry.name}`;
    const childPath = path.join(absolute, entry.name);
    assert.ok(!entry.isSymbolicLink(), `source tree symlink is not accepted: ${childPath}`);
    if (entry.isDirectory()) walkFiles(childPath, child, `${destinationPrefix}/${entry.name}`);
    else if (entry.isFile()) install(childPath, child, `${destinationPrefix}/${entry.name}`);
    else throw new Error(`unsupported source entry: ${childPath}`);
  }
}

for (const [source, destination] of [
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/STATUS.md', 'spec/STATUS.md'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.md', 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.md'],
  ['plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-review-20.md', 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-review-20.md'],
  ['plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-pr-diff-review-30.md', 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-pr-diff-review-30.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['spec/public/icon.svg', 'icon.svg'],
  ['spec/public/logo.svg', 'logo.svg'],
]) installRepoFile(source, destination);

for (const name of [
  'c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12',
  'c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12',
]) {
  walkFiles(
    path.join(repo, 'spec/internal/evidence', name),
    `spec/internal/evidence/${name}`,
    `internal/evidence/${name}`,
  );
}

for (const route of [
  'STATUS.html',
  'spec/STATUS.html',
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.html',
  'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-review-20.html',
  'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-pr-diff-review-30.html',
  'internal/0059-c11-grid-layout.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/README.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/README.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/platform-smoke.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/pr-diff-review-30.html',
]) installBuiltFile(route);

walkFiles(path.join(buildSite, 'static'), 'build/spinon/roadmap-after-pr1365-2026-11-12/site/static', 'static');

const report = {
  schema: 'spinon.tailnet-preview-selected-sync/v1',
  sourceCommit: '5a7ed74d197c78d287cc12a0823a2abc77050f3f',
  previewRoot: preview,
  previewServerPid: 90711,
  previewServerPort: 8768,
  selectedSite: 'build/spinon/roadmap-after-pr1365-2026-11-12/site',
  backupDirectory: backupRelative,
  fullSiteBuildClaim: false,
  fullDocsDeadLinkCheck: false,
  fileCount: records.length,
  changedFileCount: writes.length,
  backedUpFileCount: records.filter((item) => item.backup !== null).length,
  addedFileCount: records.filter((item) => item.action === 'added').length,
  unchangedFileCount: records.filter((item) => item.action === 'unchanged').length,
  deletedFileCount: 0,
  records,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, records: undefined }, null, 2)}\n`);
