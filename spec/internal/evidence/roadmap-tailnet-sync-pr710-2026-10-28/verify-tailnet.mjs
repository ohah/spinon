import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const publication = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'selected-publication.json'), 'utf8'));
const copy = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'tailnet-copy.json'), 'utf8'));
const reportPath = path.join(evidenceDirectory, 'tailnet-verify.json');
const localBase = 'http://127.0.0.1:8768/spinon/';
const remoteBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite verification report');
assert.equal(copy.sourceHead, publication.sourceHead);
assert.equal(copy.routeCount, publication.routeCount);
assert.equal(copy.noRoutesDeleted, true);

function urlFor(base, route) {
  return `${base}${route.split('/').map(encodeURIComponent).join('/')}`;
}

const results = new Array(publication.routes.length);
let cursor = 0;
async function worker() {
  while (true) {
    const index = cursor;
    cursor += 1;
    if (index >= publication.routes.length) return;
    const expected = publication.routes[index];
    const sourceBytes = fs.readFileSync(path.join(repositoryRoot, publication.publicationRoot, expected.path));
    assert.equal(sha256(sourceBytes), expected.sha256, `local publication source changed: ${expected.path}`);
    const [localResponse, remoteResponse] = await Promise.all([
      fetch(urlFor(localBase, expected.path), { signal: AbortSignal.timeout(30000), redirect: 'manual' }),
      fetch(urlFor(remoteBase, expected.path), { signal: AbortSignal.timeout(30000), redirect: 'manual' }),
    ]);
    const [localBytes, remoteBytes] = await Promise.all([localResponse.arrayBuffer(), remoteResponse.arrayBuffer()]);
    const localDigest = sha256(Buffer.from(localBytes));
    const remoteDigest = sha256(Buffer.from(remoteBytes));
    assert.equal(localResponse.status, 200, `local HTTP ${localResponse.status}: ${expected.path}`);
    assert.equal(remoteResponse.status, 200, `Tailnet HTTP ${remoteResponse.status}: ${expected.path}`);
    assert.equal(localBytes.byteLength, expected.bytes, `local size mismatch: ${expected.path}`);
    assert.equal(remoteBytes.byteLength, expected.bytes, `Tailnet size mismatch: ${expected.path}`);
    assert.equal(localDigest, expected.sha256, `local hash mismatch: ${expected.path}`);
    assert.equal(remoteDigest, expected.sha256, `Tailnet hash mismatch: ${expected.path}`);
    results[index] = {
      path: expected.path,
      bytes: expected.bytes,
      sha256: expected.sha256,
      localStatus: localResponse.status,
      tailnetStatus: remoteResponse.status,
    };
  }
}

await Promise.all(Array.from({ length: 4 }, worker));
const report = {
  sourceHead: publication.sourceHead,
  sourceTree: publication.sourceTree,
  targetRoot: copy.targetRoot,
  localBase,
  tailnetBase: remoteBase,
  routeCount: results.length,
  totalBytes: results.reduce((sum, entry) => sum + entry.bytes, 0),
  allLocalHttp200: results.every((entry) => entry.localStatus === 200),
  allTailnetHttp200: results.every((entry) => entry.tailnetStatus === 200),
  allHashesMatchPublication: true,
  routes: results,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
