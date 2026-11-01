import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const base = path.join(repo, 'build/spinon/roadmap-tailnet-sync-c11-4-unicode-2026-11-02/pass01');
const root = path.join(base, 'root');
const site = path.join(base, 'site-locked');
const publication = path.join(base, 'publication');
const reportPath = path.join(evidence, 'selected-route-audit.json');
const manifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication.json'), 'utf8'));
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(!fs.existsSync(reportPath), `기존 audit report 보존: ${reportPath}`);

const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
const marker = '#991 Rebase and merge';
const markerIndex = roadmap.indexOf(marker);
assert.ok(markerIndex >= 0, 'raw roadmap C11.3.3 card 누락');
const card = roadmap.slice(roadmap.lastIndexOf('<p ', markerIndex), roadmap.indexOf('</p>', markerIndex) + 4);
assert.ok(card.includes('108d0eed91220f81971f8c2eb7c4d2b727b26452'));
assert.ok(card.includes('72/213'));
assert.ok(card.includes('C11.4/C11은 미완료'));

const routes = new Map(manifest.routes.map((item) => [item.path, item]));
const checked = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `unexpected roadmap href: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && !route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe roadmap href: ${href}`);
  const item = routes.get(route);
  assert.ok(item, `local roadmap target omitted from publication: ${route}`);
  const generated = path.join(site, route);
  const raw = path.join(publication, route);
  const source = route.endsWith('.png') || route.endsWith('.svg') ? raw : generated;
  const bytes = fs.readFileSync(source);
  assert.equal(bytes.length, item.bytes, `route byte mismatch: ${route}`);
  assert.equal(hash(bytes), item.sha256, `route hash mismatch: ${route}`);
  checked.push({ href, route, bytes: bytes.length, sha256: item.sha256, verified: true });
}

const statusSource = fs.readFileSync(path.join(root, 'STATUS.md'), 'utf8');
assert.match(statusSource, /^- \[ \] \*\*C11 CSS Grid/m);
assert.ok(statusSource.includes('공식 `72/213`'));
const statusHtml = fs.readFileSync(path.join(site, 'STATUS.html'), 'utf8');
assert.ok(statusHtml.includes('72/213'));
assert.ok(statusHtml.includes('C11.4 anonymous Grid Unicode'));

const report = {
  sourceHead: manifest.sourceHead,
  rawRoadmapSha256: manifest.rawRoadmapSha256,
  officialCount: '72/213',
  c11Complete: false,
  roadmapCardMarker: marker,
  mergeSha: '108d0eed91220f81971f8c2eb7c4d2b727b26452',
  localRoadmapLinks: checked.length,
  fullSiteDeadLinkCheck: false,
  checks: checked,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
