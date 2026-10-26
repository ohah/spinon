# C11.4 image auto/max mixed-track — pass05 수정 후 상태

**기준:** image plan branch를 `040a5360be92f73e141797aa7dcd4ef00ab24942`로 rebase한 뒤, parent C11.4 entry와 evidence README를 갱신한 payload다. 이 파일은 pass05 원본 20관점 report의 수정 후 상태를 기록하며 fresh independent review가 아니다.

## 수정한 blocker

| pass05 관점 | 원래 발견 | 수정 후 상태 |
|---:|---|---|
| #2 — stale base | 이전 checkout은 `a33b2459`, remote main은 `040a5360`이었다. | Branch를 `040a5360be92f73e141797aa7dcd4ef00ab24942`에 rebase했다. 재검토 전까지 이 기록은 author 확인이지 독립 review verdict가 아니다. |
| #3 — parent entry mismatch | parent plan은 pass03 `20/0`과 단일 축 overflow rejection control을 현재 범위처럼 적었고 pass04/pass05를 링크하지 않았다. | Parent plan은 overflow rejection/acceptance를 제거하고 Chrome CSSOM `clip/clip`을 CSSOM-only로 제한한다. pass04 `18/2`와 원래 payload를 대상으로 한 pass05 `14/4/2` report를 연결하고, pass05 follow-up과 current pending review 상태를 적었다. |
| #18 — README 언어 | Evidence README 마지막 runtime-scope paragraph가 영어였다. | 그 문단을 한글로 옮기고 overflow 제외 및 runtime acceptance 경계를 유지했다. |

## 유지하는 PARTIAL과 검토 한계

- **pass05 #1 — 원격 PR 없음 / candidate payload untracked:** PR을 만들지 않았고 문서/evidence는 현재 worktree에만 있다. 이를 PR diff나 원격 검토 상태로 표현하지 않는다.
- **pass05 #15 — CSS Images raw response bytes 없음:** URL, revision/date, byte count, SHA는 남아 있으나 hash preimage를 제공하지 않았다. PARTIAL을 유지한다.
- **pass05 #20 — browser cleanup raw transcript 없음:** metadata는 session inactive, 빈 active-session list, dedicated Chrome/crashpad 0을 기록하지만 CLI/process 원문 transcript는 없다. PARTIAL을 유지한다.

Pass05 원본 결과 `14 PASS / 4 PARTIAL / 2 FAIL`은 위 수정을 하기 전 candidate에 대한 historical verdict다. 위 세 partial은 그대로이고 두 blocker와 문서 언어 항목은 계획 문서상 수정됐지만, 이를 새 PASS 집계로 바꾸지 않는다. 수정된 child/parent/evidence payload에는 fresh independent targeted review가 필요하다. 원본 [plan PR diff-30 report](./c11-4-image-auto-max-mixed-track-plan-pr-diff-review-30-2026-10-26.md)의 FAIL finding도 targeted recheck 전까지 보존한다.

이번 수정에서 product/runtime code, tests, browser, simulator 또는 PR payload는 건드리거나 실행하지 않았고 commit/merge하지 않았다. C11.4 official status와 `72/213`은 변경하지 않았다.
