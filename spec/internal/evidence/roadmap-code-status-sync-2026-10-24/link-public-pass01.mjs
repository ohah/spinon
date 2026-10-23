import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-root-pass01-selected/public');
const outputRoot = path.join(repositoryRoot, 'build/spinon/roadmap-code-status-sync-20261024-site-pass01-selected');
const reportPath = path.join(evidenceDirectory, 'public-hardlink-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(sourceRoot).isDirectory(), 'selected public source is missing');
assert.ok(fs.lstatSync(outputRoot).isDirectory(), 'selected build output is missing');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite public asset report');

const entries = [];
function collect(directory, relative = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    const source = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected public assets: ${child}`);
    if (entry.isDirectory()) collect(source, child);
    else if (entry.isFile()) entries.push({ path: child, source });
    else throw new Error(`unsupported selected public entry: ${child}`);
  }
}
collect(sourceRoot);

let linked = 0;
let identical = 0;
let bytesLinked = 0;
for (const entry of entries) {
  const destination = path.join(outputRoot, entry.path);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  if (fs.existsSync(destination)) {
    const sourceStat = fs.lstatSync(entry.source);
    const destinationStat = fs.lstatSync(destination);
    assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `unsafe public source: ${entry.path}`);
    assert.ok(destinationStat.isFile() && !destinationStat.isSymbolicLink(), `public route collision: ${entry.path}`);
    assert.equal(sha256(fs.readFileSync(destination)), sha256(fs.readFileSync(entry.source)), `public route differs: ${entry.path}`);
    identical += 1;
  } else {
    fs.linkSync(entry.source, destination);
    linked += 1;
    bytesLinked += fs.statSync(entry.source).size;
  }
}
const report = {
  pass: 'pass01',
  source: path.relative(repositoryRoot, sourceRoot),
  output: path.relative(repositoryRoot, outputRoot),
  files: entries.length,
  linked,
  identical,
  bytesLinked,
  method: 'same-volume hard links; no public asset contents duplicated locally',
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
