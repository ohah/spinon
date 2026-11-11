import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidenceDirectory, '../../../..');
const preview = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const manifestPath = path.join(evidenceDirectory, process.argv[2] ?? 'tailnet-copy-manifest.json');
const reportPath = path.join(evidenceDirectory, process.argv[3] ?? 'served-verification.json');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
const localBase = 'http://127.0.0.1:8768/spinon/';
const tailnetBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const records = [];

assert.equal(manifest.previewServerPid, 90711);
assert.ok(!fs.existsSync(reportPath), `existing verification report is preserved: ${reportPath}`);

async function verify(record) {
  const source = fs.readFileSync(path.join(repo, record.source));
  assert.equal(source.length, record.bytes, `source byte count differs: ${record.source}`);
  assert.equal(sha256(source), record.sha256, `source hash differs: ${record.source}`);
  const file = fs.readFileSync(path.join(preview, record.destination));
  assert.equal(file.length, record.bytes, `preview file byte count differs: ${record.destination}`);
  assert.equal(sha256(file), record.sha256, `preview file hash differs: ${record.destination}`);
  if (record.backup) {
    const backup = fs.readFileSync(path.join(preview, record.backup));
    assert.equal(backup.length, record.before.bytes, `backup byte count differs: ${record.backup}`);
    assert.equal(sha256(backup), record.before.sha256, `backup hash differs: ${record.backup}`);
  }
  const route = record.destination.split('/').map(encodeURIComponent).join('/');
  const [localResponse, tailnetResponse] = await Promise.all([
    fetch(localBase + route),
    fetch(tailnetBase + route),
  ]);
  assert.equal(localResponse.status, 200, `local HTTP ${localResponse.status}: ${record.destination}`);
  assert.equal(tailnetResponse.status, 200, `Tailnet HTTP ${tailnetResponse.status}: ${record.destination}`);
  const [local, tailnet] = await Promise.all([localResponse.arrayBuffer(), tailnetResponse.arrayBuffer()]);
  const localBytes = Buffer.from(local);
  const tailnetBytes = Buffer.from(tailnet);
  assert.equal(localBytes.length, record.bytes, `local HTTP byte count differs: ${record.destination}`);
  assert.equal(tailnetBytes.length, record.bytes, `Tailnet HTTP byte count differs: ${record.destination}`);
  assert.equal(sha256(localBytes), record.sha256, `local HTTP hash differs: ${record.destination}`);
  assert.equal(sha256(tailnetBytes), record.sha256, `Tailnet HTTP hash differs: ${record.destination}`);
  return {
    route: `/spinon/${record.destination}`,
    source: record.source,
    bytes: record.bytes,
    sha256: record.sha256,
    localHttp: localResponse.status,
    tailnetHttp: tailnetResponse.status,
    action: record.action,
  };
}

const concurrency = 8;
for (let start = 0; start < manifest.records.length; start += concurrency) {
  const batch = await Promise.all(manifest.records.slice(start, start + concurrency).map(verify));
  records.push(...batch);
  process.stdout.write(`VERIFIED ${records.length}/${manifest.records.length}\n`);
}

const report = {
  schema: 'spinon.tailnet-preview-served-verification/v1',
  sourceCommit: manifest.sourceCommit,
  previewServerPid: manifest.previewServerPid,
  localBase,
  tailnetBase,
  totalFiles: records.length,
  allLocalHttp200: records.every((item) => item.localHttp === 200),
  allTailnetHttp200: records.every((item) => item.tailnetHttp === 200),
  allSourcePreviewLocalTailnetHashesMatch: true,
  backedUpFilesVerified: manifest.records.filter((item) => item.backup !== null).length,
  fullSiteBuildClaim: false,
  records,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, records: undefined }, null, 2)}\n`);
