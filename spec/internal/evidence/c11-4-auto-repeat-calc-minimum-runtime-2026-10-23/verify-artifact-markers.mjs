import { createHash } from 'node:crypto';
import { readFileSync, statSync } from 'node:fs';

const [archivePath, expectedArg] = process.argv.slice(2);
if (!archivePath || !['present', 'absent'].includes(expectedArg)) {
  throw new Error('usage: node verify-artifact-markers.mjs <archive> <present|absent>');
}

const markers = [
  'c11_4_auto_repeat_calc_minimum',
  'columns-calc-fill',
  'columns-fixed-fill-control',
  'columns-calc-fit',
  'rows-calc-fill',
  'rows-fixed-fill-control',
  'rows-calc-fit',
];
const bytes = readFileSync(archivePath);
const checks = markers.map((marker) => ({
  marker,
  present: bytes.includes(Buffer.from(marker, 'utf8')),
}));
const expectedPresent = expectedArg === 'present';
const pass = checks.every(({ present }) => present === expectedPresent);
console.log(JSON.stringify({
  archivePath,
  bytes: statSync(archivePath).size,
  sha256: createHash('sha256').update(bytes).digest('hex'),
  expectedMarkers: expectedArg,
  checks,
  pass,
}, null, 2));
if (!pass) process.exitCode = 1;
