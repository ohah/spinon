import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../../../..');
const [pass = 'initial'] = process.argv.slice(2);
assert.ok(['initial', 'pass03'].includes(pass), 'usage: node audit-selected-routes.mjs [initial|pass03]');
const buildBase = path.join(repo, pass === 'pass03'
  ? 'build/spinon/roadmap-c11-4-pr1277-status-docs-pass03-2026-11-08'
  : 'build/spinon/roadmap-c11-4-pr1277-status-docs-2026-11-08');
const root = path.join(buildBase, 'root');
const site = path.join(buildBase, 'site');
const roadmapSource = path.join(repo, 'spec/public/roadmap.html');
const reportPath = path.join(evidence, pass === 'pass03' ? 'selected-route-audit-pass03.json' : 'selected-route-audit.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.ok(fs.statSync(site).isDirectory(), 'Rspress site must exist');
assert.ok(!fs.existsSync(reportPath), `기존 evidence를 덮어쓰지 않음: ${reportPath}`);

function walk(relative = '') {
  const files = [];
  for (const entry of fs.readdirSync(path.join(site, relative), { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const child = relative ? `${relative}/${entry.name}` : entry.name;
    assert.ok(!entry.isSymbolicLink(), `generated symlink 거부: ${child}`);
    if (entry.isDirectory()) files.push(...walk(child));
    else if (entry.isFile()) files.push(child);
    else throw new Error(`지원하지 않는 build 항목: ${child}`);
  }
  return files;
}

const roadmap = fs.readFileSync(roadmapSource, 'utf8');
const card = [...roadmap.matchAll(/<p class="kanban-note hero-progress">[\s\S]*?<\/p>/g)]
  .map((match) => match[0]).find((html) => html.includes('PR #1277 Rebase and merge'));
assert.ok(card, 'PR #1277 progress card missing');
const checked = [];
for (const [, href] of card.matchAll(/href="([^"]+)"/g)) {
  if (/^(?:https?:|mailto:)/i.test(href)) continue;
  assert.ok(href.startsWith('/spinon/'), `unexpected local link: ${href}`);
  const route = decodeURIComponent(href.slice('/spinon/'.length).split(/[?#]/, 1)[0]);
  assert.ok(route && !route.startsWith('/') && route.split('/').every((part) => part && part !== '.' && part !== '..'), `unsafe local route: ${href}`);
  const target = path.join(site, route);
  const stat = fs.lstatSync(target);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), `missing generated route for ${href}`);
  const bytes = fs.readFileSync(target);
  checked.push({ href, route, bytes: bytes.length, sha256: sha256(bytes) });
}

const activeRoadmap = fs.readFileSync(path.join(root, 'roadmap.html'), 'utf8');
for (const marker of ['진행 중: C11.4 Grid 트랙 크기', "title:'Grid 트랙 크기'", "task.code + ' · 진행 중'", '78/214']) {
  assert.ok(activeRoadmap.includes(marker), `raw roadmap marker missing in generated site: ${marker}`);
}
for (const [route, marker] of [
  ['internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/README.html', 'C11.4 진행 표시와 로드맵 링크 확인'],
  ['internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-status-review-20.html', '로드맵 진행 상태 화면 적대적 검토 20개'],
  ['internal/evidence/roadmap-tailnet-sync-pr1277-c11-4-2026-11-08/roadmap-pr-diff-review-30.html', '로드맵·Tailnet 동기화 PR 변경 실패 관점 검토 30개'],
  ['internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08/pr-diff-review-30.html', 'PR diff 검토'],
]) {
  assert.ok(fs.readFileSync(path.join(site, route), 'utf8').includes(marker), `route marker missing: ${route}`);
}
const files = walk();
const report = {
  rspressVersion: '2.0.22',
  buildLog: 'selected-rspress-build.log',
  fullDocsDeadLinkCheck: false,
  roadmapSource: 'spec/public/roadmap.html',
  roadmapActiveMarkers: ['진행 중: C11.4 Grid 트랙 크기', "title:'Grid 트랙 크기'", "task.code + ' · 진행 중'", '78/214'],
  checkedCardLocalLinks: checked.length,
  checkedCardRoutes: checked,
  generatedFileCount: files.length,
  generatedBytes: files.reduce((sum, route) => sum + fs.statSync(path.join(site, route)).size, 0),
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
