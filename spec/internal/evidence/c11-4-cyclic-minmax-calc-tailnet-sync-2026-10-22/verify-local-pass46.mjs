import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { gunzipSync, gzipSync } from 'node:zlib';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const preflightPath = path.join(evidenceDirectory, 'preflight-pass46.json.gz');
const preflight = JSON.parse(gunzipSync(fs.readFileSync(preflightPath)).toString('utf8'));
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-tailnet-site-pass46');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-cyclic-minmax-calc-tailnet-sync-2026-10-22/pass46';

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function readRegularFile(root, relative) {
  assert.ok(!path.isAbsolute(relative) && !relative.split('/').includes('..'), `unsafe relative path: ${relative}`);
  let current = root;
  const rootStat = fs.lstatSync(root);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), `unsafe root: ${root}`);
  const parts = relative.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    const stat = fs.lstatSync(current);
    assert.ok(!stat.isSymbolicLink(), `symlink path: ${relative}`);
    if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory parent: ${relative}`);
    else assert.ok(stat.isFile(), `non-file target: ${relative}`);
  }
  return fs.readFileSync(current);
}

const targetResults = [];
const backupResults = [];
for (const entry of preflight.files) {
  const source = readRegularFile(sourceRoot, entry.path);
  const target = readRegularFile(targetRoot, entry.path);
  const verified = source.byteLength === entry.source_bytes
    && sha256(source) === entry.source_sha256
    && target.byteLength === entry.source_bytes
    && sha256(target) === entry.source_sha256;
  targetResults.push({
    path: entry.path,
    bytes: target.byteLength,
    sha256: sha256(target),
    expected_sha256: entry.source_sha256,
    verified,
  });
  if (entry.status === 'replace') {
    const backupPath = path.join(backupRoot, entry.path);
    const backupRelative = path.relative(backupRoot, backupPath).split(path.sep).join('/');
    const backup = readRegularFile(backupRoot, backupRelative);
    const backupVerified = backup.byteLength === entry.previous_bytes
      && sha256(backup) === entry.previous_sha256;
    backupResults.push({
      path: entry.path,
      backup_path: `SpinonPreviewBackups/c11-4-cyclic-minmax-calc-tailnet-sync-2026-10-22/pass46/${entry.path}`,
      bytes: backup.byteLength,
      sha256: sha256(backup),
      expected_sha256: entry.previous_sha256,
      verified: backupVerified,
    });
  }
}

const output = {
  pass: 'pass46',
  target_count: targetResults.length,
  source_counts: preflight.counts,
  all_targets_verified: targetResults.every((entry) => entry.verified),
  backup_count: backupResults.length,
  all_backups_verified: backupResults.every((entry) => entry.verified),
  no_deletes: true,
  targets: targetResults,
  backups: backupResults,
};
fs.writeFileSync(
  path.join(evidenceDirectory, 'local-verification-pass46.json.gz'),
  gzipSync(Buffer.from(`${JSON.stringify(output, null, 2)}\n`), { level: 9 }),
);
process.stdout.write(`${JSON.stringify({
  pass: output.pass,
  target_count: output.target_count,
  all_targets_verified: output.all_targets_verified,
  backup_count: output.backup_count,
  all_backups_verified: output.all_backups_verified,
  no_deletes: output.no_deletes,
}, null, 2)}\n`);
if (!output.all_targets_verified || !output.all_backups_verified) process.exitCode = 1;
