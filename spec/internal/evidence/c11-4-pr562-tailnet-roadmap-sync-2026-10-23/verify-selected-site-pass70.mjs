import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const sourceHead = '22eafb6cad66b6be8facaf264299e7bd6073d0f5';
const branchHead = '10ae04a09f9f0b1b97056693dad0438a2c022b62';
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-pr562-tailnet-site-pass70-selected');
const verificationPath = path.join(evidenceDirectory, 'selected-site-verification-final-pass70.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const required = [
  'roadmap.html',
  'STATUS.html',
  '0001-conformance.html',
  'internal/README.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-4-auto-repeat-positive-percent-minimum.html',
  'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-precomparison-2026-10-23/README.html',
  'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/README.html',
  'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/pr-diff-review-20.html',
  'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/android-avd-c114-positive-percent-minimum-commit-android-avd-20261023T104649350Z-da0f371b/percentage-overflow-columns-calc.png',
  'internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/ios-simulator-c114-positive-percent-minimum-commit-ios-simulator-20261023T104719112Z-2d91e097/percentage-overflow-columns-calc.png',
  'internal/evidence/c11-4-positive-percent-status-update-2026-10-23/pr-diff-review-20.html',
];
for (const relative of required) assert.ok(fs.lstatSync(path.join(outputRoot, relative)).isFile(), `missing built route/asset: ${relative}`);

const roadmapPath = path.join(outputRoot, 'roadmap.html');
const roadmap = fs.readFileSync(roadmapPath, 'utf8');
const status = fs.readFileSync(path.join(outputRoot, 'STATUS.html'), 'utf8');
const committedRoadmap = execFileSync('git', ['show', `${branchHead}:spec/public/roadmap.html`], { cwd: repositoryRoot, maxBuffer: 32 * 1024 * 1024 });
const roadmapSource = committedRoadmap;
assert.ok(roadmap.includes('ff4233d0df00a632d71c28398c9a8274fb4abc16'), 'roadmap missing PR #561 merge SHA');
assert.ok(roadmap.includes('72/213'), 'roadmap completion total mismatch');
assert.ok(roadmap.includes('positive percentage auto-repeat minimum'), 'roadmap missing C11.4 slice');
assert.ok(status.includes('72/213'), 'status completion total mismatch');
assert.ok(status.includes('ff4233d0df00a632d71c28398c9a8274fb4abc16'), 'status missing PR #561 merge SHA');
assert.ok(status.includes('c11-css-grid'), 'C11.4 status anchor missing');

const localLinks = [];
const cardStart = roadmap.indexOf('definite positive percentage auto-repeat minimum runtime slice');
assert.ok(cardStart >= 0, 'roadmap missing C11.4 slice');
const cardEnd = roadmap.indexOf('</p>', cardStart);
assert.ok(cardEnd > cardStart, 'roadmap C11.4 slice paragraph is incomplete');
const card = roadmap.slice(cardStart, cardEnd);
for (const match of card.matchAll(/<a\b[^>]*href="([^"]+)"/g)) {
  const href = match[1];
  if (!href.startsWith('/spinon/')) continue;
  const relative = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(fs.lstatSync(path.join(outputRoot, relative)).isFile(), `broken roadmap route: ${href}`);
  localLinks.push({ href, path: relative });
}

assert.equal(sha256(fs.readFileSync(roadmapPath)), sha256(roadmapSource), 'built roadmap differs from the working source');
const manifest = {
  pass: 'pass70', sourceHead, branchHead,
  requiredRoutesAndAssets: required,
  publicRoadmapInternalLinks: localLinks,
  roadmapSource: 'spec/public/roadmap.html at branch head',
  roadmapSha256: sha256(roadmapSource),
  officialCount: '72/213', c11_4Incomplete: true, allVerified: true,
};
fs.writeFileSync(verificationPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(manifest, null, 2)}\n`);
