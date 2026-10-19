import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const run = process.argv[2];
if (!['pass10', 'pass11', 'pass12', 'pass13', 'pass14'].includes(run)) throw new Error('usage: node prepare-selected-root.mjs pass10|pass11|pass12|pass13|pass14');

const specRoot = path.join(repoRoot, 'spec');
const outputRoot = path.join(repoRoot, 'build/spinon', `c11-4-cyclic-grid-gap-tailnet-root-${run}`);
const sourceBranch = execFileSync('git', ['branch', '--show-current'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
if (fs.existsSync(outputRoot)) throw new Error(`refusing to replace existing staging root for ${run}`);

const selected = [
  ['spec/0001-conformance.md', '0001-conformance.md'],
  ['spec/STATUS.md', 'STATUS.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'internal/0059-c11-grid-layout.md'],
  ['spec/internal/README.md', 'internal/README.md'],
  ['plan/c11-css-grid.md', 'plan/c11-css-grid.md'],
  ['plan/c11-4-grid-track-sizing.md', 'plan/c11-4-grid-track-sizing.md'],
  ['plan/c11-4-image-ratio-fixed-gutter.md', 'plan/c11-4-image-ratio-fixed-gutter.md'],
  ['spec/internal/0059-c11-grid-layout.md', 'spec/internal/0059-c11-grid-layout.md'],
  ['spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19', 'internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19'],
  ['spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19', 'spec/internal/evidence/c11-4-image-ratio-fixed-gutter-precomparison-2026-10-19'],
  ['spec/internal/evidence/c11-4-image-ratio-fixed-gutter-2026-10-20', 'internal/evidence/c11-4-image-ratio-fixed-gutter-2026-10-20'],
  ['spec/internal/evidence/c11-4-image-ratio-fixed-gutter-2026-10-20', 'spec/internal/evidence/c11-4-image-ratio-fixed-gutter-2026-10-20'],
  ['tests/fixtures/css/c11/grid-track-sizing-c114-image-ratio-fixed-gutter.html', 'public/tests/fixtures/css/c11/grid-track-sizing-c114-image-ratio-fixed-gutter.html'],
  ['spec/public/icon.svg', 'public/icon.svg'],
  ['spec/public/logo.svg', 'public/logo.svg'],
  ['spec/public/roadmap.svg', 'public/roadmap.svg'],
  ['spec/public/roadmap.html', 'public/roadmap.html'],
  ['spec/internal/evidence/c11-4-image-intrinsic-matrix-2026-10-19', 'internal/evidence/c11-4-image-intrinsic-matrix-2026-10-19'],
  ['spec/internal/evidence/c11-4-image-tailnet-sync-2026-10-19', 'internal/evidence/c11-4-image-tailnet-sync-2026-10-19'],
  ['spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/README.md', 'internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/README.md'],
  ['spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/chrome155-observation.json', 'internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/chrome155-observation.json'],
  ['spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/chrome155-percentage-gaps.png', 'internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/chrome155-percentage-gaps.png'],
  ['spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/runs/c114-gap-20261019T181753Z/android-avd/screenshot-grid-gap-cyclic-calc-min-basis.png', 'internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/runs/c114-gap-20261019T181753Z/android-avd/screenshot-grid-gap-cyclic-calc-min-basis.png'],
  ['spec/internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/runs/c114-gap-20261019T181753Z/ios-simulator/screenshot-grid-gap-cyclic-calc-min-basis.png', 'internal/evidence/c11-4-cyclic-grid-gap-precomparison-2026-10-20/runs/c114-gap-20261019T181753Z/ios-simulator/screenshot-grid-gap-cyclic-calc-min-basis.png'],
  ['spec/internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/README.md', 'internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/README.md'],
  ['spec/internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/sync-review-20.md', 'internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/sync-review-20.md'],
  ['spec/internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/pr-diff-review-20.md', 'internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/pr-diff-review-20.md'],
  ['spec/internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/SHA256SUMS-preview-inputs.txt', 'internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/SHA256SUMS-preview-inputs.txt'],
];
const previewLinkRewrites = [];
function rewriteMarkdownPreviewLinks(sourceRelative, destinationRelative, source) {
  let count = 0;
  let localCount = 0;
  let githubCount = 0;
  const text = source.replace(/(!?\]\()([^)]+)(\))/g, (match, open, href, close) => {
    if (!href || href.startsWith('#') || href.startsWith('/') || /^[a-z][a-z\d+.-]*:/i.test(href)) return match;
    const suffixIndex = href.search(/[?#]/);
    const relativePath = suffixIndex === -1 ? href : href.slice(0, suffixIndex);
    const suffix = suffixIndex === -1 ? '' : href.slice(suffixIndex);
    if (!relativePath || /\s/.test(relativePath)) return match;
    const target = path.posix.normalize(path.posix.join(path.posix.dirname(sourceRelative), relativePath));
    if (target === '..' || target.startsWith('../') || path.posix.isAbsolute(target)) return match;
    const mappings = selected
      .filter(([selectedSource]) => target === selectedSource || target.startsWith(`${selectedSource}/`))
      .sort((left, right) => right[0].length - left[0].length);
    let outputHref;
    if (mappings.length > 0) {
      const [selectedSource, selectedDestination] = mappings[0];
      let destination = path.posix.join(selectedDestination, target.slice(selectedSource.length));
      if (destination.startsWith('public/')) destination = destination.slice('public/'.length);
      if (destination.endsWith('.md')) destination = `${destination.slice(0, -3)}.html`;
      outputHref = `/spinon/${destination.split('/').map(encodeURIComponent).join('/')}${suffix}`;
      localCount += 1;
    } else {
      outputHref = `https://github.com/ohah/spinon/blob/main/${target}${suffix}`;
      githubCount += 1;
    }
    count += 1;
    return `${open}${outputHref}${close}`;
  });
  return { text, source: sourceRelative, destination: destinationRelative, count, localCount, githubCount };
}
if (run === 'pass11') {
  const evidenceRoot = 'spec/internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20';
  const evidenceFiles = [
    'preflight-pass10.json',
    'open-handles-pass10.json',
    'copy-and-backup-pass10.json',
    'served-verification-pass10.json',
    'browser-check-pass10.json',
    'browser-close-pass10.json',
    'html-route-check-pass10.json',
    'prepare-root-pass10.json',
    'sanitize-pass10.json',
    'build-pass10.log',
    'pass10-tailnet-roadmap-agent-browser.png',
    'pass10-tailnet-status-agent-browser.png',
    'pass10-tailnet-plan-agent-browser.png',
    'pass10-tailnet-evidence-agent-browser.png',
  ];
  for (const name of evidenceFiles) {
    const relative = `${evidenceRoot}/${name}`;
    selected.push([relative, `internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/${name}`]);
  }
}
if (run === 'pass12') {
  const evidenceRoot = 'spec/internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20';
  const evidenceFiles = [
    'preflight-pass10.json',
    'open-handles-pass10.json',
    'copy-and-backup-pass10.json',
    'served-verification-pass10.json',
    'browser-check-pass10.json',
    'browser-close-pass10.json',
    'html-route-check-pass10.json',
    'prepare-root-pass10.json',
    'sanitize-pass10.json',
    'build-pass10.log',
    'pass10-tailnet-roadmap-agent-browser.png',
    'pass10-tailnet-status-agent-browser.png',
    'pass10-tailnet-plan-agent-browser.png',
    'pass10-tailnet-evidence-agent-browser.png',
    'preflight-pass11.json',
    'open-handles-pass11.json',
    'copy-and-backup-pass11.json',
    'served-verification-pass11.json',
    'browser-check-pass11.json',
    'browser-close-pass11.json',
    'html-route-check-pass11.json',
    'prepare-root-pass11.json',
    'sanitize-pass11.json',
    'build-pass11.log',
  ];
  for (const name of evidenceFiles) {
    const relative = `${evidenceRoot}/${name}`;
    selected.push([relative, `internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/${name}`]);
  }
}
if (run === 'pass13') {
  const evidenceRoot = 'spec/internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20';
  const evidenceFiles = [
    'preflight-pass12.json',
    'open-handles-pass12.json',
    'copy-and-backup-pass12.json',
    'served-verification-pass12.json',
    'browser-check-pass12.json',
    'browser-close-pass12.json',
    'html-route-check-pass12.json',
    'prepare-root-pass12.json',
    'sanitize-pass12.json',
    'build-pass12.log',
    'final-tailnet-sync-agent-browser-pass12.png',
  ];
  for (const name of evidenceFiles) {
    const relative = `${evidenceRoot}/${name}`;
    selected.push([relative, `internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/${name}`]);
  }
}
if (run === 'pass14') {
  const evidenceRoot = 'spec/internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20';
  const evidenceFiles = [
    'preflight-pass10.json', 'open-handles-pass10.json', 'copy-and-backup-pass10.json',
    'served-verification-pass10.json', 'browser-check-pass10.json', 'browser-close-pass10.json',
    'html-route-check-pass10.json', 'prepare-root-pass10.json', 'sanitize-pass10.json', 'build-pass10.log',
    'pass10-tailnet-roadmap-agent-browser.png', 'pass10-tailnet-status-agent-browser.png',
    'pass10-tailnet-plan-agent-browser.png', 'pass10-tailnet-evidence-agent-browser.png',
    'preflight-pass11.json', 'open-handles-pass11.json', 'copy-and-backup-pass11.json',
    'served-verification-pass11.json', 'browser-check-pass11.json', 'browser-close-pass11.json',
    'html-route-check-pass11.json', 'prepare-root-pass11.json', 'sanitize-pass11.json', 'build-pass11.log',
    'preflight-pass12.json', 'open-handles-pass12.json', 'copy-and-backup-pass12.json',
    'served-verification-pass12.json', 'browser-check-pass12.json', 'browser-close-pass12.json',
    'html-route-check-pass12.json', 'prepare-root-pass12.json', 'sanitize-pass12.json', 'build-pass12.log',
    'final-tailnet-sync-agent-browser-pass12.png',
    'preflight-pass13.json', 'open-handles-pass13.json', 'copy-and-backup-pass13.json',
    'served-verification-pass13.json', 'browser-check-pass13.json', 'browser-close-pass13.json',
    'html-route-check-pass13.json', 'prepare-root-pass13.json', 'sanitize-pass13.json', 'build-pass13.log',
  ];
  for (const name of evidenceFiles) {
    const relative = `${evidenceRoot}/${name}`;
    selected.push([relative, `internal/evidence/c11-4-cyclic-grid-gap-tailnet-sync-2026-10-20/${name}`]);
  }
}

const staticEvidenceExtensions = new Set([
  '.bin', '.css', '.csv', '.html', '.json', '.log', '.mjs', '.pdf', '.png', '.sh', '.svg', '.tar', '.ts', '.txt', '.webp',
]);
function copySelected(sourceRelative, destinationRelative) {
  if (sourceRelative.endsWith(`/prepare-root-${run}.json`)) return;
  const source = path.join(repoRoot, sourceRelative);
  if (!fs.existsSync(source)) throw new Error(`missing selected source: ${sourceRelative}`);
  const stat = fs.lstatSync(source);
  if (stat.isSymbolicLink()) throw new Error(`symlink in selected source: ${sourceRelative}`);
  if (stat.isDirectory()) {
    for (const name of fs.readdirSync(source).sort()) {
      copySelected(`${sourceRelative}/${name}`, `${destinationRelative}/${name}`);
    }
    return;
  }
  if (!stat.isFile()) throw new Error(`unsupported selected source: ${sourceRelative}`);
  const isEvidenceAsset = (destinationRelative.startsWith('internal/evidence/')
    || destinationRelative.startsWith('spec/internal/evidence/'))
    && path.extname(source) !== '.md'
    && staticEvidenceExtensions.has(path.extname(source));
  const isMarkdownImage = ['.png', '.svg', '.webp'].includes(path.extname(source));
  let bytes = fs.readFileSync(source);
  if (path.extname(source) === '.md') {
    const rewritten = rewriteMarkdownPreviewLinks(sourceRelative, destinationRelative, bytes.toString('utf8'));
    bytes = Buffer.from(rewritten.text, 'utf8');
    if (rewritten.count > 0) previewLinkRewrites.push({
      source: rewritten.source,
      destination: rewritten.destination,
      count: rewritten.count,
      local_count: rewritten.localCount,
      github_count: rewritten.githubCount,
    });
  }
  const destinations = isEvidenceAsset
    ? (isMarkdownImage
      ? [destinationRelative, path.join('public', destinationRelative)]
      : [path.join('public', destinationRelative)])
    : [destinationRelative];
  for (const stagedRelative of destinations) {
    const destination = path.join(outputRoot, stagedRelative);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, bytes);
    fs.chmodSync(destination, stat.mode & 0o777);
  }
}

for (const [sourceRelative, destinationRelative] of selected) {
  copySelected(sourceRelative, destinationRelative);
}

const textExtensions = new Set(['.md', '.html', '.json', '.log', '.txt', '.mjs', '.ts', '.sh']);
const redacted = [];
function visit(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      visit(fullPath);
      continue;
    }
    if (!entry.isFile() || !textExtensions.has(path.extname(entry.name))) continue;
    const before = fs.readFileSync(fullPath, 'utf8');
    const after = before
      .replace(/\/Users\/[^/\s"'<>:,)}\]]+(?:\/[^\s"'<>:,)}\]]*)*/g, '[host-path-redacted]')
      .replace(/\/private\/tmp(?:\/[^\s"'<>:,)}\]]*)*/g, '[worktree-path-redacted]');
    if (after !== before) {
      fs.writeFileSync(fullPath, after, 'utf8');
      redacted.push(path.relative(outputRoot, fullPath).split(path.sep).join('/'));
    }
  }
}
visit(outputRoot);

const files = [];
function collect(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) collect(fullPath);
    else if (entry.isFile()) files.push(path.relative(outputRoot, fullPath).split(path.sep).join('/'));
  }
}
collect(outputRoot);
files.sort();
process.stdout.write(`${JSON.stringify({
  run,
  source_branch: sourceBranch,
  source_commit: sourceCommit,
  output_root_relative: path.relative(repoRoot, outputRoot).split(path.sep).join('/'),
  selected_sources: selected.map(([source, destination]) => ({ source, destination })),
  file_count: files.length,
  redacted_file_count: redacted.length,
  redacted_files: redacted.sort(),
  preview_link_rewrites: previewLinkRewrites,
  files,
}, null, 2)}\n`);
