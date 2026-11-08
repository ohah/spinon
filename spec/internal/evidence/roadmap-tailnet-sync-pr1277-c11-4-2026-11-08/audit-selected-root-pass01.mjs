import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const buildBase = path.join(repo, 'build/spinon/roadmap-c11-4-pr1277-status-docs-2026-11-08');
const root = path.join(buildBase, 'root');
const manifestPath = path.join(evidence, 'selected-root-manifest.json');
const reportPath = path.join(evidence, 'stage-marker-audit-pass01.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
for (const target of [root, buildBase]) {
  const stat = fs.lstatSync(target);
  assert.ok(stat.isDirectory() && !stat.isSymbolicLink(), `안전하지 않은 경로: ${target}`);
}
for (const target of [manifestPath, reportPath]) assert.ok(!fs.existsSync(target), `기존 evidence 보존: ${target}`);

function walk(relative = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(root, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    const stat = fs.lstatSync(path.join(root, child));
    assert.ok(!entry.isSymbolicLink(), `symlink 거부: ${child}`);
    if (entry.isDirectory()) files.push(...walk(child));
    else if (entry.isFile()) files.push(child);
    else throw new Error(`지원하지 않는 staged 항목: ${child}`);
  }
  return files;
}

function sourceFor(route) {
  if (['roadmap.html', 'public/roadmap.html', 'spec/public/roadmap.html'].includes(route)) return 'spec/public/roadmap.html';
  if (['STATUS.md', 'spec/STATUS.md'].includes(route)) return 'spec/STATUS.md';
  if (['icon.svg', 'public/icon.svg'].includes(route)) return 'spec/public/icon.svg';
  if (['logo.svg', 'public/logo.svg'].includes(route)) return 'spec/public/logo.svg';
  return route.startsWith('internal/') ? `spec/${route}` : route;
}

const staged = walk().map((route) => {
  const source = sourceFor(route);
  const sourceStat = fs.lstatSync(path.join(repo, source));
  assert.ok(sourceStat.isFile() && !sourceStat.isSymbolicLink(), `원본 파일이 아님: ${source}`);
  const bytes = fs.readFileSync(path.join(root, route));
  assert.equal(sha256(bytes), sha256(fs.readFileSync(path.join(repo, source))), `원본과 staged hash 불일치: ${route}`);
  return { source, route, bytes: bytes.length, sha256: sha256(bytes) };
});
const roadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['진행 중: C11.4 Grid 트랙 크기', "title:'Grid 트랙 크기'", "task.code + ' · 진행 중'", '78/214', '로드맵 진행 표시와 링크 점검']) {
  assert.ok(roadmap.includes(marker), `roadmap marker 누락: ${marker}`);
}

const manifest = {
  buildBase: path.relative(repo, buildBase).split(path.sep).join('/'),
  selectedRoot: path.relative(repo, root).split(path.sep).join('/'),
  stagedFileCount: staged.length,
  stagedBytes: staged.reduce((sum, item) => sum + item.bytes, 0),
  allStagedFilesMatchTheirSources: true,
  noSymlinks: true,
  fullDocsDeadLinkCheck: false,
  roadmapMarkers: ['진행 중: C11.4 Grid 트랙 크기', "title:'Grid 트랙 크기'", "task.code + ' · 진행 중'", '78/214'],
  staged,
};
const markerAudit = {
  firstStageAssertion: 'failed only because it expected the runtime-generated label C11.4 · 진행 중 as one source literal',
  sourceRepresentation: "task.code + ' · 진행 중'",
  correctedAudit: 'walked every staged file and verified its SHA-256 against its source; all markers passed',
  stagedFileCount: staged.length,
  allStagedFilesMatchTheirSources: true,
};
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, { flag: 'wx' });
fs.writeFileSync(reportPath, `${JSON.stringify(markerAudit, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify({ ...manifest, staged: undefined }, null, 2)}\n`);
