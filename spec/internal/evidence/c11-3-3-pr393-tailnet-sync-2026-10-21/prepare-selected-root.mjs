import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const run = process.argv[2];
if (run !== 'pass1') throw new Error('usage: node prepare-selected-root.mjs pass1');

const outputRoot = path.join(repoRoot, 'build/spinon', `c11-3-3-pr393-tailnet-root-${run}`);
if (fs.existsSync(outputRoot)) throw new Error(`refusing to replace existing staging root: ${path.relative(repoRoot, outputRoot)}`);
const seedPath = path.join(repoRoot, 'spec/internal/evidence/c11-4-pr391-tailnet-sync-2026-10-20/prepare-root-pass18.json');
const seed = JSON.parse(fs.readFileSync(seedPath, 'utf8'));
const selected = seed.selected_sources.map(({ source, destination }) => [source, destination]);
const evidenceRoot = 'spec/internal/evidence';
const followupRoot = evidenceRoot + '/c11-3-3-unknown-area-implicit-lines-followup-2026-10-21';
const matrixRoot = evidenceRoot + '/c11-3-3-grid-template-areas-runtime-2026-10-21';
const androidAvd = matrixRoot + '/android-avd-c1133-matrix-20261020T150911869Z-7abbeeff';
const androidPhysical = matrixRoot + '/android-physical-c1133-matrix-20261020T152039508Z-5886c27a';
const iosSimulator = matrixRoot + '/ios-simulator-c1133-ios-matrix-20261020T151033155Z-4d6bf988';
const planReview = evidenceRoot + '/c11-3-3-unknown-named-area-plan-review-20-2026-10-20.md';
const additions = [
  ['spec/0008-css-compatibility.md', '0008-css-compatibility.md'],
  ['plan/c11-3-3-unknown-named-area-implicit-lines.md', 'plan/c11-3-3-unknown-named-area-implicit-lines.md'],
  [planReview, planReview.replace('spec/', '')],
  [followupRoot, followupRoot.replace('spec/', '')],
  [androidAvd, androidAvd.replace('spec/', '')],
  [androidPhysical, androidPhysical.replace('spec/', '')],
  [iosSimulator, iosSimulator.replace('spec/', '')],
  [androidPhysical + '/unknown-area-overflow-pixelcopy.png', androidPhysical + '/unknown-area-overflow-pixelcopy.png'],
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
