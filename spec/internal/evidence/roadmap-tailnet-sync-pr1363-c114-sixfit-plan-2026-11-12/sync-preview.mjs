import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidenceDirectory, '../../../..');
const buildSite = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1363-c114-sixfit-plan-2026-11-12/site');
const preview = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRelative = 'backup/roadmap-tailnet-sync-pr1363-c114-sixfit-plan-2026-11-12-pass02';
const backup = path.join(preview, backupRelative);
const manifestPath = path.join(evidenceDirectory, 'tailnet-copy-manifest-pass02.json');
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

function targetBytes(source, sourceRelative) {
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `source must be a regular file: ${sourceRelative}`);
  return fs.readFileSync(source);
}

function install(source, sourceRelative, destinationRelative) {
  safeRelative(sourceRelative);
  safeRelative(destinationRelative);
  assertNoSymlinkComponents(repo, sourceRelative);
  assertNoSymlinkComponents(preview, destinationRelative);
  const sourcePath = path.join(repo, sourceRelative);
  const destinationPath = path.join(preview, destinationRelative);
  const bytes = targetBytes(sourcePath, sourceRelative);
  const next = sha256(bytes);
  let prior = null;
  let backupPath = null;
  if (fs.existsSync(destinationPath)) {
    const oldStat = fs.lstatSync(destinationPath);
    assert.ok(oldStat.isFile() && !oldStat.isSymbolicLink(), `existing destination must be regular file: ${destinationRelative}`);
    const oldBytes = fs.readFileSync(destinationPath);
    prior = { bytes: oldBytes.length, sha256: sha256(oldBytes) };
    if (prior.sha256 === next) {
      records.push({ source: sourceRelative, destination: destinationRelative, bytes: bytes.length, sha256: next, before: prior, backup: null, action: 'unchanged' });
      return;
    }
    backupPath = path.join(backup, destinationRelative);
    fs.mkdirSync(path.dirname(backupPath), { recursive: true });
    fs.writeFileSync(backupPath, oldBytes, { flag: 'wx' });
    assert.equal(sha256(fs.readFileSync(backupPath)), prior.sha256, `backup hash mismatch: ${destinationRelative}`);
  }
  fs.mkdirSync(path.dirname(destinationPath), { recursive: true });
  const temporary = `${destinationPath}.codex-pr1363.tmp`;
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

function walkFiles(absolute, relative = '') {
  const stat = fs.lstatSync(absolute);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `source tree must be a regular directory: ${absolute}`);
  const entries = fs.readdirSync(absolute, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name));
  for (const entry of entries) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    const childPath = path.join(absolute, entry.name);
    assert.ok(!entry.isSymbolicLink(), `source tree symlink is not accepted: ${childPath}`);
    if (entry.isDirectory()) walkFiles(childPath, child);
    else if (entry.isFile()) {
      const sourceRelative = path.relative(repo, childPath).split(path.sep).join('/');
      install(sourceRelative, sourceRelative, child);
    } else throw new Error(`unsupported source entry: ${childPath}`);
  }
}

const directPairs = [
  ['spec/public/roadmap.html', 'roadmap.html'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.md', 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.md'],
  ['plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-review-20.md', 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-review-20.md'],
  ['plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-pr-diff-review-30.md', 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-pr-diff-review-30.md'],
  ['tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-six-fit-auto-minimum-precomparison.html', 'tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-six-fit-auto-minimum-precomparison.html'],
  ['spec/public/icon.svg', 'icon.svg'],
  ['spec/public/logo.svg', 'logo.svg'],
];

for (const [source, destination] of directPairs) install(source, source, destination);
for (const route of [
  'plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.html',
  'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-review-20.html',
  'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-pr-diff-review-30.html',
]) {
  const sourceRelative = path.relative(repo, path.join(buildSite, route)).split(path.sep).join('/');
  install(sourceRelative, sourceRelative, route);
}
walkFiles(
  path.join(repo, 'spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12'),
  'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12',
);
walkFiles(
  path.join(buildSite, 'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12'),
  'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12',
);
walkFiles(path.join(buildSite, 'static'), 'static');

const report = {
  schema: 'spinon.tailnet-preview-selected-sync/v1',
  sourceCommit: 'cc944e4e3ff64c78298a5330f8cdb02234376710',
  previewRoot: preview,
  previewServerPid: 90711,
  selectedSite: path.relative(repo, buildSite).split(path.sep).join('/'),
  backupDirectory: backupRelative,
  fullSiteBuildClaim: false,
  fullDocsDeadLinkCheck: false,
  fileCount: records.length,
  changedFileCount: writes.length,
  backedUpFileCount: records.filter((item) => item.backup !== null).length,
  unchangedFileCount: records.filter((item) => item.action === 'unchanged').length,
  records,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, records: undefined }, null, 2)}\n`);
