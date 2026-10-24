import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash, randomUUID } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const outputRoot = path.join(repositoryRoot, 'build/spinon/c11-4-fixed-repeat-auto-repeat-site-postmerge-selected');
const reportPath = path.join(evidenceDirectory, 'postmerge-selected-build-verification.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.lstatSync(outputRoot).isDirectory(), 'post-merge selected RSPress build is required');
assert.ok(!fs.existsSync(reportPath), 'refusing to overwrite post-merge build verification');

const rawAssets = [
  { route: 'roadmap.html', source: 'spec/public/roadmap.html' },
  { route: 'STATUS.md', source: 'spec/STATUS.md' },
];
for (const asset of rawAssets) {
  const bytes = fs.readFileSync(path.join(repositoryRoot, asset.source));
  const destination = path.join(outputRoot, asset.route);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  const temporary = `${destination}.spinon-${randomUUID()}.tmp`;
  fs.writeFileSync(temporary, bytes, { flag: 'wx' });
  try {
    fs.renameSync(temporary, destination);
  } catch (error) {
    if (fs.existsSync(temporary)) fs.unlinkSync(temporary);
    throw error;
  }
  assert.equal(sha256(fs.readFileSync(destination)), sha256(bytes), `raw asset mismatch: ${asset.route}`);
}

const routes = [
  'roadmap.html',
  'STATUS.html',
  'STATUS.md',
  'plan/c11-4-auto-repeat-fixed-repeat.html',
  'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/README.html',
  'internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/roadmap-pr-diff-review-30.html',
];
const markers = new Map([
  ['roadmap.html', ['PR #591', '79a2f3e2ec412d8a039125cf8930b3c633c04b63', '72/213']],
  ['STATUS.md', ['fixed-repeat + auto-repeat 제한 runtime slice 구현·병합 기록', '79a2f3e2ec412d8a039125cf8930b3c633c04b63', '72/213']],
  ['STATUS.html', ['79a2f3e2ec412d8a039125cf8930b3c633c04b63', '72/213']],
  ['plan/c11-4-auto-repeat-fixed-repeat.html', ['PR #591', '79a2f3e2ec412d8a039125cf8930b3c633c04b63']],
  ['internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/README.html', ['PR #591', '79a2f3e2ec412d8a039125cf8930b3c633c04b63']],
  ['internal/evidence/c11-4-fixed-repeat-auto-repeat-runtime-2026-10-24/roadmap-pr-diff-review-30.html', ['30개의 서로 다른 문서 실패 경로']],
]);
const selectedRoutes = [];
for (const route of routes) {
  const file = path.join(outputRoot, route);
  const stat = fs.lstatSync(file);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing/unsafe route: ${route}`);
  const bytes = fs.readFileSync(file);
  const content = bytes.toString('utf8');
  for (const marker of markers.get(route) ?? []) assert.ok(content.includes(marker), `${route} lacks marker ${marker}`);
  selectedRoutes.push({ path: route, bytes: bytes.length, sha256: sha256(bytes) });
}
const report = {
  selectedOutputRoot: path.relative(repositoryRoot, outputRoot),
  selectedRouteCount: selectedRoutes.length,
  selectedRoutes,
  rawAssets,
  noDeletes: true,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ selectedOutputRoot: report.selectedOutputRoot, selectedRouteCount: selectedRoutes.length, noDeletes: true }, null, 2)}\n`);
