import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const previousManifest = JSON.parse(fs.readFileSync(path.join(evidence, 'selected-root-manifest.json'), 'utf8'));
const buildBaseRelative = 'build/spinon/roadmap-c11-4-pr1277-status-docs-pass03-2026-11-08';
const buildBase = path.join(repo, buildBaseRelative);
const root = path.join(buildBase, 'root');
const manifestPath = path.join(evidence, 'selected-root-manifest-pass03.json');
const reportPath = path.join(evidence, 'stage-pass03.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
for (const target of [manifestPath, reportPath]) assert.ok(!fs.existsSync(target), `기존 evidence 보존: ${target}`);
for (const target of [buildBase, root]) {
  const stat = fs.lstatSync(target);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 경로: ${target}`);
}

const updatedRoutes = new Set([
  'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/README.md',
  'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-status-review-20.md',
  'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-pr-diff-review-30.md',
]);
function walk(relative = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(root, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const route = relative ? `${relative}/${entry.name}` : entry.name;
    const target = path.join(root, route);
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${route}`);
    if (entry.isDirectory()) files.push(...walk(route));
    else if (entry.isFile()) files.push(route);
    else throw new Error(`지원하지 않는 staged 항목: ${route}`);
  }
  return files;
}

const staged = walk().map((route) => {
  const sourceRelative = updatedRoutes.has(route) ? `spec/${route}` : previousManifest.staged.find((item) => item.route === route)?.source;
  assert.ok(sourceRelative, `source mapping 누락: ${route}`);
  const bytes = fs.readFileSync(path.join(root, route));
  assert.equal(sha256(bytes), sha256(fs.readFileSync(path.join(repo, sourceRelative))), `source hash mismatch: ${route}`);
  return { source: sourceRelative, route, bytes: bytes.length, sha256: sha256(bytes) };
});
assert.equal(staged.length, previousManifest.stagedFileCount + 2, '새 검토 파일 두 개만 추가됨');
const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['진행 중: C11.4 Grid 트랙 크기', "task.code + ' · 진행 중'", '78/214']) assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
const readme = fs.readFileSync(path.join(root, 'internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/README.md'), 'utf8');
for (const marker of ['selected-publication-result-pass03.json', 'served-link-verification-pass03.json', 'roadmap-pr-diff-review-30.md']) assert.ok(readme.includes(marker), `README marker 누락: ${marker}`);

const report = {
  buildBase: buildBaseRelative,
  selectedRoot: `${buildBaseRelative}/root`,
  copiedFromPreviousSelectedRoot: previousManifest.selectedRoot,
  updatedRoutes: [...updatedRoutes],
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  allFilesMatchCurrentSources: true,
  noSymlinks: true,
  fullDocsDeadLinkCheck: false,
  staged,
};
const stageCorrection = {
  firstPassAssertion: 'expected runtime generated label C11.4 · 진행 중 as a static source substring',
  actualSourceExpression: "task.code + ' · 진행 중'",
  correctedVerification: 'all staged file hashes match current source; static header and runtime expression markers pass',
  stagedFileCount: staged.length,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(reportPath, `${JSON.stringify(stageCorrection, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...report, staged: undefined }, null, 2)}\n`);
