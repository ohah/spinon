import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const run = process.argv[2];
if (run !== 'pass18') throw new Error('usage: node prepare-selected-root-pass18.mjs pass18');

const specRoot = path.join(repoRoot, 'spec');
const outputRoot = path.join(repoRoot, 'build/spinon', 'c11-4-positive-implicit-tailnet-root-' + run);
const sourceBranch = execFileSync('git', ['branch', '--show-current'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
if (fs.existsSync(outputRoot)) throw new Error(`refusing to replace existing staging root for ${run}`);

const seedConfigRelative = 'spec/internal/evidence/c11-4-positive-implicit-tailnet-sync-2026-10-20/prepare-root-seed-pass18.json';
const seedConfig = JSON.parse(fs.readFileSync(path.join(repoRoot, seedConfigRelative), 'utf8'));
const selected = seedConfig.selected_sources.map(({ source, destination }) => [source, destination]);
const syncEvidenceRoot = 'spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20';
const syncPreviewRoot = 'internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20';
const implementationRoot = 'spec/internal/evidence/c11-4-auto-fit-min-size-axis-implementation-2026-10-20';
const runtimeEvidenceRoot = 'spec/internal/evidence/c11-4-auto-fit-min-size-axis-2026-10-20';
const implementationPreviewRoot = 'internal/evidence/c11-4-auto-fit-min-size-axis-implementation-2026-10-20';
const addedFiles = [
  ['plan/c11-4-auto-fit-min-size-axis.md', 'plan/c11-4-auto-fit-min-size-axis.md'],
  [implementationRoot + '/README.md', implementationPreviewRoot + '/README.md'],
  [implementationRoot + '/README.md', 'spec/' + implementationPreviewRoot + '/README.md'],
  [implementationRoot + '/implementation-review-20.md', implementationPreviewRoot + '/implementation-review-20.md'],
  [implementationRoot + '/implementation-review-20.md', 'spec/' + implementationPreviewRoot + '/implementation-review-20.md'],
  [implementationRoot + '/pr-diff-review-30.md', implementationPreviewRoot + '/pr-diff-review-30.md'],
  [implementationRoot + '/pr-diff-review-30.md', 'spec/' + implementationPreviewRoot + '/pr-diff-review-30.md'],
  [syncEvidenceRoot + '/README.md', syncPreviewRoot + '/README.md'],
  [syncEvidenceRoot + '/README.md', 'spec/' + syncPreviewRoot + '/README.md'],
  [syncEvidenceRoot + '/pass15-android-api37-overflow.png', syncPreviewRoot + '/pass15-android-api37-overflow.png'],
  [syncEvidenceRoot + '/pass15-iphone17pro-overflow.png', syncPreviewRoot + '/pass15-iphone17pro-overflow.png'],
  [syncEvidenceRoot + '/pass15-android-api37-second-repeat.png', syncPreviewRoot + '/pass15-android-api37-second-repeat.png'],
  [syncEvidenceRoot + '/pass15-iphone17pro-second-repeat.png', syncPreviewRoot + '/pass15-iphone17pro-second-repeat.png'],
];
const screenshotFiles = [
  [runtimeEvidenceRoot + '/android-avd-c114-autofit-minsize-android-avd-20261020T010237848Z-6e767f6f/auto-fill-overflow-neighbors.png', implementationPreviewRoot + '/android-avd-c114-autofit-minsize-android-avd-20261020T010237848Z-6e767f6f/auto-fill-overflow-neighbors.png'],
  [runtimeEvidenceRoot + '/ios-simulator-c114-autofit-minsize-ios-simulator-20261020T010922301Z-e8f2c7bb/auto-fill-overflow-neighbors.png', implementationPreviewRoot + '/ios-simulator-c114-autofit-minsize-ios-simulator-20261020T010922301Z-e8f2c7bb/auto-fill-overflow-neighbors.png'],
  [runtimeEvidenceRoot + '/android-avd-c114-autofit-minsize-android-avd-20261020T010237848Z-6e767f6f/auto-fit-min-columns-second-repeat-200.png', implementationPreviewRoot + '/android-avd-c114-autofit-minsize-android-avd-20261020T010237848Z-6e767f6f/auto-fit-min-columns-second-repeat-200.png'],
  [runtimeEvidenceRoot + '/ios-simulator-c114-autofit-minsize-ios-simulator-20261020T010922301Z-e8f2c7bb/auto-fit-min-columns-second-repeat-200.png', implementationPreviewRoot + '/ios-simulator-c114-autofit-minsize-ios-simulator-20261020T010922301Z-e8f2c7bb/auto-fit-min-columns-second-repeat-200.png'],
];
for (const [source, destination] of screenshotFiles) addedFiles.push([source, destination], [source, 'spec/' + destination]);
const positiveImplicitAddedFiles = [
  ['plan/c11-4-positive-implicit-track-sizing.md', 'plan/c11-4-positive-implicit-track-sizing.md'],
  ['spec/internal/evidence/c11-4-implicit-intrinsic-auto-tracks-precomparison-2026-10-20', 'internal/evidence/c11-4-implicit-intrinsic-auto-tracks-precomparison-2026-10-20'],
  ['spec/internal/evidence/c11-4-implicit-intrinsic-auto-tracks-precomparison-2026-10-20', 'spec/internal/evidence/c11-4-implicit-intrinsic-auto-tracks-precomparison-2026-10-20'],
  ['spec/internal/evidence/c11-4-implicit-intrinsic-auto-tracks-2026-10-20', 'internal/evidence/c11-4-implicit-intrinsic-auto-tracks-2026-10-20'],
  ['spec/internal/evidence/c11-4-implicit-intrinsic-auto-tracks-2026-10-20', 'spec/internal/evidence/c11-4-implicit-intrinsic-auto-tracks-2026-10-20'],
];
addedFiles.push(...positiveImplicitAddedFiles);
selected.push(...addedFiles);
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
