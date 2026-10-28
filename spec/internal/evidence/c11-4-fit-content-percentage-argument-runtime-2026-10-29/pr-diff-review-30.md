# C11.4 `fit-content(50%)` PR diff review · 30 checks

**검토 대상:** PR #771, `impl/c11-4-fit-content-percentage-argument`, 코드 최종 commit `98b8a7ff8964a21534877bb223f54b7180cbfca1` 및 해당 시점의 변경 파일.

## 검토 결과

30개 경계를 독립 확인했다. 첫 pass에서 track trace validator가 `Initialize` 단계에서도 400px growth limit을 기대하는 오류를 찾았다. 실제 trace는 `Initialize`의 row growth limit이 unresolved (`None`)이며 intrinsic sizing 이후 `400px`이다. 단계별 기대값과 변조 회귀 테스트를 수정하고 runtime test, Android AVD, iOS Simulator를 다시 실행했다. 수정 뒤 open finding은 없다.

1. **Pinned upstream input** — WPT path, revision `c999c58338ee1d223df5ad62e48be1202ced0d37`, SHA-256이 계획·fixture evidence와 같다.
2. **Independent expected value** — definite 300px row basis의 50% 값 150px을 CSS Grid 2025 CRD로 산출했다. 기대값을 Chrome observed style에서 역산하지 않았다.
3. **Scope isolation** — 새 runtime source는 allowlist된 `fit-content-percentage-row-wpt` scene 하나만 반환한다.
4. **Root ownership** — root는 문서 root의 직접 자식이어야 하며 다른 parent는 거부한다.
5. **Namespace and tag** — 세 노드는 HTML namespace `div`여야 한다.
6. **Node identity** — root, item, child ID와 순서를 모두 검사한다.
7. **Tree shape** — root는 item 하나, item은 child 하나만 가져야 한다. Parent 관계와 child 수를 검증한다.
8. **Computed-style set** — 정확히 세 고유 NodeId의 style만 받는다. 누락·중복·추가 style을 거부한다.
9. **Profile gate** — `RuntimeBlockFlexGridTrackSizingV1` 이외 profile은 거부한다.
10. **Root geometry/style** — display, width/height, template columns/rows, auto-flow, alignment, spacing, border와 typed projection을 고정한다.
11. **Definite percentage basis** — root height는 definite `300px`; projected `fit-content(50%)`는 `Percentage(0.5)`다.
12. **Implicit row shape** — explicit rows는 비어 있고, auto rows만 지정된 single track list여야 한다.
13. **Item minimum and overflow** — item은 `min-height:0`, computed overflow visible이어야 한다. Fixture source는 author `overflow:visible`을 추가하지 않는다.
14. **Overflowing child** — child는 fixed `30×400px`이며 sample에서 root 바깥까지 그려야 한다.
15. **Revision binding** — style, HostDocument generation/document/render-tree, layout style, environment revision이 동일 key와 맞아야 한다.
16. **Layout source** — snapshot은 `HostDocument` source여야 하며 tree-only revision은 거부한다.
17. **Grid input cardinality** — root 하나의 typed Grid input만 허용하고 해당 root identity를 검사한다.
18. **Frame cardinality** — 총 frame 수와 fixture frame 수가 모두 세 개여야 한다. 추가 frame을 거부한다.
19. **Frame geometry** — root `[0,0,60,300]`, item `[0,0,60,150]`, child `[0,0,30,400]`을 정확히 비교한다.
20. **Trace owner/pass count** — root trace 하나에 Columns와 Rows pass가 각각 하나 있어야 한다.
21. **Stage sequence** — Rows의 여섯 stage 및 sequence 1을 순서대로 검사한다.
22. **Track topology** — 모든 stage에서 internal index, track/gutter 구분과 collapsed flag를 고정한다.
23. **Base-size distribution** — stage base sequence `[0,0,0,150,150,150]px`를 검사한다.
24. **Stage-specific growth limit** — `Initialize` row는 `None`; 이후 row는 400px이다. 첫 review에서 초기 stage에도 400px을 요구하던 오류를 수정했다. [실패 AVD 로그](./android-avd-c114-fit-content-percentage-argument-android-avd-20261028T203642666Z-8ac55e2a/runtime-fit-content-percentage-row-wpt.log).
25. **Growability state** — gutter와 row의 `infinitely_growable`를 모두 false로 검사한다.
26. **Contribution vector** — Minimum 0, MinContent 400, MaxContent 400을 순서와 값까지 검사한다.
27. **Contribution attribution** — 동일 item ID, span 1, non-flex crossing, `SpecifiedMinimum` source를 검사한다.
28. **Negative controls** — changed percentage, explicit row, stage order/base/growth-limit/growability, contribution, wrong item ID를 거부한다.
29. **Android path** — API 37 ARM64 AVD matcher가 style/input/revision/frame/six-stage trace를 검사한다. PixelCopy 4/4와 draw identity가 최종 commit에서 통과했다.
30. **iOS and publication path** — iPhone 17 Pro / iOS 26.2 Simulator matcher와 Metal 표본 4/4가 통과했다. PR의 Android 1080×2400 및 iOS 1206×2622 이미지는 Agent Browser에서 `complete=true`와 natural size를 확인했다. 두 host의 Metal/PixelCopy 경계와 미지원 범위는 PR 본문에 표시했다.

## 최종 증거

- [최종 Android AVD 결과와 PixelCopy](./android-avd-c114-fit-content-percentage-argument-android-avd-20261028T203836259Z-305b93a9/result.json)
- [최종 iOS Simulator 결과와 Metal drawable](./ios-simulator-c114-fit-content-percentage-argument-ios-simulator-20261028T203925688Z-404d2893/result.json)
- [최종 runtime test](./checks/runtime-test-review-final.log) · [FFI route test](./checks/ffi-test-review-final.log) · [module boundary 18/18](./checks/fixture-module-boundary-review-final.log)
- [구현/runtime 검토 20개](./implementation-runtime-review-20.md) · [런타임·빌드·archive 범위](./README.md)
- PR check 조회 결과 `no checks reported`; main required status-check 설정도 없다. 이를 CI 통과로 표시하지 않는다.
