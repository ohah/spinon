import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const initialPreflightPath = path.join(evidenceDirectory, 'tailnet-preflight-pass08.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-review20-alias-correction-pass08.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-25/pass08-untracked-review20-correction';
const origin = 'https://macstudio.tailed42f2.ts.net';
const untrackedSource = 'spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/merge-roadmap-pr-diff-review-20.md';
const expectedSourceHash = '5b884466141024373300b95a6bddc85e62ed6ac84e633283d5206a9378d784c2';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const required of [initialPreflightPath]) assert.ok(fs.lstatSync(required).isFile(), `missing initial pass08 preflight: ${required}`);
assert.ok(!fs.existsSync(reportPath), 'refusing to replace correction report');
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse correction backup root: ${backupRoot}`);
assert.ok(fs.lstatSync(targetRoot).isDirectory() && !fs.lstatSync(targetRoot).isSymbolicLink(), 'unsafe Tailnet preview root');

const initial = JSON.parse(fs.readFileSync(initialPreflightPath, 'utf8'));
assert.equal(initial.pass, 'pass08');
const aliases = initial.files.filter((entry) => entry.path.endsWith('/merge-roadmap-pr-diff-review-20.md'));
assert.equal(aliases.length, 4, 'expected exactly four aliases from the untracked review source');
assert.ok(aliases.every((entry) => entry.path.includes('c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25')));
assert.ok(aliases.every((entry) => entry.status === 'create' && entry.target_exists === false), 'only paths newly created by pass08 may be corrected');
assert.ok(aliases.every((entry) => entry.source_sha256 === expectedSourceHash && entry.source_bytes === 8133));

function inspect(root, relative) {
  let current = root;
  for (const [index, part] of relative.split('/').entries()) {
    current = path.join(current, part);
    const stat = fs.lstatSync(current);
    assert.ok(!stat.isSymbolicLink(), `symlink in path: ${relative}`);
    if (index < relative.split('/').length - 1) assert.ok(stat.isDirectory(), `non-directory in path: ${relative}`);
    else assert.ok(stat.isFile(), `non-file in path: ${relative}`);
  }
  return fs.readFileSync(current);
}

function writeBackup(destination, bytes) {
  fs.mkdirSync(path.dirname(destination), { recursive: true, mode: 0o755 });
  const fd = fs.openSync(destination, 'wx', 0o644);
  try {
    fs.writeFileSync(fd, bytes);
    fs.fsyncSync(fd);
  } finally {
    fs.closeSync(fd);
  }
}

fs.mkdirSync(backupRoot, { recursive: true, mode: 0o755 });
assert.ok(fs.lstatSync(backupRoot).isDirectory() && !fs.lstatSync(backupRoot).isSymbolicLink(), 'unsafe correction backup root');
const corrected = [];
for (const entry of aliases) {
  const target = path.join(targetRoot, entry.path);
  const before = inspect(targetRoot, entry.path);
  assert.equal(before.byteLength, entry.source_bytes, `unexpected current file size: ${entry.path}`);
  assert.equal(sha256(before), entry.source_sha256, `unexpected current file hash: ${entry.path}`);

  const beforeLsof = spawnSync('lsof', ['-nP', target], { encoding: 'utf8' });
  assert.equal(beforeLsof.error, undefined, `lsof failed: ${beforeLsof.error}`);
  assert.equal(beforeLsof.status, 1, `refusing to remove open file: ${entry.path}\n${beforeLsof.stdout}`);

  const backupPath = path.join(backupRoot, entry.path);
  writeBackup(backupPath, before);
  const backup = fs.readFileSync(backupPath);
  assert.equal(backup.byteLength, entry.source_bytes, `backup size mismatch: ${entry.path}`);
  assert.equal(sha256(backup), entry.source_sha256, `backup hash mismatch: ${entry.path}`);

  const current = inspect(targetRoot, entry.path);
  assert.equal(current.byteLength, entry.source_bytes, `file changed before correction: ${entry.path}`);
  assert.equal(sha256(current), entry.source_sha256, `file hash changed before correction: ${entry.path}`);
  const finalLsof = spawnSync('lsof', ['-nP', target], { encoding: 'utf8' });
  assert.equal(finalLsof.error, undefined, `lsof failed: ${finalLsof.error}`);
  assert.equal(finalLsof.status, 1, `refusing to remove newly open file: ${entry.path}\n${finalLsof.stdout}`);

  fs.unlinkSync(target);
  assert.ok(!fs.existsSync(target), `route still exists after correction: ${entry.path}`);
  const encodedRoute = entry.path.split('/').map(encodeURIComponent).join('/');
  const response = await fetch(`${origin}/spinon/${encodedRoute}?sync=pass08-review20-correction`, { headers: { 'cache-control': 'no-cache' }, signal: AbortSignal.timeout(15000) });
  assert.equal(response.status, 404, `Tailnet route did not disappear: ${entry.path}, HTTP ${response.status}`);
  corrected.push({
    path: entry.path,
    source: untrackedSource,
    initial_status: entry.status,
    initial_target_exists: entry.target_exists,
    bytes: backup.byteLength,
    sha256: sha256(backup),
    backup_path: path.relative(backupRoot, backupPath),
    local_removed: !fs.existsSync(target),
    remote_http_status: response.status,
    remote_removed: response.status === 404,
  });
}

const report = {
  pass: 'pass08-review20-correction',
  reason: 'exclude one untracked review source that was absent from the pinned origin/main tree',
  source: { path: untrackedSource, sha256: expectedSourceHash, aliases: aliases.length },
  correction_backup_root: backupRoot,
  backed_up: corrected.length,
  removed_routes: corrected.length,
  all_backups_byte_exact: corrected.every((entry) => entry.bytes === 8133 && entry.sha256 === expectedSourceHash),
  all_local_routes_removed: corrected.every((entry) => entry.local_removed),
  all_remote_routes_http_404: corrected.every((entry) => entry.remote_removed),
  only_initially_new_paths_removed: aliases.every((entry) => entry.status === 'create' && entry.target_exists === false),
  routes: corrected,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
assert.ok(report.all_backups_byte_exact && report.all_local_routes_removed && report.all_remote_routes_http_404, 'pass08 review20 alias correction failed');
