# C11.1 parsed Root HTML Grid definite-width `fr` implicit columns

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [sparse column-flow](./c11-root-html-grid-column-flow.md) · [fixed implicit columns](./c11-root-html-grid-column-flow-implicit-columns.md) · [percentage implicit columns](./c11-root-html-grid-column-flow-percent-implicit-columns.md) · [explicit `fr` tracks](./c11-root-html-grid-fr-track.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 기준:** [Chrome 156 관측과 독립 계산](../spec/internal/evidence/c11-root-html-grid-column-flow-fr-auto-columns-precomparison-2026-11-02/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-column-flow-fr-auto-columns-precomparison.html) · [plan 실패 관점 20개](./c11-root-html-grid-column-flow-fr-implicit-columns-plan-review-20.md) · [계획 PR diff 검토 30개](../spec/internal/evidence/c11-root-html-grid-column-flow-fr-auto-columns-precomparison-2026-11-02/plan-pr-diff-review-30.md).

**상태:** 계획만 작성한다. 별도 계획 PR이 병합되기 전에는 제품 구현을 시작하지 않는다. 이 계획과 사전 비교는 C11.1 또는 C11 완료, 공식 `72/213` 증가를 뜻하지 않는다.

## 구현 전 확인한 결손

현재 `RootHtmlColumnFlowV1`은 기본 parsed Root HTML의 `grid-auto-flow:column` sparse placement에서 implicit column pattern의 direct finite non-negative px와 percentage를 허용한다. [Layout validator](../crates/spinon-layout/src/grid_placement/root_html_column_flow.rs)와 [Root HTML column-flow projection](../crates/spinon-style-to-layout/src/projection/style_values/root_html_column_flow.rs)은 `GridTrackSize::Fraction`을 거부한다. Direct `fr`는 기본 Root HTML의 explicit track profile에서, direct px/percentage는 `grid-auto-columns` profile에서 각각 검증됐지만 둘은 결합되지 않았다. 이 계획은 그 조합만 연결한다.

기준은 [CSS Grid Layout Module Level 1, 2025-03-26 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)다. §7.6의 `grid-auto-columns` pattern은 필요한 implicit column에 반복된다. §7.7의 sparse column flow는 자동 배치 item을 source order대로 다음 column에 놓는다. Definite grid content width에서 §11.7/§11.7.1은 fixed track과 column gaps 이후 남는 공간에 flexible track을 배분하고, factor 합이 1보다 작으면 1로 계산하며, hypothetical size가 track base보다 작은 flexible track은 freeze한 뒤 다시 계산한다. §6.6의 automatic item minimum 때문에 `0fr`가 항상 0px track을 뜻하지 않는다. 고정 10px item의 `0fr 1fr` 기준은 `[10,70]px`다. Chrome 156은 독립 관측자이고 규범 기대값의 출처는 명세와 산술이다.

## 목표와 제한 범위

기본 `RuntimeSession::new_runtime_gpu_with_root_html()` parsed HTML author-style 경로에서 `RootHtmlColumnFlowV1`이 direct finite non-negative px, percentage, fraction으로 이뤄진 `grid-auto-columns` scalar/list를 허용한다. 기존 explicit columns/rows는 finite px로 유지한다. Fraction 또는 percentage가 포함된 implicit-column pattern은 같은 computed-style snapshot의 definite finite non-negative CSS-pixel `width`를 basis로 삼는다. Existing width override gate는 min/max width, aspect ratio 및 math-backed width를 거부한다.

- 기본 sparse `grid-auto-flow:column` 및 기존 fixed-px explicit columns/rows, fixed-px gaps, `grid-auto-rows:auto`, empty direct HTML `div` items, fixed non-negative px item size, `direction:ltr`, `writing-mode:horizontal-tb`, 단일 non-nested Grid와 revision/publication gate를 유지한다.
- `grid-auto-columns` scalar/list는 direct finite non-negative `px`, `%`, `fr`만 포함할 수 있다. 비어 있지 않은 목록 순서는 첫 implicit column부터 순환 적용한다. `0fr`, fraction sum below one, px/fr 및 px/%/fr mixed lists를 포함한다.
- Percentage basis는 definite grid content-box width이며 column gap을 빼지 않는다. Fraction share는 definite width의 available space에서 fixed/percentage tracks와 column gaps를 한 번 제외해 계산한다. Overflow와 free trailing space를 clamp하지 않는다.
- Fraction/percentage 목록은 fixed container width가 없거나 computed min/max/aspect ratio/math가 basis를 바꿀 수 있으면 concrete property 진단으로 거부한다. `width:0px`는 definite basis다.
- Semantic selector, element ID, class, DOM child index를 성공 조건으로 쓰지 않는다. Rename 및 source-order controls에서 track pattern과 source-order placement를 각각 확인한다.
- 새 CSS admission 전역 규칙, fixture-only route/FFI, public API/ABI 및 내부 contract version `0.1.0`은 변경하지 않는다.

## 고정 사전 비교와 판정 기준

고정 source, raw Chrome observation, screenshot hash와 독립 계산은 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-column-flow-fr-auto-columns-precomparison-2026-11-02/README.md)에 있다. Fixture는 `HeadlessChrome/156.0.0.0`, viewport `1280×633`, DPR 1에서 Agent Browser로 열었다. Root-relative DOMRect와 used track sizes는 독립 rational 계산에서 최대 `0.02 CSS px` 이내여야 한다.

| 장면 | 입력과 독립 기대 | 검증 목적 |
|---|---|---|
| `list` | width 240px, explicit `[30,40]`, four implicit tracks, `1fr 2fr`; flex space 145px, ideal sizes `145/6`, `145/3` 반복 | definite available space, list cycling, gap을 두 번 빼지 않음 |
| `scalar` | width 240px, explicit `[30,40]`, `1fr`; four implicit tracks 각 36.25px | scalar repetition |
| `mixed` | width 240px, explicit `[30,40]`, `10px 1fr`; four implicit tracks `[10,62.5,10,62.5]` | fixed breadth 후 remaining free-space 배분 |
| `composite` | width 300px, explicit `[30,40]`, six implicit tracks, `10px 10% 1fr`; percentage는 30px, flex는 각 57.5px | px/percentage/fraction pattern cycling 및 percentage basis |
| `subunit` | width 120px, explicit `[30]`, 두 implicit `0.25fr`; 각각 20px, trailing free space 40px | factor 합 0.5의 §11.7.1 floor |
| `zero` | width 120px, explicit `[30]`, 두 implicit `0fr 1fr`, item width 10px; tracks `[10,70]px` | auto minimum과 zero-factor freeze/retry |

이 기대값은 CSS Grid sizing 단계의 결과만 주장한다. Chrome `DOMRect` column starts는 1/64 CSS px 단위로 관찰돼 rational 산술과 구분하고, computed track serialization 문자열은 소수 넷째 자리까지 별도 보존한다. 화면은 Chrome oracle fixture일 뿐 Spinon runtime 또는 pixel parity 근거가 아니다. Four implicit `1e38fr` tracks는 factor 합이 f32에서 넘쳐도 유효한 finite factor다. vendored Taffy의 f64 accumulator를 유지하는 별도 runtime regression에서 `[30,30,30,30]px`를 검사하고, Chrome 156의 별도 direct-`fr` 관측 divergence는 규범적 기대를 바꾸는 데 사용하지 않는다([기존 fr 비교 기록](./c11-root-html-grid-fr-track.md)).

## 구현 및 검증 관문

1. 이 계획 PR을 병합하기 전에 별도 plan failure review 20개와 계획 PR diff failure review 30개를 완료한다. 계획 PR 병합 뒤 최신 `main`에서 구현을 시작한다.
2. Layout placement/contract validator와 style projection validator가 같은 direct px/percentage/fraction finite non-negative subset을 사용하도록 한다. Negative, NaN, infinite factors, empty/over-limit lists와 non-direct track values를 거부한다.
3. Fraction 또는 percentage가 들어간 pattern에는 exact Grid owner의 computed width와 min/max/aspect ratio/math boundary를 검사한다. Percentage-only px behavior와 width-zero basis를 회귀 테스트한다. `width:auto`, unresolved/mathematical width와 aspect-ratio/min/max override는 거부한다.
4. Positive Rust layout/projection/runtime matrix는 위 여섯 장면과 4×`1e38fr` regression, renamed selectors, source reorder, scalar/list cycle, cumulative track starts, fixed-size item frames, root bounds, same-source computed/style/layout keys를 확인한다. Expected CSS auto minimum을 무시하거나 item frame을 track width와 동일하다고 가정하지 않는다.
5. Negative matrix는 unsupported `auto`, intrinsic, `minmax()`, `fit-content()`, math, repeat, line names, named areas, authored placement, dense/row flow, implicit rows/columns outside quota, nested Grid, item text/image/intrinsic size, nonzero box edges, non-visible overflow, `order`, invalid width basis와 CSS-invalid fallback을 분리한다. Rejection 전 partial track/frame publication이 없고 이후 valid request가 복구되는지 확인한다.
6. 기존 fixed-px 및 percentage implicit-column tests, explicit fraction/percentage Root HTML tracks, sparse placement, 10,000-track ceiling, allocation-failure atomicity, stale-revision gates와 C11.3.3 regressions를 실행한다. 변경하지 않은 pre-existing failures는 새 후보의 통과로 세지 않는다.
7. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 parsed Root HTML source hash를 기본 app route로 확인한다. Layout frames, draw/present, screenshot을 구분한다. PixelCopy/Metal surface readback이 없으면 pixel equality를 주장하지 않는다. 실제 Android device/hardware GPU와 iOS compositor 이후 표본은 별도다.
8. Fixture FFI를 추가하지 않는다. 기본 Android/iOS artifact 경계가 유지되는지 확인하고 `cargo fmt --all -- --check`, 영향받은 focused Rust tests와 관련 crate Clippy를 실행한다. PR checks가 비어 있으면 CI 통과로 보고하지 않는다.
9. 구현/runtime은 계획 review와 다른 failure 관점 20개로 검토한다. 구현 PR diff는 별도로 30개 관점에서 적대적으로 검토한다. 결함을 수정한 검토 관점과 그에 영향받은 검증을 반복한다.
10. 구현 병합 후 `spec/internal/0059-c11-grid-layout.md`, `plan/c11-css-grid.md`, 이 계획, `spec/STATUS.md`의 C11 note 및 public roadmap preview를 제한 범위로 갱신한다. C11.1/C11 체크와 공식 완료 `72/213`은 별도 완료 계약이 생기기 전까지 유지한다. Tailnet preview의 served hash와 Agent Browser 화면을 확인하고 사용한 Agent Browser/Chrome 프로세스를 종료한다.

## 범위 밖

이 slice는 arbitrary CSS admission, auto/intrinsic/minmax/fit-content sizing, explicit non-px tracks의 column-flow 조합, named lines/areas, authored placement, dense flow, implicit rows, nested Grid, text/image contribution, WPT conformance, physical device/hardware GPU, iOS post-compositor pixel equality 또는 전체 C11.1/C11 완료를 주장하지 않는다.
