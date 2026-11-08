import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const manifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-publication-manifest-pass04.json'), 'utf8'));
const reportPath = path.join(evidence, 'served-verification-pass04.json');
const baseUrl = 'https://macstudio.tailed42f2.ts.net/spinon/';
const routes = [
  'roadmap.html',
  'STATUS.html',
  'spec/STATUS.html',
  'internal/0059-c11-grid-layout.html',
  'plan/c11-4-root-html-fit-min-max-content-auto-rows.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/README.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/plan-pr-diff-review-30.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08/README.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08/implementation-runtime-review-20.html',
  'internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08/pr-diff-review-30.html',
];
const results = [];
for (const route of routes) {
  const expected = manifest.entries.find((entry) => entry.route === route);
  assert.ok(expected, `publication manifest에 route 없음: ${route}`);
  const response = await fetch(new URL(route, baseUrl), { cache: 'no-store', signal: AbortSignal.timeout(20000) });
  const bytes = Buffer.from(await response.arrayBuffer());
  const sha256 = createHash('sha256').update(bytes).digest('hex');
  results.push({ route, status: response.status, cacheControl: response.headers.get('cache-control'), bytes: bytes.length, sha256, expectedSha256: expected.sha256, matchesPublishedBytes: sha256 === expected.sha256 });
}
const report = {
  url: baseUrl,
  verifiedAt: new Date().toISOString(),
  status200: results.every((entry) => entry.status === 200),
  hashesMatchPublication: results.every((entry) => entry.matchesPublishedBytes),
  checkedRoutes: results.length,
  fullSiteDeadLinkCheck: false,
  entries: results,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, entries: undefined }, null, 2)}\n`);
assert.ok(report.status200 && report.hashesMatchPublication, 'Tailnet 응답이 게시 manifest와 다름');
