# C11.4 definite-height percentage `fit-content()` 계획 적대 검토 20개

이 검토는 구현 전 계획과 사전 비교의 서로 다른 실패 경로를 살폈다. 구현/runtime 검토와 계획 PR diff 검토를 대체하지 않는다.

1. **다른 WPT 파일이나 revision을 pin할 수 있음 — PASS.** Exact source path, revision `c999c58338ee1d223df5ad62e48be1202ced0d37`, SHA-256을 함께 기록한다.
2. **percentage test가 아닌 인접 assertion을 대상으로 삼을 수 있음 — PASS.** Target은 원본 48행 `percentageArgument`와 107행 `150px` assertion이다.
3. **live WPT가 pinned source와 달라질 수 있음 — PASS.** 2026-10-29 Agent Browser fetch SHA-256과 pinned checkout hash가 일치한다.
4. **Chrome 결과를 Spinon 결과로 잘못 셀 수 있음 — PASS.** 13/13은 upstream Chrome baseline이며 Spinon runtime은 미실행이다.
5. **harness가 target assertion보다 먼저 멈출 수 있음 — PASS.** Agent Browser는 13 Pass, target row `150px`, root height `300px`를 기록한다.
6. **percentage basis 축을 잘못 정할 수 있음 — PASS.** CRD §7.2.1의 row percentage basis인 inner block size를 쓴다.
7. **cyclic percentage 경로와 혼동할 수 있음 — PASS.** Root height 300px은 definite라 intrinsic cyclic fallback을 사용하지 않는다.
8. **root border/padding으로 basis가 바뀔 수 있음 — PASS.** Root padding과 border는 0이며 content box는 300px이다.
9. **50% limit을 잘못 계산할 수 있음 — PASS.** Independent calculation은 `300 × 0.5 = 150px`이다.
10. **400px child가 limit을 넘지 않을 수 있음 — PASS.** Positive source는 fixed 400px child를 보존해 cap을 실제로 드러낸다.
11. **automatic minimum이 150px cap을 넘을 수 있음 — PASS.** WPT `.zeroMinimumSize`와 item `min-height:0`을 유지한다.
12. **implicit auto row가 explicit row로 변형될 수 있음 — PASS.** `grid-template-rows:none` 및 단일 `grid-auto-rows`를 계약에 둔다.
13. **viewport width가 row 결과를 바꿀 수 있음 — PASS.** Runtime root와 sole column을 60px로 고정하고 auto-width normalization을 공개한다.
14. **visual child 폭이 block-axis sizing을 바꿀 수 있음 — PASS.** Child width 30px는 paint sample을 위한 normalized value이고 height 400px은 고정이다. 추가 item/span은 허용하지 않는다.
15. **root·track·item frames를 혼동할 수 있음 — PASS.** Root 300px, row/item 150px, child 400px를 분리한다.
16. **frame과 track trace가 다른 revision에서 섞일 수 있음 — PASS.** 같은 document/style/environment revision을 요구한다.
17. **unknown/mutated fixture가 일반 route로 유입될 수 있음 — PASS.** Exact allowlist, fail-closed mutation controls와 artifact boundary를 요구한다.
18. **root 아래 overflow가 화면에서 잘릴 수 있음 — PASS.** Root 아래 50px pixel center에서 visible child를 양 플랫폼 surface에서 sample한다.
19. **callback/frame log만으로 paint를 단정할 수 있음 — PASS.** PixelCopy, Metal drawable sample 및 draw/present가 acceptance다.
20. **한 scene이 일반 support나 C11.4 완료로 과장될 수 있음 — PASS.** 나머지 12 assertions, 일반 CSS admission, physical hardware와 전체 C11.4는 제외하고 `72/213`을 유지한다.

**판정:** 계획 범위, pinned input, independent axis basis, expected row와 검증 경계에서 보완할 실패 경로를 찾지 못했다. 계획 PR diff는 별도 hostile review로 검토한다.
