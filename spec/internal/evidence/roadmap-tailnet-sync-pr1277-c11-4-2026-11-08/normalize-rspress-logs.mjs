import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { gzipSync, gunzipSync } from 'node:zlib';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const reportPath = path.join(evidence, 'rspress-log-normalization.json');
const names = ['selected-rspress-build.log', 'selected-rspress-build-pass03.log'];
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(reportPath), `기존 normalization report 보존: ${reportPath}`);
const reports = [];

for (const name of names) {
  const file = path.join(evidence, name);
  const compressedPath = `${file}.raw.gz`;
  assert.ok(!fs.existsSync(compressedPath), `기존 raw archive 보존: ${compressedPath}`);
  const original = fs.readFileSync(file);
  const rawText = original.toString('utf8');
  assert.ok(Buffer.from(rawText, 'utf8').equals(original), `invalid UTF-8 in captured log: ${name}`);
  const rawLines = rawText.replace(/\r\n/g, '\n').split('\n');
  const normalizedText = `${rawLines.map((line) => line.replace(/[\t ]+$/g, '')).join('\n').replace(/\n+$/g, '')}\n`;
  const normalized = Buffer.from(normalizedText, 'utf8');
  const compressed = gzipSync(original, { level: 9, mtime: 0 });
  fs.writeFileSync(compressedPath, compressed, { flag: 'wx' });
  assert.ok(gunzipSync(fs.readFileSync(compressedPath)).equals(original), `raw log archive roundtrip mismatch: ${name}`);
  const temporary = `${file}.normalize-${randomUUID()}.tmp`;
  const fd = fs.openSync(temporary, 'wx', fs.statSync(file).mode & 0o777);
  try { fs.writeFileSync(fd, normalized); fs.fsyncSync(fd); } finally { fs.closeSync(fd); }
  fs.renameSync(temporary, file);
  reports.push({
    file: name,
    rawArchive: `${name}.raw.gz`,
    capturedBytes: original.length,
    capturedSha256: sha256(original),
    compressedBytes: compressed.length,
    compressedSha256: sha256(compressed),
    normalizedBytes: normalized.length,
    normalizedSha256: sha256(normalized),
    normalization: 'trim trailing horizontal whitespace and trailing blank lines only',
    exactRawBytesRecoverable: true,
  });
}

const report = {
  buildExitCodes: { 'selected-rspress-build.log': 0, 'selected-rspress-build-pass03.log': 0 },
  report: reports,
  semanticBuildOutputChanged: false,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
