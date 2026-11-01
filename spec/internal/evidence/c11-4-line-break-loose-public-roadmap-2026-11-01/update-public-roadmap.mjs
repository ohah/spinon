import assert from 'node:assert/strict';
import fs from 'node:fs';
import { createHash } from 'node:crypto';

const roadmapPath = 'spec/public/roadmap.html';
const reportPath = 'spec/internal/evidence/c11-4-line-break-loose-public-roadmap-2026-11-01/public-roadmap-update.json';
const sha256 = (text) => createHash('sha256').update(text).digest('hex');
const original = fs.readFileSync(roadmapPath, 'utf8');

assert.ok(!fs.existsSync(reportPath), `기존 update report를 덮어쓰지 않음: ${reportPath}`);
assert.ok(!original.includes('PR #962'), 'PR #962 카드가 이미 있으므로 중복 추가를 거부한다');
const priorCard = [...original.matchAll(/<p class="kanban-note hero-progress">[\s\S]*?<\/p>/g)]
  .map((match) => match[0])
  .filter((card) => card.includes('PR #945') && card.includes('<code>line-break</code>'));
assert.equal(priorCard.length, 1, '기존 computed line-break PR #945 카드를 하나만 찾아야 한다');

const card = '<p class="kanban-note hero-progress"><strong>2026-11-01 C11.4 anonymous Grid text computed <code>line-break: loose</code> · 제한 opt-in fixture 구현 · <a href="https://github.com/ohah/spinon/pull/962">구현 PR #962 Rebase and merge</a> (2026-11-01T02:41:05Z, merge SHA <code>5831014125e26079748f0b99cb99d3c21613358c</code>):</strong> 기존 normal/strict 5개와 Japanese loose min-/max-content, English loose, Japanese normal prefix control 4개를 합친 고정 9-scene 행렬을 구성했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 9/9 scene을 통과했다. PR check run은 보고되지 않아 CI 성공으로 세지 않는다. 화면 캡처는 route/status를 보이며 glyph paint를 증명하지 않는다. 일반 CSS admission, 임의 font/locale/text, WPT, 실기기/hardware GPU, iOS compositor 이후 pixels 및 전체 C11.4/C11 지원은 포함하지 않으며 공식 <code>72/213</code>과 미완료 상태를 유지한다. <a href="/spinon/plan/c11-4-anonymous-grid-text-line-break-loose.html">계획</a> · <a href="/spinon/internal/0059-c11-grid-layout.html">내부 계약 0059</a> · <a href="/spinon/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/README.html">Chrome 사전 비교</a> · <a href="/spinon/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/README.html">Android/iOS 실행 근거</a> · <a href="/spinon/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/pr-diff-review-30.html">PR diff 검토 30개</a> · <a href="/spinon/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/post-merge-status-roadmap-review-30.html">병합 후 상태·로드맵 검토 30개</a></p>';

const anchor = priorCard[0];
const index = original.indexOf(anchor);
assert.ok(index >= 0 && original.indexOf(anchor, index + 1) < 0, '기존 card 위치를 유일하게 확인해야 한다');
const updated = `${original.slice(0, index + anchor.length)}\n${card}${original.slice(index + anchor.length)}`;
assert.equal((updated.match(/PR #962/g) ?? []).length, 1, 'PR #962 카드는 하나여야 한다');
for (const marker of ['5831014125e26079748f0b99cb99d3c21613358c', '72/213', '9/9 scene', 'Rebase and merge']) {
  assert.ok(updated.includes(marker), `공개 roadmap marker 누락: ${marker}`);
}

const requiredSources = [
  'plan/c11-4-anonymous-grid-text-line-break-loose.md',
  'spec/internal/0059-c11-grid-layout.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/README.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/README.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/pr-diff-review-30.md',
  'spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-runtime-2026-11-01/post-merge-status-roadmap-review-30.md',
];
for (const source of requiredSources) assert.ok(fs.statSync(source).isFile(), `공개 카드 근거 source 누락: ${source}`);

fs.writeFileSync(roadmapPath, updated);
const report = {
  sourceHead: (await import('node:child_process')).execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
  sourceTreeBefore: (await import('node:child_process')).execFileSync('git', ['rev-parse', 'HEAD^{tree}'], { encoding: 'utf8' }).trim(),
  roadmap: roadmapPath,
  originalSha256: sha256(original),
  updatedSha256: sha256(updated),
  insertedAfter: 'PR #945 computed line-break card',
  addedImplementationPr: 962,
  mergeAt: '2026-11-01T02:41:05Z',
  mergeCommit: '5831014125e26079748f0b99cb99d3c21613358c',
  checks: 'no checks reported',
  officialCompletion: '72/213',
  c11AndC114RemainIncomplete: true,
  requiredSources,
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
