import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const manifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication-manifest-pass02.json'), 'utf8'));
const publishReport = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication-result-pass02.json'), 'utf8'));
const reportPath = path.join(evidence, 'served-verification-pass02.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const routes = [
  { route: 'roadmap.html', markers: ['다음 구현: C11.4 반복 intrinsic tail 30순서', 'PR #1284', '78/214'] },
  { route: 'STATUS.html', markers: ['C11.4', '78/214'] },
  { route: 'spec/STATUS.html', markers: ['C11.4', '78/214'] },
  { route: 'plan/c11-4-grid-track-sizing.html', markers: ['PR #1284', '30개 고유 순서'] },
  { route: 'plan/c11-4-root-html-repeated-intrinsic-auto-rows.html', markers: ['PR #1284', 'Tailnet'] },
  { route: 'plan/c11-4-root-html-repeated-intrinsic-tail-permutations.html', markers: ['30개 고유 순서', 'C11.4'] },
  { route: 'plan/c11-4-root-html-repeated-intrinsic-tail-permutations-plan-review-20.html', markers: ['계획 실패 관점 검토 20개'] },
  { route: 'internal/0059-c11-grid-layout.html', markers: ['C11.4 next bounded candidate', '30개 고유 순서'] },
  { route: 'spec/internal/0059-c11-grid-layout.html', markers: ['C11.4 next bounded candidate', '30개 고유 순서'] },
  { route: 'internal/evidence/c11-4-root-html-repeated-intrinsic-tail-permutations-precomparison-2026-11-08/README.html', markers: ['120/120 scene', '9,480'] },
  { route: 'internal/evidence/c11-4-root-html-repeated-intrinsic-tail-permutations-precomparison-2026-11-08/chrome-agent-browser-157.png', contentType: 'image/png' },
  { route: 'internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/README.html', markers: ['Grid 24순서 구현', '다음'] },
  { route: 'internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/roadmap-status-review-20.html', markers: ['roadmap 상태·화면 실패 관점 검토 20개'] },
  { route: 'internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/roadmap-pr-diff-review-30.html', markers: ['roadmap sync PR diff 실패 관점 검토 30개'] },
  { route: 'internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/live-before-desktop.png', contentType: 'image/png' },
  { route: 'tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-tail-permutations-precomparison.html', contentType: 'text/html' },
];
assert.ok(!fs.existsSync(reportPath), `preserve existing served report: ${reportPath}`);
assert.equal(publishReport.noDeletes, true, 'publication must not delete files');
assert.equal(publishReport.serverRestarted, false, 'preview server must not restart');

const assets = new Set();
for (const { route } of routes) {
  if (!route.endsWith('.html')) continue;
  const entry = manifest.entries.find((item) => item.route === route);
  assert.ok(entry, `route missing from publication manifest: ${route}`);
  const html = fs.readFileSync(path.join(repo, manifest.buildBase, entry.kind === 'rspress-generated' ? 'site' : 'root', route), 'utf8');
  for (const [, url] of html.matchAll(/(?:src|href)="(\/spinon\/static\/[^"?#]+)"/g)) {
    assets.add(decodeURIComponent(url.slice('/spinon/'.length)));
  }
}

async function verifyOne(route, base) {
  const response = await fetch(`${base}${encodeURI(route)}`, { cache: 'no-store', signal: AbortSignal.timeout(30000) });
  const bytes = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, `${base}${route} HTTP ${response.status}`);
  const expected = manifest.entries.find((item) => item.route === route);
  assert.ok(expected, `route missing from manifest: ${route}`);
  assert.equal(sha256(bytes), expected.sha256, `${base}${route} served hash mismatch`);
  const check = routes.find((item) => item.route === route);
  if (check?.markers) {
    const body = bytes.toString('utf8');
    for (const marker of check.markers) assert.ok(body.includes(marker), `${base}${route} missing marker: ${marker}`);
  }
  if (check?.contentType) {
    assert.ok((response.headers.get('content-type') ?? '').includes(check.contentType), `${base}${route} unexpected content type`);
  }
  return {
    url: `${base}${route}`,
    status: response.status,
    contentType: response.headers.get('content-type'),
    bytes: bytes.length,
    sha256: sha256(bytes),
    markers: check?.markers ?? [],
  };
}

const bases = ['http://127.0.0.1:8768/spinon/', 'https://macstudio.tailed42f2.ts.net/spinon/'];
const checks = [];
for (const base of bases) {
  for (const item of routes) checks.push(await verifyOne(item.route, base));
  for (const route of [...assets].sort()) checks.push(await verifyOne(route, base));
}
const report = {
  verifiedAt: new Date().toISOString(),
  selectedPageAndImageRoutesPerBase: routes.length,
  generatedStaticAssetsPerBase: assets.size,
  httpAndHashChecks: checks.length,
  allHttp200: true,
  servedHashesMatchPublishedManifest: true,
  fullSiteDeadLinkCheck: false,
  noDeletes: true,
  serverRestarted: false,
  checks,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
