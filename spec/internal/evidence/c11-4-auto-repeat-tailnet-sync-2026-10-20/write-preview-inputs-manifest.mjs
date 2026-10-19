import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const evidenceRoot = 'spec/internal/evidence/c11-4-image-tailnet-preview-update-2026-10-20';
const outputEvidenceRoot = 'spec/internal/evidence/c11-4-auto-repeat-tailnet-sync-2026-10-20';
const run = process.env.SPINON_TAILNET_RUN;
if (run !== 'pass10') throw new Error('SPINON_TAILNET_RUN must be pass10');
const manifestRelative = `${outputEvidenceRoot}/SHA256SUMS-preview-inputs.txt`;
const selected = [
  'spec/0001-conformance.md',
  'spec/STATUS.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/internal/README.md',
  'plan/c11-css-grid.md',
  'plan/c11-4-grid-track-sizing.md',
  'plan/c11-4-image-ratio-fixed-gutter.md',
  'tests/fixtures/css/c11/grid-track-sizing-c114-image-ratio-fixed-gutter.html',
  'spec/public/icon.svg',
  'spec/public/logo.svg',
  'spec/public/roadmap.svg',
  'spec/public/roadmap.html',
  'spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19',
  'spec/internal/evidence/c11-4-image-ratio-fixed-gutter-2026-10-20',
  'spec/internal/evidence/c11-4-image-intrinsic-matrix-2026-10-19',
  'spec/internal/evidence/c11-4-image-tailnet-sync-2026-10-19',
  'spec/internal/evidence/c11-4-auto-repeat-explicit-neighbors-precomparison-2026-10-20',
  'spec/internal/evidence/c11-4-auto-repeat-neighbors-2026-10-20',
];
if (run === 'pass10') {
  selected.push(
    `${evidenceRoot}/README.md`,
    `${evidenceRoot}/SHA256SUMS-preview-inputs.txt`,
    `${evidenceRoot}/sync-review-20.md`,
    `${evidenceRoot}/preflight-pass6.json`,
    `${evidenceRoot}/open-handles-pass6.json`,
    `${evidenceRoot}/copy-and-backup-pass6.json`,
    `${evidenceRoot}/served-verification-pass6.json`,
    `${evidenceRoot}/pass6-tailnet-roadmap-agent-browser.png`,
    `${evidenceRoot}/pass6-tailnet-plan-agent-browser.png`,
    `${evidenceRoot}/pass6-tailnet-evidence-agent-browser.png`,
    `${evidenceRoot}/preflight-pass7.json`,
    `${evidenceRoot}/open-handles-pass7.json`,
    `${evidenceRoot}/copy-and-backup-pass7.json`,
    `${evidenceRoot}/served-verification-pass7.json`,
    `${evidenceRoot}/browser-check-pass7.json`,
    `${evidenceRoot}/pass7-tailnet-roadmap-agent-browser.png`,
    `${evidenceRoot}/pass7-tailnet-status-agent-browser.png`,
    `${evidenceRoot}/pass7-tailnet-plan-agent-browser.png`,
    `${evidenceRoot}/pass7-tailnet-evidence-agent-browser.png`,
    `${evidenceRoot}/preflight-pass8.json`,
    `${evidenceRoot}/open-handles-pass8.json`,
    `${evidenceRoot}/copy-and-backup-pass8.json`,
    `${evidenceRoot}/served-verification-pass8.json`,
    `${evidenceRoot}/browser-check-pass8.json`,
    `${evidenceRoot}/browser-close-pass8.json`,
    `${evidenceRoot}/cleanup-wrong-staging-root-pass8.json`,
    `${evidenceRoot}/preflight-pass8-wrong-staging-root.json`,
    `${evidenceRoot}/copy-and-backup-pass8-wrong-staging-root.json`,
    `${evidenceRoot}/final-tailnet-roadmap-agent-browser.png`,
    `${evidenceRoot}/final-tailnet-status-agent-browser.png`,
    `${evidenceRoot}/final-tailnet-plan-agent-browser.png`,
    `${evidenceRoot}/final-tailnet-evidence-agent-browser.png`,
    `${evidenceRoot}/final-tailnet-sync-agent-browser.png`,
  );
}
const finalEvidenceManifest = `${evidenceRoot}/SHA256SUMS.txt`;

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
  if (relative === manifestRelative || relative === finalEvidenceManifest) return;
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
  excluded_self_referential_inputs: [manifestRelative, finalEvidenceManifest],
}, null, 2)}\n`);
