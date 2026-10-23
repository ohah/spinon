import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-root-pass70-selected');
const reportPath = path.join(evidenceDirectory, 'image-path-normalization-pass70.json');
assert.ok(fs.lstatSync(selectedRoot).isDirectory(), 'selected root is required');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite image-path report');
const files = [];
let replacements = 0;
const bareRelativeImage = /(!\[[^\]]*\]\()\s*((?!(?:\.{1,2}\/|\/|[A-Za-z][A-Za-z0-9+.-]*:|#))[^\s)]+)(?=\))/g;
function walk(directory, relative = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const childRelative = relative ? `${relative}/${entry.name}` : entry.name;
    const absolute = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected root: ${childRelative}`);
    if (entry.isDirectory()) walk(absolute, childRelative);
    else if (entry.isFile() && entry.name.toLowerCase().endsWith('.md')) {
      const original = fs.readFileSync(absolute, 'utf8');
      let count = 0;
      const normalized = original.replace(bareRelativeImage, (_match, prefix, target) => {
        count += 1;
        return `${prefix}./${target}`;
      });
      if (count) {
        assert.ok(fs.lstatSync(absolute).isFile(), `unsafe markdown path: ${childRelative}`);
        fs.unlinkSync(absolute);
        fs.writeFileSync(absolute, normalized, { flag: 'wx' });
        files.push({ path: childRelative, imagePathsNormalized: count });
        replacements += count;
      }
    }
  }
}
walk(selectedRoot);
const report = { pass: 'pass70', filesChanged: files.length, replacements, files, sourceWorkspaceFilesPreserved: true };
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ pass: report.pass, filesChanged: report.filesChanged, replacements, files }, null, 2)}\n`);
