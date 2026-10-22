import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const repositoryRoot = process.cwd();
const previewRoot = process.env.SPINON_PREVIEW_ROOT;
const backupRoot = process.env.SPINON_PREVIEW_BACKUP_ROOT;
assert.ok(previewRoot && backupRoot, 'set preview and backup roots');

const relativePage = path.join(
  'internal',
  'evidence',
  'c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22',
  'README.html',
);
const siteRoot = path.join(
  repositoryRoot,
  'build/spinon/c11-4-cyclic-minmax-calc-cross-axis-span-groups-tailnet-site-pass49',
);
const generatedPage = path.join(siteRoot, relativePage);
const previewPage = path.join(previewRoot, relativePage);
const backupPage = path.join(backupRoot, 'review-correction-pass49', relativePage);
const evidencePath = path.join(
  repositoryRoot,
  'spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-tailnet-sync-2026-10-22/runtime-route-correction-pass49.json',
);

function sha256(bytes) {
  return crypto.createHash('sha256').update(bytes).digest('hex');
}

function replaceExactlyOnce(source, oldValue, newValue, label) {
  const first = source.indexOf(oldValue);
  assert.notEqual(first, -1, `${label}: old text is missing`);
  assert.equal(source.indexOf(oldValue, first + oldValue.length), -1, `${label}: duplicate old text`);
  return source.slice(0, first) + newValue + source.slice(first + oldValue.length);
}

function transform(source) {
  const oldSection = /<h2 class="rp-toc-include" id="남은-검증"><a href="#남은-검증" class="rp-header-anchor rp-link" aria-hidden="true">#<\/a>남은 검증<\/h2>\n<ul>\n[\s\S]*?\n<\/ul>/;
  const match = source.match(oldSection);
  assert.ok(match, 'old verification section is missing');
  const newSection = [
    '<h2 class="rp-toc-include" id="검증-상태"><a href="#검증-상태" class="rp-header-anchor rp-link" aria-hidden="true">#</a>검증 상태</h2>',
    '<ul>',
    '<li><a href="/spinon/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22/implementation-runtime-review-20.html" class="rp-link">구현·runtime 실패 관점 검토 20개</a>를 완료했다. Android definite presentation 누락을 부분 검증 판정에 반영했다.</li>',
    '<li><a href="/spinon/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22/pr-diff-review-30.html" class="rp-link">PR diff 실패 관점 검토 30개</a>를 완료했다. C11.4 완료 수 <code>77/213</code>과 fixture 전용 범위를 유지한다.</li>',
    '<li>Android definite forward/reverse 장면의 renderer draw·PixelCopy 표본과 4개 전체 AVD scene 실행.</li>',
    '<li>이 fixture build를 포함한 기존 C11.4 회귀 matrix.</li>',
    '<li>AVD 재시작이 필요하다. host free space <code>1.3 GiB</code>에서 emulator preflight가 거부됐으나 필요 공간의 정확한 기준은 출력되지 않았다. <code>1.3 GiB</code>를 기준값으로 추정하지 않는다.</li>',
    '</ul>',
  ].join('\n');
  let result = source.replace(oldSection, newSection);
  const oldToc = '<a href="#남은-검증" class="rp-toc-item rp-link" title="남은 검증" style="padding-left:0" data-depth="0"><span class="rp-toc-item__text rp-doc">남은 검증</span></a>';
  const newToc = '<a href="#검증-상태" class="rp-toc-item rp-link" title="검증 상태" style="padding-left:0" data-depth="0"><span class="rp-toc-item__text rp-doc">검증 상태</span></a>';
  result = replaceExactlyOnce(result, oldToc, newToc, 'outline item');
  assert.ok(result.includes('구현·runtime 실패 관점 검토 20개'));
  assert.ok(result.includes('PR diff 실패 관점 검토 30개'));
  assert.ok(!result.includes('Implementation/runtime failure review, 20 perspectives'));
  return result;
}

function assertNoOpenHandle(filePath) {
  try {
    const output = execFileSync('lsof', ['-nP', '-F', 'p', filePath], { encoding: 'utf8' });
    const pids = output.split('\n').filter((line) => line.startsWith('p')).map((line) => line.slice(1));
    assert.equal(pids.length, 0, `open handles for ${filePath}: ${pids.join(', ')}`);
  } catch (error) {
    if (error.status !== 1 || error.stdout) throw error;
  }
}

const generatedBefore = fs.readFileSync(generatedPage);
const previewBefore = fs.readFileSync(previewPage);
assert.equal(sha256(generatedBefore), sha256(previewBefore), 'generated and served page hashes differ before correction');
const generatedAfter = Buffer.from(transform(generatedBefore.toString('utf8')));
const previewAfter = Buffer.from(transform(previewBefore.toString('utf8')));
assert.equal(sha256(generatedAfter), sha256(previewAfter), 'correction output differs by destination');

assertNoOpenHandle(previewPage);
fs.mkdirSync(path.dirname(backupPage), { recursive: true });
fs.copyFileSync(previewPage, backupPage, fs.constants.COPYFILE_EXCL);
assert.equal(sha256(fs.readFileSync(backupPage)), sha256(previewBefore), 'backup hash mismatch');

const previewStat = fs.statSync(previewPage);
assertNoOpenHandle(previewPage);
for (const [target, bytes] of [[previewPage, previewAfter], [generatedPage, generatedAfter]]) {
  const temporary = `${target}.pass49-correction.tmp`;
  fs.writeFileSync(temporary, bytes, { mode: previewStat.mode & 0o777 });
  fs.chmodSync(temporary, previewStat.mode & 0o777);
  fs.renameSync(temporary, target);
}

const finalPreview = fs.readFileSync(previewPage);
const finalGenerated = fs.readFileSync(generatedPage);
assert.equal(sha256(finalPreview), sha256(finalGenerated));
const report = {
  route: `SpinonPreview/${relativePage.split(path.sep).join('/')}`,
  build_output: `build/spinon/c11-4-cyclic-minmax-calc-cross-axis-span-groups-tailnet-site-pass49/${relativePage.split(path.sep).join('/')}`,
  target_before_sha256: sha256(previewBefore),
  backup_sha256: sha256(fs.readFileSync(backupPage)),
  target_after_sha256: sha256(finalPreview),
  generated_after_sha256: sha256(finalGenerated),
  target_bytes_before: previewBefore.length,
  target_bytes_after: finalPreview.length,
  changed_files: 1,
  target_deletions: 0,
  open_handles_at_write: 0,
  full_site_tree_hash_note: 'pass49 full-site tree hash predates this one-route correction',
};
fs.writeFileSync(evidencePath, `${JSON.stringify(report, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
