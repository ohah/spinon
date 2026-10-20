import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const previousRoot = path.join(repoRoot, 'build/spinon/c11-4-text-intrinsic-physical-tailnet-root-pass24');
const selectedRoot = path.join(repoRoot, 'build/spinon/c11-4-typed-implicit-tailnet-root-pass25');
const evidenceName = 'c11-4-typed-implicit-physical-android-2026-10-21';

if (!fs.statSync(previousRoot).isDirectory()) throw new Error(`previous selected root missing: ${previousRoot}`);
if (fs.existsSync(selectedRoot)) throw new Error(`refusing to reuse selected root: ${selectedRoot}`);

function assertPlainTree(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`symlink in previous selected root: ${path.relative(previousRoot, fullPath)}`);
    if (entry.isDirectory()) assertPlainTree(fullPath);
    else if (!entry.isFile()) throw new Error(`unsupported previous-root entry: ${path.relative(previousRoot, fullPath)}`);
  }
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function copyFile(sourceRelative, targetRelatives) {
  const source = path.join(repoRoot, sourceRelative);
  const bytes = fs.readFileSync(source);
  for (const targetRelative of targetRelatives) {
    const target = path.join(selectedRoot, targetRelative);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.copyFileSync(source, target);
  }
  overlay.push({ source: sourceRelative, targets: targetRelatives, bytes: bytes.byteLength, sha256: sha256(bytes) });
}

function copyTree(sourceRelative, targetRelatives) {
  const source = path.join(repoRoot, sourceRelative);
  const files = [];
  const visit = (directory, prefix = '') => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
      const fullPath = path.join(directory, entry.name);
      if (entry.isSymbolicLink()) throw new Error(`symlink in source evidence: ${relative}`);
      if (entry.isDirectory()) visit(fullPath, relative);
      else if (entry.isFile()) files.push(relative);
      else throw new Error(`unsupported source evidence entry: ${relative}`);
    }
  };
  visit(source);
  for (const targetRelative of targetRelatives) {
    const target = path.join(selectedRoot, targetRelative);
    if (fs.existsSync(target)) throw new Error(`refusing to overlay existing evidence tree: ${targetRelative}`);
    fs.cpSync(source, target, { recursive: true, errorOnExist: true, force: false });
  }
  overlay.push({ source: sourceRelative, targets: targetRelatives, file_count: files.length, files });
}

assertPlainTree(previousRoot);
fs.cpSync(previousRoot, selectedRoot, { recursive: true, errorOnExist: true, force: false });

const overlay = [];
copyFile('spec/STATUS.md', ['STATUS.md', 'spec/STATUS.md']);
copyFile('plan/c11-4-grid-track-sizing.md', ['plan/c11-4-grid-track-sizing.md', 'spec/plan/c11-4-grid-track-sizing.md']);
copyFile('spec/internal/README.md', ['internal/README.md', 'spec/internal/README.md']);
copyFile('spec/public/roadmap.html', ['public/roadmap.html']);
copyTree(`spec/internal/evidence/${evidenceName}`, [
  `internal/evidence/${evidenceName}`,
  `spec/internal/evidence/${evidenceName}`,
]);

const result = {
  previous_root: path.relative(repoRoot, previousRoot),
  selected_root: path.relative(repoRoot, selectedRoot),
  source_revision: '4d69a05e316685497ea3950bb325c81a79608819',
  overlays: overlay,
};
const output = path.join(evidenceDir, 'selected-inputs-pass25.json');
fs.writeFileSync(output, `${JSON.stringify(result, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ selected_root: result.selected_root, overlay_count: overlay.length, evidence_files: overlay.at(-1).file_count, manifest: path.relative(repoRoot, output) }, null, 2)}\n`);
