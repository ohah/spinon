# C11.4 · direct text와 image의 행 축 intrinsic contribution

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**선행 slice:** [direct text/image 비-spanning 열 기여](./c11-4-mixed-direct-text-image-intrinsic.md) · [계획 실패 관점 20개](./c11-4-mixed-direct-text-image-intrinsic-plan-review-20.md) · [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-mixed-direct-text-image-intrinsic-precomparison-2026-10-25/README.md) · [구현/runtime 근거](../spec/internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/README.md)

**새 행 축 사전 비교:** [Chrome 156 geometry·화면·hash](../spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md)

**상태:** 제한 fixture 계획 초안. #611은 intrinsic column을 검증하고 두 row는 `24px`로 고정해 intrinsic row sizing을 명시적으로 제외했다. 이 후속은 반대 축에서 direct text item과 natural image가 같은 intrinsic row에 기여하는지 확인한다. 계획 검토는 [별도 20 관점 파일](./c11-4-mixed-direct-text-image-intrinsic-row-axis-plan-review-20.md)에 기록했다. 네 scene의 Chrome 156 geometry 사전 비교는 완료됐다. Spinon runtime 및 Android/iOS 검증은 구현 후 gate다. C11.4 checkbox와 공식 완료 `72/213`은 바꾸지 않는다.

## 목적과 경계

현재 #611 fixture는 `XX XXXXX` text item과 24×16 PNG를 같은 column에 놓지만 서로 다른 fixed rows에 둔다. 이는 min/max-content **column**의 비-spanning aggregation을 검증하며, item 둘이 같은 intrinsic **row**에 기여하는 경우는 다루지 않는다. 그 계획의 제외 범위에도 intrinsic row sizing이 적혀 있다. 이 slice는 그 한 경계를 보완한다.

한 scene에서 `horizontal-tb` Grid의 columns를 definite `50px 24px`로 고정하고, 한 행을 `min-content` 또는 `max-content`로 둔다. 직접 자식인 일반 `div` text item은 첫 column, natural-size `img`는 둘째 column에 명시 배치하고 둘 다 같은 행을 차지한다. DOM 순서만 앞뒤로 바꿔도 각 item의 배치와 행 결과가 유지되어야 한다.

텍스트는 CSS placement를 줄 수 있는 `<div>` Grid item이며 **익명 raw text Grid item이 아니다**. Anonymous Grid text, 자동 배치, spanning, 다른 writing mode, percentage/cyclic breadth, row gap, 장식, 임의 HTML admission은 범위 밖이다. 기존 fixture-test opt-in 경로만 사용하고 public API·일반 CSS 지원 범위·계약 버전을 넓히지 않는다.

## 기준과 독립 기대값

CSS Grid 2025 CRD [§7.2.1 Track Sizes](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.1 Grid Sizing](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content), [§11.5 Track Sizing Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)와 CSS Sizing Level 3 Working Draft [§2.2–2.3·§5.2](https://www.w3.org/TR/2026/WD-css-sizing-3-20260904/)의 intrinsic contribution 정의를 참고한다. CSS Grid는 columns를 먼저 계산한 뒤 그 결과 column size에서 rows를 계산한다. 비-spanning 항목 두 개가 한 intrinsic row에 있으면 해당 track은 같은 종류의 block-axis 기여 중 큰 값을 사용한다.

Chrome Agent Browser `HeadlessChrome/156`의 네 target scene은 [행 축 사전 비교](../spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md)에 고정되어 있다. 해당 HTML의 embedded Ahem/PNG byte hash는 저장소 `Ahem.ttf`와 `c14-1-png-basic.png`의 SHA-256과 각각 일치한다. 관측은 columns `[50px,24px]`, row `20px`, text `(0,0) 50×20px`, image `(50,0) 24×16px`, root `74×20px`를 네 scene 모두에서 기록했다. 이전 #611 Chrome 사전 비교는 동일 input asset과 source-order 사례의 보조 baseline이며, 이 새 row observation을 대체하지 않는다. CSS Grid 독립 계산도 같은 결과를 낸다. Browser evidence는 Chrome 관찰이지 Spinon runtime 증거가 아니다.

고정 입력은 다음과 같다.

- Grid content box `74px × auto`, columns `50px 24px`, row 하나, gap 0, margin/padding/border 0.
- root style: content width `74px`, auto height, `grid-template-columns:50px 24px`, column/row gap 0, padding/border 0, `writing-mode:horizontal-tb`, `direction:ltr`, `align-items:start`. `grid-template-rows`는 scene별로 한 track `min-content` 또는 `max-content`를 사용한다. Browser fixture는 네 root를 한 페이지에 쌓기 위해 `position:absolute`를 쓰고 `html`/`body` margin을 0으로 둔다. Runtime scene은 normal flow에서 실행하되 root-relative CSS px 좌표를 비교한다.
- text item: direct child `<div>` with `display:block`, auto width/height, column 1 / row 1, `justify-self:stretch;align-self:start`; Ahem `10px/10px`, `XX XXXXX`, `white-space:normal`, `word-break:normal`, `overflow-wrap:normal`, margin/padding/border 0.
- image item: direct child `<img>` with auto width/height, column 2 / row 1, `justify-self:start;align-self:start`; 저장소의 `c14-1-png-basic.png`, natural size `24×16px`, 성공한 decode 이후 측정.
- 두 item 모두 명시적으로 column 1/2, row 1에 둔다. 정방향/역방향은 DOM source order만 바꾸며 item별 placement는 유지한다.
- `min-content`와 `max-content` row scene 각각을 둔다. 둘 사이에 행 높이 차이가 있다고 주장하지 않는다.

Columns가 `50px 24px`로 확정된 뒤 Ahem text의 first word `XX`는 20px이고 second word `XXXXX`는 50px이다. 공백은 soft wrap opportunity이므로 50px column에서 두 개의 10px line box가 생긴다. Text item의 block-axis min-content와 max-content contribution은 모두 20px이다. Replaced PNG의 natural block size는 두 기여 모두 16px이다. 따라서 두 row track 모두 `max(20px, 16px) = 20px`이며 결과 geometry는 다음과 같다.

| Runtime scene | Chrome case | Root | Columns | Row | Text item | Image item |
| --- | --- | --- | --- | --- | --- | --- |
| `direct-row-min-content-forward` | `min-forward` | `(0,0) 74×20px` | `[50,24]px` | `[20]px` | `(0,0) 50×20px` | `(50,0) 24×16px` |
| `direct-row-min-content-reversed` | `min-reversed` | `(0,0) 74×20px` | `[50,24]px` | `[20]px` | `(0,0) 50×20px` | `(50,0) 24×16px` |
| `direct-row-max-content-forward` | `max-forward` | `(0,0) 74×20px` | `[50,24]px` | `[20]px` | `(0,0) 50×20px` | `(50,0) 24×16px` |
| `direct-row-max-content-reversed` | `max-reversed` | `(0,0) 74×20px` | `[50,24]px` | `[20]px` | `(0,0) 50×20px` | `(50,0) 24×16px` |

Chrome precomparison은 grid root에 absolute `track-probe`도 넣어 single-row track rect를 읽는다. 이 probe는 out-of-flow 측정 장치이며 sizing item이 아니다. Runtime fixture에는 in-flow text/image item 두 개만 둔다.

Root-relative geometry는 CSS px 기준이며 1/64 CSS px 이내로 일치해야 한다. 행 기여의 최댓값과 source-order 불변성이 목표 불변식이다. 이 fixture에서는 각 track 함수에 해당하는 두 item의 block 기여가 동일하므로 출력도 같아야 한다. 따라서 이 matrix는 min-content와 max-content track 알고리즘을 **구별하지 않는다**.

## 구현 전 확인 관문

1. 최신 `main`에서 #611 계획, source와 merge SHA를 다시 확인하고 미커밋/미추적 변경을 보존한다. 새 worktree를 만들거나 기존 dirty evidence를 옮기거나 정리하는 일은 이 계획에 포함하지 않는다.
2. [저장된 Chrome 156 row observation](../spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md)의 Ahem/PNG hashes, font readiness, natural size/decode, browser metadata, CSS source와 geometry를 검토한다. 새 source/hash가 다르면 Chrome precomparison을 다시 수집한다.
3. 네 저장 관측을 위 독립 규범 계산과 대조한다. 차이가 나오면 구현 전에 computed styles, containing block, root box, line fragments 및 sizing preconditions를 조사하고 계획을 개정한다. Chrome 출력을 규범값으로 역산하지 않는다.
4. 기존 opt-in direct mixed text/image scene이 row axis에서 어떤 trace/source/revision을 보존하는지 source를 읽고, 고정 allowlist 외 input은 계속 거부하는지 확인한다. 이 네 scene의 runtime 실패가 확인되지 않으면 production projection/track sizing 변경을 추가하지 않는다.

## 구현과 검증 게이트

1. 전용 scene ID를 기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture matrix에만 추가한다. #611의 네 fixed-row scenes와 일반 mixed-input rejection은 유지한다.
2. Runtime adapter는 root identity, `horizontal-tb`, root box, 고정 columns, intrinsic row 함수, 0인 edges/gaps, 정확한 direct child 수·순서·tag·text·source, image decode/natural size와 placement를 검증한다. 알 수 없거나 변형된 입력은 부분 결과 없이 명시적으로 거부한다.
3. Rust/runtime 검증에서 scene별 child source order, item identity, image intrinsic revision, text source, row/column contributions, row/column trace, track list와 root/item frames를 각각 비교한다. `MinContent`/`MaxContent` row 기대값은 규범상 모두 20px이며 두 함수 사이의 차이를 assertion하지 않는다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator gate는 각 host adapter에 새 scene ID를 연결한 뒤 실행한다.
   - `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java`는 `C114_GRID_TRACK_SIZING_SCENE_IDS` allowlist와 scene-specific root/item frame, column/row track, intrinsic contribution 및 `Columns`→`Rows` trace 기대값을 소유한다. 네 `direct-row-*` ID를 추가하고 root `74×20px`, text `(0,0) 50×20px`, image `(50,0) 24×16px`, columns `[50,24]px`, row `[20]px`를 각각 검증한다.
   - `platforms/ios/Sources/C0410RuntimeGpuDemo.swift`는 C11.4 scene ID sets/allowlist 및 node frames, root/track 기대값과 intrinsic trace 검증을 소유한다. 동일한 네 ID·frame·track·trace 조건을 Android 결과와 별도로 확인한다.
   - 두 adapter에서 item identity/source order, text와 image의 row-axis contribution kind/revision, column/row stage completeness, draw/present 조건도 scene별로 검사한다. 두 Simulator의 결과를 하나의 성공 수로 합쳐 누락을 숨기지 않는다.
5. 기존 #611의 `spec/internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/capture-direct-mixed.mjs`는 이전 네 column scene만 고정하므로 수정하거나 재사용하지 않는다. row slice 전용 `spec/internal/evidence/c11-4-direct-mixed-text-image-row-axis-runtime-2026-10-25/capture-direct-mixed-row-axis.mjs`를 만들고, 새 README 및 Android/iOS run output·screenshots·app/source/artifact hashes를 그 새 evidence directory에만 저장한다. 이 경로는 기존 #611 runtime evidence를 덮어쓰지 않는다. 각 scene에서 `[50,24]` columns, `[20]` row, `74×20` root, 두 item frame, complete Columns/Rows trace 및 draw/present를 확인한다. Screenshot은 box 배치 참고 자료로만 쓰며 glyph/PNG texel equality, Android 실기기, hardware GPU, iOS compositor 이후 pixels를 입증한다고 쓰지 않는다.
6. Focused Rust/runtime/FFI tests, formatter, 관련 Clippy, fixture-module boundary, 기본/product negative-control과 opt-in positive-control static artifacts, Android/iOS fixture app builds를 수행한다. 수행하지 않은 gate는 미검증으로 남긴다.
7. 구현/runtime 변경에는 기존 계획 검토와 별개인 새 실패 관점 20개를 작성하고, 실제 PR diff에는 프로젝트 규칙에 따른 독립 실패 관점 30개를 별도 작성한다. 해당 plan review를 구현 검토로 재사용하지 않는다.
8. 관련 문서와 내부 progress log에는 제한 fixture 상태만 기록한다. 전체 C11.4 checkbox와 공식 완료 `72/213`은 유지한다.

## 파일 경계와 병렬성

예상 구현 소유 영역은 다음과 같다.

- **Rust runtime adapter와 회귀:** `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs`, `c11_4_mixed/style.rs`, `c11_4_mixed/tests.rs`가 exact root/style/child/placement allowlist, per-scene expected values 및 contribution source/revision/frame assertions를 담당한다.
- **FFI scene input:** 새 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/mixed_direct_text_image_intrinsic_rows.rs`가 네 row scene의 IDs/source order/CSS를 보존한다. 필요한 경우 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs`에서 module/dispatcher만 연결하며 기존 #611 column scene output을 바꾸지 않는다.
- **Android host adapter:** `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java`가 scene allowlist와 scene별 root/item frame, column/row track, intrinsic input 및 track-sizing trace expectations를 소유한다.
- **iOS host adapter:** `platforms/ios/Sources/C0410RuntimeGpuDemo.swift`가 scene allowlist와 scene별 node frame, root/track, intrinsic input 및 trace expectations를 소유한다.
- **Capture와 evidence:** 별도 `spec/internal/evidence/c11-4-direct-mixed-text-image-row-axis-runtime-2026-10-25/` 아래 row-only capture harness, README, per-platform run output/screenshots/hash가 들어간다. 이전 #611 `capture-direct-mixed.mjs`와 runtime evidence는 변경하지 않는다.

Chrome input·observation·화면은 [row-axis precomparison evidence](../spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md)에 기록했다. 현재 경로는 공유 Taffy layout projection을 감싼 fixture allowlist이므로 이 정확한 scene에서 불일치가 관찰되지 않으면 공용 track sizing algorithm을 바꾸지 않는다.

병합된 PR #615의 fixed-repeat `minmax()`/flex candidate는 별도 fixture module과 repeat/projection 경로를 사용한다. 이 계획은 고정 column declarations만 써 repeat expansion과 분리된다. 다만 중앙 scene routing, shared evidence summaries 또는 병행 중인 C11.4 문서는 공통 수정 지점이 될 수 있으므로 구현 branch를 나누기 전에 해당 파일의 ownership을 조정한다.
