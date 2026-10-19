import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const evidenceRoot = 'spec/internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20';
const run = process.env.SPINON_TAILNET_RUN;
if (!['pass10', 'pass11', 'pass12', 'pass13', 'pass14'].includes(run)) throw new Error('SPINON_TAILNET_RUN must be pass10, pass11, pass12, pass13, or pass14');
const manifestRelative = `${evidenceRoot}/SHA256SUMS-preview-inputs.txt`;
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
  'spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/README.md',
  'spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/chrome155-observation.json',
  'spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/chrome155-percentage-gaps.png',
  'spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/runs/c114-gap-20261019T181753Z/android-avd/screenshot-grid-gap-cyclic-calc-min-basis.png',
  'spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/runs/c114-gap-20261019T181753Z/ios-simulator/screenshot-grid-gap-cyclic-calc-min-basis.png',
  `${evidenceRoot}/README.md`,
  `${evidenceRoot}/sync-review-20.md`,
  `${evidenceRoot}/pr-diff-review-20.md`,
  manifestRelative,
];
if (run === 'pass11') {
  selected.push(
    `${evidenceRoot}/preflight-pass10.json`,
    `${evidenceRoot}/open-handles-pass10.json`,
    `${evidenceRoot}/copy-and-backup-pass10.json`,
    `${evidenceRoot}/served-verification-pass10.json`,
    `${evidenceRoot}/browser-check-pass10.json`,
    `${evidenceRoot}/browser-close-pass10.json`,
    `${evidenceRoot}/html-route-check-pass10.json`,
    `${evidenceRoot}/prepare-root-pass10.json`,
    `${evidenceRoot}/sanitize-pass10.json`,
    `${evidenceRoot}/build-pass10.log`,
    `${evidenceRoot}/pass10-tailnet-roadmap-agent-browser.png`,
    `${evidenceRoot}/pass10-tailnet-status-agent-browser.png`,
    `${evidenceRoot}/pass10-tailnet-plan-agent-browser.png`,
    `${evidenceRoot}/pass10-tailnet-evidence-agent-browser.png`,
  );
}
if (run === 'pass12') {
  selected.push(
    `${evidenceRoot}/preflight-pass10.json`,
    `${evidenceRoot}/open-handles-pass10.json`,
    `${evidenceRoot}/copy-and-backup-pass10.json`,
    `${evidenceRoot}/served-verification-pass10.json`,
    `${evidenceRoot}/browser-check-pass10.json`,
    `${evidenceRoot}/browser-close-pass10.json`,
    `${evidenceRoot}/html-route-check-pass10.json`,
    `${evidenceRoot}/prepare-root-pass10.json`,
    `${evidenceRoot}/sanitize-pass10.json`,
    `${evidenceRoot}/build-pass10.log`,
    `${evidenceRoot}/pass10-tailnet-roadmap-agent-browser.png`,
    `${evidenceRoot}/pass10-tailnet-status-agent-browser.png`,
    `${evidenceRoot}/pass10-tailnet-plan-agent-browser.png`,
    `${evidenceRoot}/pass10-tailnet-evidence-agent-browser.png`,
    `${evidenceRoot}/preflight-pass11.json`,
    `${evidenceRoot}/open-handles-pass11.json`,
    `${evidenceRoot}/copy-and-backup-pass11.json`,
    `${evidenceRoot}/served-verification-pass11.json`,
    `${evidenceRoot}/browser-check-pass11.json`,
    `${evidenceRoot}/browser-close-pass11.json`,
    `${evidenceRoot}/html-route-check-pass11.json`,
    `${evidenceRoot}/prepare-root-pass11.json`,
    `${evidenceRoot}/sanitize-pass11.json`,
    `${evidenceRoot}/build-pass11.log`,
  );
}
if (run === 'pass13') {
  selected.push(
    `${evidenceRoot}/preflight-pass12.json`,
    `${evidenceRoot}/open-handles-pass12.json`,
    `${evidenceRoot}/copy-and-backup-pass12.json`,
    `${evidenceRoot}/served-verification-pass12.json`,
    `${evidenceRoot}/browser-check-pass12.json`,
    `${evidenceRoot}/browser-close-pass12.json`,
    `${evidenceRoot}/html-route-check-pass12.json`,
    `${evidenceRoot}/prepare-root-pass12.json`,
    `${evidenceRoot}/sanitize-pass12.json`,
    `${evidenceRoot}/build-pass12.log`,
    `${evidenceRoot}/final-tailnet-sync-agent-browser-pass12.png`,
  );
}
if (run === 'pass14') {
  selected.push(
    `${evidenceRoot}/preflight-pass10.json`, `${evidenceRoot}/open-handles-pass10.json`,
    `${evidenceRoot}/copy-and-backup-pass10.json`, `${evidenceRoot}/served-verification-pass10.json`,
    `${evidenceRoot}/browser-check-pass10.json`, `${evidenceRoot}/browser-close-pass10.json`,
    `${evidenceRoot}/html-route-check-pass10.json`, `${evidenceRoot}/prepare-root-pass10.json`,
    `${evidenceRoot}/sanitize-pass10.json`, `${evidenceRoot}/build-pass10.log`,
    `${evidenceRoot}/pass10-tailnet-roadmap-agent-browser.png`, `${evidenceRoot}/pass10-tailnet-status-agent-browser.png`,
    `${evidenceRoot}/pass10-tailnet-plan-agent-browser.png`, `${evidenceRoot}/pass10-tailnet-evidence-agent-browser.png`,
    `${evidenceRoot}/preflight-pass11.json`, `${evidenceRoot}/open-handles-pass11.json`,
    `${evidenceRoot}/copy-and-backup-pass11.json`, `${evidenceRoot}/served-verification-pass11.json`,
    `${evidenceRoot}/browser-check-pass11.json`, `${evidenceRoot}/browser-close-pass11.json`,
    `${evidenceRoot}/html-route-check-pass11.json`, `${evidenceRoot}/prepare-root-pass11.json`,
    `${evidenceRoot}/sanitize-pass11.json`, `${evidenceRoot}/build-pass11.log`,
    `${evidenceRoot}/preflight-pass12.json`, `${evidenceRoot}/open-handles-pass12.json`,
    `${evidenceRoot}/copy-and-backup-pass12.json`, `${evidenceRoot}/served-verification-pass12.json`,
    `${evidenceRoot}/browser-check-pass12.json`, `${evidenceRoot}/browser-close-pass12.json`,
    `${evidenceRoot}/html-route-check-pass12.json`, `${evidenceRoot}/prepare-root-pass12.json`,
    `${evidenceRoot}/sanitize-pass12.json`, `${evidenceRoot}/build-pass12.log`,
    `${evidenceRoot}/final-tailnet-sync-agent-browser-pass12.png`,
    `${evidenceRoot}/preflight-pass13.json`, `${evidenceRoot}/open-handles-pass13.json`,
    `${evidenceRoot}/copy-and-backup-pass13.json`, `${evidenceRoot}/served-verification-pass13.json`,
    `${evidenceRoot}/browser-check-pass13.json`, `${evidenceRoot}/browser-close-pass13.json`,
    `${evidenceRoot}/html-route-check-pass13.json`, `${evidenceRoot}/prepare-root-pass13.json`,
    `${evidenceRoot}/sanitize-pass13.json`, `${evidenceRoot}/build-pass13.log`,
  );
}
const finalEvidenceManifest = `${evidenceRoot}/SHA256SUMS.txt`;

function visit(relative, files) {
  if (relative === manifestRelative || relative === finalEvidenceManifest) return;
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
