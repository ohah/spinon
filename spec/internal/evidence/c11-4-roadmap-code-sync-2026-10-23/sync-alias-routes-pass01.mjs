import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-roadmap-code-sync-site-pass01-selected');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const previewOrigin = 'https://macstudio.tailed42f2.ts.net/spinon/';
const reportPath = path.join(evidenceDirectory, 'alias-route-verification-pass01.json');
const routes = [
  'spec/internal/evidence/c11-4-roadmap-code-sync-2026-10-23/README.html',
  'spec/internal/evidence/c11-4-roadmap-code-sync-2026-10-23/pr-diff-review-20.html',
];
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite alias route report');

function ensureDirectoryChain(relative) {
  let current = targetRoot;
  for (const part of relative.split('/').slice(0, -1)) {
    current = path.join(current, part);
    try {
      const stat = fs.lstatSync(current);
      assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `unsafe alias directory: ${current}`);
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
      fs.mkdirSync(current);
    }
  }
}

const verified = [];
for (const relative of routes) {
  const sourcePath = path.join(sourceRoot, relative);
  const sourceStat = fs.lstatSync(sourcePath);
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `missing built alias: ${relative}`);
  const sourceBytes = fs.readFileSync(sourcePath);
  const targetPath = path.join(targetRoot, relative);
  ensureDirectoryChain(relative);
  let operation = 'already-present';
  if (fs.existsSync(targetPath)) {
    const current = fs.lstatSync(targetPath);
    assert.ok(current.isFile() && !current.isSymbolicLink(), `unsafe existing route: ${relative}`);
    assert.equal(sha256(fs.readFileSync(targetPath)), sha256(sourceBytes), `existing alias differs; refusing overwrite: ${relative}`);
  } else {
    const temporaryPath = `${targetPath}.spinon-alias-pass01-${randomUUID()}.tmp`;
    try {
      fs.writeFileSync(temporaryPath, sourceBytes, { flag: 'wx' });
      fs.renameSync(temporaryPath, targetPath);
    } catch (error) {
      if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
      throw error;
    }
    operation = 'added';
  }
  const localBytes = fs.readFileSync(targetPath);
  assert.equal(sha256(localBytes), sha256(sourceBytes), `local alias hash mismatch: ${relative}`);
  const response = await fetch(new URL(relative, previewOrigin));
  const remoteBytes = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, `HTTP status ${response.status}: ${relative}`);
  assert.equal(sha256(remoteBytes), sha256(sourceBytes), `served alias hash mismatch: ${relative}`);
  verified.push({ path: relative, operation, status: response.status, bytes: remoteBytes.length, sha256: sha256(remoteBytes) });
}

const report = { pass: 'pass01', routes: verified, allHttp200AndHashesMatch: true, noDeletes: true };
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
