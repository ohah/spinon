import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourcePath = path.join(repositoryRoot, 'spec/public/roadmap.html');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-site-pass70-selected');
const reportPath = path.join(evidenceDirectory, 'public-roadmap-staging-pass70.json');
const branchHead = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repositoryRoot, encoding: 'utf8' }).trim();
const expectedBranchHead = '10ae04a09f9f0b1b97056693dad0438a2c022b62';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.equal(branchHead, expectedBranchHead, 'unexpected Tailnet docs candidate revision');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite public roadmap staging report');
const sourceBytes = fs.readFileSync(sourcePath);
const sourceText = sourceBytes.toString('utf8');
assert.ok(sourceText.includes('/spinon/internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/'), 'roadmap evidence routes must use the established /internal/ path');
assert.ok(!sourceText.includes('/spinon/spec/internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/'), 'stale /spec/internal/ evidence route remains');

const routes = ['roadmap.html'];
const staged = [];
for (const relative of routes) {
  const targetPath = path.join(outputRoot, relative);
  const targetStat = fs.lstatSync(targetPath);
  assert.ok(targetStat.isFile() && !targetStat.isSymbolicLink(), `unsafe build route: ${relative}`);
  const temporaryPath = `${targetPath}.spinon-pass70-${randomUUID()}.tmp`;
  try {
    fs.copyFileSync(sourcePath, temporaryPath, fs.constants.COPYFILE_EXCL);
    fs.renameSync(temporaryPath, targetPath);
  } catch (error) {
    if (fs.existsSync(temporaryPath)) fs.unlinkSync(temporaryPath);
    throw error;
  }
  const bytes = fs.readFileSync(targetPath);
  assert.equal(sha256(bytes), sha256(sourceBytes), `staged route hash mismatch: ${relative}`);
  staged.push({ path: relative, bytes: bytes.byteLength, sha256: sha256(bytes), method: 'unique sibling temporary file followed by atomic rename' });
}
const report = {
  pass: 'pass70',
  source: 'spec/public/roadmap.html at branch head',
  branchHead,
  sourceBytes: sourceBytes.byteLength,
  sourceSha256: sha256(sourceBytes),
  staged,
  noOtherBuildRoutesChanged: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
