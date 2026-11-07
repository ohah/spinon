# C11.4 parsed Root HTML `align-content:baseline` safe-start fallback

**상태:** Chrome 157 사전 비교, 계획 실패 관점 20개 검토, 고정 `RootHtmlAutoRowStretchV1` profile의 최상위 Grid 구현과 RuntimeSession·Android/iOS Simulator 검증을 마쳤다. 구현/runtime 검토 20개를 끝냈고 실제 PR diff 30개 검토와 PR 병합 전이다. baseline-sharing group을 이루는 중첩 Grid나 일반 CSS 지원은 이 slice에 포함하지 않는다. C11.4/C11 및 공식 `78/214`는 미완료·변경 없음.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [align-content content-distribution 선행](./c11-root-html-auto-row-content-alignment.md) · [safe/unsafe positional 선행](./c11-root-html-auto-row-safe-flex-end.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 목적과 규범 기준

기본 parsed Root HTML의 기존 `RootHtmlAutoRowStretchV1` fixed profile에서 computed `align-content:baseline`만 허용한다. 이 profile은 하나의 최상위 40×40px Grid와 고정 10px row, 두 implicit `auto` rows에 제한되며, Grid 자체가 baseline-sharing group의 구성원이 되는 부모 Flex/Grid 정렬 문맥을 만들지 않는다. 이 고정 입력에서 baseline content-alignment는 baseline-sharing group 부재에 따른 CSS Box Alignment의 fallback인 safe start를 사용한다. 기존 layout mapping의 `FirstBaseline → flex-start`는 이 profile에서 같은 start-edge 배치를 내므로 새 baseline algorithm은 추가하지 않는다.

CSSOM은 `first baseline`을 computed `baseline`으로 직렬화한다. 그 외 `last baseline`, overflow modifier와 unknown serialization은 이 slice에서 허용하지 않는다. `last baseline`은 별도 fallback·Taffy layout 표현이 필요하고 이 계획의 기준 브라우저에서는 지정값이 computed `normal`로 돌아가므로 범위를 넓히지 않는다.

규범 기준은 [CSS Box Alignment Level 3 §4.2](https://www.w3.org/TR/css-align-3/#baseline-values), [§5.4](https://www.w3.org/TR/css-align-3/#baseline-align-content), [§9](https://www.w3.org/TR/css-align-3/#baseline-rules) 및 [CSS Grid Layout Module Level 1](https://www.w3.org/TR/css-grid-1/)이다. Box Alignment는 baseline-aligned content를 사용할 shared alignment context가 없을 때 fallback alignment를 적용한다. Browser observation은 fixed top-level scene에서의 상호운용 확인이며, 명세의 독립 기대값을 대체하지 않는다.

## 기준 입력과 독립 기대값

고정 입력은 [`root-html-auto-row-align-content-baseline-fallback-precomparison.html`](../tests/fixtures/css/c11/root-html-auto-row-align-content-baseline-fallback-precomparison.html), SHA-256 `2c3a61b019d1b4ecc708724e932e86329b4cb9245860e23a8e4fc5741c43c675`다. viewport는 1280×633 CSS px, DPR 1이며 Grid는 `horizontal-tb`/LTR, definite 40×40px content box, 40px column, 10px explicit row, `grid-auto-rows:auto`, gap 0 또는 `calc(120% - 2px)`, fixed empty item과 10px/10px monospace text `A`/`B`다.

두 text item이 implicit row minimum contribution으로 각 10px를 만들기 때문에 initial/used rows는 `[10,10,10]px`다. Positive scene은 10px free space를 가진다. Baseline fallback `start`의 item y offsets는 `[0,10,20]px`이고 `end` control은 `[10,20,30]px`다. Overflow scene의 definite 40px block basis에서 row-gap은 46px, 두 gutter를 포함한 grid subject는 122px이다. Baseline의 safe-start fallback은 `[0,56,112]px`; unsafe `end` control은 `[-82,-26,30]px`다.

HeadlessChrome 157 Agent Browser 관측은 [machine-readable observation](../spec/internal/evidence/c11-root-html-auto-row-align-content-baseline-fallback-precomparison-2026-11-07/chrome-observation.json)에 저장했다. `baseline`의 computed serialization은 `baseline`이며 positive·overflow 좌표는 독립 fallback 계산과 exact 일치한다. `start`, `safe end`, `end`, `unsafe end` controls도 보존했다. [독립 기대값](../spec/internal/evidence/c11-root-html-auto-row-align-content-baseline-fallback-precomparison-2026-11-07/expected-values.json)과 [실행·한계](../spec/internal/evidence/c11-root-html-auto-row-align-content-baseline-fallback-precomparison-2026-11-07/README.md)를 따른다.

## Admission과 실패 경계

- computed `baseline`만 이 Root HTML fixed profile의 기존 `align-content` allowlist에 추가한다. 별도 parser, selector, API, FFI, fixture ABI 또는 Taffy baseline algorithm을 만들지 않는다.
- `FirstBaseline` layout enum은 이 profile에서만 허용한다. 기존 Taffy 변환의 `FLEX_START` 매핑을 유지하고 CSS 규범상 이 고정 최상위 Grid의 safe-start fallback과 geometry가 같은지 확인한다.
- `normal|stretch`만 기존 auto-track stretch stage를 사용한다. `baseline`은 tracks `[10,10,10]px`를 늘리지 않고 start 위치에 둔다.
- 다른 `RootHtmlIntrinsicTrackSizingV1`, general Grid, baseline-sharing ancestor, `last baseline`, `safe|unsafe baseline`, malformed/unknown computed values와 잘못된 fixed shape는 계속 거부한다.
- `place-content` shorthand에서도 computed `align-content`만 이 제한 allowlist에 들어가야 하며 `justify-content` 축은 넓히지 않는다.
- 실패는 `align-content` property에 귀속되고 layout/frame/draw snapshot을 부분 발행하지 않아야 한다. 같은 `RuntimeSession`에서 잘못된 요청 뒤 valid `start` scene이 성공해야 한다.
- baseline 정렬이 동작하는 nested/participating Grid, arbitrary children·tracks, writing mode 변경, 일반 author CSS admission, WPT, physical device, hardware GPU와 전체 C11.4/C11 완료는 제외한다.

## 구현과 수용 관문

1. Chrome 157 사전 비교와 CSS Box Alignment fallback의 독립 계산을 고정 fixture/hash·viewport·DPR와 연결한다.
2. Style-to-layout projection 및 Root HTML runtime candidate는 computed `baseline`을 유지해 `LayoutAlignContent::FirstBaseline`으로 전달한다. 기존 profile 밖의 baseline은 거부한다.
3. Layout profile gate는 이 후보의 `FirstBaseline`만 받는다. Positive·overflow rows/gutters/frame 전체를 exact CSS px 기대값에 비교하고 auto-track stretch stage가 실행되지 않는지 확인한다.
4. RuntimeSession에서 computed serialization 보존, atomic rejection, property attribution, same-session recovery와 기존 `start`/`end`/`safe|unsafe` regressions를 확인한다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route에서 같은 source hash의 baseline positive·overflow scene을 실행한다. Screenshots는 visual bounds이며 PixelCopy/Metal drawable readback으로 부르지 않는다.
6. 변경 crate focused tests, affected Clippy, `cargo fmt --all -- --check`, fixture-module/artifact boundary의 필요한 검사와 `git diff --check`를 실행한다. 빈 PR check rollup은 CI 성공으로 기록하지 않는다.
7. 구현/runtime은 계획 검토와 겹치지 않는 실패 관점 20개로, 실제 구현 PR diff는 별도 실패 관점 30개로 검토한다. 발견 결함은 수정 후 영향을 받은 관점을 재검증한다.
8. 병합 뒤 내부 contract `0059`는 숫자 버전 `0.1.0`을 유지한다. C11.4/C11 완료 check와 공식 완료 수 `78/214`도 유지한다.

## 계획 단계 리뷰

- 계획 실패 관점: [20개](./c11-root-html-auto-row-align-content-baseline-fallback-plan-review-20.md).

## 구현/runtime 검증 결과

- Layout·style projection·RuntimeSession focused filters는 각각 20/20, 11/11, 41/41 passed이고 changed-crate scoped Clippy 및 `cargo fmt --all -- --check`도 passed다.
- Positive frame offsets는 Chrome 157·독립 계산과 일치한다. overflow expected `[0,56,112]px`는 Android/iOS OS screenshot color bounds와 exact 일치한다. Positive Android screenshot은 DPR pixel quantization으로 `[0,9.9048,19.8095]px`, iOS는 `[0,10,20]px`를 측정했다(허용 오차 1 CSS px).
- Layout track trace에서 `StretchAutoTracks { applied:false }`, row sizes `[10,10,10]px`, positive/overflow gutters `[0,0]`/`[46,46]px`를 검사한다.
- dark media item-shape change의 atomic rejection과 같은 RuntimeSession recovery를 확인했다. 지원하지 않는 `last baseline`은 current pinned parser의 fatal `CascadeDiagnostic`으로 projection 전 거부된다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator default Root HTML route는 두 장면에서 layout-ready·6 boxes·2 text draws·present에 도달했다. 화면은 visual bounds이며 PixelCopy/Metal drawable readback이나 compositor 이후 pixel parity가 아니다. 앱·build/bootstrap 복원 hash는 pass03·pass04 모두 일치한다.
- [실행 근거와 screenshot](../spec/internal/evidence/c11-root-html-auto-row-align-content-baseline-fallback-runtime-2026-11-07/README.md) · [구현/runtime 실패 검토 20개](../spec/internal/evidence/c11-root-html-auto-row-align-content-baseline-fallback-runtime-2026-11-07/implementation-runtime-review-20.md).
