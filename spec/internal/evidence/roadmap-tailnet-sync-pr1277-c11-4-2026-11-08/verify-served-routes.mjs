import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const [pass = 'pass02'] = process.argv.slice(2);
assert.ok(['pass02', 'pass03'].includes(pass), 'usage: node verify-served-routes.mjs [pass02|pass03]');
const manifest = JSON.parse(fs.readFileSync(path.join(evidence, `selected-publication-manifest-${pass}.json`), 'utf8'));
const publishReport = JSON.parse(fs.readFileSync(path.join(evidence, `selected-publication-result-${pass}.json`), 'utf8'));
const reportPath = path.join(evidence, `served-link-verification-${pass}.json`);
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const routes = [
  { route: 'roadmap.html', markers: ['진행 중: C11.4 Grid 트랙 크기', '78/214'] },
  { route: 'plan/c11-4-root-html-fit-content-min-content-auto-remaining-order.html', markers: ['fit-content', 'C11.4'] },
  { route: 'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-precomparison-2026-11-08/README.html', markers: ['Chrome 157', 'C11.4'] },
  { route: 'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08/README.html', markers: ['Android', 'iOS'] },
  { route: 'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08/pr-diff-review-30.html', markers: ['C11.4'] },
  { route: 'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/README.html', markers: ['C11.4 진행 표시와 로드맵 링크 확인'] },
  { route: 'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-status-review-20.html', markers: ['로드맵 진행 상태 화면 적대적 검토 20개'] },
  { route: 'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-pr-diff-review-30.html', markers: ['로드맵·Tailnet 동기화 PR 변경 실패 관점 검토 30개'] },
  { route: 'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08/android/final/screenshot.png', contentType: 'image/png' },
  { route: 'internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08/ios/final/screenshot.png', contentType: 'image/png' },
];
assert.ok(!fs.existsSync(reportPath), `기존 served report를 덮어쓰지 않음: ${reportPath}`);
assert.equal(publishReport.noDeletes, true, 'publication must not delete files');
assert.equal(publishReport.serverRestarted, false, 'preview server must not be restarted');

const assets = new Set();
for (const item of manifest.entries) {
  if (!item.route.endsWith('.html')) continue;
  if (!['roadmap.html', ...routes.map(({ route }) => route)].includes(item.route)) continue;
  const html = fs.readFileSync(path.join(repo, manifest.buildBase, item.kind === 'rspress-generated' ? 'site' : 'root', item.route), 'utf8');
  for (const [, url] of html.matchAll(/(?:src|href)="(\/spinon\/static\/[^"?#]+)"/g)) {
    assets.add(decodeURIComponent(url.slice('/spinon/'.length)));
  }
}

async function verifyOne(route, base) {
  const response = await fetch(`${base}${encodeURI(route)}`, { cache: 'no-store', signal: AbortSignal.timeout(30000) });
  const bytes = Buffer.from(await response.arrayBuffer());
  assert.equal(response.status, 200, `${base}${route} HTTP ${response.status}`);
  const expected = manifest.entries.find((item) => item.route === route);
  assert.ok(expected, `route missing from selected publication manifest: ${route}`);
  assert.equal(sha256(bytes), expected.sha256, `${base}${route} served hash mismatch`);
  const check = routes.find((item) => item.route === route);
  if (check?.markers) {
    const text = bytes.toString('utf8');
    for (const marker of check.markers) assert.ok(text.includes(marker), `${base}${route} marker missing: ${marker}`);
  }
  if (check?.contentType) assert.ok((response.headers.get('content-type') ?? '').includes(check.contentType), `${base}${route} unexpected content-type`);
  return {
    url: `${base}${route}`,
    status: response.status,
    contentType: response.headers.get('content-type'),
    bytes: bytes.length,
    sha256: sha256(bytes),
    markers: check?.markers ?? [],
  };
}

const localBase = 'http://127.0.0.1:8768/spinon/';
const tailnetBase = 'https://macstudio.tailed42f2.ts.net/spinon/';
const checks = [];
for (const base of [localBase, tailnetBase]) {
  for (const { route } of routes) checks.push(await verifyOne(route, base));
  for (const route of [...assets].sort()) checks.push(await verifyOne(route, base));
}
const report = {
  verifiedAt: new Date().toISOString(),
  localAndTailnetChecks: checks.length,
  pageAndImageRoutesPerBase: routes.length,
  generatedStaticAssetsPerBase: assets.size,
  allHttp200: true,
  servedHashesMatchPublishedManifest: true,
  fullSiteDeadLinkCheck: false,
  checks,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
