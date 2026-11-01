import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { parse } from 'parse5';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-c11-plan-rollup-pr956-tailnet-20261101-pass02');
const site = path.join(buildBase, 'site');
const publicationManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication-tailnet-pass02.json'), 'utf8'));
const output = path.join(evidence, 'plan-link-route-audit-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const routes = new Map(publicationManifest.routes.map((route) => [route.path, route]));
const relevant = [];
let c11OverviewLink = null;

assert.ok(!fs.existsSync(output), `기존 audit를 덮어쓰지 않음: ${output}`);

function findElement(node, tagName) {
  if (node.tagName === tagName) return node;
  for (const child of node.childNodes ?? []) {
    const found = findElement(child, tagName);
    if (found) return found;
  }
  return null;
}

function textContent(node) {
  if (node.nodeName === '#text') return node.value;
  return (node.childNodes ?? []).map(textContent).join('');
}

function walkLinks(node, callback) {
  if (node.tagName === 'a') callback(node);
  for (const child of node.childNodes ?? []) walkLinks(child, callback);
}

for (const page of ['plan/c11-css-grid.html', 'plan/c11-4-grid-track-sizing.html']) {
  const html = fs.readFileSync(path.join(site, page), 'utf8');
  const main = findElement(parse(html), 'main');
  assert.ok(main, `렌더된 main이 없음: ${page}`);
  const pageLinks = [];
  walkLinks(main, (anchor) => {
    const href = anchor.attrs?.find((attr) => attr.name === 'href')?.value;
    if (!href || href.startsWith('#')) return;
    const label = textContent(anchor).trim().replace(/\s+/g, ' ');
    if (!/overflow-wrap|text-transform|plan-rollup/i.test(`${label} ${href}`)) return;
    const url = new URL(href, `https://preview.invalid/spinon/${page}`);
    if (url.origin !== 'https://preview.invalid' || !url.pathname.startsWith('/spinon/')) return;
    const routePath = decodeURIComponent(url.pathname.slice('/spinon/'.length));
    const route = routes.get(routePath);
    assert.ok(route, `${page}의 관련 local 링크가 publication에 없음: ${href} -> ${routePath}`);
    const item = { page, label, href, route: routePath, bytes: route.bytes, sha256: route.sha256 };
    pageLinks.push(item);
    relevant.push(item);
  });
  if (page === 'plan/c11-css-grid.html') {
    const target = [...routes.keys()].find((route) => route === 'plan/c11-4-grid-track-sizing.html');
    assert.ok(target, 'C11 overview detail route가 publication에 없음');
    c11OverviewLink = { page, route: target, bytes: routes.get(target).bytes, sha256: routes.get(target).sha256 };
  } else assert.ok(pageLinks.length > 0, `검사할 overflow-wrap/text-transform 링크가 없음: ${page}`);
}

const requiredRoutes = [
  'plan/c11-4-anonymous-grid-text-computed-overflow-wrap.html',
  'plan/c11-4-anonymous-grid-text-computed-text-transform.html',
  'spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-precomparison-2026-11-01/README.html',
  'spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-runtime-2026-11-01/README.html',
  'spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-runtime-2026-11-01/implementation-runtime-review-20.html',
  'spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-runtime-2026-11-01/pr-diff-review-30.html',
  'spec/internal/evidence/c11-4-anonymous-grid-text-transform-precomparison-2026-11-01/README.html',
  'spec/internal/evidence/c11-4-anonymous-grid-text-transform-runtime-2026-11-01/README.html',
  'spec/internal/evidence/c11-4-plan-rollup-sync-pr952-2026-11-01/README.html',
];
for (const route of requiredRoutes) assert.ok(routes.has(route), `필수 관련 route가 publication에 없음: ${route}`);

const report = {
  sourceHead: publicationManifest.sourceHead,
  publicationRoot: publicationManifest.publicationRoot,
  pageCount: 2,
  c11OverviewLink,
  fullDocsDeadLinkCheck: false,
  checkedLinks: relevant,
  checkedLinkCount: relevant.length,
  requiredRoutes: requiredRoutes.map((route) => ({ route, ...routes.get(route) })),
  allCheckedLinksInPublication: true,
};
fs.writeFileSync(output, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, checkedLinks: undefined }, null, 2)}\n`);
