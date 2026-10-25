import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const preflightPath = path.join(evidenceDirectory, 'tailnet-preflight-pass08.json');
const selectedBuildPath = path.join(evidenceDirectory, 'selected-build-verification-pass08-retry03.json');
const copyReportPath = path.join(evidenceDirectory, 'tailnet-copy-pass08.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-verify-pass08.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-25/pass08';
const origin = (process.env.SPINON_PREVIEW_ORIGIN || 'https://macstudio.tailed42f2.ts.net').replace(/\/$/, '');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const required of [preflightPath, selectedBuildPath, copyReportPath]) assert.ok(fs.lstatSync(required).isFile(), `required pass08 input is missing: ${required}`);
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite pass08 verify report');
const preflight = JSON.parse(fs.readFileSync(preflightPath, 'utf8'));
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
const copyReport = JSON.parse(fs.readFileSync(copyReportPath, 'utf8'));
assert.equal(preflight.pass, 'pass08');
assert.equal(copyReport.pass, 'pass08');
assert.equal(copyReport.selected_source_head, selectedBuild.sourceHead);
assert.equal(copyReport.selected_source_tree, selectedBuild.sourceTree);
assert.equal(preflight.files.length, selectedBuild.selectedRouteCount);
const sourceRoot = path.join(repositoryRoot, selectedBuild.selectedOutputRoot);

function inspectPath(root, relative) {
  const rootStat = fs.lstatSync(root);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), `unsafe root: ${root}`);
  let current = root;
  const parts = relative.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in verification path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory in verification path: ${relative}`);
      else assert.ok(stat.isFile(), `non-file verification path: ${relative}`);
      if (index === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current) };
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

const localTargets = [];
const backups = [];
for (const entry of preflight.files) {
  const sourceBytes = fs.readFileSync(path.join(sourceRoot, entry.path));
  assert.equal(sourceBytes.byteLength, entry.source_bytes, `selected source size changed: ${entry.path}`);
  assert.equal(sha256(sourceBytes), entry.source_sha256, `selected source hash changed: ${entry.path}`);
  const target = inspectPath(targetRoot, entry.path);
  const verified = target.exists && target.bytes.byteLength === entry.source_bytes && sha256(target.bytes) === entry.source_sha256;
  localTargets.push({ path: entry.path, bytes: target.exists ? target.bytes.byteLength : null, sha256: target.exists ? sha256(target.bytes) : null, verified });
  assert.ok(verified, `local preview hash mismatch: ${entry.path}`);
  if (entry.status === 'replace') {
    const backupRelative = entry.backup_path.replace(/^SpinonPreviewBackups\/roadmap-tailnet-sync-2026-10-25\/pass08\//, '');
    const backup = inspectPath(backupRoot, backupRelative);
    const backupVerified = backup.exists && backup.bytes.byteLength === entry.previous_bytes && sha256(backup.bytes) === entry.previous_sha256;
    backups.push({ path: entry.path, backup_path: entry.backup_path, bytes: backup.exists ? backup.bytes.byteLength : null, sha256: backup.exists ? sha256(backup.bytes) : null, verified: backupVerified });
    assert.ok(backupVerified, `byte-exact backup hash mismatch: ${entry.path}`);
  }
}

const remoteTargets = new Array(preflight.files.length);
let nextIndex = 0;
async function verifyWorker() {
  while (true) {
    const index = nextIndex++;
    if (index >= preflight.files.length) return;
    const entry = preflight.files[index];
    const encodedRoute = entry.path.split('/').map(encodeURIComponent).join('/');
    const url = `${origin}/spinon/${encodedRoute}?sync=roadmap-20261025-pass08`;
    let lastFailure;
    for (let attempt = 1; attempt <= 3; attempt += 1) {
      try {
        const response = await fetch(url, { headers: { 'cache-control': 'no-cache' }, signal: AbortSignal.timeout(20000) });
        const bytes = Buffer.from(await response.arrayBuffer());
        const digest = sha256(bytes);
        const verified = response.status === 200 && bytes.byteLength === entry.source_bytes && digest === entry.source_sha256;
        remoteTargets[index] = { path: entry.path, http_status: response.status, bytes: bytes.byteLength, sha256: digest, attempts: attempt, verified };
        if (verified) { lastFailure = undefined; break; }
        lastFailure = new Error(`HTTP ${response.status}, ${bytes.byteLength} bytes, SHA ${digest}`);
      } catch (error) {
        lastFailure = error;
      }
    }
    if (lastFailure) remoteTargets[index] = { path: entry.path, error: String(lastFailure), verified: false };
  }
}
await Promise.all(Array.from({ length: 10 }, () => verifyWorker()));

const report = {
  pass: 'pass08',
  selected_source_head: selectedBuild.sourceHead,
  selected_source_tree: selectedBuild.sourceTree,
  selected_route_count: preflight.files.length,
  local_targets_verified: localTargets.length,
  backups_verified: backups.length,
  remote_targets_verified: remoteTargets.filter((entry) => entry?.verified).length,
  all_local_targets_verified: localTargets.every((entry) => entry.verified),
  all_backups_verified: backups.every((entry) => entry.verified),
  all_remote_targets_http_200_and_hash_verified: remoteTargets.length === preflight.files.length && remoteTargets.every((entry) => entry?.verified),
  no_deletes: true,
  local_targets: localTargets,
  backups,
  remote_targets: remoteTargets,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, local_targets: undefined, backups: undefined, remote_targets: undefined }, null, 2)}\n`);
assert.ok(report.all_local_targets_verified && report.all_backups_verified && report.all_remote_targets_http_200_and_hash_verified, 'pass08 sync verification failed; see preserved report');
