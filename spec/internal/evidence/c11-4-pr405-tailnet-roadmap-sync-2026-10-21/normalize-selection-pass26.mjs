import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(evidenceDir, '../../../..');
const selectedRoot = path.join(repoRoot, 'build/spinon/c11-4-pr405-tailnet-root-pass26');
const excludedEvidenceDirs = [
  'c11-4-pr397-tailnet-roadmap-sync-2026-10-21',
  'c11-4-typed-implicit-tailnet-sync-2026-10-21',
];

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function inventory(directory) {
  let files = 0;
  let bytes = 0;
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const target = path.join(directory, entry.name);
    if (entry.isSymbolicLink()) throw new Error(`refusing symlink in selected evidence: ${target}`);
    if (entry.isDirectory()) {
      const child = inventory(target);
      files += child.files;
      bytes += child.bytes;
    } else if (entry.isFile()) {
      files += 1;
      bytes += fs.statSync(target).size;
    } else {
      throw new Error(`unsupported selected evidence entry: ${target}`);
    }
  }
  return { files, bytes };
}

const excluded = [];
for (const base of ['internal/evidence', 'spec/internal/evidence']) {
  for (const name of excludedEvidenceDirs) {
    const relative = `${base}/${name}`;
    const target = path.join(selectedRoot, relative);
    if (!fs.existsSync(target)) continue;
    const contents = inventory(target);
    fs.rmSync(target, { recursive: true, force: false });
    excluded.push({ path: relative, ...contents, reason: 'operational Tailnet build scripts are not linked from the current official status or roadmap' });
  }
}

const pathRepairs = [
  {
    relative: 'internal/evidence/c13-3-android-physical-2026-10-21/README.md',
    replacements: [['](gesture-positive-after.png)', '](./gesture-positive-after.png)']],
  },
  {
    relative: 'spec/internal/evidence/c13-3-android-physical-2026-10-21/README.md',
    replacements: [['](gesture-positive-after.png)', '](./gesture-positive-after.png)']],
  },
  {
    relative: 'internal/evidence/c13-3-ios-recognizer-event-pairing-2026-10-21/README.md',
    replacements: [
      ['](screenshots/direct-touch-controls.png)', '](./screenshots/direct-touch-controls.png)'],
      ['](screenshots/scroll-type-masks.png)', '](./screenshots/scroll-type-masks.png)'],
    ],
  },
  {
    relative: 'spec/internal/evidence/c13-3-ios-recognizer-event-pairing-2026-10-21/README.md',
    replacements: [
      ['](screenshots/direct-touch-controls.png)', '](./screenshots/direct-touch-controls.png)'],
      ['](screenshots/scroll-type-masks.png)', '](./screenshots/scroll-type-masks.png)'],
    ],
  },
];

const repairs = [];
for (const entry of pathRepairs) {
  const markdownPath = path.join(selectedRoot, entry.relative);
  let text = fs.readFileSync(markdownPath, 'utf8');
  const beforeSha256 = sha256(Buffer.from(text));
  let replacementCount = 0;
  for (const [before, after] of entry.replacements) {
    if (!text.includes(before)) throw new Error(`expected source image link missing in ${entry.relative}: ${before}`);
    const target = before.slice(2, -1);
    if (!fs.statSync(path.join(path.dirname(markdownPath), target)).isFile()) {
      throw new Error(`relative image target missing: ${entry.relative} -> ${target}`);
    }
    text = text.replace(before, after);
    replacementCount += 1;
  }
  fs.writeFileSync(markdownPath, text);
  repairs.push({
    path: entry.relative,
    source_sha256_before: beforeSha256,
    selected_sha256_after: sha256(Buffer.from(text)),
    replacement_count: replacementCount,
    rationale: 'RSPress treats bare markdown image paths as package imports; dot-relative paths load the tracked adjacent evidence images',
  });
}

const manifest = {
  selected_root_relative: path.relative(repoRoot, selectedRoot).split(path.sep).join('/'),
  excluded_evidence_dirs: excluded,
  deleted_from_tailnet_target: false,
  path_repairs: repairs,
};
fs.writeFileSync(path.join(evidenceDir, 'selection-normalization-pass26.json'), `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({
  selected_root: manifest.selected_root_relative,
  excluded_directory_count: excluded.length,
  excluded_file_count: excluded.reduce((sum, entry) => sum + entry.files, 0),
  markdown_path_repair_count: repairs.reduce((sum, entry) => sum + entry.replacement_count, 0),
  deleted_from_tailnet_target: false,
  manifest_path: 'selection-normalization-pass26.json',
}, null, 2)}\n`);
