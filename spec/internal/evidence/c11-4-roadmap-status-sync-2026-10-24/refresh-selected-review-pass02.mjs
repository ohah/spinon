import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-status-sync-pass01-root-selected');
const sourceRelative = 'spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/plan-pr-diff-review-20.md';
const reportPath = path.join(evidenceDirectory, 'selected-root-refresh-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite pass02 refresh report');
assert.ok(fs.lstatSync(selectedRoot).isDirectory(), 'selected root is missing');

const source = path.join(repositoryRoot, sourceRelative);
const sourceStat = fs.lstatSync(source);
assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), 'unsafe review source');
const bytes = fs.readFileSync(source);
const relativePaths = [
  sourceRelative,
  sourceRelative.slice('spec/'.length),
  `public/${sourceRelative.slice('spec/'.length)}`,
  `public/${sourceRelative}`,
];
const refreshed = [];
for (const relative of relativePaths) {
  const target = path.join(selectedRoot, relative);
  const stat = fs.lstatSync(target);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe selected alias: ${relative}`);
  fs.unlinkSync(target);
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `selected alias mismatch: ${relative}`);
  refreshed.push({ path: relative, bytes: bytes.length, sha256: sha256(bytes) });
}
const report = { pass: 'pass02', source: sourceRelative, sha256: sha256(bytes), refreshed, noDeletes: true };
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ refreshedAliases: refreshed.length, sha256: report.sha256, noDeletes: true }, null, 2)}\n`);
