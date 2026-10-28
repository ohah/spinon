# C11.4 fit-content spanning WPT 계획 실패 관점 검토 20개

**검토 대상:** [`c11-4-fit-content-spanning-wpt.md`](./c11-4-fit-content-spanning-wpt.md)와 사전 비교 자료. 구현은 아직 시작하지 않았다.

1. **대상 WPT assertion을 다른 fit-content case와 혼동하는가? — PASS.** Source의 span-2 item은 56–58행, assertion은 114–116행으로 고정했다. auto-neighbor, min-height, automatic-minimum floor와 분리했다.
2. **Pinned source가 실제 checkout과 일치하는가? — PASS.** Revision `c999c58338ee1d223df5ad62e48be1202ced0d37`, SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`를 로컬에서 확인했다.
3. **Live Chrome 실행을 pinned revision 실행이라고 잘못 부르는가? — PASS.** README는 `wpt.live`의 현재 upstream 실행과 local pinned source를 분리하고 response hash 동일성을 주장하지 않는다.
4. **Chrome upstream 결과를 Spinon 성공으로 합산하는가? — PASS.** 13/13은 upstream harness 값이며 `spinonRuntimeExecuted:false`와 Spinon 미실행 경계를 함께 기록했다.
5. **Chrome 엔진/환경을 재현할 만큼 남겼는가? — PASS.** HeadlessChrome 156 user agent, 1280×633 viewport, DPR 1을 기록했고 screenshot을 첨부했다.
6. **WPT block-axis sizing이 viewport-dependent inline size에 묶이는가? — PASS.** Runtime만 width 60px로 고정하며 fixture에 text wrapping을 넣지 않아 row sizing content와 block-axis 입력은 고정된다.
7. **Root height가 바뀌거나 auto가 되는가? — PASS.** WPT definite `height:300px`와 runtime 300px을 유지한다.
8. **두 track이 명시적 template row로 바뀌어 auto-track 경로를 우회하는가? — PASS.** Target은 `grid-template-rows:none`, `grid-auto-rows:fit-content(40px)`이며 두 implicit rows를 요구한다.
9. **Track count가 두 개라는 사실이 placement에서 보장되는가? — PASS.** 하나의 item만 `grid-row:span 2`로 auto-place되고 다른 in-flow node는 없다.
10. **Row gap이 contribution 계산에 누락되는가? — PASS.** WPT 기본값과 runtime scene 모두 row-gap 0px이다.
11. **Item automatic minimum이 fit-content floor와 섞이는가? — PASS.** Target의 `min-height:0`을 보존해 item minimum 0과 max-content child contribution을 구분한다.
12. **Nested content가 실제 200px intrinsic contribution을 주는가? — PASS.** 유일한 child는 고정 block height 200px이고 text/font/async asset은 없다.
13. **`fit-content(40px)` 상한 계산이 명세와 어긋나는가? — PASS.** §11.2의 auto minimum/max-content maximum과 §11.5 span-2 분배를 구분해 cap 40px을 각 implicit row의 상한으로 적용한다.
14. **Span-2 contribution을 single-span 규칙으로 계산하는가? — PASS.** 독립 계산은 §11.5 spanning-items 단계에 명시적으로 연결하고 span group trace를 요구한다.
15. **최종 rows `[40,40]`이 Chrome 관찰을 그대로 복사한 값인가? — PASS.** 독립 계산 절차를 별도 서술했고 Chrome CSSOM은 교차 확인으로 분리했다.
16. **Item border box와 child overflow를 같은 크기로 오인하는가? — PASS.** 80px track sum에 stretch되는 item과 200px nested child의 120px visible overflow를 별도 기대값으로 둔다.
17. **남은 definite root free space가 fit-content rows를 늘리는가? — PASS.** fit-content max sizing function은 auto가 아니므로 220px을 두 row에 stretch하지 않는다고 명시했다.
18. **Variant CSS/node 입력이 fixture 성공으로 통과할 수 있는가? — PASS.** Exact scene allowlist, identity/revision 검사, unknown ID 및 지정 mutation 거부를 수용 기준에 포함했다.
19. **기본 제품 artifact에 검증 route가 누출되는가? — PASS.** `c04-runtime-gpu-fixture-test` 전용, default/product negative control, fixture-test positive control을 요구했다.
20. **단일 fixture 결과가 일반 CSS 지원이나 C11.4 완료로 과장되는가? — PASS.** 범위와 마무리 관문에서 WPT 전체, 일반 admission, 하드웨어 GPU, iOS compositor, C11.4 checkbox 및 `72/213`을 분리했다.

**판정:** 차단 사항 없음. 위 20개는 계획 자체 검토이며, 구현/runtime 검토와 계획 PR diff 검토에는 재사용하지 않는다.
