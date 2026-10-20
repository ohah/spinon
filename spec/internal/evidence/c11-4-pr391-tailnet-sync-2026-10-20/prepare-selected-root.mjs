import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const run = process.argv[2];
if (!['pass16', 'pass17', 'pass18'].includes(run)) throw new Error('usage: node prepare-selected-root.mjs pass16|pass17|pass18');

const outputRoot = path.join(repoRoot, 'build/spinon', `c11-4-pr391-tailnet-root-${run}`);
if (fs.existsSync(outputRoot)) throw new Error(`refusing to replace existing staging root: ${path.relative(repoRoot, outputRoot)}`);
const seedPath = path.join(repoRoot, 'spec/internal/evidence/c11-4-auto-fit-tailnet-sync-2026-10-20/prepare-root-pass15.json');
const seed = JSON.parse(fs.readFileSync(seedPath, 'utf8'));
const selected = seed.selected_sources.map(({ source, destination }) => [source, destination]);
const runtimeRoot = 'spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20';
const tailnetRoot = 'spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20';
const additions = [
  ['plan/c11-4-auto-repeat-typed-implicit-tracks.md', 'plan/c11-4-auto-repeat-typed-implicit-tracks.md'],
  [runtimeRoot + '/README.md', 'internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/README.md'],
  [runtimeRoot + '/README.md', 'spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/README.md'],
  [runtimeRoot + '/cross-platform-comparison-20261020.json', 'internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/cross-platform-comparison-20261020.json'],
  [runtimeRoot + '/cross-platform-comparison-20261020.json', 'spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/cross-platform-comparison-20261020.json'],
  [runtimeRoot + '/auto-repeat-typed-implementation-review-20.md', 'internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/auto-repeat-typed-implementation-review-20.md'],
  [runtimeRoot + '/auto-repeat-typed-implementation-review-20.md', 'spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/auto-repeat-typed-implementation-review-20.md'],
  [runtimeRoot + '/auto-repeat-typed-pr-diff-review-30.md', 'internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/auto-repeat-typed-pr-diff-review-30.md'],
  [runtimeRoot + '/auto-repeat-typed-pr-diff-review-30.md', 'spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/auto-repeat-typed-pr-diff-review-30.md'],
  [runtimeRoot + '/android-avd-c114-negative-implicit-android-avd-20261020T125416760Z-ef57f92f/columns-fill-negative-positive-cycle.png', 'internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/android-avd-c114-negative-implicit-android-avd-20261020T125416760Z-ef57f92f/columns-fill-negative-positive-cycle.png'],
  [runtimeRoot + '/android-avd-c114-negative-implicit-android-avd-20261020T125416760Z-ef57f92f/columns-fill-negative-positive-cycle.png', 'spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/android-avd-c114-negative-implicit-android-avd-20261020T125416760Z-ef57f92f/columns-fill-negative-positive-cycle.png'],
  [runtimeRoot + '/ios-simulator-c114-negative-implicit-ios-simulator-20261020T130111829Z-d5866712/columns-fill-negative-positive-cycle.png', 'internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/ios-simulator-c114-negative-implicit-ios-simulator-20261020T130111829Z-d5866712/columns-fill-negative-positive-cycle.png'],
  [runtimeRoot + '/ios-simulator-c114-negative-implicit-ios-simulator-20261020T130111829Z-d5866712/columns-fill-negative-positive-cycle.png', 'spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/ios-simulator-c114-negative-implicit-ios-simulator-20261020T130111829Z-d5866712/columns-fill-negative-positive-cycle.png'],
  [runtimeRoot + '/ios-simulator-c114-negative-implicit-ios-simulator-20261020T130111829Z-d5866712/typed-auto.png', 'internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/ios-simulator-c114-negative-implicit-ios-simulator-20261020T130111829Z-d5866712/typed-auto.png'],
  [runtimeRoot + '/ios-simulator-c114-negative-implicit-ios-simulator-20261020T130111829Z-d5866712/typed-auto.png', 'spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/ios-simulator-c114-negative-implicit-ios-simulator-20261020T130111829Z-d5866712/typed-auto.png'],
  [tailnetRoot + '/README.md', 'internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/README.md'],
  [tailnetRoot + '/README.md', 'spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/README.md'],
  [tailnetRoot + '/sync-review-20.md', 'internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/sync-review-20.md'],
  [tailnetRoot + '/sync-review-20.md', 'spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/sync-review-20.md'],
  [tailnetRoot + '/pr-diff-review-30.md', 'internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/pr-diff-review-30.md'],
  [tailnetRoot + '/pr-diff-review-30.md', 'spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/pr-diff-review-30.md'],
];
selected.push(...additions);

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

const staticEvidenceExtensions = new Set(['.bin', '.css', '.csv', '.html', '.json', '.log', '.mjs', '.pdf', '.png', '.sh', '.svg', '.tar', '.ts', '.txt', '.webp']);
function copySelected(sourceRelative, destinationRelative) {
  const source = path.join(repoRoot, sourceRelative);
  if (!fs.existsSync(source)) throw new Error(`missing selected source: ${sourceRelative}`);
  const stat = fs.lstatSync(source);
  if (stat.isSymbolicLink()) throw new Error(`symlink in selected source: ${sourceRelative}`);
  if (stat.isDirectory()) {
    for (const name of fs.readdirSync(source).sort()) copySelected(`${sourceRelative}/${name}`, `${destinationRelative}/${name}`);
    return;
  }
  if (!stat.isFile()) throw new Error(`unsupported selected source: ${sourceRelative}`);
  const extension = path.extname(source);
  const isEvidenceAsset = (destinationRelative.startsWith('internal/evidence/') || destinationRelative.startsWith('spec/internal/evidence/'))
    && extension !== '.md' && staticEvidenceExtensions.has(extension);
  const isMarkdownImage = ['.png', '.svg', '.webp'].includes(extension);
  let bytes = fs.readFileSync(source);
  if (extension === '.md') {
    const rewritten = rewriteMarkdownPreviewLinks(sourceRelative, destinationRelative, bytes.toString('utf8'));
    bytes = Buffer.from(rewritten.text, 'utf8');
    if (rewritten.count > 0) previewLinkRewrites.push({ source: rewritten.source, destination: rewritten.destination, count: rewritten.count, local_count: rewritten.localCount, github_count: rewritten.githubCount });
  }
  const destinations = isEvidenceAsset ? (isMarkdownImage ? [destinationRelative, path.join('public', destinationRelative)] : [path.join('public', destinationRelative)]) : [destinationRelative];
  for (const stagedRelative of destinations) {
    const target = path.join(outputRoot, stagedRelative);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.writeFileSync(target, bytes);
    fs.chmodSync(target, stat.mode & 0o777);
  }
}

for (const [source, destination] of selected) copySelected(source, destination);

const textExtensions = new Set(['.md', '.html', '.json', '.log', '.txt', '.mjs', '.ts', '.sh']);
const redacted = [];
function redactHostPaths(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) { redactHostPaths(fullPath); continue; }
    if (!entry.isFile() || !textExtensions.has(path.extname(entry.name))) continue;
    const before = fs.readFileSync(fullPath, 'utf8');
    const after = before
      .replace(/\/Users\/[^/\s"'<>:,)}\]]+(?:\/[^\s"'<>:,)}\]]*)*/g, '[host-path-redacted]')
      .replace(/\/private\/tmp(?:\/[^\s"'<>:,)}\]]*)*/g, '[worktree-path-redacted]');
    if (after !== before) { fs.writeFileSync(fullPath, after, 'utf8'); redacted.push(path.relative(outputRoot, fullPath).split(path.sep).join('/')); }
  }
}
redactHostPaths(outputRoot);

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
const sourceBranch = execFileSync('git', ['branch', '--show-current'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
process.stdout.write(`${JSON.stringify({
  run, source_branch: sourceBranch, source_commit: sourceCommit,
  output_root_relative: path.relative(repoRoot, outputRoot).split(path.sep).join('/'),
  selected_sources: selected.map(([source, destination]) => ({ source, destination })),
  file_count: files.length, redacted_file_count: redacted.length, redacted_files: redacted.sort(),
  preview_link_rewrites: previewLinkRewrites, files,
}, null, 2)}\n`);
