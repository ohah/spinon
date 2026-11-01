import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1011-20261102/pass07');
const site = path.join(buildBase, 'site');
const previewRoot = '/Users/yoonhb/Library/Application Support/SpinonPreview';
const roadmapPath = path.join(repo, 'spec/public/roadmap.html');
const statusPath = path.join(repo, 'spec/STATUS.md');
const reviewPath = path.join(evidence, 'roadmap-pr-diff-review-30.md');
const reportPath = path.join(evidence, 'candidate-link-verification-pass07.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

assert.ok(!fs.existsSync(reportPath), `preserving existing report: ${reportPath}`);
const roadmap = fs.readFileSync(roadmapPath, 'utf8');
const cards = [...roadmap.matchAll(/<p class="kanban-note hero-progress">([\s\S]*?)<\/p>/g)].map((match) => match[1]);
const card = cards.find((html) => html.includes('PR #1011 Rebase and merge'));
assert.ok(card, 'merged PR #1011 roadmap card missing');
assert.ok(card.includes('40725c44d5cfe7ea187fe7731c47fed439c3b98f'), 'merge SHA missing from roadmap card');

const status = fs.readFileSync(statusPath, 'utf8');
for (const [pattern, label] of [
  [/^- \[ \] \*\*C11 CSS Grid/m, 'C11 must remain unchecked'],
  [/^  - \[ \] \*\*C11\.3 /m, 'C11.3 must remain unchecked'],
  [/^  - \[x\] \*\*C11\.2\.2 /m, 'C11.2.2 checkbox mismatch'],
  [/^  - \[x\] \*\*C11\.2\.3 /m, 'C11.2.3 checkbox mismatch'],
  [/^- \[ \] \*\*C11\.4 /m, 'C11.4 must remain unchecked'],
]) assert.ok(pattern.test(status), label);
assert.ok(status.includes('72/213'), 'official completion count missing');

const localRoutes = [];
for (const match of card.matchAll(/href="([^"]+)"/g)) {
  const href = match[1];
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  const url = new URL(href, 'https://preview.invalid/spinon/roadmap.html');
  assert.equal(url.origin, 'https://preview.invalid', `unexpected local link origin: ${href}`);
  assert.ok(url.pathname.startsWith('/spinon/'), `unexpected local base path: ${href}`);
  const route = decodeURIComponent(url.pathname.slice('/spinon/'.length));
  const siteTarget = path.join(site, route);
  const liveTarget = path.join(previewRoot, route);
  const sourceTarget = path.join(repo, 'spec', route);
  const candidates = [siteTarget, liveTarget, sourceTarget];
  const target = candidates.find((candidate) => {
    try { return fs.lstatSync(candidate).isFile(); } catch { return false; }
  });
  assert.ok(target, `local roadmap link has no build/live/source file: ${href}`);
  const bytes = fs.readFileSync(target);
  localRoutes.push({ href, route, resolvedFrom: path.relative(repo, target), bytes: bytes.length, sha256: sha256(bytes) });
}
assert.ok(localRoutes.length >= 5, 'expected roadmap card local links were not checked');

const generatedRoutes = [
  'STATUS.html',
  'plan/c11-css-grid.html',
  'plan/c11-root-html-grid-anonymous-text.html',
  'internal/evidence/c11-root-html-grid-anonymous-text-precomparison-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-anonymous-text-implementation-2026-11-02/README.html',
  'internal/evidence/c11-root-html-grid-anonymous-text-implementation-2026-11-02/implementation-runtime-review-20.html',
  'internal/evidence/c11-root-html-grid-anonymous-text-implementation-2026-11-02/implementation-pr-diff-review-30.html',
  'internal/evidence/roadmap-tailnet-sync-pr1011-2026-11-02/roadmap-pr-diff-review-30.html',
  'plan/c11-root-html-grid-anonymous-text-plan-review-20.html',
  'spec/STATUS.html',
  'spec/plan/c11-css-grid.html',
  'spec/plan/c11-root-html-grid-anonymous-text.html',
  'spec/internal/evidence/c11-root-html-grid-anonymous-text-precomparison-2026-11-02/README.html',
  'spec/internal/evidence/c11-root-html-grid-anonymous-text-implementation-2026-11-02/README.html',
  'spec/internal/evidence/roadmap-tailnet-sync-pr1011-2026-11-02/roadmap-pr-diff-review-30.html',
  'spec/plan/c11-root-html-grid-anonymous-text-plan-review-20.html',
];
for (const route of generatedRoutes) assert.ok(fs.statSync(path.join(site, route)).isFile(), `generated route missing: ${route}`);
const renderedStatus = fs.readFileSync(path.join(site, 'STATUS.html'), 'utf8');
const renderedPlan = fs.readFileSync(path.join(site, 'plan/c11-root-html-grid-anonymous-text.html'), 'utf8');
const renderedReview = fs.readFileSync(path.join(site, 'internal/evidence/roadmap-tailnet-sync-pr1011-2026-11-02/roadmap-pr-diff-review-30.html'), 'utf8');
for (const [label, html] of [['STATUS', renderedStatus], ['plan', renderedPlan], ['review', renderedReview]]) {
  assert.ok(html.includes('40725c44d5cfe7ea187fe7731c47fed439c3b98f'), `${label} rendered without merge SHA`);
}
assert.ok(renderedStatus.includes('72/213'), 'rendered official status count missing');
assert.ok(renderedReview.includes('Tailnet preview'), 'rendered review boundary missing');

const report = {
  sourceHead: JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-pass07.json'), 'utf8')).sourceHead,
  generatedRouteCount: generatedRoutes.length,
  checkedRoadmapLocalRouteCount: localRoutes.length,
  officialC11Checkboxes: { C11: 'unchecked', C11_2_2: 'checked', C11_2_3: 'checked', C11_3: 'unchecked', C11_4: 'unchecked' },
  officialCount: '72/213',
  fullDocsDeadLinkCheck: false,
  candidateRoutes: localRoutes,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, candidateRoutes: undefined }, null, 2)}\n`);
