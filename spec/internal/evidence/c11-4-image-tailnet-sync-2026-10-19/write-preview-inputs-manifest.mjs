import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const manifestRelative = 'spec/internal/evidence/c11-4-image-tailnet-sync-2026-10-19/SHA256SUMS-preview-inputs.txt';
const selected = [
  'spec/0001-conformance.md',
  'spec/STATUS.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/internal/README.md',
  'plan/c11-css-grid.md',
  'plan/c11-4-grid-track-sizing.md',
  'spec/public/icon.svg',
  'spec/public/logo.svg',
  'spec/public/roadmap.svg',
  'spec/public/roadmap.html',
  'spec/internal/evidence/c11-4-image-intrinsic-matrix-2026-10-19',
  'spec/internal/evidence/c11-4-image-tailnet-sync-2026-10-19',
];
const postPrepareNames = new Set([
  'prepare-root-pass5.json',
  'selected-build-pass5-final.log',
  'sanitize-pass5.json',
  'preflight-pass5.json',
  'open-handles-pass5.json',
  'copy-and-backup-pass5.json',
  'copy-verification-pass5.json',
  'backup-verification-pass5.json',
  'served-verification-pass5.json',
]);
const finalEvidenceManifest = 'spec/internal/evidence/c11-4-image-tailnet-sync-2026-10-19/SHA256SUMS.txt';

function visit(relative, files) {
  const fullPath = path.join(repoRoot, relative);
  const stat = fs.lstatSync(fullPath);
  if (stat.isSymbolicLink()) throw new Error(`symlink in selected inputs: ${relative}`);
  if (stat.isDirectory()) {
    for (const name of fs.readdirSync(fullPath).sort()) {
      visit(path.posix.join(relative, name), files);
    }
    return;
  }
  if (!stat.isFile()) throw new Error(`unsupported selected input: ${relative}`);
  if (relative === manifestRelative || relative === finalEvidenceManifest
      || postPrepareNames.has(path.posix.basename(relative))) return;
  files.push(relative);
}

const files = [];
for (const source of selected) visit(source, files);
files.sort();
const lines = files.map((relative) => {
  const bytes = fs.readFileSync(path.join(repoRoot, relative));
  const digest = createHash('sha256').update(bytes).digest('hex');
  return `${digest}  ${relative}`;
});
fs.writeFileSync(path.join(repoRoot, manifestRelative), `${lines.join('\n')}\n`);
process.stdout.write(`${JSON.stringify({
  manifest: manifestRelative,
  source_file_count: files.length,
  excluded_generated_names: [...postPrepareNames].sort(),
}, null, 2)}\n`);
