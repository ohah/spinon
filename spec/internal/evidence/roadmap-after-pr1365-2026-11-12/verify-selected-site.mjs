import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidenceDirectory, '../../../..');
const site = path.join(repo, 'build/spinon/roadmap-after-pr1365-2026-11-12/site');
const reportPath = path.join(evidenceDirectory, 'selected-site-verification.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const pages = [
  { route: 'STATUS.html', markers: ['1365', '78/214', 'C11.4'] },
  { route: 'spec/STATUS.html', markers: ['1365', '78/214', 'C11.4'] },
  { route: 'plan/c11-4-grid-track-sizing.html', markers: ['1365', '다음 bounded 후보', '78/214'] },
  { route: 'plan/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum.html', markers: ['RuntimeSession', '검증 완료', '#1365', '78/214'] },
  { route: 'internal/0059-c11-grid-layout.html', markers: ['1365', 'C11.4'] },
  { route: 'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/README.html', markers: ['11,520', '544 passed / 1 failed', 'PR #1365'] },
  { route: 'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/implementation-runtime-review-20.html', markers: ['20개', 'C11.4'] },
  { route: 'internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-runtime-2026-11-12/pr-diff-review-30.html', markers: ['30개', 'PR #1365'] },
];
assert.ok(!fs.existsSync(reportPath), `existing verification report is preserved: ${reportPath}`);
const checked = [];
const assets = new Set();
for (const { route, markers } of pages) {
  const bytes = fs.readFileSync(path.join(site, route));
  const html = bytes.toString('utf8');
  for (const marker of markers) assert.ok(html.includes(marker), `missing marker ${JSON.stringify(marker)} in ${route}`);
  for (const match of html.matchAll(/(?:src|href)="(\/spinon\/static\/[^"?#]+)"/g)) {
    const assetRoute = decodeURIComponent(match[1].slice('/spinon/'.length));
    assert.ok(fs.statSync(path.join(site, assetRoute)).isFile(), `missing built asset ${assetRoute}`);
    assets.add(assetRoute);
  }
  checked.push({ route, bytes: bytes.length, sha256: sha256(bytes), markers });
}
const report = {
  schema: 'spinon.tailnet-preview-selected-site-verification/v1',
  sourceCommit: '5a7ed74d197c78d287cc12a0823a2abc77050f3f',
  rspressVersion: '2.0.22',
  selectedPagesOnly: true,
  selectedPageCount: checked.length,
  referencedStaticAssets: assets.size,
  fullDocsDeadLinkCheck: false,
  fullSiteBuildClaim: false,
  pages: checked,
  assets: [...assets].sort().map((route) => {
    const bytes = fs.readFileSync(path.join(site, route));
    return { route, bytes: bytes.length, sha256: sha256(bytes) };
  }),
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
