import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const copy = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'tailnet-copy.json'), 'utf8'));
const reportPath = path.join(evidenceDirectory, 'backup-verification.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite backup verification report');
assert.equal(copy.backupCount, copy.backups.length);
assert.equal(copy.backupBytes, copy.backups.reduce((sum, entry) => sum + entry.bytes, 0));

const verified = copy.backups.map((entry) => {
  const bytes = fs.readFileSync(path.join(copy.backupRoot, entry.path));
  const actual = { path: entry.path, bytes: bytes.byteLength, sha256: sha256(bytes) };
  assert.equal(actual.bytes, entry.bytes, `backup size mismatch: ${entry.path}`);
  assert.equal(actual.sha256, entry.sha256, `backup hash mismatch: ${entry.path}`);
  return actual;
});

const report = {
  backupRoot: copy.backupRoot,
  backupCount: verified.length,
  backupBytes: verified.reduce((sum, entry) => sum + entry.bytes, 0),
  allSizesMatch: true,
  allHashesMatch: true,
  backups: verified,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
