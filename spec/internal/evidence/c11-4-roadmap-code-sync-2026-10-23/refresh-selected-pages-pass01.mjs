import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-code-sync-root-pass01-selected');
const reportPath = path.join(evidenceDirectory, 'selected-page-refresh-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const files = [
  'spec/internal/evidence/c11-4-roadmap-code-sync-2026-10-23/README.md',
  'spec/internal/evidence/c11-4-roadmap-code-sync-2026-10-23/pr-diff-review-20.md',
];
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite selected-page refresh report');

const refreshed = [];
for (const relative of files) {
  const source = path.join(repositoryRoot, relative);
  const stat = fs.lstatSync(source);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe refresh source: ${relative}`);
  const bytes = fs.readFileSync(source);
  const aliases = [
    relative,
    relative.slice('spec/'.length),
    `public/${relative.slice('spec/'.length)}`,
    `public/${relative}`,
  ];
  for (const alias of aliases) {
    const target = path.join(selectedRoot, alias);
    const targetStat = fs.lstatSync(target);
    assert.ok(targetStat.isFile() && !targetStat.isSymbolicLink(), `unsafe selected route: ${alias}`);
    fs.writeFileSync(target, bytes);
    assert.equal(sha256(fs.readFileSync(target)), sha256(bytes), `selected refresh hash mismatch: ${alias}`);
    refreshed.push({ source: relative, path: alias, bytes: bytes.length, sha256: sha256(bytes) });
  }
}

const report = { pass: 'pass01', refreshed, noDeletes: true };
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
