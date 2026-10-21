# C11.4 nested Grid의 replaced item intrinsic contribution 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 고정된 세 장면의 opt-in fixture 구현과 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행을 마쳤다. Rust regression, artifact feature 경계, formatter, workspace Clippy, 앱 fixture build도 확인했다. 구현 검토와 PR 검토는 별도 evidence로 남긴다. 이 제한 slice는 C11.4 완료나 공식 완료 수 증가를 뜻하지 않는다.

## 목적과 경계

[기존 일반 nested Grid 계획](./c11-4-nested-grid-intrinsic.md)은 nested Grid 내부의 text/image 혼합과 replaced item을 명시적으로 제외했다. 기존 `prepare_c11_4_nested_grid_intrinsic` 경로는 nested Grid의 직접 text item만 수집하고, 일반 이미지 경로는 Grid root 바로 아래의 단일 `img`만 허용한다. 그러므로 이 변경은 제품 layout algorithm의 결함을 미리 가정하지 않고, 한 단계 일반 nested Grid 안의 Ahem text와 직접 자식 HTML `<img>`가 결합된 opt-in 입력을 재현·검증한다. 기대한 track, frame, contribution 값이 현재 layout algorithm과 맞으면 알고리즘 변경은 하지 않는다.

대상은 바깥 Grid 안의 독립적인 nested Grid 한 단계다. nested Grid의 두 direct child 중 첫 번째는 고정 Ahem text block, 두 번째는 natural-size HTML replaced `<img>`다. Subgrid, 여러 단계의 nesting, 이미지의 aspect-ratio 전이, arbitrary CSS parser 지원, 이미지 로딩의 비동기 갱신 계약, WPT, 일반 nested Grid 지원은 이 계획에 포함하지 않는다. 이 fixture는 `c11-4-grid-track-sizing-fixture-test` opt-in 경계에만 둔다. 공식 C11.4 상태와 완료 수는 바꾸지 않는다.

## 고정 입력과 oracle

독립 기준은 [CSS Grid Layout Module Level 2, 2025 CRD §3.4, §5.2, §6.6, §12.5, §12.5.1](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/)이다. §3.4의 일반 nested Grid는 parent와 독립적으로 layout 된다. §5.2는 Grid의 intrinsic size가 해당 제약 아래의 track과 gutter 합으로 정해진다고 설명하며, §12.5는 non-spanning item sizing 후 spanning item의 추가 공간을 분배한다. §6.6의 auto minimum 규칙은 definite flex control 장면에만 적용한다. Agent Browser / Chrome 156은 별도 구현 관찰자다. browser 관찰값을 규범 계산 대신 쓰지 않는다.

세 장면은 같은 DOM topology를 쓰고 outer item placement와 outer track/width 조건만 바꾼다.

| 입력 | 고정값 |
| --- | --- |
| outer Grid | `display:grid`; border/padding/margin 0; column gap 4px; row gap 0; height 20px; 두 columns |
| nested Grid | outer의 direct item; `display:grid`; width auto; height 20px; columns `min-content max-content`; column gap 4px; row gap 0; `align-items:start`; `justify-items:normal` |
| text item | nested의 첫 direct child; `grid-column:1`; Ahem 10px/10px; `XX XXXXXXX`; margin/border/padding 0 |
| image item | nested의 둘째 direct child; HTML namespace `img`; `grid-column:2`; auto width/height; item alignment 초기값; 고정 PNG source; natural size 24×16px |
| assets | `tests/fixtures/resources/c15/Ahem.ttf`, `tests/fixtures/resources/c14-1-png-basic.png`; 앱 fixture에서는 현재 C11.4 image snapshot 방식으로 고정 bytes를 decode |

Ahem text의 독립 기대 intrinsic widths는 min-content 70px, max-content 100px다. `<img>`의 독립 기대 intrinsic size는 24×16px이다. 이에 따라 nested Grid의 inline min-content width는 `70 + 4 gap + 24 = 98px`, max-content width는 `100 + 4 + 24 = 128px`다. 아래 세 acceptance 장면은 min-content outer contribution과 definite flex의 zero automatic minimum 경계를 검증한다. 128px max-content 수치는 계산 근거에 기록하되 별도 장면에서 측정하지 않았으므로 앱 acceptance 결과로 세지 않는다.

| scene | outer columns / width | nested placement | 독립 기대값 / Chrome 156 관찰 |
| --- | --- | --- | --- |
| `nested-grid-text-image-span-two` | `min-content min-content`, `width:max-content` | 두 outer track span | outer tracks `47/47px`, outer/nested width `98px`; inner tracks `70/24px`; nested min-content contribution `98px` |
| `nested-grid-text-image-one-track` | `min-content min-content`, `width:max-content` | outer 첫 track 하나 | outer tracks `98/0px`, outer width `102px`, nested width `98px`; inner tracks `70/24px`; contribution span 1, `98px` |
| `nested-grid-text-image-definite-flex-control` | `minmax(auto,1fr)` 두 track, `width:120px` | 두 outer flex track span | outer tracks `58/58px`, nested used width `120px`; inner tracks `70/24px`, image frame `24×16px`; §6.6 automatic minimum contribution `0px` |

사전 browser 입력과 관찰 파일은 [초기 oracle HTML](../spec/internal/evidence/c11-4-nested-grid-replaced-item-precomparison-2026-10-21/chrome-nested-replaced-oracle.html), [초기 DOM 관찰 JSON](../spec/internal/evidence/c11-4-nested-grid-replaced-item-precomparison-2026-10-21/chrome156-observation.json), [초기 화면](../spec/internal/evidence/c11-4-nested-grid-replaced-item-precomparison-2026-10-21/chrome156-nested-replaced-scenes.png)에 보존한다. runtime profile이 `justify-items:start`를 거부하는 것을 focused test에서 발견해 최종 고정 입력은 initial value인 `justify-items:normal`로 조정했다. 세 replaced/text geometry가 변하지 않는지 별도 Agent Browser 실행을 [runtime-profile alignment addendum](../spec/internal/evidence/c11-4-nested-grid-replaced-item-precomparison-2026-10-21/runtime-profile-alignment-addendum.md)에 기록한다. 이미지 source, `complete`, natural size와 Ahem font load 상태를 함께 기록한다. CSS Grid track widths와 geometry는 CSSOM 및 DOMRect로 측정한다.

## 구현 및 판정

1. runtime fixture adapter는 정확한 HTML namespace/tag/id, parent-child topology, source URL, computed style, text, image natural size, profile, 세 scene ID를 확인한다. 알 수 없는 같은-family scene과 다른 source/topology/style은 partial layout 없이 fail closed 한다.
2. 새 adapter는 text `ParagraphStyle`/Ahem engine과 image `LayoutIntrinsicSizeSnapshot`을 한 `C11_4MixedInput`으로 묶고 기존 combined layout entry point를 사용한다. layout algorithm이나 일반 cascade는 수정하지 않는다. trace에서 outer item은 nested Grid node로, inner image intrinsic source는 `img` node로 식별되어야 한다.
3. direct layout regression과 runtime fixture test는 각 scene의 외부·nested Grid·text·image frames, outer/inner columns/rows, outer contribution kind/value/span/source order, image intrinsic source/revision을 독립 기대값과 비교한다. definite flex control은 nested used width 120px와 zero automatic minimum을 분리해 확인한다.
4. FFI scene source는 고정 allowlist에만 추가하고 이미지 URL과 `<img>` node를 그대로 보존한다. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator의 opt-in fixture 앱에서 세 scene을 실행한다. scene/frame/track/trace/draw-presents 수, app 및 설치 artifact SHA-256을 기록한다. renderer/surface readback을 실제 수행하면 표본 좌표와 결과를 별도로 기록한다.
5. 기본 Android/iOS product artifact에서 fixture symbol, scene 문자열, PNG/Ahem payload가 빠지는지 검사하고 opt-in fixture-test artifact에 포함되는 positive control을 확인한다.
6. 관련 focused Rust tests, C11.4 fixture regression, `cargo fmt --all -- --check`, workspace Clippy `cargo clippy --locked --workspace --all-targets -- -D warnings`, Android/iOS fixture build를 수행한다. 미실행 플랫폼/검사는 통과로 쓰지 않는다.
7. 구현/runtime 변경을 기존 계획 검토와 분리해 새 실패 관점 20개로 검토한다. PR diff도 사용자의 현재 지시에 따라 별개의 실패 관점 20개로 검토한다. PR 본문은 변경 ID·명세·동작 화면·실행 근거와 범위를 기록한다. 현재 GitHub required checks 상태를 확인한 뒤 Rebase and merge하고, 병합 후 immutable commit 기준 화면 링크가 열리는지 Agent Browser로 확인한다. 확인 뒤 Agent Browser 프로세스와 창을 종료한다.

## 범위 한계

- 이 입력은 이미지의 자연 너비·높이를 직접 intrinsic dimension으로 사용한다. ratio 전이, author width/height 제약, `object-fit`, `srcset`, async completion, EXIF orientation을 검증하지 않는다.
- 한 단계 nested Grid + 고정 두 item topology만 검사한다. multi-level nesting, subgrid, anonymous items, extra/missing/duplicate image, arbitrary descendants/style, general HTML page support는 제외한다.
- Android 결과는 API 37 ARM64 AVD evidence이고 iOS 결과는 iOS 26.2 Simulator evidence다. 실기기·hardware GPU·iOS compositor 이후 표면, WPT, 전체 CSS Grid 적합성을 입증하지 않는다.
- fixture pass는 C11.4 완료/checkbox/count 증가가 아니다. C11.4 official count는 현재 72/213 상태를 유지한다.

## 계획 근거

- [계획 자체 실패 관점 20개](../spec/internal/evidence/c11-4-nested-grid-replaced-item-precomparison-2026-10-21/plan-review-20.md)
- [CSS Grid 2 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/)
- [Chrome 156 사전 비교 및 SHA-256](../spec/internal/evidence/c11-4-nested-grid-replaced-item-precomparison-2026-10-21/README.md)
- [Android/iOS Simulator 실행, 화면 및 artifact 경계](../spec/internal/evidence/c11-4-nested-grid-replaced-item-2026-10-21/README.md)
- [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-4-nested-grid-replaced-item-2026-10-21/implementation-review-20.md)
