import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const publicationPath = path.join(evidenceDirectory, 'selected-publication.json');
const reportPath = path.join(evidenceDirectory, 'tailnet-preflight.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupRoot = '/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-pr710-2026-10-28/pass01';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite preflight report');
assert.ok(!fs.existsSync(backupRoot), `refusing to reuse backup root: ${backupRoot}`);
const targetStat = fs.lstatSync(targetRoot);
assert.ok(targetStat.isDirectory() && !targetStat.isSymbolicLink(), `unsafe target root: ${targetRoot}`);
const publication = JSON.parse(fs.readFileSync(publicationPath, 'utf8'));
const entries = [];

function inspectTarget(relative) {
  let current = targetRoot;
  const parts = relative.split('/');
  for (const [index, part] of parts.entries()) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(!stat.isSymbolicLink(), `symlink in target path: ${relative}`);
      if (index < parts.length - 1) assert.ok(stat.isDirectory(), `non-directory target component: ${relative}`);
      else {
        assert.ok(stat.isFile(), `non-file target: ${relative}`);
        return { exists: true, bytes: fs.readFileSync(current), mode: stat.mode & 0o777 };
      }
    } catch (error) {
      if (error.code === 'ENOENT') return { exists: false };
      throw error;
    }
  }
  return { exists: false };
}

for (const entry of publication.routes) {
  const source = fs.readFileSync(path.join(repositoryRoot, publication.publicationRoot, entry.path));
  assert.equal(source.byteLength, entry.bytes, `publication size changed: ${entry.path}`);
  assert.equal(sha256(source), entry.sha256, `publication hash changed: ${entry.path}`);
  const current = inspectTarget(entry.path);
  let status;
  if (!current.exists) status = 'create';
  else if (current.bytes.byteLength === entry.bytes && sha256(current.bytes) === entry.sha256) status = 'same';
  else status = 'replace';

  let openHandleCheck = null;
  if (status === 'replace') {
    const destination = path.join(targetRoot, entry.path);
    const lsof = spawnSync('lsof', ['-nP', destination], { encoding: 'utf8' });
    assert.equal(lsof.error, undefined, `lsof failed for ${entry.path}: ${lsof.error}`);
    assert.ok(lsof.status === 0 || lsof.status === 1, `unexpected lsof status ${lsof.status} for ${entry.path}`);
    openHandleCheck = { noOpenHandles: lsof.status === 1, output: lsof.stdout.trim() };
  }
  entries.push({
    path: entry.path,
    source_bytes: entry.bytes,
    source_sha256: entry.sha256,
    target_exists: current.exists,
    previous_bytes: current.exists ? current.bytes.byteLength : null,
    previous_sha256: current.exists ? sha256(current.bytes) : null,
    previous_mode: current.exists ? current.mode : null,
    status,
    openHandleCheck,
  });
}

const counts = entries.reduce((result, entry) => {
  result[entry.status] = (result[entry.status] ?? 0) + 1;
  return result;
}, {});
const replacements = entries.filter((entry) => entry.status === 'replace');
const report = {
  sourceHead: publication.sourceHead,
  sourceTree: publication.sourceTree,
  targetRoot,
  backupRoot,
  routeCount: entries.length,
  counts,
  allReplacementTargetsClosed: replacements.every((entry) => entry.openHandleCheck?.noOpenHandles === true),
  noRoutesDeleted: true,
  entries,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, entries: undefined }, null, 2)}\n`);
