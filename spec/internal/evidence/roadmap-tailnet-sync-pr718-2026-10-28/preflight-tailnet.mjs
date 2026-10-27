import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { spawnSync, execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const publicationPath = path.join(evidenceDirectory, 'selected-publication.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-preflight.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr718-2026-10-28/pass01';
const previewUrl = 'https://macstudio.tailed42f2.ts.net/spinon/roadmap.html';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(fs.lstatSync(publicationPath).isFile());
assert.ok(!fs.existsSync(reportPath), `refusing to overwrite report: ${reportPath}`);
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup path: ${backupRoot}`);
const publication = JSON.parse(fs.readFileSync(publicationPath, 'utf8'));
const targetStat = fs.lstatSync(targetRoot);
assert.ok(targetStat.isDirectory() && !targetStat.isSymbolicLink(), 'unsafe preview root');

const tailscale = JSON.parse(execFileSync('tailscale', ['status', '--json'], { encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 }));
assert.equal(tailscale.BackendState, 'Running', 'Tailscale backend is not running');
assert.equal(tailscale.Self?.Online, true, 'Mac Studio is not online on Tailnet');
const listen = spawnSync('lsof', ['-nP', '-iTCP:8768', '-sTCP:LISTEN'], { encoding: 'utf8' });
assert.equal(listen.status, 0, 'preview server is not listening on port 8768');

const baselineResponse = await fetch(`${previewUrl}?preflight=${Date.now()}`, { cache: 'no-store' });
assert.equal(baselineResponse.status, 200, 'existing Tailnet roadmap is not HTTP 200');
const baselineBytes = Buffer.from(await baselineResponse.arrayBuffer());
const baseline = { status: baselineResponse.status, bytes: baselineBytes.length, sha256: sha256(baselineBytes) };
assert.equal(baseline.sha256, '5adfb59f9c4823e099fa74ad3b276edc06750fd2f4f94cf8aa62bcf278fc3bd8', 'served roadmap changed after baseline capture');
assert.deepEqual(fs.readFileSync(path.join(targetRoot, 'roadmap.html')), baselineBytes, 'server root roadmap differs from the served baseline');

function inspectPath(relative) {
  const parts = relative.split('/');
  let current = targetRoot;
  for (let index = 0; index < parts.length; index += 1) {
    current = path.join(current, parts[index]);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in target path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory target parent: ${relative}`);
      else {
        assert.ok(stat.isFile(), `non-file target: ${relative}`);
        const bytes = fs.readFileSync(current);
        return { exists: true, bytes: bytes.length, sha256: sha256(bytes), mode: stat.mode & 0o777 };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

const entries = [];
for (const source of publication.routes) {
  const current = inspectPath(source.path);
  let openHandleCheck = { checked: false, noOpenHandles: true, output: '' };
  if (current.exists && current.sha256 !== source.sha256) {
    const result = spawnSync('lsof', ['-nP', path.join(targetRoot, source.path)], { encoding: 'utf8' });
    openHandleCheck = { checked: true, noOpenHandles: result.status !== 0, output: result.stdout.trim() };
    assert.notEqual(result.status, 0, `replacement target is open: ${source.path}\n${result.stdout}`);
  }
  entries.push({
    path: source.path,
    source_bytes: source.bytes,
    source_sha256: source.sha256,
    target_exists: current.exists,
    previous_bytes: current.bytes ?? null,
    previous_sha256: current.sha256 ?? null,
    previous_mode: current.mode ?? null,
    status: !current.exists ? 'create' : current.sha256 === source.sha256 ? 'same' : 'replace',
    openHandleCheck,
  });
}

const report = {
  sourceHead: publication.sourceHead,
  sourceTree: publication.sourceTree,
  targetRoot,
  backupRoot,
  tailscale: { backendState: tailscale.BackendState, selfOnline: tailscale.Self.Online, hostname: tailscale.Self.DNSName },
  serverListen: listen.stdout.trim(),
  baselineUrl: previewUrl,
  baselineResponse: baseline,
  targetCount: entries.length,
  createCount: entries.filter((entry) => entry.status === 'create').length,
  replaceCount: entries.filter((entry) => entry.status === 'replace').length,
  sameCount: entries.filter((entry) => entry.status === 'same').length,
  allReplacementTargetsClosed: entries.every((entry) => entry.openHandleCheck.noOpenHandles),
  noRoutesDeleted: true,
  entries,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, entries: undefined }, null, 2)}\n`);
