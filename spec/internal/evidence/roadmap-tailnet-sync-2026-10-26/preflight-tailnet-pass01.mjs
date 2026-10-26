import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedBuildPath = path.join(evidenceDirectory, 'selected-build-pass01.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-preflight-pass01.json');
const handleReportPath = path.join(evidenceDirectory, 'target-open-handles-pass01.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-26/pass01';
const origin = (process.env.SPINON_PREVIEW_ORIGIN || 'https://macstudio.tailed42f2.ts.net').replace(/\/$/, '');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(fs.lstatSync(selectedBuildPath).isFile(), 'selected build manifest is missing');
for (const candidate of [reportPath, handleReportPath]) assert.ok(!fs.existsSync(candidate), `refusing to overwrite ${candidate}`);
const selectedBuild = JSON.parse(fs.readFileSync(selectedBuildPath, 'utf8'));
assert.equal(selectedBuild.pass, 'pass01');
assert.equal(execFileSync('git', ['rev-parse', 'origin/main'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), selectedBuild.sourceHead);
assert.equal(execFileSync('git', ['rev-parse', 'origin/main^{tree}'], { cwd: repositoryRoot, encoding: 'utf8' }).trim(), selectedBuild.sourceTree);

const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8' }));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend is not running');
assert.equal(tailscale.Self?.Online, true, 'Mac Studio is offline on Tailnet');
assert.ok(tailscale.CertDomains?.includes('macstudio.tailed42f2.ts.net'), 'expected Tailnet DNS certificate is missing');
const listener = spawnSync('lsof', ['-nP', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' });
assert.equal(listener.error, undefined, `lsof failed: ${listener.error}`);
assert.equal(listener.status, 0, 'preview server has no listener on port 8768');
assert.match(listener.stdout, /127\.0\.0\.1:8768 \(LISTEN\)/, 'preview server is not listening on loopback:8768');
const remoteUrl = `${origin}/spinon/roadmap.html?sync=roadmap-20261026-pass01-preflight`;
const response = await fetch(remoteUrl, { method: 'HEAD', headers: { 'cache-control': 'no-cache' }, signal: AbortSignal.timeout(20000) });
assert.equal(response.status, 200, `Tailnet preview preflight returned HTTP ${response.status}`);

function inspectPath(root, relative) {
  const rootStat = fs.lstatSync(root);
  assert.ok(rootStat.isDirectory() && !rootStat.isSymbolicLink(), `unsafe target root: ${root}`);
  let current = root;
  const parts = relative.split('/');
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in target path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory in target path: ${relative}`);
      else assert.ok(stat.isFile(), `non-file target path: ${relative}`);
      if (index === parts.length - 1) return { exists: true, bytes: fs.readFileSync(current), mode: stat.mode & 0o777 };
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

const targetRootStat = fs.lstatSync(targetRoot);
assert.ok(targetRootStat.isDirectory() && !targetRootStat.isSymbolicLink(), 'preview target root is missing or unsafe');
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
const sourceRoot = path.join(repositoryRoot, selectedBuild.selectedOutputRoot);
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected output root is missing');
const files = selectedBuild.selectedRoutes.map((entry) => {
  const route = entry.path;
  assert.ok(!route.startsWith('/') && !route.includes('\\') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe selected route: ${route}`);
  const sourceBytes = fs.readFileSync(path.join(sourceRoot, route));
  assert.equal(sourceBytes.byteLength, entry.bytes, `selected route size changed: ${route}`);
  const sourceHash = sha256(sourceBytes);
  assert.equal(sourceHash, entry.sha256, `selected route hash changed: ${route}`);
  const previous = inspectPath(targetRoot, route);
  const previousHash = previous.exists ? sha256(previous.bytes) : null;
  const status = !previous.exists ? 'create' : previousHash === sourceHash ? 'same' : 'replace';
  return {
    path: route,
    status,
    source_bytes: sourceBytes.byteLength,
    source_sha256: sourceHash,
    target_exists: previous.exists,
    previous_bytes: previous.exists ? previous.bytes.byteLength : null,
    previous_sha256: previousHash,
    previous_mode: previous.exists ? previous.mode : null,
    backup_path: status === 'replace' ? `SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-26/pass01/${route}` : null,
  };
});
assert.equal(files.length, new Set(files.map((entry) => entry.path)).size, 'selected routes contain duplicates');
const counts = Object.fromEntries(['create', 'replace', 'same'].map((status) => [status, files.filter((entry) => entry.status === status).length]));

const targets = [];
for (const entry of files.filter((candidate) => candidate.status === 'replace')) {
  const absolute = path.join(targetRoot, entry.path);
  const lsof = spawnSync('lsof', ['-nP', absolute], { encoding: 'utf8' });
  assert.equal(lsof.error, undefined, `lsof failed for ${entry.path}: ${lsof.error}`);
  targets.push({ path: entry.path, open: lsof.status === 0, lsof_status: lsof.status, output: lsof.stdout.trim() });
  assert.equal(lsof.status, 1, `replacement target is open: ${entry.path}\n${lsof.stdout}`);
}

const report = {
  pass: 'pass01',
  selected_source_head: selectedBuild.sourceHead,
  selected_source_tree: selectedBuild.sourceTree,
  selected_route_count: files.length,
  source_bytes_total: files.reduce((sum, entry) => sum + entry.source_bytes, 0),
  backup_bytes_total: files.filter((entry) => entry.status === 'replace').reduce((sum, entry) => sum + entry.previous_bytes, 0),
  counts,
  preview_root_relative: 'SpinonPreview',
  backup_root_relative: 'SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-26/pass01',
  tailnet: { backend_state: tailscale.BackendState, online: tailscale.Self.Online, self_dns: tailscale.Self.DNSName, self_ips: tailscale.Self.TailscaleIPs },
  preview_listener: listener.stdout.trim(),
  remote_roadmap_preflight: { url: `${origin}/spinon/roadmap.html`, status: response.status },
  replacement_targets_closed: targets.length === counts.replace && targets.every((entry) => !entry.open),
  no_deletes: true,
  files,
};
const handleReport = { pass: 'pass01', checked_replacement_targets: targets.length, all_replacements_closed: targets.every((entry) => !entry.open), targets };
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(handleReportPath, `${JSON.stringify(handleReport, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, files: undefined, replacement_targets_closed: handleReport.all_replacements_closed }, null, 2)}\n`);
