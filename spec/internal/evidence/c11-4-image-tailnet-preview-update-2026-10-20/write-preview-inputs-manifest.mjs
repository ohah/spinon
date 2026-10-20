import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const evidenceRoot = 'spec/internal/evidence/c11-4-image-tailnet-preview-update-2026-10-20';
const operationEvidenceRoot = 'spec/internal/evidence/c11-4-pr395-tailnet-roadmap-sync-2026-10-21';
const run = process.env.SPINON_TAILNET_RUN;
if (!['pass6', 'pass7', 'pass8', 'pass9', 'pass11', 'pass12', 'pass13', 'pass15', 'pass16'].includes(run)) throw new Error('SPINON_TAILNET_RUN must be pass6, pass7, pass8, pass9, pass11, pass12, pass13, pass15, or pass16');
const manifestRelative = ['pass15', 'pass16'].includes(run)
  ? `${operationEvidenceRoot}/SHA256SUMS-preview-inputs.txt`
  : `${evidenceRoot}/SHA256SUMS-preview-inputs.txt`;
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
];
if (run === 'pass8') {
  selected.push(
    `${evidenceRoot}/README.md`,
    `${evidenceRoot}/preflight-pass6.json`,
    `${evidenceRoot}/open-handles-pass6.json`,
    `${evidenceRoot}/copy-and-backup-pass6.json`,
    `${evidenceRoot}/served-verification-pass6.json`,
    `${evidenceRoot}/pass6-tailnet-roadmap-agent-browser.png`,
    `${evidenceRoot}/pass6-tailnet-plan-agent-browser.png`,
    `${evidenceRoot}/pass6-tailnet-evidence-agent-browser.png`,
  );
}
if (run === 'pass9') {
  selected.push(
    `${evidenceRoot}/README.md`,
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
if (run === 'pass11') {
  selected.push(
    'plan/c11-4-auto-repeat-two-track-min-size.md',
    'spec/internal/evidence/c11-4-auto-repeat-neighbors-2026-10-20',
    'spec/internal/evidence/c11-4-auto-repeat-two-track-min-size-precomparison-2026-10-20',
    'spec/internal/evidence/c11-4-auto-repeat-two-track-min-size-2026-10-20',
  );
}
if (run === 'pass8') {
  selected.push(
    `${evidenceRoot}/sync-review-20.md`,
    `${evidenceRoot}/preflight-pass7.json`,
    `${evidenceRoot}/open-handles-pass7.json`,
    `${evidenceRoot}/copy-and-backup-pass7.json`,
    `${evidenceRoot}/served-verification-pass7.json`,
    `${evidenceRoot}/browser-check-pass7.json`,
    `${evidenceRoot}/pass7-tailnet-roadmap-agent-browser.png`,
    `${evidenceRoot}/pass7-tailnet-status-agent-browser.png`,
    `${evidenceRoot}/pass7-tailnet-plan-agent-browser.png`,
    `${evidenceRoot}/pass7-tailnet-evidence-agent-browser.png`,
  );
}
if (['pass15', 'pass16'].includes(run)) {
  selected.push(
    'plan/c11-4-auto-fit-min-size-axis.md',
    'tests/fixtures/css/c11/grid-auto-fit-min-size-axis-c114.html',
    'spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20',
    'tests/fixtures/css/c11/grid-track-sizing-c114-multispan-ordering.html',
  );
}
if (['pass12', 'pass13', 'pass15', 'pass16'].includes(run)) {
  selected.push(
    'plan/c11-4-two-axis-auto-repeat.md',
    'tests/fixtures/css/c11/grid-auto-repeat-both-axes-c114.html',
    'spec/internal/evidence/c11-4-two-axis-auto-repeat-precomparison-2026-10-20',
    'spec/internal/evidence/c11-4-two-axis-auto-repeat-2026-10-20',
  );
  const syncEvidence = 'spec/internal/evidence/c11-4-two-axis-tailnet-sync-2026-10-20';
  const publishedEvidence = [
    'README.md',
    'sync-review-20.md',
    'SHA256SUMS-preview-inputs-pass12.txt',
    'input-manifest-write-initial-failure-pass13.json',
    'open-handles-initial-pass13.json',
    'preserved-open-targets-pass13.json',
    'prepare-root-pass12.json',
    'selected-build-pass12.log',
    'sanitize-pass12.json',
    'preflight-pass12.json',
    'open-handles-pass12.json',
    'copy-and-backup-pass12.json',
    'served-verification-pass12.json',
    'browser-check-pass12.json',
    'status-check-pass12.json',
    'plan-check-pass12.json',
    'evidence-check-pass12.json',
    'precomparison-check-pass12.json',
    'sync-report-check-pass12.json',
    'html-route-check-pass12.json',
    'route-check-initial-overstrict-pass12.json',
    'new-c11-4-routes-pass12.json',
    'browser-close-pass12.json',
    'tailnet-roadmap-agent-browser-pass12.png',
    'tailnet-status-agent-browser-pass12.png',
    'tailnet-plan-agent-browser-pass12.png',
    'tailnet-evidence-agent-browser-pass12.png',
  ];
  if (run === 'pass13') {
    publishedEvidence.push(
      'browser-check-pass13.json',
      'status-check-pass13.json',
      'plan-check-pass13.json',
      'tailnet-roadmap-agent-browser-pass13.png',
    );
  }
  selected.push(...(run === 'pass12'
    ? [`${syncEvidence}/README.md`]
    : publishedEvidence.map((name) => `${syncEvidence}/${name}`)));
}
if (['pass15', 'pass16'].includes(run)) {
  const multispanEvidence = 'spec/internal/evidence/c11-4-multispan-ordering-physical-android-2026-10-21';
  selected.push(
    `${multispanEvidence}/README.md`,
    `${multispanEvidence}/chrome155-observation.json`,
    `${multispanEvidence}/chrome155-multispan-precomparison.png`,
    `${multispanEvidence}/implementation-review-20.md`,
    `${multispanEvidence}/ios-simulator-review-20.md`,
    `${multispanEvidence}/pr-diff-review-30.md`,
    `${multispanEvidence}/physical-multispan-observation.json`,
    `${multispanEvidence}/physical-final-span3-before-span2-app-view.png`,
    `${multispanEvidence}/physical-final-auto-fill-overflow-neighbors-pixelcopy.png`,
    `${multispanEvidence}/runtime-physc114-final-rebuild-20261021-01-competing-span2-forward.log`,
    `${multispanEvidence}/runtime-physc114-final-rebuild-20261021-02-competing-span2-reversed.log`,
    `${multispanEvidence}/runtime-physc114-final-rebuild-20261021-03-span2-before-span3.log`,
    `${multispanEvidence}/runtime-physc114-final-rebuild-20261021-04-span3-before-span2.log`,
    `${multispanEvidence}/runtime-physc114-final-rebuild-20261021-overflow-02-auto-fill-overflow-neighbors-pixelcopy.log`,
    `${multispanEvidence}/runtime-span2-before-span3-validator-prerun.log`,
    `${multispanEvidence}/runtime-span3-before-span2-validator-prerun.log`,
    `${multispanEvidence}/android-connected-2026-10-21`,
    `${multispanEvidence}/ios-simulator-c114-multispan-ordering-ios-simulator-20261020T165531254Z-8ec3cd96`,
  );
}
const finalEvidenceManifest = `${evidenceRoot}/SHA256SUMS.txt`;
const runManifestRelative = ['pass15', 'pass16'].includes(run)
  ? `${operationEvidenceRoot}/SHA256SUMS-preview-inputs-${run}.txt`
  : `spec/internal/evidence/c11-4-two-axis-tailnet-sync-2026-10-20/SHA256SUMS-preview-inputs-${run}.txt`;

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
  if (relative === manifestRelative || relative === finalEvidenceManifest || relative === runManifestRelative) return;
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
fs.mkdirSync(path.dirname(path.join(repoRoot, manifestRelative)), { recursive: true });
fs.writeFileSync(path.join(repoRoot, manifestRelative), `${lines.join('\n')}\n`);
fs.writeFileSync(path.join(repoRoot, runManifestRelative), `${lines.join('\n')}\n`);
process.stdout.write(`${JSON.stringify({
  manifest: manifestRelative,
  run_manifest: runManifestRelative,
  source_file_count: files.length,
  excluded_self_referential_inputs: [manifestRelative, finalEvidenceManifest, runManifestRelative],
}, null, 2)}\n`);
