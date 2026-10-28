import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(here, '../../../..');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const backupPassIndex = process.argv.indexOf('--backup-pass');
const backupPass = backupPassIndex >= 0 ? process.argv[backupPassIndex + 1] : 'pass01';
if (!/^pass\d{2}$/.test(backupPass ?? '')) throw new Error('Use --backup-pass passNN.');
const backupRoot = `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr768-2026-10-29/${backupPass}`;
const renderedRoot = process.env.SPINON_TAILNET_RENDERED_ROOT;
const apply = process.argv.includes('--apply');
const pr = 768;
const implementationMergeSha = '7c004e8f5ed3197d696abcc8f84bcace83b97fe2';

if (!renderedRoot || !fs.existsSync(renderedRoot)) {
  throw new Error('Set SPINON_TAILNET_RENDERED_ROOT to the completed selected Rspress output directory.');
}
if (!fs.existsSync(previewRoot)) throw new Error(`Preview root is missing: ${previewRoot}`);
if (apply && fs.existsSync(backupRoot)) throw new Error(`Backup directory already exists; refusing to reuse it: ${backupRoot}`);

function sha256(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}

function collectFiles(root, prefix = '') {
  const output = [];
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    const absolute = path.join(root, entry.name);
    if (entry.isDirectory()) output.push(...collectFiles(absolute, relative));
    else if (entry.isFile()) output.push({ absolute, relative });
  }
  return output;
}

const sources = new Map();
function addSource(absolute, target, label) {
  if (!fs.existsSync(absolute) || !fs.statSync(absolute).isFile()) throw new Error(`Missing source file: ${absolute}`);
  if (path.isAbsolute(target) || target.split(path.sep).includes('..')) throw new Error(`Unsafe target path: ${target}`);
  const previous = sources.get(target);
  if (previous && previous.absolute !== absolute) throw new Error(`Duplicate target path ${target}`);
  sources.set(target, { absolute, target, sourceLabel: label });
}

const renderedPages = [
  'STATUS.html', 'spec/STATUS.html',
  'internal/0059-c11-grid-layout.html', 'spec/internal/0059-c11-grid-layout.html',
  'plan/c11-4-grid-track-sizing.html', 'spec/plan/c11-4-grid-track-sizing.html',
  'plan/c11-4-fit-content-auto-minimum-floor.html', 'spec/plan/c11-4-fit-content-auto-minimum-floor.html',
  'plan/c11-4-fit-content-auto-neighbor-runtime.html', 'spec/plan/c11-4-fit-content-auto-neighbor-runtime.html',
  'plan/c11-4-fit-content-spanning-wpt.html', 'spec/plan/c11-4-fit-content-spanning-wpt.html',
  'internal/evidence/c11-4-fit-content-spanning-precomparison-2026-10-29/README.html',
  'spec/internal/evidence/c11-4-fit-content-spanning-precomparison-2026-10-29/README.html',
  'internal/evidence/c11-4-fit-content-spanning-simulators-2026-10-29/README.html',
  'internal/evidence/c11-4-fit-content-spanning-simulators-2026-10-29/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-fit-content-spanning-simulators-2026-10-29/pr-diff-review-30.html',
  'spec/internal/evidence/c11-4-fit-content-spanning-simulators-2026-10-29/README.html',
  'spec/internal/evidence/c11-4-fit-content-spanning-simulators-2026-10-29/implementation-runtime-review-20.html',
  'spec/internal/evidence/c11-4-fit-content-spanning-simulators-2026-10-29/pr-diff-review-30.html',
  'internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29/README.html',
  'internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29/docs-sync-pr-diff-review-30.html',
  'spec/internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29/README.html',
  'spec/internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29/docs-sync-pr-diff-review-30.html',
];
for (const relative of renderedPages) {
  const absolute = path.join(renderedRoot, relative);
  addSource(absolute, relative, `selected-rspress:${relative}`);
}
for (const file of collectFiles(path.join(renderedRoot, 'static'))) {
  addSource(file.absolute, path.join('static', file.relative), `selected-rspress:static/${file.relative}`);
}
addSource(path.join(renderedRoot, 'icon.svg'), 'icon.svg', 'selected-rspress:icon.svg');
addSource(path.join(repo, 'spec/public/roadmap.html'), 'roadmap.html', 'spec/public/roadmap.html');

function addTree(sourceRelative, targetRelative) {
  const source = path.join(repo, sourceRelative);
  for (const file of collectFiles(source)) {
    if (['tailnet-sync-manifest.json', 'tailnet-served-hash-check.json'].includes(path.posix.basename(file.relative))) continue;
    addSource(file.absolute, path.posix.join(targetRelative, file.relative), `${sourceRelative}/${file.relative}`);
  }
}

const simulatorEvidence = 'spec/internal/evidence/c11-4-fit-content-spanning-simulators-2026-10-29';
const simulatorFiles = [
  'README.md',
  'implementation-runtime-review-20.md',
  'pr-diff-review-30.md',
  'android-avd-c114-fit-content-spanning-android-avd-20261028T183501979Z-69e492b8',
  'ios-simulator-c114-fit-content-spanning-ios-simulator-20261028T183506443Z-c6b59126',
  'android-build-final.log', 'android-capture-runner-final.log',
  'ios-build-final.log', 'ios-capture-runner-final.log',
  'runtime-test-final.log', 'ffi-test-final.log',
  'runtime-clippy-final.log', 'ffi-clippy-final.log', 'fmt-final.log',
  'fixture-module-boundary-final.log', 'capture-script-syntax-final.log',
  'archive-boundary/product-build-final.log',
  'archive-boundary/product-archive-final.sha256',
  'archive-boundary/product-marker-scan-final.txt',
  'archive-boundary/fixture-test-build-final.log',
  'archive-boundary/fixture-test-archive-final.sha256',
  'archive-boundary/fixture-test-marker-scan-final.txt',
  'archive-boundary/result.txt',
];
for (const relative of simulatorFiles) {
  const source = path.join(simulatorEvidence, relative);
  const absolute = path.join(repo, source);
  const targets = [
    path.posix.join('internal/evidence/c11-4-fit-content-spanning-simulators-2026-10-29', relative),
    path.posix.join('spec/internal/evidence/c11-4-fit-content-spanning-simulators-2026-10-29', relative),
  ];
  if (fs.statSync(absolute).isDirectory()) {
    for (const file of collectFiles(absolute)) {
      const childRelative = path.posix.join(relative, file.relative);
      for (const target of targets.map((base) => path.posix.join(path.posix.dirname(base), childRelative))) {
        addSource(file.absolute, target, `${source}/${file.relative}`);
      }
    }
  } else {
    for (const target of targets) addSource(absolute, target, source);
  }
}
addTree(
  'spec/internal/evidence/c11-4-fit-content-spanning-precomparison-2026-10-29',
  'internal/evidence/c11-4-fit-content-spanning-precomparison-2026-10-29',
);
addTree(
  'spec/internal/evidence/c11-4-fit-content-spanning-precomparison-2026-10-29',
  'spec/internal/evidence/c11-4-fit-content-spanning-precomparison-2026-10-29',
);
addTree(
  'spec/internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29',
  'internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29',
);
addTree(
  'spec/internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29',
  'spec/internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29',
);

const entries = [...sources.values()].sort((a, b) => a.target.localeCompare(b.target));
for (const entry of entries) {
  const destination = path.resolve(previewRoot, entry.target);
  if (destination !== previewRoot && !destination.startsWith(`${previewRoot}${path.sep}`)) {
    throw new Error(`Target escapes preview root: ${entry.target}`);
  }
  const stat = fs.existsSync(destination) ? fs.statSync(destination) : null;
  if (stat && !stat.isFile()) throw new Error(`Target exists but is not a file: ${destination}`);
  entry.bytes = fs.statSync(entry.absolute).size;
  entry.sha256 = sha256(entry.absolute);
  entry.prior = stat ? {
    exists: true,
    bytes: stat.size,
    mode: stat.mode & 0o777,
    sha256: sha256(destination),
  } : { exists: false };
  entry.disposition = !stat ? 'create' : entry.prior.sha256 === entry.sha256 ? 'same' : 'replace';
  if (entry.disposition === 'replace') entry.backupPath = path.join(backupRoot, entry.target);
}

const counts = Object.fromEntries(['create', 'replace', 'same'].map((kind) => [
  kind,
  entries.filter((entry) => entry.disposition === kind).length,
]));

if (!apply) {
  console.log(JSON.stringify({
    mode: 'plan-only', pr, implementationMergeSha, previewRoot, backupRoot,
    renderedRoot, noDeletes: true, targetCount: entries.length, counts,
    changedTargetSample: entries.filter((entry) => entry.disposition !== 'same').slice(0, 24).map((entry) => ({
      target: entry.target,
      disposition: entry.disposition,
      bytes: entry.bytes,
      priorSha256: entry.prior.sha256 ?? null,
      sha256: entry.sha256,
    })),
  }, null, 2));
  process.exit(0);
}

fs.mkdirSync(backupRoot, { recursive: true });
for (const entry of entries) {
  const destination = path.resolve(previewRoot, entry.target);
  if (entry.disposition === 'same') continue;
  if (entry.disposition === 'replace') {
    const backup = path.resolve(backupRoot, entry.target);
    fs.mkdirSync(path.dirname(backup), { recursive: true });
    fs.copyFileSync(destination, backup);
    fs.chmodSync(backup, entry.prior.mode);
  }
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(entry.absolute, destination);
  fs.chmodSync(destination, fs.statSync(entry.absolute).mode & 0o777);
  if (sha256(destination) !== entry.sha256) throw new Error(`Post-copy SHA-256 mismatch: ${entry.target}`);
}

const manifest = {
  version: 1,
  pr,
  implementationMergeSha,
  implementationMergedAt: '2026-10-28T18:41:57Z',
  generatedAt: new Date().toISOString(),
  previewRoot,
  previewRoutePrefix: '/spinon/',
  backupPass,
  backupRoot,
  sourceRenderedRoot: renderedRoot,
  noDeletes: true,
  deletionPaths: [],
  targetCount: entries.length,
  counts,
  targets: entries.map(({ absolute, ...entry }) => entry),
  manifestRoute: '/spinon/internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29/tailnet-sync-manifest.json',
};
const manifestPath = path.join(here, 'tailnet-sync-manifest.json');
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');
const manifestTarget = path.resolve(previewRoot, 'internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29/tailnet-sync-manifest.json');
fs.mkdirSync(path.dirname(manifestTarget), { recursive: true });
fs.copyFileSync(manifestPath, manifestTarget);
fs.chmodSync(manifestTarget, 0o644);
if (sha256(manifestPath) !== sha256(manifestTarget)) throw new Error('Manifest target SHA-256 mismatch.');
const specManifestTarget = path.resolve(previewRoot, 'spec/internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29/tailnet-sync-manifest.json');
fs.mkdirSync(path.dirname(specManifestTarget), { recursive: true });
fs.copyFileSync(manifestPath, specManifestTarget);
fs.chmodSync(specManifestTarget, 0o644);
if (sha256(manifestPath) !== sha256(specManifestTarget)) throw new Error('spec/ manifest target SHA-256 mismatch.');

console.log(JSON.stringify({
  mode: 'applied', pr, previewRoot, backupRoot, noDeletes: true,
  targetCount: entries.length, counts,
  manifestPath,
  manifestBytes: fs.statSync(manifestPath).size,
  manifestSha256: sha256(manifestPath),
}, null, 2));
