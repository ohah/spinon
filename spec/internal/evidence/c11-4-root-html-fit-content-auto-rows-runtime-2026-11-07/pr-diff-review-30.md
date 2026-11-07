# C11.4 `grid-auto-rows:fit-content(px)` implicit-row PR diff 실패 관점 검토 30개 · 2026-11-07

검토 대상은 base `5389aae4981bb6dccc7083b554c446f055a72708` 이후의 제한 구현, 테스트, 계약·상태·로드맵 문서와 실행 근거다. 계획 실패 관점 20개와 구현/runtime 실패 관점 20개를 반복하지 않고, 제안된 변경 diff에서 깨질 수 있는 계약을 별도로 추적했다. 최종 pass 4는 trace 플래그 수정이 포함된 working-tree source hash로 Android/iOS 앱을 다시 빌드했다.

## 검토 항목

1. **Profile 경계** — 새 candidate는 `RuntimeRootHtmlTextPaintV1`에서만 만들어지고 다른 style profile은 새 auto-row type을 projection하지 않는다. **통과.**
2. **Authored text 대신 computed type 사용** — projection은 `[FitContent(LengthPx(cap))]` typed shape만 받는다. CSS 문자열만 우연히 비슷한 입력은 route를 통과하지 않는다. **통과.**
3. **절대 길이 정규화** — `calc(12px + 12px)`와 `0.25in`이 `LengthPx(24)`가 되는 RuntimeSession scene을 literal `24px`과 같은 rows/frames로 확인했다. **통과.**
4. **Percentage basis 누출** — `fit-content(24%)`와 percentage를 포함한 `Calc` AST는 px cap candidate가 아니며, default RuntimeSession에서 fail-closed된다. **통과.**
5. **유한한 비음수 cap** — negative, NaN, positive infinity 및 unsupported typed percentage를 layout boundary에서 거부한다. **통과.**
6. **0px cap** — 0 cap에서 row는 0px이고 nested 34px payload는 visible overflow로 남으며 다음 probe 위치는 10px이다. **통과.**
7. **Singleton 문법** — 여러 implicit auto-row track, fit-content와 다른 track size의 혼합, 빈 목록은 허용하지 않는다. **통과.**
8. **기존 explicit `fit-content` 회귀** — 새 mode는 `RootHtmlFitContentV1`에서 분리되고 기존 explicit-fit-content focused regression 2/2가 통과했다. **통과.**
9. **기존 intrinsic·dense route와 우선순위** — intrinsic explicit tracks는 새 candidate에서 제외되고 dense-flow mode 선택도 기존 분기를 유지한다. **통과.**
10. **Explicit track 및 definite geometry** — 하나의 positive fixed-px column, 하나의 fixed-px explicit row, positive definite 40×100px border-box Grid만 받아 implicit-row sizing이 다른 축·explicit track에 번지지 않는다. **통과.**
11. **Track alias·area·repeat 경계** — named line, template area, fixed/auto repeat, intrinsic explicit track은 candidate/layout contract 중 하나에서 거부된다. **통과.**
12. **Placement 축과 implicit columns** — sparse row flow 및 `grid-auto-columns:auto`만 허용한다. Column/dense flow와 implicit column track 조합은 통과하지 않는다. **통과.**
13. **명시 배치 혼입** — 세 fixture item은 모두 auto placement여야 하고, explicit probe placement는 RuntimeSession negative control에서 scene publication 전에 거부된다. **통과.**
14. **Hidden child와 visible cardinality** — hidden subtree를 visible item count에서 제외하되 layout owner·세 visible direct item의 수를 다시 확인한다. **통과.**
15. **DOM kind 및 item 순서** — 세 visible direct HTML `div`의 explicit item/capped item/probe 순서를 고정하고 text, nested Grid, 비-div node가 역할을 바꾸지 못하게 한다. **통과.**
16. **Grid owner 수와 오류 owner** — projection은 정확히 하나의 parsed Grid owner를 요구하고 rejection의 `node_id`는 실제 Grid owner다. **통과.**
17. **Box model / gap** — margin·padding·border·row/column gap은 0만 허용하며 `row-gap:1px` control은 실패한다. Cap 계산에 edge spacing이 조용히 합쳐지지 않는다. **통과.**
18. **Alignment effect** — `align-content:start`, `justify-content:start`, normal item alignment와 visible overflow만 받으므로 free-space distribution이나 item stretch가 row oracle을 바꾸지 않는다. **통과.**
19. **Item sizing 입력** — first/probe fixed heights, capped item auto height와 `min-height:0|28px`, automatic max constraints 및 no aspect-ratio 경계를 projection/layout 둘 다 검사한다. **통과.**
20. **Intrinsic contribution source** — capped item 하나에 direct fixed-34px payload 하나만 허용한다. 임의 descendant, text intrinsic size, extra child가 max-content contribution을 바꾸지 않는다. **통과.**
21. **Track expansion과 반복 수** — sparse auto-placement는 singleton track pattern을 필요한 implicit-row count만큼 적용하고 기존 Grid track/item capacity 상한을 유지한다. **통과.**
22. **Taffy fit-content formula** — cap24/max-content34 → row24, cap40 → row34, minimum28/cap24 → row28을 layout trace와 frame으로 확인했다. **통과.**
23. **Following item 위치** — probe y는 각 row 결과 뒤로 이동하고 explicit row/column size는 고정된다. RuntimeSession은 paint frame에서 이를 대조한다. **통과.**
24. **Failure atomicity** — unsupported percentage cap은 `grid-auto-rows` error와 함께 실패하고 completed layout/render scene을 게시하지 않는다. **통과.**
25. **Same-session recovery 및 revision** — light→dark reject→light recovery를 같은 RuntimeSession에서 수행하고 completed layout과 computed cascade key 일치를 확인했다. **통과.**
26. **Positive/negative regression breadth** — zero/min/max/calc/unit positive cases와 mixed-list/percentage calc/wrong-flow/explicit-placement/gap/intrinsic-column negative cases를 분리해 검증한다. **통과.**
27. **Fixture-test flag 경계** — pass 4 Android/iOS build는 C11 fixture-test flags를 모두 `0`으로 설정하고 기본 Root HTML route에서 실행했다. **통과.**
28. **불필요한 production trace 비용** — 최초 diff는 새 candidate에도 `track_sizing_trace_enabled`를 켜 product layout에 test trace 비용을 추가했다. 이 review에서 발견해 projection을 기존 intrinsic-row 조건으로 되돌렸다. Trace가 꺼진 layout/runtime focused tests와 pass 4 app 실행이 통과한다. **수정 후 통과.**
29. **Simulator evidence 한계와 data 보존** — 최종 pass 4 artifacts는 current-source hashes에 bound됐다. Android는 기존 API 37 ARM64 AVD에서 `install -r`를 썼고 `pm clear`/uninstall은 하지 않았지만 app-data contents는 검사하지 않았다. iOS는 새 빈 iPhone 17 Pro Simulator를 사용해 이전 app data를 건드리지 않았다. Screenshots는 visual smoke이며 PixelCopy/Metal readback이 아니다. **통과.**
30. **Status·roadmap·완료 집계** — `STATUS.md`, contract 0059, public roadmap과 계획 모두 C11.4/C11을 미완료로 두고 공식 `78/214`를 보존한다. Active detail `C11.4 · ACTIVE`는 칸반에서 `fit-content(px)`로 표시된다. **통과.**

## 검토 결과

제안된 diff에서 차단 결함은 남지 않았다. 한 수정사항은 28번에 기록했다. 최종 focused tests, workspace all-target Clippy, rustfmt, `git diff --check`와 pass 4 Android/iOS default-route run이 current source hash에 대응한다. Artifact·device hash와 visual-evidence 한계는 [`README.md`](README.md) 및 [`final-pass04-source-revision.json`](final-pass04-source-revision.json)에 있다.

이 제한 slice는 일반 CSS admission, 전체 C11.4/C11, WPT 적합성, 실기기/hardware GPU 또는 rendered-surface pixel parity를 주장하지 않는다. GitHub PR checks는 PR 생성 후 별도로 확인하며, 이 문서는 CI 성공으로 세지 않는다.
