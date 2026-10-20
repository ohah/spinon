import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '../../../..');
const evidenceRelative = 'spec/internal/evidence/c04-12-root-html-page-scale-tailnet-sync-2026-10-20';
const implementationRelative = 'spec/internal/evidence/c04-12-root-html-page-scale-runtime-2026-10-20';
const run = process.argv[2];
if (!['pass01', 'pass02', 'pass03', 'pass04', 'pass05', 'pass06', 'pass07', 'pass08'].includes(run ?? '')) {
  throw new Error('usage: node prepare-selected-root.mjs pass01|pass02|pass03|pass04|pass05|pass06|pass07|pass08');
}

const outputRoot = path.join(repoRoot, 'build/spinon', `c04-12-page-scale-tailnet-root-${run}`);
if (fs.existsSync(outputRoot)) throw new Error(`refusing to replace existing staging root: ${outputRoot}`);
const generatedMetadata = new Set(['prepare-root.json', 'SHA256SUMS-preview-inputs.txt']);
const selected = new Map();
function add(source, destination) {
  selected.set(`${source}\0${destination}`, { source, destination });
}

const seedPath = 'spec/internal/evidence/c11-4-pr385-tailnet-sync-2026-10-20/prepare-root-pass16.json';
const seed = JSON.parse(fs.readFileSync(path.join(repoRoot, seedPath), 'utf8'));
const seedFiles = new Set(seed.files);
function expandSeed(entry) {
  const sourcePath = path.join(repoRoot, entry.source);
  const stat = fs.lstatSync(sourcePath);
  if (stat.isSymbolicLink()) throw new Error(`symlink in seed source: ${entry.source}`);
  if (stat.isDirectory()) {
    for (const name of fs.readdirSync(sourcePath).sort()) {
      expandSeed({ source: path.posix.join(entry.source, name), destination: path.posix.join(entry.destination, name) });
    }
  } else if (stat.isFile()) {
    const publicDestination = path.posix.join('public', entry.destination);
    if (seedFiles.has(entry.destination) || seedFiles.has(publicDestination)) add(entry.source, entry.destination);
  } else throw new Error(`unsupported seed source: ${entry.source}`);
}
for (const entry of seed.selected_sources) expandSeed(entry);

for (const source of [
  'plan/c04-root-html-document-environment.md',
  'plan/c04-12-root-html-default-rendering.md',
  'plan/c04-12-page-scale-rendering.md',
  'plan/c04-12-root-html-page-scale-runtime.md',
  'spec/internal/0077-c04-12-root-html-document.md',
  'spec/internal/0081-c04-12-root-html-default-runtime.md',
  'spec/internal/evidence/c04-12-root-html-page-scale-runtime-plan-review-20-2026-10-20.md',
]) {
  add(source, source.startsWith('plan/') ? source : source.replace(/^spec\//, ''));
}
add('spec/public/roadmap.html', 'public/roadmap.html');

const implementationFiles = execFileSync('git', ['ls-files', '-z', implementationRelative], {
  cwd: repoRoot,
  encoding: 'utf8',
}).split('\0').filter(Boolean);
for (const source of implementationFiles) {
  add(source, source.replace(/^spec\//, ''));
}

function walkEvidence(relative) {
  for (const entry of fs.readdirSync(path.join(repoRoot, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.posix.join(relative, entry.name);
    if (entry.isDirectory()) walkEvidence(child);
    else if (entry.isFile() && !generatedMetadata.has(entry.name)) {
      add(child, child.replace(/^spec\//, ''));
    } else if (!entry.isFile()) throw new Error(`unsupported selected evidence entry: ${child}`);
  }
}
walkEvidence(evidenceRelative);

const selectedSources = [...selected.values()].sort((a, b) => a.source.localeCompare(b.source) || a.destination.localeCompare(b.destination));
const sourceBranch = execFileSync('git', ['branch', '--show-current'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
const sourceDirty = execFileSync('git', ['status', '--porcelain'], { cwd: repoRoot, encoding: 'utf8' }).length > 0;
const previewLinkRewrites = [];
const staticEvidenceExtensions = new Set(['.bin', '.css', '.csv', '.html', '.json', '.log', '.mjs', '.pdf', '.png', '.sh', '.svg', '.tar', '.ts', '.txt', '.webp']);
const textExtensions = new Set(['.md', '.html', '.json', '.log', '.mjs', '.ts', '.sh', '.txt']);
const inputFiles = new Map();

function hash(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function addInput(relative) {
  const fullPath = path.join(repoRoot, relative);
  const stat = fs.lstatSync(fullPath);
  if (stat.isSymbolicLink()) throw new Error(`symlink in selected input: ${relative}`);
  if (stat.isDirectory()) {
    for (const name of fs.readdirSync(fullPath).sort()) addInput(path.posix.join(relative, name));
  } else if (stat.isFile()) {
    const bytes = fs.readFileSync(fullPath);
    inputFiles.set(relative, { bytes: bytes.byteLength, sha256: hash(bytes) });
  } else throw new Error(`unsupported selected input: ${relative}`);
}
for (const entry of selectedSources) addInput(entry.source);

const inputManifestPath = path.join(repoRoot, evidenceRelative, 'SHA256SUMS-preview-inputs.txt');
const inputManifest = [...inputFiles.entries()].sort(([left], [right]) => left.localeCompare(right))
  .map(([relative, value]) => `${value.sha256}  ${relative}`).join('\n') + '\n';
fs.writeFileSync(inputManifestPath, inputManifest);

function rewriteMarkdown(sourceRelative, destinationRelative, text) {
  let count = 0;
  let localCount = 0;
  let githubCount = 0;
  const rewritten = text.replace(/(!?\]\()([^)]+)(\))/g, (match, open, href, close) => {
    if (!href || href.startsWith('#') || href.startsWith('/') || /^[a-z][a-z\d+.-]*:/i.test(href)) return match;
    const suffixIndex = href.search(/[?#]/);
    const relativePath = suffixIndex === -1 ? href : href.slice(0, suffixIndex);
    const suffix = suffixIndex === -1 ? '' : href.slice(suffixIndex);
    if (!relativePath || /\s/.test(relativePath)) return match;
    const target = path.posix.normalize(path.posix.join(path.posix.dirname(sourceRelative), relativePath));
    if (target === '..' || target.startsWith('../') || path.posix.isAbsolute(target)) return match;
    const mappings = selectedSources.filter((entry) => target === entry.source || target.startsWith(`${entry.source}/`))
      .sort((left, right) => right.source.length - left.source.length);
    let outputHref;
    if (mappings.length > 0) {
      const mapping = mappings[0];
      let destination = path.posix.join(mapping.destination, target.slice(mapping.source.length));
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
  if (count > 0) previewLinkRewrites.push({ source: sourceRelative, destination: destinationRelative, count, local_count: localCount, github_count: githubCount });
  return rewritten;
}

function writeStaged(destinationRelative, bytes, mode = 0o644) {
  const destination = path.join(outputRoot, destinationRelative);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes);
  fs.chmodSync(destination, mode);
}

function copySelected(sourceRelative, destinationRelative) {
  const sourcePath = path.join(repoRoot, sourceRelative);
  const stat = fs.lstatSync(sourcePath);
  if (stat.isSymbolicLink()) throw new Error(`symlink in selected source: ${sourceRelative}`);
  if (stat.isDirectory()) {
    for (const name of fs.readdirSync(sourcePath).sort()) copySelected(path.posix.join(sourceRelative, name), path.posix.join(destinationRelative, name));
    return;
  }
  if (!stat.isFile()) throw new Error(`unsupported selected source: ${sourceRelative}`);
  let bytes = fs.readFileSync(sourcePath);
  if (path.extname(sourcePath) === '.md') bytes = Buffer.from(rewriteMarkdown(sourceRelative, destinationRelative, bytes.toString('utf8')));
  const ext = path.extname(sourcePath);
  const isEvidenceAsset = (destinationRelative.startsWith('internal/evidence/') || destinationRelative.startsWith('spec/internal/evidence/'))
    && ext !== '.md' && staticEvidenceExtensions.has(ext);
  if (isEvidenceAsset) {
    const isMarkdownImage = ['.png', '.svg', '.webp'].includes(ext);
    if (isMarkdownImage) writeStaged(destinationRelative, bytes, stat.mode & 0o777);
    writeStaged(path.posix.join('public', destinationRelative), bytes, stat.mode & 0o777);
  } else writeStaged(destinationRelative, bytes, stat.mode & 0o777);
}

fs.mkdirSync(outputRoot, { recursive: true });
for (const entry of selectedSources) copySelected(entry.source, entry.destination);

const redacted = [];
function sanitize(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) sanitize(fullPath);
    else if (entry.isFile() && textExtensions.has(path.extname(entry.name))) {
      const before = fs.readFileSync(fullPath, 'utf8');
      const after = before
        .replace(/\/Users\/[^/\s"'<>:,)}\]]+(?:\/[^\s"'<>:,)}\]]*)*/g, '[host-path-redacted]')
        .replace(/\/private\/tmp(?:\/[^\s"'<>:,)}\]]*)*/g, '[worktree-path-redacted]');
      if (before !== after) {
        fs.writeFileSync(fullPath, after, 'utf8');
        redacted.push(path.relative(outputRoot, fullPath).split(path.sep).join('/'));
      }
    }
  }
}
sanitize(outputRoot);

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
const missingSeedFiles = seed.files.filter((relative) => !files.includes(relative));
if (missingSeedFiles.length > 0) throw new Error(`seed files not reproduced: ${missingSeedFiles.slice(0, 10).join(', ')}`);
const prepare = {
  run,
  source_branch: sourceBranch,
  source_commit: sourceCommit,
  source_dirty_at_prepare: sourceDirty,
  output_root_relative: path.relative(repoRoot, outputRoot).split(path.sep).join('/'),
  selected_sources: selectedSources,
  selected_source_file_count: inputFiles.size,
  selected_source_bytes: [...inputFiles.values()].reduce((sum, file) => sum + file.bytes, 0),
  input_manifest: 'SHA256SUMS-preview-inputs.txt',
  staged_file_count: files.length,
  staged_files: files,
  redacted_file_count: redacted.length,
  redacted_files: redacted.sort(),
  preview_link_rewrites: previewLinkRewrites,
};
const prepareBytes = Buffer.from(`${JSON.stringify(prepare, null, 2)}\n`);
const manifestBytes = Buffer.from(inputManifest);
const metadataBase = path.posix.join('internal/evidence/c04-12-root-html-page-scale-tailnet-sync-2026-10-20');
for (const [name, bytes] of [['prepare-root.json', prepareBytes], ['SHA256SUMS-preview-inputs.txt', manifestBytes]]) {
  const relative = path.posix.join(metadataBase, name);
  writeStaged(relative, bytes);
  writeStaged(path.posix.join('public', relative), bytes);
}
fs.writeFileSync(path.join(repoRoot, evidenceRelative, 'prepare-root.json'), prepareBytes);
process.stdout.write(`${JSON.stringify({ ...prepare, selected_source_count: selectedSources.length }, null, 2)}\n`);
