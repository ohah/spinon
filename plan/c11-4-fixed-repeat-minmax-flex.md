# C11.4 fixed-repeat `minmax(px, fr)` + auto-repeat 제한 구현 계획

**구현 후보 상태 · 2026-10-25:** 계획 PR #614가 main에 병합된 뒤 구현했다. Cascade→style-to-layout에 fixed-repeat `MinMax(px, fr)` provenance를 보존하고, 일반 authored neighbor의 동일 shape는 거부한다. Stylo·projection·layout·FFI 검증과 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 6/6 runtime matrix, final-source Rust archive boundary를 확인했다. Chrome 156 raw root/item rect를 독립적으로 대조하는 FFI assertion을 포함한다. 실행 근거·화면·hash·남은 한계는 [runtime evidence](../spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/README.md)에 있다. 구현/runtime 20-view review와 별도 PR diff 30-view review를 마쳤다. [PR diff review](../spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/pr-diff-review-30.md)는 제품 코드 차단 결함을 찾지 않았고, 공개 roadmap의 simulator 화면 링크를 final-source run으로 맞췄다. PR #615가 열려 있다. 이 limited candidate는 C11.4 checkbox와 공식 `72/213`을 완료시키지 않는다.

## 목표

현재 opt-in Grid profile이 유효한 CSS track shape인 `repeat(2, minmax(12px, 1fr)) repeat(auto-fill, minmax(24px, 1fr))`를 Stylo computed-value projection에서 거부한다. fixed integer repeat의 모든 track이 `LengthPx`여야 한다고 확인하는 두 gate를 좁혀, fixed minimum이 px이고 maximum이 flex fraction인 1–2 track pattern을 보존·layout하는 계획이다.

기준은 CSS Grid Level 1 [2025 CRD §7.2.3.1–7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation), [§7.2.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#fr-unit), §11.7–§11.7.1이다. 사전 입력, Chrome 관찰, 독립 산술과 한계는 [precomparison README](../spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/README.md)에 고정했다.

## 지원 범위

- 한 축의 한 explicit track list에서 하나의 `auto-fill` 또는 `auto-fit`과 정수 횟수 fixed-repeat group들을 authored order로 결합한다. 기존 group 수, expanded track, line metadata 예산 및 fallible reservation 경계를 유지한다.
- fixed-repeat pattern은 1개 또는 2개 track으로 한정한다. 각 entry는 다음 중 하나다.
  - finite nonnegative computed `LengthPx`;
  - `ComputedCssGridTrackSize::MinMax { min: LengthPx, max: Fraction }`이며 양쪽 수가 finite·nonnegative.
- 0fr 및 1 미만 factor를 포함해 CSS `<flex [0,∞]>` 의미를 따른다. `0fr`은 fixed minimum에서 확장하지 않고, flex factor 합이 1 미만이면 CSS Grid의 partial-fill 규칙에 따라 남는 공간을 모두 채우지 않아도 된다.
- Taffy 0.15.0의 f32 flex-factor 누적기는 overflow 검사를 하지 않는다. 새 mixed-repeat 경로는 Taffy 호출 전에 fixed/ordinary track factor와 `MAX_COMPUTED_GRID_TRACKS` 경계에서 가능한 최대 auto-repeat expansion의 factor 합 상한을 checked f32 산술로 계산한다. 중간 합·곱이 비유한이면 해당 node/property의 `UnsupportedComputedGridValue`로 fail-closed한다. 이는 finite 개별 factor가 Taffy accumulator를 무한대로 만드는 것을 막는다.
- fixed-repeat의 `minmax(px, fr)`는 auto-repeat count 계산에서는 fixed px minimum으로 기여하고, 최종 flexible sizing에는 보존된 fraction factor로 참여한다.
- auto-repeat pattern, auto-fill/auto-fit 점유·collapse, fixed-repeat line-name seam, ordinary neighbor, template areas, overflow 및 existing one-axis limitations는 현재 계약을 유지한다.
- authored 표기는 profile 판정 기준이 아니다. Stylo가 minimum `calc(20px)`를 computed `LengthPx(20)`로 정규화하면 typed allowlist에 따라 허용하고, computed `Calc`로 남으면 거부한다. 별도의 calc evaluator를 추가하지 않는다.

## 거부 범위

- 일반 CSS admission이나 제품 CSS profile 승격은 하지 않는다.
- `minmax(percentage, fr)`, computed `Calc` minimum, `minmax(px, max-content)`, `minmax(auto, fr)`와 지원하지 않는 track breadth는 제한 fixture profile에서 fail-closed한다.
- CSS에서 유효한 개별 factor라도 expanded-axis factor-sum upper bound가 f32에서 비유한이면 fail-closed한다. 음수·NaN·±infinity typed operand는 layout/Taffy 호출 전에 `UnsupportedComputedGridValue`와 node/property를 보존한다. CSS parser가 invalid syntax로 거부한 경우는 computed-profile error로 분류하지 않는다.
- invalid `minmax(fr, px)`는 computed-value profile error가 아니라 CSS parser의 invalid syntax 경로로 남는다.
- 복수 auto-repeat, 두 축 동시 repeat 조합 추가, 더 넓은 nested/implicit Grid, WPT 적합성 및 전체 C11.4 완료는 범위 밖이다.

## 고정 판정 입력

[`spec-derived-expectations.json`](../spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-precomparison-2026-10-25/spec-derived-expectations.json)의 6개 Chrome 관찰 장면과 독립 계산, synthetic controls, resource-boundary controls를 고정한다.

| 장면 | 입력 | 규범 기대 |
| --- | --- | --- |
| `columns-fill` | columns, 200px, fixed `repeat(2,minmax(12px,1fr))`, auto-fill min 24px | auto count 6, tracks `[14,14,24,24,24,24,24,24]` |
| `columns-fit` | columns, 200px, auto-fit min 24px 다음 fixed 2× `minmax(12px,1fr)` | auto count 6, 빈 auto tracks 5개 collapse, active tracks 64/64/64px |
| `rows-fill` | rows, 120px, fixed 2× `minmax(12px,1fr)` 다음 auto-fill min 24px | auto count 3, tracks `[16,16,24,24,24]` |
| `rows-fit` | rows, 120px, auto-fit min 24px 다음 fixed 2× `minmax(12px,1fr)` | auto count 3, 두 빈 auto tracks collapse, active tracks 각각 112/3px |
| `columns-groups` | columns, 300px, 두 fixed group pattern과 auto-fill | auto count 7, tracks `[12,8,12,8,24,24,24,24,24,24,24,22,22]` |
| `columns-flex-factor-low` | columns, 200px, fixed `minmax(12px,0.5fr)`와 auto-fill `minmax(24px,0fr)` | auto count 6, tracks `[16,24,24,24,24,24,24]`; 16px 남는 공간은 사용하지 않음 |

유효하지만 미지원인 percentage minimum, intrinsic maximum, computed calc control은 `UnsupportedComputedGridValue` 및 node/property를 확인한다. invalid flex minimum은 CSS parser rejection을 확인한다. valid `1e38fr` controls는 각각 fixed-repeat expanded sum과 auto-repeat 최대 가능 확장 sum이 f32에서 overflow하므로 Taffy 전 거부한다. 합산에 참여하는 `0fr`, `0.5fr`, ordinary flex·auto-repeat factor의 exact expanded-axis factor-bound도 Rust projection test로 고정한다. Chrome fractional used track과 fractional item offset은 1/64 CSS px, f32 runtime geometry/frame은 독립 기대값 대비 0.02 CSS px까지 허용한다. 정수 track·offset과 collapsed zero는 exact 비교한다.

Resource regression controls는 component list 10,000/10,001, projection의 `ComputedGridTracks` descriptor 10,000/10,001, expanded line metadata 1,048,576/1,048,577 bytes를 맞닿은 양쪽 경계에서 고정한다. projection descriptor는 fixed-repeat을 count만큼 확장하지만 auto-repeat pattern은 한 번 보존하는 단계의 총량이다. Taffy에서 실제 계산되는 auto-repeat 사용 횟수는 별도 `MAX_GRID_TRACKS=10,000` ceiling으로 유지하며 descriptor 수와 같다고 주장하지 않는다. `MinMax` pattern을 사용한 track/group/name reservation failpoint에서는 부분 결과가 없고 동일 worker의 다음 유효 호출이 성공해야 한다. line-name seam 입력은 fixed-repeat pattern shape가 `MinMax`여도 기존 authored order와 physical-line merge를 유지하는지 확인한다.

## 구현 및 검증 관문

1. 계획 PR 전 계획 자체를 별도 20 failure view로 검토하고, 빠진 control·범위 경계를 수정한다.
2. 계획 PR을 Rebase and merge한 뒤 최신 `main`에서 구현 브랜치를 만든다.
3. Stylo projection의 fixed-repeat preflight/expansion gate를 typed allowlist로 바꾸고 computed track metadata, fixed repeat boundary, unsupported diagnostic을 focused style tests로 고정한다. `LengthPx`로 fold된 absolute calc와 unresolved `Calc`의 computed projection 경계를 분리한다.
4. Taffy 진입 전 conservative expanded-axis f32 factor-sum upper bound를 검사한다. `0fr`, `0.5fr`, 1fr/2fr, fixed/auto-repeat 각각의 valid `1e38fr` aggregate overflow, negative/non-finite synthetic typed values를 단위·cascade 경계에서 고정한다.
5. `spinon-layout`/runtime fixture에서 여섯 positive scene과 parser/profile rejection controls를 검증한다. columns/rows, fill/fit, group prefix/suffix, multiple groups, fractional fill, collapse, line-name seam, 10,000/10,001 resource boundaries와 failure atomicity를 포함한다.
6. 앱 fixture는 전용 opt-in fixture-test feature에만 둔다. 기본 product FFI archive에서 신규 fixture marker/symbol/payload가 없고, positive-control fixture-test artifact에서 들어 있음을 증거로 남긴다. archive 결과를 APK/IPA 전체 제외로 일반화하지 않는다.
7. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 여섯 scene의 source/revision, used tracks, frames, trace, draw/present를 각각 확인한다. 앱 실행 화면을 저장하고 구현 PR 본문에 이미지로 표시한 뒤 Agent Browser로 이미지가 실제로 로드되는지 확인한다. 확인 후 해당 Agent Browser 세션을 닫고 관련 Agent Browser/preview 프로세스가 남지 않았는지 검사한다. 캡처와 픽셀 표본을 구분하고 물리 기기나 hardware GPU 증거로 표현하지 않는다.
8. Rust workspace tests, Clippy `-D warnings`, `cargo fmt --all -- --check`, fixture module boundary 및 default/opt-in artifact 검사를 실행한다. 구현 후 새 실패 관점 20개, PR diff 변경 관점 30개를 각각 수행한다.
9. 내부 계약 0059, `spec/STATUS.md`, 공개 roadmap와 PR 본문에서 지원 shape와 미지원 범위를 같은 표현으로 기록한다. C11.4 checkbox 및 공식 72/213은 그대로 둔다.

## 완료 판정과 남은 범위

이 계획 자체는 일반 CSS support claim이 아니다. 구현은 계획 PR #614가 main에 병합된 뒤 시작했고 제한 fixture 후보 검증을 마쳤다. 계획 PR의 병합이나 이 구현 PR의 병합도 C11.4 전체, WPT, 일반 CSS admission, 실기기 GPU 또는 공식 체크리스트 항목을 자동 완료시키지 않는다.
