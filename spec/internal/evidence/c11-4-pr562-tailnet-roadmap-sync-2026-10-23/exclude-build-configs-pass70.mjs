import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-root-pass70-selected');
const reportPath = path.join(evidenceDirectory, 'build-config-exclusion-pass70.json');
assert.ok(fs.lstatSync(selectedRoot).isDirectory(), 'selected root is required');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite build-config exclusion report');
const files = [];
function walk(directory, relative = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const childRelative = relative ? `${relative}/${entry.name}` : entry.name;
    const absolute = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected root: ${childRelative}`);
    if (entry.isDirectory()) walk(absolute, childRelative);
    else if (entry.isFile() && entry.name.startsWith('rspress') && entry.name.endsWith('.config.ts')) files.push({ absolute, relative: childRelative });
  }
}
walk(selectedRoot);
assert.ok(files.length > 0, 'expected copied Tailnet build config sources');
for (const file of files) {
  const stat = fs.lstatSync(file.absolute);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `unsafe build config: ${file.relative}`);
  fs.unlinkSync(file.absolute);
}
const report = {
  pass: 'pass70', excludedSourceFiles: files.map((file) => file.relative).sort(),
  reason: 'build configuration helpers are not documentation routes',
  sourceWorkspaceFilesPreserved: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
