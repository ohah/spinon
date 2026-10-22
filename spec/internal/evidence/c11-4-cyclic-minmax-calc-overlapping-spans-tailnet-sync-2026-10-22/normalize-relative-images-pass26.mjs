import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const pass = process.env.SPINON_TAILNET_PASS;
assert.ok(['pass26-initial', 'pass26-final'].includes(pass ?? ''), 'set pass26-initial or pass26-final');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-spans-tailnet-root-' + pass + '-selected');
const outputPath = path.join(evidenceDirectory, 'image-normalization-' + pass + '.json');
const changes = [];
const skippedMissing = [];
const visited = [];

function digest(value) {
  return createHash('sha256').update(value).digest('hex');
}

function collectMarkdown(directory, relativeDirectory = '') {
  const result = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const fullPath = path.join(directory, entry.name);
    const relativePath = relativeDirectory ? relativeDirectory + '/' + entry.name : entry.name;
    if (entry.isSymbolicLink()) throw new Error('symlink in selected root: ' + relativePath);
    if (entry.isDirectory()) result.push(...collectMarkdown(fullPath, relativePath));
    else if (entry.isFile() && /\.mdx?$/.test(entry.name)) result.push({ fullPath, relativePath });
  }
  return result;
}

function normalizeLine(line, relativePath) {
  return line.replace(/(!\[[^\]]*\]\()([^)]*)(\))/g, (whole, prefix, destinationAndTitle, suffix) => {
    const leading = (destinationAndTitle.match(/^\s*/) || [''])[0];
    const remainder = destinationAndTitle.slice(leading.length);
    let target;
    let wrapped = false;
    let tail;
    if (remainder.startsWith('<')) {
      const closing = remainder.indexOf('>');
      if (closing < 0) return whole;
      target = remainder.slice(1, closing);
      wrapped = true;
      tail = remainder.slice(closing + 1);
    } else {
      const match = remainder.match(/^(\S+)([\s\S]*)$/);
      if (!match) return whole;
      target = match[1];
      tail = match[2];
    }
    if (!target || target.startsWith('/') || target.startsWith('./') || target.startsWith('../')
        || /^[a-z][a-z0-9+.-]*:/i.test(target) || target.startsWith('#')) return whole;
    const localTarget = target.split(/[?#]/, 1)[0];
    let decodedTarget = localTarget;
    try { decodedTarget = decodeURIComponent(localTarget); } catch {}
    const absoluteTarget = path.resolve(selectedRoot, path.dirname(relativePath), decodedTarget);
    if (!absoluteTarget.startsWith(selectedRoot + path.sep)) return whole;
    let stat;
    try { stat = fs.lstatSync(absoluteTarget); } catch (error) {
      if (error.code === 'ENOENT') {
        skippedMissing.push({ source: relativePath, target });
        return whole;
      }
      throw error;
    }
    if (!stat.isFile() || stat.isSymbolicLink()) return whole;
    const normalizedTarget = './' + target;
    const replacement = wrapped ? '<' + normalizedTarget + '>' + tail : normalizedTarget + tail;
    changes.push({ source: relativePath, before: target, after: normalizedTarget });
    return prefix + leading + replacement + suffix;
  });
}

for (const file of collectMarkdown(selectedRoot)) {
  visited.push(file.relativePath);
  const before = fs.readFileSync(file.fullPath, 'utf8');
  const beforeHash = digest(before);
  const lines = before.split('\n');
  let fenceCharacter = null;
  const after = lines.map((line) => {
    const fence = line.match(/^\s*(\x60{3,}|~{3,})/);
    if (fence) {
      const marker = fence[1];
      const character = marker[0];
      if (fenceCharacter === null) fenceCharacter = character;
      else if (fenceCharacter === character) fenceCharacter = null;
      return line;
    }
    if (fenceCharacter !== null) return line;
    return normalizeLine(line, file.relativePath);
  }).join('\n');
  if (after !== before) {
    fs.writeFileSync(file.fullPath, after);
    changes.push({ file: file.relativePath, before_sha256: beforeHash, after_sha256: digest(after) });
  }
}

const output = {
  schemaVersion: 1,
  pass,
  selectedRoot: path.relative(repositoryRoot, selectedRoot).split(path.sep).join('/'),
  markdownFilesVisited: visited.length,
  changedMarkdownFiles: [...new Set(changes.filter((entry) => entry.file).map((entry) => entry.file))].length,
  normalizedImageReferences: changes.filter((entry) => entry.before && entry.after).length,
  skippedMissingLocalImageReferences: skippedMissing.length,
  changes,
  skippedMissing,
  scope: 'selected RSPress root only; tracked Markdown sources remain unchanged',
};
fs.writeFileSync(outputPath, JSON.stringify(output, null, 2) + '\n', { flag: 'wx' });
process.stdout.write(JSON.stringify({
  pass,
  selectedRoot: output.selectedRoot,
  markdownFilesVisited: output.markdownFilesVisited,
  changedMarkdownFiles: output.changedMarkdownFiles,
  normalizedImageReferences: output.normalizedImageReferences,
  skippedMissingLocalImageReferences: output.skippedMissingLocalImageReferences,
  outputPath: path.relative(repositoryRoot, outputPath),
}, null, 2) + '\n');
