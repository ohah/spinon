import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidenceDirectory, '../../../..');
const site = path.join(repo, 'build/spinon/roadmap-tailnet-sync-pr1363-c114-sixfit-plan-2026-11-12/site');
const reportPath = path.join(evidenceDirectory, 'selected-site-verification.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const pageChecks = [
  {
    route: 'plan/c11-4-grid-track-sizing.html',
    markers: ['C11.4 six-fit repeated-text row-span', '11,520 scenes', '계획 PR 병합 전에는 제품 코드를 수정하지 않는다'],
  },
  {
    route: 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.html',
    markers: ['C11.4 six-fit repeated-text row-span', '11,520 scenes', '78/214'],
  },
  {
    route: 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-review-20.html',
    markers: ['계획 실패 관점 검토 20개', 'fit-content'],
  },
  {
    route: 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-plan-pr-diff-review-30.html',
    markers: ['계획 PR 변경 검토 30개', 'payload-floor'],
  },
  {
    route: 'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/README.html',
    markers: ['11,520 scenes', '4,800', 'Spinon runtime'],
  },
];
const checked = [];
const assets = new Set();
for (const { route, markers } of pageChecks) {
  const absolute = path.join(site, route);
  const bytes = fs.readFileSync(absolute);
  const html = bytes.toString('utf8');
  for (const marker of markers) assert.ok(html.includes(marker), `missing marker ${JSON.stringify(marker)} in ${route}`);
  for (const match of html.matchAll(/(?:src|href)="(\/spinon\/static\/[^"?#]+)"/g)) {
    const assetRoute = decodeURIComponent(match[1].slice('/spinon/'.length));
    const assetPath = path.join(site, assetRoute);
    assert.ok(fs.statSync(assetPath).isFile(), `missing built asset ${assetRoute}`);
    assets.add(assetRoute);
  }
  checked.push({ route, bytes: bytes.length, sha256: sha256(bytes), markers });
}
const report = {
  rspressVersion: '2.0.22',
  selectedRootOnly: true,
  fullSiteBuildClaim: false,
  fullDocsDeadLinkCheck: false,
  checkedPages: checked.length,
  checkedAssets: assets.size,
  pages: checked,
  assets: [...assets].sort().map((route) => {
    const bytes = fs.readFileSync(path.join(site, route));
    return { route, bytes: bytes.length, sha256: sha256(bytes) };
  }),
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
