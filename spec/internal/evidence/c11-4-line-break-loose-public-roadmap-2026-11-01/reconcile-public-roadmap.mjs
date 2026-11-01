import assert from 'node:assert/strict';
import fs from 'node:fs';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';

const roadmapPath = 'spec/public/roadmap.html';
const reportPath = 'spec/internal/evidence/c11-4-line-break-loose-public-roadmap-2026-11-01/public-roadmap-reconcile.json';
const sha256 = (text) => createHash('sha256').update(text).digest('hex');
const original = fs.readFileSync(roadmapPath, 'utf8');
assert.ok(!fs.existsSync(reportPath), `기존 reconcile report를 덮어쓰지 않음: ${reportPath}`);

const cards = [...original.matchAll(/<p class="kanban-note hero-progress">[\s\S]*?<\/p>/g)].map((match) => match[0]);
const findOne = (label, predicate) => {
  const matches = cards.filter(predicate);
  assert.equal(matches.length, 1, `${label} 카드를 하나만 찾아야 한다`);
  return matches[0];
};
const looseCard = findOne('line-break loose PR #962', (card) => card.includes('PR #962') && card.includes('line-break: loose'));
const flagsPlanCard = findOne('text-indent flags 계획 후보', (card) => card.includes('text-indent: hanging') && card.includes('each-line') && card.includes('계획 후보'));
const transformCard = findOne('text-transform PR #952', (card) => card.includes('PR #952') && card.includes('text-transform'));

const flagsCard = '<p class="kanban-note hero-progress"><strong>2026-11-01 C11.4 anonymous Grid text computed <code>text-indent: hanging</code> / <code>each-line</code> · 제한 opt-in fixture 구현 · <a href="https://github.com/ohah/spinon/pull/959">구현 PR #959 Rebase and merge</a> (2026-11-01T02:06:32Z, merge SHA <code>590f5946e97e0252113abf34b23ae31b13819acf</code>):</strong> Parley 0.12.0 <code>IndentOptions</code>와 줄별 indent 전달을 20개 고정 scene에 연결했다. 기존 양수 18개와 양수·음수 30개 행렬을 회귀로 유지했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 flags 20/20, positive 18/18, positive+negative 30/30을 확인했다. 두 플랫폼의 여섯 행렬 총 136 scene은 draw/present 성공, 예상 root frame, 양수 surface texture 및 superseded retry 0을 기록했다. PR check rollup과 required status check이 보고되지 않아 CI 성공으로 세지 않는다. 일반 CSS admission, WPT, glyph pixels, hardware GPU 및 전체 C11.4/C11 지원은 포함하지 않고 공식 <code>72/213</code>과 미완료 상태를 유지한다. <a href="/spinon/plan/c11-4-anonymous-grid-text-indent-flags.html">계획</a> · <a href="/spinon/internal/0059-c11-grid-layout.html">내부 계약 0059</a> · <a href="/spinon/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/README.html">실행·build hash·한계</a> · <a href="/spinon/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/pr-diff-review-30.html">PR diff 검토 30개</a> · <a href="/spinon/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/post-merge-status-review-30.html">병합 후 상태 검토 30개</a></p>';

const remaining = original.replace(looseCard, '').replace(flagsPlanCard, '');
const anchorIndex = remaining.indexOf(transformCard);
assert.ok(anchorIndex >= 0 && remaining.indexOf(transformCard, anchorIndex + 1) < 0, 'PR #952 card 위치를 유일하게 찾아야 한다');
const updated = `${remaining.slice(0, anchorIndex)}${looseCard}\n${flagsCard}\n${remaining.slice(anchorIndex)}`;

const updatedCards = [...updated.matchAll(/<p class="kanban-note hero-progress">[\s\S]*?<\/p>/g)].map((match) => match[0]);
for (const [marker, expected] of [['PR #962', 1], ['PR #959', 1], ['PR #952', 1]]) {
  assert.equal(updatedCards.filter((card) => card.includes(marker)).length, expected, `공개 roadmap card 수 불일치: ${marker}`);
}
assert.ok(!updated.includes(flagsPlanCard), '병합된 text-indent flags가 계획 후보 상태로 남아 있지 않아야 한다');
assert.equal((updated.match(/72\/213/g) ?? []).length, (original.match(/72\/213/g) ?? []).length, '공식 완료 count 표기 횟수를 보존해야 한다');
assert.ok(updated.indexOf('PR #962') < updated.indexOf('PR #959'), '신규 merge 카드 순서가 최신순이어야 한다');
assert.ok(updated.indexOf('PR #959') < updated.indexOf('PR #952'), 'PR #959 카드가 PR #952보다 앞서야 한다');

const requiredSources = [
  'plan/c11-4-anonymous-grid-text-line-break-loose.md',
  'plan/c11-4-anonymous-grid-text-indent-flags.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/README.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/README.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/README.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/pr-diff-review-30.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/post-merge-status-review-30.md',
];
for (const source of requiredSources) assert.ok(fs.statSync(source).isFile(), `공개 카드 근거 source 누락: ${source}`);

fs.writeFileSync(roadmapPath, updated);
const report = {
  sourceHead: execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
  roadmap: roadmapPath,
  priorRoadmapSha256: sha256(original),
  reconciledRoadmapSha256: sha256(updated),
  supersededPass01Hash: JSON.parse(fs.readFileSync('spec/internal/evidence/c11-4-line-break-loose-public-roadmap-2026-11-01/public-roadmap-update.json', 'utf8')).updatedSha256,
  movedAndUpdatedCards: [
    { pr: 962, mergeAt: '2026-11-01T02:41:05Z', position: 0 },
    { pr: 959, mergeAt: '2026-11-01T02:06:32Z', position: 1 },
  ],
  orderedBeforeExistingCard: 952,
  supersededTextIndentPlanPlaceholder: true,
  preservedOfficialProgressMarkerCount: (updated.match(/72\/213/g) ?? []).length,
  requiredSources,
  fullDocsBuild: 'failed on repository-wide pre-existing dead links outside this selected C11 roadmap scope',
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
