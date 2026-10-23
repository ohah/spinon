import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr540-tailnet-root-pass65-selected/public');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr540-tailnet-site-pass65-selected');
const reportPath = path.join(evidenceDirectory, 'public-hardlink-pass65.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(fs.lstatSync(sourceRoot).isDirectory(), `missing selected public directory: ${sourceRoot}`);
assert.ok(fs.lstatSync(outputRoot).isDirectory(), `missing selected build output: ${outputRoot}`);
assert.ok(!fs.existsSync(reportPath), `refusing to overwrite report: ${reportPath}`);

const entries = [];
function collect(directory, relative = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const childRelative = relative ? `${relative}/${entry.name}` : entry.name;
    const sourcePath = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected public files: ${childRelative}`);
    if (entry.isDirectory()) collect(sourcePath, childRelative);
    else if (entry.isFile()) entries.push({ relative: childRelative, sourcePath });
    else throw new Error(`unsupported selected public entry: ${childRelative}`);
  }
}
collect(sourceRoot);

let linked = 0;
let same = 0;
let bytesLinked = 0;
for (const entry of entries) {
  const outputPath = path.join(outputRoot, entry.relative);
  fs.mkdirSync(path.dirname(outputPath), { recursive: true });
  if (fs.existsSync(outputPath)) {
    const sourceBytes = fs.readFileSync(entry.sourcePath);
    const outputStat = fs.lstatSync(outputPath);
    assert.ok(outputStat.isFile() && !outputStat.isSymbolicLink(), `public path collides with generated output: ${entry.relative}`);
    assert.equal(sha256(fs.readFileSync(outputPath)), sha256(sourceBytes), `public path differs from generated output: ${entry.relative}`);
    same += 1;
    continue;
  }
  fs.linkSync(entry.sourcePath, outputPath);
  linked += 1;
  bytesLinked += fs.statSync(entry.sourcePath).size;
}
const report = {
  pass: 'pass65',
  sourceRoot: 'build/spinon/c11-4-pr540-tailnet-root-pass65-selected/public',
  outputRoot: 'build/spinon/c11-4-pr540-tailnet-site-pass65-selected',
  files: entries.length, linked, same, bytesLinked,
  storageMethod: 'same-volume hard links; no public file contents duplicated locally',
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
