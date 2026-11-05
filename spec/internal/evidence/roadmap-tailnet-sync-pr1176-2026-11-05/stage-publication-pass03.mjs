import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1176-2026-11-05/pass03/pass01');
const root = path.join(base, 'root');
const site = path.join(base, 'site');
const staging = path.join(base, 'publication-staging');
const publication = path.join(base, 'publication');
const rootManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-pass03.json'), 'utf8'));
const manifestPath = path.join(evidence, 'selected-publication-pass03.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

for (const target of [staging, publication, manifestPath]) assert.ok(!fs.existsSync(target), `기존 publication 보존: ${target}`);
for (const directory of [root, site]) {
  const stat = fs.lstatSync(directory);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 디렉터리: ${directory}`);
}
fs.mkdirSync(staging, { recursive: false });

const selected = new Map();
function safeRoute(route) {
  assert.ok(!route.startsWith('/') && !route.includes('\\'), `잘못된 route: ${route}`);
  assert.ok(route.split('/').every((part) => part && part !== '.' && part !== '..'), `잘못된 publication route: ${route}`);
}
function add(route, bytes, source, kind) {
  safeRoute(route);
  const digest = sha256(bytes);
  const previous = selected.get(route);
  if (previous) {
    assert.equal(previous.sha256, digest, `route 내용 충돌: ${route}`);
    return;
  }
  const target = path.join(staging, route);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, bytes, { flag: 'wx' });
  assert.equal(sha256(fs.readFileSync(target)), digest, `publication hash 불일치: ${route}`);
  selected.set(route, { path: route, source, kind, bytes: bytes.length, sha256: digest });
}
function walkSite(relative = '') {
  for (const entry of fs.readdirSync(path.join(site, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = path.join(relative, entry.name);
    const absolute = path.join(site, child);
    assert.ok(!entry.isSymbolicLink(), `generated site symlink 거부: ${child}`);
    if (entry.isDirectory()) walkSite(child);
    else if (entry.isFile()) {
      const route = child.split(path.sep).join('/');
      if (route !== 'index.html' && route !== '404.html' && route !== 'spec/plan/c11-css-grid.html') {
        add(route, fs.readFileSync(absolute), `rspress:${route}`, 'generated-page-or-asset');
      }
    } else throw new Error(`지원하지 않는 generated site 항목: ${child}`);
  }
}
walkSite();

for (const item of rootManifest.staged) {
  if (item.source === 'generated:index.md') continue;
  const source = path.join(root, item.route);
  const bytes = fs.readFileSync(source);
  assert.equal(bytes.length, item.bytes, `selected root 크기 변경: ${item.route}`);
  assert.equal(sha256(bytes), item.sha256, `selected root hash 변경: ${item.route}`);
  add(item.route, bytes, `selected-root:${item.source}`, 'source-page-or-asset');
}

const routes = [...selected.values()].sort((a, b) => a.path.localeCompare(b.path));
const byRoute = new Map(routes.map((item) => [item.path, item]));
const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
function checkCard(pr, mergeSha) {
  const marker = `PR #${pr} Rebase and merge`;
  const start = roadmap.indexOf(marker);
  assert.ok(start >= 0, `roadmap #${pr} card 누락`);
  const cardStart = roadmap.lastIndexOf('<p ', start);
  const cardEnd = roadmap.indexOf('</p>', start);
  assert.ok(cardStart >= 0 && cardEnd >= 0, `#${pr} roadmap paragraph 경계 누락`);
  const card = roadmap.slice(cardStart, cardEnd + 4);
  assert.ok(card.includes(mergeSha), `#${pr} merge SHA 누락`);
  const links = [];
  const sourceLinks = [];
  for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
    const sourceMatch = href.match(/^https:\/\/github\.com\/ohah\/spinon\/blob\/main\/(.+)$/);
    if (sourceMatch) {
      const source = decodeURIComponent(sourceMatch[1].split(/[?#]/, 1)[0]);
      execFileSync('git', ['ls-files', '--error-unmatch', '--', source], { cwd: repo, stdio: 'ignore' });
      sourceLinks.push({ href, source });
      continue;
    }
    if (/^(?:https?:|mailto:)/i.test(href)) continue;
    assert.ok(href.startsWith('/spinon/'), `예상하지 않은 local href: ${href}`);
    const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
    assert.ok(route && route.split('/').every((part) => part && part !== '.' && part !== '..'), `안전하지 않은 href: ${href}`);
    assert.ok(byRoute.has(route), `#${pr} roadmap 링크 route 누락: ${route}`);
    links.push({ href, route, sha256: byRoute.get(route).sha256 });
  }
  assert.ok(links.length + sourceLinks.length >= 3, `#${pr} roadmap 검증 가능한 link 수 부족: ${links.length + sourceLinks.length}`);
  return [...links, ...sourceLinks];
}

const checkedRoadmapLinks = [
  ...checkCard('1161', 'f774af7f1b7441708495b74a063796122a4f1128'),
  ...checkCard('1164', '3e5aed60a95a07195002f8b8df8fb4da9eefb80b'),
  ...checkCard('1176', 'da67a18032f449d8477cdd5d7ea0aba02e691b0b'),
];
for (const required of [
  'roadmap.html',
  'STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-root-html-grid-text-aspect-ratio.html',
  'plan/c11-root-html-grid-direct-text-aspect-ratio-values.html',
  'internal/evidence/c11-root-html-grid-direct-text-aspect-ratio-values-runtime-2026-11-05/README.html',
  'internal/evidence/c11-root-html-grid-direct-text-aspect-ratio-values-runtime-2026-11-05/android/screenshot.png',
  'internal/evidence/c11-root-html-grid-direct-text-aspect-ratio-values-runtime-2026-11-05/ios/screenshot.png',
  'plan/c11-root-html-grid-intrinsic-gap-percentage.html',
  'internal/evidence/c11-root-html-intrinsic-gap-percentage-precomparison-2026-11-05/README.html',
  'internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/README.html',
  'internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/android/simulator-screen.png',
  'internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/ios/simulator-screen.png',
  'internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/implementation-runtime-review-20.html',
  'internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/pr-diff-review-30.html',
  'internal/evidence/roadmap-tailnet-sync-pr1176-2026-11-05/README.html',
  'internal/evidence/roadmap-tailnet-sync-pr1176-2026-11-05/pr-diff-review-30.html',
  'internal/evidence/roadmap-tailnet-sync-pr1176-2026-11-05/tailnet-roadmap-agent-browser-desktop.png',
  'internal/evidence/roadmap-tailnet-sync-pr1176-2026-11-05/tailnet-roadmap-agent-browser-mobile.png',
  'internal/evidence/roadmap-tailnet-sync-pr1176-2026-11-05/agent-browser-verification.json',
  'internal/evidence/roadmap-tailnet-sync-pr1176-2026-11-05/agent-browser-lifecycle.json',
]) assert.ok(byRoute.has(required), `필수 route 누락: ${required}`);
for (const marker of ['3e5aed60a95a07195002f8b8df8fb4da9eefb80b', 'da67a18032f449d8477cdd5d7ea0aba02e691b0b', '78/214']) {
  assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
}
const statusSource = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
assert.ok(statusSource.includes('78/214'), 'STATUS 집계 불일치');
assert.match(statusSource, /- \[ \] <span id="c11-css-grid"/u, 'C11 checkbox가 미완료가 아님');
const contract = fs.readFileSync(path.join(root, 'internal/0059-c11-grid-layout.md'), 'utf8');
assert.ok(contract.includes('2026-11-05T06:26:10Z') && contract.includes('da67a18032f449d8477cdd5d7ea0aba02e691b0b'), '내부 계약 #1176 병합 metadata 불일치');

fs.renameSync(staging, publication);
const report = {
  sourceHead: rootManifest.sourceHead,
  sourceTree: rootManifest.sourceTree,
  trackedDiffSha256: rootManifest.trackedDiffSha256,
  selectedRootFileCount: rootManifest.stagedFileCount,
  publicationRoot: path.relative(repo, publication).split(path.sep).join('/'),
  routeCount: routes.length,
  totalBytes: routes.reduce((sum, item) => sum + item.bytes, 0),
  officialCount: '78/214',
  c11Complete: false,
  checkedRoadmapLinks,
  excludedRoutes: [
    { path: 'index.html', reason: '기존 Tailnet preview 시작 페이지를 보존함' },
    { path: '404.html', reason: '기존 Tailnet 404 route를 보존함' },
    { path: 'spec/plan/c11-css-grid.html', reason: '선택 카드에서 사용하지 않는 기존 열린 route를 보존함' },
  ],
  fullDocsLinkValidation: false,
  noDeletes: true,
  routes,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, routes: undefined }, null, 2)}\n`);
