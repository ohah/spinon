# C11.4 · orthogonal text와 natural image의 intrinsic row contribution

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**관련 slice:** [direct text/image 행 축 계획](./c11-4-mixed-direct-text-image-intrinsic-row-axis.md) · [기존 행 축 Chrome 사전 비교](../spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md) · [새 orthogonal 행 축 사전 비교](../spec/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/README.md) · [계획 검토 20개 및 후속 조치](../spec/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/plan-review-20.md)

**상태:** Agent Browser의 Chrome 156 사전 관측과 CSS Grid 2025 CRD에 따른 독립 계산이 네 장면에서 일치했다. 예상 row `[50]px`와 `[80]px`가 실제로 갈린다. 이 문서는 제한 runtime fixture 확장을 위한 계획 후보이며 Spinon 구현·Android/iOS 실행 근거가 아니다. 기존 행 축 slice의 `horizontal-tb`/direct block text profile이 허용하지 않는 `vertical-rl` text item을 새 고정 profile로 제안한다. C11.4 checkbox와 공식 완료 `72/213`은 바꾸지 않는다.

## 목적과 겹침 경계

기존 direct text/image 행 축 slice는 root와 text 모두 `horizontal-tb`, definite columns `[50,24]px`, Ahem `XX XXXXX`, natural PNG `24×16px`을 사용한다. 일반 block text의 min-content/max-content block size는 같고 이미지 natural block contribution도 모두 `16px`이어서 두 행 함수 모두 `20px`이다. 기존 계획과 증거도 이 장면이 두 row 알고리즘을 구별하지 못한다고 명시한다.

이 계획은 같은 고정 columns와 direct text/image 구조를 보존하되, text Grid item 하나만 `writing-mode:vertical-rl`로 둔다. [CSS Writing Modes Level 3 §6.1 Abstract Dimensions](https://www.w3.org/TR/css-writing-modes-3/#abstract-dimensions)는 `horizontal-tb`의 block axis와 `vertical-rl`의 inline axis가 모두 물리 세로 축이라고 정의한다. §7.3 [Orthogonal Flows](https://www.w3.org/TR/css-writing-modes-3/#orthogonal-flows)는 두 writing mode가 서로 수직임을 정의하고, horizontal parent 안 vertical child의 물리 높이가 child inline size에 대응하는 사례를 든다. 따라서 이 grid의 물리 세로 row contribution은 vertical text의 inline contribution이며, Ahem min/max 값 차이가 row `[50]`/`[80]`을 판별한다. DOM 순서 forward/reversed를 함께 둬 min/max 결과와 source-order 불변성을 각각 관찰한다.

이는 기존 행 축 PR의 누락 fixture를 사후에 통과로 간주하려는 계획이 아니다. 그 PR의 수평 쓰기 전용 profile은 그대로 두고, 현재 adapter가 거부하는 orthogonal writing mode를 exact opt-in scene으로 추가할지 별도 검토한다. CSS Grid 전체의 orthogonal flow, C11.4 전체, public CSS admission, WPT 전체 또는 제품 완료로 승격하지 않는다.

## 규범 기준과 독립 기대값

기준은 CSS Grid Layout Module Level 1 2025 CRD의 [§7.2.1 Track Sizes](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§6.2 Grid Item Sizing](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing), [§11.1 Grid Sizing Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content), [§11.5 Track Sizing Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)이다. 논리 축과 물리 축의 매핑에는 W3C Recommendation인 [CSS Writing Modes Level 3 §6.1](https://www.w3.org/TR/css-writing-modes-3/#abstract-dimensions)과 [§7.3](https://www.w3.org/TR/css-writing-modes-3/#orthogonal-flows)를 사용한다. Sizing 용어와 inline contribution 정의는 CSS Box Sizing Module Level 3 [§2.1–2.3·§5.2](https://www.w3.org/TR/2026/WD-css-sizing-3-20260904/#intrinsic-contribution)를 참고한다. 이 Sizing 문서는 2026-09-04 Working Draft이므로 Grid track sizing 절차를 대체하는 규범으로 쓰지 않는다.

고정 입력은 다음과 같다.

- Parent grid: `writing-mode:horizontal-tb`, `direction:ltr`, `width:74px`, `height:auto`, `grid-template-columns:50px 24px`, 한 행 `min-content` 또는 `max-content`, 모든 gap·margin·padding·border 0.
- Grid root와 text/image는 CSS size를 `border-box`로 고정한다. 새 orthogonal scene의 root computed alignment는 `align-items:normal`, `align-content:start`, `justify-items:normal`, `justify-content:start`이며 item의 `align-self:auto`·`justify-self:auto`를 쓴다. `normal`은 orthogonal text item을 row area에 stretch하고 replaced image는 natural size로 유지하는 이 후보의 입력이다. 현재 common validator의 기존 `align-items:start` 정책은 #611/#620 scene에서 그대로 유지한다. 새 scene은 별도 exact matcher가 이 네 root 값을 검사하도록 하고 공용 기본값을 넓히지 않는다.
- Text는 column 1 / row 1의 direct `<div>`이며 `writing-mode:vertical-rl`, `direction:ltr`, computed `text-orientation:mixed`, Ahem 400 10px/10px, `XX XXXXX`, computed `white-space:normal`/`white-space-collapse:collapse`/`text-wrap-mode:wrap`, `word-break:normal`, `overflow-wrap:normal`, `line-break:auto`, 0 edges다. Computed `font-family:Ahem`, `font-size:10px`, `font-weight:400`, `font-style:normal`, `line-height:10px`도 고정한다. `XX XXXXX`는 Ahem advance 기준 8개의 10px cell(문자 7개와 공백 1개)이다. 가장 긴 공백 없는 단어 `XXXXX`가 min-content inline contribution `50px`, 전체가 한 inline line에 놓이는 max-content contribution `80px`를 만든다.
- Image는 column 2 / row 1의 direct `<img>`이며 저장소 `c14-1-png-basic.png` (`24×16px`, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`)를 natural size로 사용한다. 두 row 기여 모두 `16px`이다.
- Image computed style은 상속된 `writing-mode:horizontal-tb`, `direction:ltr`, `text-orientation:mixed`, `align-self:auto`, `justify-self:auto`다. Natural size `24×16px`를 유지한다.
- CSS Grid §11.1에 따라 fixed columns `[50,24]px`가 먼저 결정된다. 한 span-1 item만 각 row track 함수에 대해 text의 같은 종류 contribution과 image의 `16px` 중 큰 값을 선택한다. 따라서 min-content row는 `max(50,16)=50px`, max-content row는 `max(80,16)=80px`다. 한 행 외 추가 track/gap가 없으므로 auto-height root는 각각 `74×50px`, `74×80px`이다.

독립 기대 geometry는 root-relative CSS px 기준이다.

| Scene | Root | Columns / Rows | Text item | Natural image |
| --- | --- | --- | --- | --- |
| `direct-orthogonal-row-min-content-forward` | `(0,0) 74×50px` | `[50,24]` / `[50]px` | `(0,0) 50×50px` | `(50,0) 24×16px` |
| `direct-orthogonal-row-min-content-reversed` | `(0,0) 74×50px` | `[50,24]` / `[50]px` | `(0,0) 50×50px` | `(50,0) 24×16px` |
| `direct-orthogonal-row-max-content-forward` | `(0,0) 74×80px` | `[50,24]` / `[80]px` | `(0,0) 50×80px` | `(50,0) 24×16px` |
| `direct-orthogonal-row-max-content-reversed` | `(0,0) 74×80px` | `[50,24]` / `[80]px` | `(0,0) 50×80px` | `(50,0) 24×16px` |

## 사전 비교 결과

[고정 HTML, Agent Browser DOM geometry JSON, 화면, 자산 hash와 계산 과정](../spec/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/README.md)을 기준으로 한다. Chrome 156 Agent Browser 관측은 forward/reversed 모두에서 min row `50px`, max row `80px`; columns `[50,24]px`; root `74×50/80px`; stretched text `50×50/80px`; natural image `24×16px`을 기록했다. 네 관측은 위 계산과 일치한다. 이것은 browser observation이며 Spinon runtime proof가 아니다.

기존 `c11-4-mixed-direct-text-image-intrinsic-row-axis.md`는 text item과 root의 computed writing mode를 `horizontal-tb`로 고정하고, writing-mode 변경을 제외한다. 현재 runtime matcher `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/style.rs`도 root 및 text `writing-mode:horizontal-tb`를 요구한다. 따라서 이 계획의 첫 구현 관문은 author style parsing에서 `vertical-rl`을 받는 것만이 아니라 Stylo→Spinon computed style→Taffy layout까지 orthogonal sizing이 보존되는지 확인하는 것이다. 값이 거부되거나 geometry가 규범·Chrome과 다르면 임의 projection/fallback을 넣지 말고 fixture를 실패 상태로 둔다. production algorithm 변경은 별도 discrepancy evidence가 있을 때만 제안한다.

## 범위 및 fail-closed 경계

후보 fixture-test opt-in 경계는 아래 exact shape만 연다.

- Root 하나, direct child 정확히 두 개(`div` text, `img`), 각 span 1, 명시 placement, 같은 row 1, column 1/2, source order forward/reversed만 허용한다.
- Root `horizontal-tb/ltr`; text item `vertical-rl/ltr`; 이미지에는 writing-mode override가 없다. 변경된 item writing mode, mixed direction, 다른 text/font/font-size/line-height/whitespace/wrap, 다른 image URL·decode·natural size는 거부한다.
- Root width `74px`, height `auto`, columns `50px 24px`, row 함수는 단일 `min-content|max-content`, align/justify computed value는 정확한 본 fixture 값이어야 한다. 다른 track grammar, row/column gap, border/edge, min/max size, absolute/fixed item, span, auto-placement, anonymous text, nested formatting context, extra child는 거부한다.
- 알 수 없는 property, missing/duplicate node ID, 잘못된 CSS, incomplete cascade, image decode 전, unsupported orthogonal flow 또는 layout/contribution revision mismatch는 부분 결과를 publish하지 않고 명시 오류로 닫는다. 실패를 horizontal layout으로 대체하지 않는다.
- 테스트용 CSS 허용은 새 public CSS support, 일반 HTML parsing admission, C17 writing-mode 완료 또는 full C11.4 완료를 의미하지 않는다. C11.4 checkbox 및 공식 완료 `72/213`은 유지한다.

## 구현 전 확인과 acceptance

1. 최신 main에서 이 계획과 사전 비교 hash를 다시 확인하고 변경 중인 다른 경로를 보존한다. 기존 row PR/evidence 및 현재 unsupported property 경계를 재검토한다.
2. `vertical-rl` text와 horizontal grid의 computed style identity, source order, 50px fixed column, row/item intrinsic source/revision 및 image decode가 같은 layout revision에 묶이는지 진단한다. Runtime profile 또는 Taffy projection이 orthogonal input을 지원하지 않으면 implementation PR을 만들지 말고 정확한 blocking layer와 관찰을 보고한다. 이 feasibility gate에서 common validator의 기존 alignment 요구를 바꿔 입력을 억지로 통과시키지 않는다.
3. Exact scene matrix는 네 case만 포함한다: min/max row × forward/reversed child order. Rust/runtime assertions, FFI scene source, Android/iOS expected matrix, capture selector는 scene ID와 순서별 root/item identity를 일치시킨다.
4. 성공 acceptance는 네 case 각각에 대해 root computed `writing-mode:horizontal-tb`, `direction:ltr`, `text-orientation:mixed`; text computed `writing-mode:vertical-rl`, `direction:ltr`, `text-orientation:mixed`, Ahem family/size/weight/style/line-height, whitespace/wrap, `word-break`, `overflow-wrap`, `line-break`; image computed writing mode/direction/orientation; root alignment `normal/start/normal/start`; child self-alignment `auto/auto`; placements; text min/max row contributions `50/80`; image natural contribution `16`; Columns `[50,24]`; Rows `[50]` 또는 `[80]`; root/text/image frames; `Columns → Rows` trace completeness를 검증한다. Browser observation은 한 정적 HTML revision에서 fonts/images readiness를 확인한 뒤 computed styles와 rectangles를 같은 capture에서 읽고, README에 HTML 및 자산 hash를 둔다. Source order 역전은 결과를 바꾸면 안 된다.
5. Runtime provenance contract는 고정 scene ID와 source hash, 같은 scene document의 `source_revision`, root/text/image node IDs가 속한 `computed_style_revision`, Ahem/PNG asset hash 및 해당 scene의 image-decode 완료 상태, `layout_revision`, `trace_revision`을 명시한다. Root/text/image computed-style snapshot, text/image contributions, columns/rows tracks, frames와 trace는 같은 `source_revision`/`computed_style_revision`/`layout_revision` 계열의 동일 input epoch를 가리켜야 하며, `trace_revision`은 성공한 layout revision과 같아야 한다. 누락·혼합 revision, 미완료 decode 또는 자산 hash 불일치는 typed error로 닫고 부분 결과를 내지 않는다.
6. Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 새 네 scene을 각각 4/4 실행한다. Host report는 scene identity/order, row & column tracks, intrinsic contributions, item/root frames, draw/present를 장면별로 검사하며 플랫폼별 결과를 합쳐 누락을 숨기지 않는다. 이는 실기기, Android hardware GPU, iOS compositor 이후 pixel proof가 아니다.
7. 일반 negative input 및 기본/product artifact 경계와 fixture-test positive control을 확인한다. Focused Rust/runtime/FFI tests, formatter, 관련 Clippy 및 fixture-module boundary gate를 실행한다. 실행하지 않은 검증은 미검증으로 남긴다.
8. 구현 변경 전 별도 plan review 20관점과 실제 PR diff의 독립 실패 관점 20개 검토를 수행한다. 아래 질문은 계획 리뷰를 구성하는 독립 관점 후보이며 한 항목의 재진술로 횟수를 채우지 않는다.

## 코드 소유 후보

- **FFI scene source:** 새 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/orthogonal_direct_text_image_intrinsic_rows.rs`가 네 scene의 고정 IDs, child order, HTML/CSS source, decoder URL을 소유한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs`는 module/dispatcher 연결만 검토한다.
- **Runtime profile/trace:** 우선 새 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows.rs`를 후보로 둔다. `orthogonal_rows.rs`의 scene-specific matcher가 root alignment `normal/start/normal/start`와 computed writing mode/direction/typography를 소유한다. 기존 `c11_4_mixed/style.rs`의 `align-items:start` common rule과 기존 #611/#620 scene 동작은 그대로 유지한다. Router가 공통 validator를 호출해야 하면 orthogonal scene만 별도 branch로 분기하고 나머지 scene 기대값은 완화하지 않는다. `tests.rs`에는 contribution axis/kind/revision, item/source/frame matrix가 필요할 때만 한정 수정한다. Shared production layout code는 precomparison/runtime discrepancy가 입증되지 않으면 소유 변경 대상으로 삼지 않는다.
- **Host adapter:** `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java` 및 `platforms/ios/Sources/C0410RuntimeGpuDemo.swift`가 새 scene allowlist와 독립 root/item/track/contribution/trace/draw/present 기대값을 각각 소유한다.
- **Capture/evidence:** `spec/internal/evidence/c11-4-orthogonal-row-intrinsic-runtime-2026-10-25/` 아래 새 capture harness와 Android/iOS 로그·화면·hash를 둔다. 기존 row-axis evidence와 capture를 덮어쓰지 않는다.

**공유 파일 순차 통합 계약:** `c11_4.rs` FFI dispatcher와 두 `C0410RuntimeGpuDemo` adapter는 cyclic percentage auto-repeat sibling이 먼저 자기 scene IDs, allowlist, trace expectations를 등록하고 shared-file integration을 끝낸 뒤 orthogonal-row slice가 최신 통합본에 rebase해 네 scene을 추가한다. Orthogonal slice는 그 전까지 새 `orthogonal_rows.rs`와 독립 scene input/expectation 파일만 소유하며 shared dispatcher/host adapter를 편집하지 않는다. 통합 담당은 기존 #611/#620 8개 row/column scene의 `align-items:start` 결과를 전후 회귀로 확인하고, orthogonal 4개에만 `normal/start/normal/start` 정책을 적용한다. 두 slice는 shared-file 작업을 병렬 편집하지 않는다. 이 순서를 지킬 수 없으면 orthogonal runtime PR은 시작하지 않고 owner conflict를 blocker로 기록한다. 이 계획과 현재 사전 비교는 plan/evidence 경로만 소유한다.

## 계획 리뷰 질문 20개

1. 부모 Grid row axis와 orthogonal child inline axis의 물리 방향을 서로 뒤집어 해석할 여지가 있는가?
2. Root `horizontal-tb`와 text `vertical-rl`이 CSS 원본과 computed style 양쪽에서 고정되는가?
3. Root의 50px text column이 definite로 결정되기 전에 row contribution을 계산할 가능성이 있는가?
4. `XX XXXXX`에서 Ahem font, font readiness, 공백 collapse와 line breaking이 독립 계산값을 보장하는가?
5. Min-content inline contribution을 첫 단어 20px 또는 전체 block 높이 20px와 혼동할 수 있는가?
6. Max-content inline contribution 80px에 공백 advance를 빠뜨리거나 두 번 더할 수 있는가?
7. Natural PNG의 `24×16px` decode 상태가 확정되기 전에 16px row contribution을 기록할 수 있는가?
8. Row track 두 함수의 span-1 처리에서 같은 종류 contribution의 최댓값을 쓰는가?
9. Track min/max base/growth limit 사이 차이가 final used track에 흐려질 수 있는가?
10. Root `height:auto`가 최종 단일 row track size를 포함하는지 확인하는가?
11. Root `align-items:normal` 및 `justify-items:normal`이 text를 채우고 replaced image를 natural size로 두는가?
12. Child `align-self:auto`/`justify-self:auto`의 computed value와 사용값 alignment를 혼동하지 않는가?
13. DOM reversed scene에서도 명시된 grid placement 때문에 item의 열이나 행이 뒤바뀌지 않는가?
14. Fixed root coordinates, screenshot page labels 또는 root의 absolute positioning이 root-relative geometry에 섞일 수 있는가?
15. 0 border/gap/margin/padding이 CSS shorthand/default/reset 누락으로 변할 가능성을 막았는가?
16. Explicit placement, span 1, 정확한 direct child 수와 source order를 fail-closed로 검사하는가?
17. Stylo, Spinon computed style, Taffy와 host trace의 revision/source identity가 한 scene의 같은 입력을 가리키는가?
18. Unsupported orthogonal layout, missing style/property, malformed source에서 부분 geometry 또는 수평 fallback을 내보낼 수 있는가?
19. Browser observation이나 opt-in fixture 성공을 C11.4/public CSS/WPT/physical-device 완료로 과장할 수 있는가?
20. Android API 37 AVD와 iOS 26.2 Simulator가 각각 네 scene을 검사하며 capture/PID cleanup을 독립 증거로 남기는가?

## 제외 범위

일반 orthogonal flow, 다른 writing mode/direction, 자동 배치, multi-span, nested/subgrid, inline/flex/multicol formatting context, aspect ratio, percentage/cyclic breadth, row/column gap, item alignment 값 확장, arbitrary HTML/CSS admission, public CSS support, full WPT, 실기기/hardware GPU, iOS compositor 이후 pixels 및 전체 C11.4 완료는 이 계획의 대상이 아니다. 이 slice 성공으로 C11.4 checkbox나 공식 완료 수 `72/213`을 갱신하지 않는다.
