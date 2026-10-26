# C11.4 generic direct-text 계획 PR diff review — targeted addendum

**기준:** branch `plan/c11-4-generic-text-intrinsic-input-20261026`, current `HEAD` `040a5360be92f73e141797aa7dcd4ef00ab24942` 및 아래 문서 수정본. 원본 [30관점 report](./c11-4-generic-direct-text-intrinsic-input-plan-pr-diff-review-30-2026-10-26.md)의 결과와 FAIL 행은 historical record로 보존한다. 이 addendum은 영향을 받은 행만 읽기 전용으로 재대조했으며 fresh 30관점 review는 아니다.

## 영향 행 재확인

| 원본 행 | 재판정 | 현재 근거와 경계 |
|---:|:---:|---|
| 2 — branch integration | PASS | 현재 branch는 `origin/main`과 같은 `040a5360be92f73e141797aa7dcd4ef00ab24942`로 rebase됐다. 이는 branch base alignment만 판정한다. |
| 3 — tracked PR payload | PARTIAL | 계획, review, evidence 파일은 이 worktree에서 untracked이고 commit/PR payload가 아니다. 사용자 지시에 따라 commit/merge하지 않았다. |
| 4 — parent index link | PASS | [상위 C11.4 계획](./c11-4-grid-track-sizing.md)은 child plan과 precomparison README를 역링크한다. |
| 10 — pass03 review baseline | PARTIAL | pass03 파일은 원래 검토한 `b58de5df...`를 historical baseline으로 보존하고 현재 `040a5360...` rebase/수정 상태를 덧붙였다. 이전 20 PASS verdict는 새 payload에 적용되지 않으며 fresh independent plan review가 필요하다. |
| 11/12 — review traceability | PASS | [child plan](./c11-4-generic-direct-text-intrinsic-input.md)의 review records가 pass01/02/03, 원본 diff-30 report, 이 addendum을 연결한다. Parent plan도 child/evidence를 찾을 수 있다. |
| 28 — unsupported stylesheet negative | PASS — plan contract corrected | 계획에서 `visibility:collapse`, `hyphens:none`, `font-variant-ligatures:none` rejection control의 위치를 item inline `style`로 바꿨다. 기대 경계는 `CssCascadeError::UnsupportedInlineAuthorProperty { node, property }`이며 error 전에 computed snapshot이 나오지 않는다. C11.4 inline scan은 [`runtime_grid.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L297)에서 [`first_unsupported_inline_property`](../crates/spinon-style/src/stylo_dom/cascade/runtime_layout.rs#L392)를 호출한다. C11.4 test는 [`orthogonal_writing_mode.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/orthogonal_writing_mode.rs#L75)에서 inline `text-orientation`의 같은 error variant를 확인하는 existing precedent다. 계획과 evidence는 제한 StylesheetSource의 unsupported declarations rejection을 주장하지 않는다. 이 recheck는 planned contract 및 기존 source boundary만 확인하며 이번 property controls의 실행 증거가 아니다. |

## 남은 gate

- Fresh independent plan review 20관점과 PR diff 30관점 review가 수정된 전체 payload에 필요하다.
- Runtime implementation, negative-control tests, browser, simulator, artifact, commit 및 GitHub PR은 이번 작업에서 실행하지 않았다.
- 공식 C11.4 상태와 `72/213`은 변경하지 않았다.
