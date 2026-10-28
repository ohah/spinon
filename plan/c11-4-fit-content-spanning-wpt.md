# C11.4 · spanning item over `fit-content()` implicit rows WPT slice

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**선행 slice:** [fit-content automatic-minimum floor](./c11-4-fit-content-auto-minimum-floor.md) · [fit-content 옆 auto implicit row](./c11-4-fit-content-auto-neighbor-runtime.md)

**사전 비교:** [Chrome 156·독립 기대값·pinned source](../spec/internal/evidence/c11-4-fit-content-spanning-precomparison-2026-10-29/README.md) · [기계 판독 기대값](../spec/internal/evidence/c11-4-fit-content-spanning-precomparison-2026-10-29/expected.json)

## 목표와 범위

Pinned WPT `css/css-grid/grid-definition/fit-content-track-sizing.html`의 span-2 assertion 한 개를 기존 opt-in C11.4 CSS→HostDocument/Stylo→typed Grid input→Taffy→native app 경로에서 확인한다. 대상은 두 implicit `fit-content(40px)` row를 span하는 하나의 item과 그 안의 고정 200px block child다. item의 `min-height:0`을 유지해 automatic minimum과 max-content spanning contribution을 구분한다.

고정 WPT는 Chrome 156에서 13/13 통과했다. WPT는 root inline width를 auto로 두므로 viewport 폭에 따라 used width가 달라진다. Runtime fixture는 row-axis 판정만 고정하기 위해 root width를 60px로 정한다. `grid-template-columns:60px`, row sizing function, row gap, row span, item min-height와 nested child height는 WPT 입력을 따른다. 이 inline-size 제한은 block-axis 기대값을 바꾸지 않는다.

이 slice는 새 일반 CSS admission, 공개 API, Grid sizing 알고리즘 일반화를 목표로 하지 않는다. `ParsedRootHtml`, CSS `zoom`, arbitrary Grid tree, scroll container, text/image contribution, WPT harness 전체 연결, C11.4 전체 완료는 범위 밖이다. 규범 기대값과 Spinon 동작이 다르면 fixture 기대값을 바꾸지 말고 source와 Taffy 단계부터 원인을 확인한다.

## 독립 기대값

기준은 [CSS Grid 2025 CRD §11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content), [span-2 item 처리](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items), [§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)다. 계산 입력과 Agent Browser 관찰은 [사전 비교 README](../spec/internal/evidence/c11-4-fit-content-spanning-precomparison-2026-10-29/README.md)에 분리했다.

| 값 | 독립 기대 |
| --- | ---: |
| Root content box | 60×300px |
| Auto rows | `fit-content(40px)` × 2 |
| Row gap | 0px |
| Span item minimum size | 0px (`min-height:0`) |
| Nested child | 60×200px |
| Final rows | `[40,40]px` |
| Final item border box | `[0,0,60,80]px` |
| Child block overflow past item | 120px, `overflow:visible` |

두 row는 implicit track이며 각 fit-content growth limit은 40px이다. span-2 item의 intrinsic contribution을 §11.5 spanning-items 단계에서 두 content-sized track에 반영해도 각 track은 그 limit을 넘을 수 없다. 두 track 합은 80px이고 `align-self:normal` item은 해당 영역에 stretch된다. Root의 definite height 300px은 fit-content track을 auto-max track으로 바꾸지 않으며 남는 220px은 이 두 row에 분배되지 않는다.

## 입력 및 runtime 경계

새 scene ID는 `fit-content-spanning-wpt`다. Root는 `display:grid; box-sizing:border-box; width:60px; height:300px; grid-template-columns:60px; grid-template-rows:none; grid-auto-rows:fit-content(40px); grid-auto-flow:row; align-content:normal`이며 margin, padding, border, gaps는 0이다. Root에는 직접 item 하나만 둔다. Item은 `grid-row:span 2; min-height:0; overflow:visible`이며 자식 하나의 `height:200px` block을 갖는다. 그 외 node, writing-mode 변경, explicit row, transformed style, root overflow style 변경은 허용하지 않는다.

Scene validator는 root/item/child ID와 정확한 parent/order, computed style, typed `GridTrackSize::FitContent(40px)`, implicit row count, row span, 같은 document/style/environment revision을 확인한다. Unknown scene, node 추가·누락·재배치, 지정 property mutation은 명시적인 오류로 닫는다. Trace는 Rows pass의 phase와 track snapshot을 출력하며 final rows `[40,40]`과 root/item/child frames를 같은 revision으로 연결한다. Trace가 기존 stage를 의미 있게 구분하지 않는 경우 출력 필드를 발명하지 말고 기존 trace 한계를 기록한다.

구현은 기존 `c04-runtime-gpu-fixture-test` 전용 경계를 사용한다. Default/product artifact에는 신규 fixture symbol·scene ID·HTML payload가 없어야 하고 fixture-test positive control에는 포함되어야 한다. 사용자 작성 HTML/CSS 경로와 product ABI는 바꾸지 않는다.

## 수용 기준

1. 고정 WPT source revision/hash, Chrome upstream 13/13, 대상 CSSOM/frame 관찰 및 독립 규범 계산을 expected JSON에 연결한다. Chrome 관찰을 Spinon 결과로 합산하지 않는다.
2. Rust layout/style-to-layout 테스트는 root, 두 implicit rows, span-2 item, child frame과 track-sizing trace를 exact scene에서 검사한다. mutation matrix는 root height/width, auto-row sizing, gap, item span, `min-height`, child size, node cardinality 및 writing mode를 바꾼 입력이 정확한 owner에서 닫히는지 확인한다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 fixture source revision을 실행한다. 각 플랫폼에서 scene 1/1의 root `[0,0,60,300]`, rows `[40,40]`, item `[0,0,60,80]`, child `[0,0,60,200]`, draw/present와 revision 일치를 확인한다. AVD는 software renderer, iOS Metal drawable은 compositor 제출 전 표본으로 구분한다.
4. 표면 검증은 root-local `(30.5,80.5)`에서 item 밖으로 보이는 nested child 색, `(30.5,200.5)`에서 root 배경을 Android PixelCopy와 iOS Simulator Metal drawable에서 확인한다. 이 점들은 item 밖 overflow와 200px child 끝을 확인하며 pixel equality나 iOS compositor 이후 표시를 입증하지 않는다. 캡처 표본의 DPR·origin→device-coordinate 변환·RGBA를 보존한다.
5. `cargo fmt --all -- --check`, 영향받는 focused Rust/runtime/FFI checks, Clippy 및 Android/iOS Simulator builds를 기록한다. 기본/product와 fixture-test archive의 marker/symbol/string positive/negative control을 각각 검사한다. 실행하지 않은 단계는 미검증으로 기록한다.
6. 구현 뒤 별도의 새 관점 20개로 코드/runtime를 검토하고 PR diff는 프로젝트 규칙에 따라 독립적인 30개 관점으로 검토한다. 결함 수정 revision에는 관련 검토 항목을 다시 적용한다.
7. Implementation PR merge 후 계획/내부 계약의 실행 범위·근거·남은 한계를 갱신한다. C11.4 checkbox 및 공식 `72/213`은 미완료로 둔다. Simulator 검증을 실기기나 전체 CSS support로 확대하지 않는다.

## 계획 실패 관점 검토 20개

1. WPT의 target assertion과 root/item/child topology가 정확히 매핑됐는가?
2. Live `wpt.live` 실행과 pinned source hash의 경계를 구분했는가?
3. Chrome 156 upstream pass를 Spinon pass로 과장하지 않았는가?
4. `width:auto` WPT와 runtime width 60px 차이가 block-axis 결과에 영향을 주지 않는 이유를 고정했는가?
5. Root의 300px definite height와 `align-content:normal`을 보존했는가?
6. 두 row가 explicit rows가 아니라 두 개의 implicit auto rows임을 입력에 담았는가?
7. `grid-auto-rows:fit-content(40px)`를 두 행 모두에 적용했는가?
8. `row-gap:0`을 명시해 span 계산에 gutter가 끼지 않게 했는가?
9. Item이 정확히 span 2이며 auto placement로 row count가 바뀌지 않는가?
10. `min-height:0`의 의미를 auto minimum contribution과 혼동하지 않도록 했는가?
11. 200px nested fixed-size child를 item의 intrinsic content로 연결했는가?
12. Fit-content의 auto minimum과 40px max-content cap을 별개로 추적하는가?
13. §11.5 span-2 batch가 single-span sizing과 구분되는가?
14. 각 row growth limit의 40px 상한을 지켜 final `[40,40]`을 독립 도출했는가?
15. Item의 80px stretch 영역과 200px child의 visible overflow를 따로 계산했는가?
16. 남은 220px root height를 fit-content rows에 부당하게 stretch하지 않는가?
17. 기대값을 Chrome/Taffy 관찰로 역산하지 않고 명세 기반으로 고정했는가?
18. Exact-scene allowlist가 unknown ID와 변형된 CSS/tree 입력을 거부하는가?
19. Android/iOS 결과와 AVD software renderer/iOS pre-compositor 표본의 증거 경계를 분리했는가?
20. 제한 fixture 한 개가 일반 CSS support, WPT suite pass, C11.4 완료 또는 공식 `72/213` 변경으로 해석되지 않는가?
