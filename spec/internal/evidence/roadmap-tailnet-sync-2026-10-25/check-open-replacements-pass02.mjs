import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const preflight = JSON.parse(fs.readFileSync(path.join(evidenceDirectory, 'tailnet-preflight-pass02.json'), 'utf8'));
const reportPath = path.join(evidenceDirectory, 'target-open-handles-pass02.json');
const targetRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const replacements = preflight.files.filter((entry) => entry.status === 'replace');
const open = [];
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite open-handle report');
for (const entry of replacements) {
  const target = path.join(targetRoot, entry.path);
  const result = spawnSync('lsof', ['-nP', target], { encoding: 'utf8' });
  assert.equal(result.error, undefined, `lsof failed for ${entry.path}: ${result.error}`);
  assert.equal(result.stderr, '', `lsof diagnostic for ${entry.path}: ${result.stderr}`);
  if (result.status === 0) open.push({ path: entry.path, lsof: result.stdout.trim() });
  else assert.equal(result.status, 1, `unexpected lsof status ${result.status} for ${entry.path}`);
}
const report = { pass: 'pass02', checked_replacements: replacements.length, open_replacements: open, all_replacements_closed: open.length === 0 };
assert.equal(open.length, 0, 'refusing to overwrite open preview files');
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
