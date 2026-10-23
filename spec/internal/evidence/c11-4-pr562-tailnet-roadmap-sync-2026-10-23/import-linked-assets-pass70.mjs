import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const selectedRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-root-pass70-selected');
const reportPath = path.join(evidenceDirectory, 'linked-assets-pass70.json');
assert.ok(fs.lstatSync(selectedRoot).isDirectory(), 'selected root is required');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite linked asset report');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const foundImages = new Map();
const missingInWorkspace = new Set();
const copied = [];
const external = /^(?:https?:|data:|blob:|#|\/\/)/i;

function repositoryCandidates(relative) {
  const candidates = [relative];
  if (relative.startsWith('internal/')) candidates.push(`spec/${relative}`);
  if (relative.startsWith('public/internal/')) candidates.push(`spec/${relative.slice('public/'.length)}`);
  if (relative.startsWith('public/spec/internal/')) candidates.push(relative.slice('public/'.length));
  if (relative.startsWith('spec/plan/')) candidates.push(relative.slice('spec/'.length));
  if (relative.startsWith('public/plan/')) candidates.push(relative.slice('public/'.length));
  if (relative.startsWith('public/spec/plan/')) candidates.push(relative.slice('public/'.length));
  if (relative.startsWith('public/')) candidates.push(`spec/${relative}`);
  return [...new Set(candidates)];
}

function normalizeTarget(markdownPath, destination) {
  const clean = destination.trim().replace(/^<|>$/g, '').split(/[?#]/, 1)[0];
  if (!clean || external.test(clean)) return null;
  let decoded;
  try { decoded = decodeURIComponent(clean); } catch { decoded = clean; }
  if (path.isAbsolute(decoded)) return null;
  const absolute = path.resolve(path.dirname(markdownPath), decoded);
  if (absolute !== selectedRoot && !absolute.startsWith(selectedRoot + path.sep)) return null;
  return path.relative(selectedRoot, absolute);
}

function walk(directory, relative = '') {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const childRelative = relative ? `${relative}/${entry.name}` : entry.name;
    const absolute = path.join(directory, entry.name);
    assert.ok(!entry.isSymbolicLink(), `symlink in selected root: ${childRelative}`);
    if (entry.isDirectory()) walk(absolute, childRelative);
    else if (entry.isFile() && entry.name.toLowerCase().endsWith('.md')) {
      const text = fs.readFileSync(absolute, 'utf8');
      const destinations = [];
      for (const match of text.matchAll(/!\[[^\]]*\]\(\s*(?:<([^>]+)>|([^\s)]+))/g)) destinations.push(match[1] ?? match[2]);
      for (const match of text.matchAll(/<img\b[^>]*\bsrc=["']([^"']+)["']/gi)) destinations.push(match[1]);
      for (const destination of destinations) {
        const relativeTarget = normalizeTarget(absolute, destination);
        if (relativeTarget) foundImages.set(`${childRelative}\0${relativeTarget}`, { markdown: childRelative, target: relativeTarget });
      }
    }
  }
}
walk(selectedRoot);

for (const { markdown, target } of foundImages.values()) {
  const targetPath = path.join(selectedRoot, target);
  if (fs.existsSync(targetPath)) continue;
  let sourcePath;
  for (const candidate of repositoryCandidates(target)) {
    const possible = path.join(repositoryRoot, candidate);
    if (!possible.startsWith(repositoryRoot + path.sep)) continue;
    try {
      const stat = fs.lstatSync(possible);
      if (stat.isFile() && !stat.isSymbolicLink()) { sourcePath = possible; break; }
    } catch {}
  }
  if (!sourcePath) {
    missingInWorkspace.add(`${markdown} -> ${target}`);
    continue;
  }
  fs.mkdirSync(path.dirname(targetPath), { recursive: true });
  fs.linkSync(sourcePath, targetPath);
  const bytes = fs.readFileSync(targetPath);
  copied.push({ markdown, target, source: path.relative(repositoryRoot, sourcePath), bytes: bytes.byteLength, sha256: sha256(bytes) });
}

const report = {
  pass: 'pass70', markdownImageReferences: foundImages.size,
  copiedCount: copied.length, copiedBytes: copied.reduce((sum, item) => sum + item.bytes, 0),
  missingWorkspaceAssets: [...missingInWorkspace].sort(), copied,
  sourceWorkspaceFilesPreserved: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({
  pass: report.pass, markdownImageReferences: report.markdownImageReferences,
  copiedCount: report.copiedCount, copiedBytes: report.copiedBytes,
  missingWorkspaceAssets: report.missingWorkspaceAssets,
}, null, 2)}\n`);
if (missingInWorkspace.size) process.exitCode = 1;
