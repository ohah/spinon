# C11.4 · definite positive percentage auto-repeat minimum 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**계획 수정:** [첫 계획 PR #559](https://github.com/ohah/spinon/pull/559)의 stop condition이 computed AST probe에서 발동했다. Stylo `0.22.0`은 `calc(0px + 10%)`의 zero length 항을 제거하고 `Percentage(0.1)`로 정규화한다. authored calc만 허용하는 건 불가능하므로 typed percentage subset으로 수정했다. [수정 전 20관점](./c11-4-auto-repeat-zero-percent-calc-plan-review-20.md)과 분리한 [수정 계획 20관점](./c11-4-auto-repeat-positive-percent-minimum-revision-review-20.md), [계획 revision PR 변경 검토 20개](../spec/internal/evidence/c11-4-auto-repeat-positive-percent-minimum-precomparison-2026-10-23/plan-revision-pr-diff-review-20.md)를 기록했다. Chrome 156의 16개 장면과 normalization log는 [사전 비교 자료](../spec/internal/evidence/c11-4-auto-repeat-positive-percent-minimum-precomparison-2026-10-23/README.md)에서 확인한다.

## 구현 결과 · PR #561 · 2026-10-23

[PR #561](https://github.com/ohah/spinon/pull/561)은 Rebase and merge로 `ff4233d0df00a632d71c28398c9a8274fb4abc16`에 병합됐다. Android API 37 ARM64 AVD와 iPhone 17/iOS 26.2 Simulator에서 각 16/16 scene의 frame·track·collapse·trace·draw/present가 통과했다. 범위는 `minmax(positive percentage, fixed px | flex)`의 definite positive content axis, zero min-size, no max-size 조건이다. zero·indefinite·cyclic basis와 clamp가 필요한 입력은 계속 fail-closed다. [실행·artifact 근거](../spec/internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/README.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/implementation-runtime-review-20.md) · [PR diff 검토 20개](../spec/internal/evidence/c11-4-auto-repeat-positive-percent-minimum-runtime-2026-10-23/pr-diff-review-20.md). PR merge 시 GitHub checks는 보고되지 않았다. C11.4 상위 체크는 미완료이며 현재 공식 집계 `72/213`을 유지한다.

## 발견한 결손

첫 계획은 `minmax(calc(0px + 10%), 1fr)`의 computed minimum이 `Calc(Sum(LengthPx(0), Percentage(0.1)))`로 남는다고 가정했다. focused Stylo cascade probe의 실제 값은 `MinMax { min: Percentage(0.1), max: Fraction(1.0) }`였다. assertion 원본은 사전 비교 디렉터리의 `stylo-cascade-normalization.log`다. 이 값은 CSS의 direct `minmax(10%,1fr)`와 동일하므로 source syntax만으로 분리할 수 없다. 이전 계획의 허용 조건을 Calc 수식 패턴으로 억지 고치지 않고, positive computed percentage minimum만을 대상으로 범위를 다시 고정한다.

CSS Grid 2025 CRD §7.2.1은 0 이상의 `<length-percentage>` track size와 열·행별 inner content basis를 정의한다. §7.2.3.1–§7.2.3.2에서 auto-repeat fixed-size minimum은 definite axis에서 content box와 gap을 사용해 반복 수를 정하고, auto-fit은 placement 후 빈 track을 collapse한다. 이번 제한 subset은 opt-in `RuntimeBlockFlexGridTrackSizingV1`의 `minmax(<positive percentage>, <fixed px | flex>)`만 허용한다. CSSOM computed style이 같은 `Percentage(p)`를 주는 direct percentage와 calc 식 모두 이 typed 범위에 포함한다.

## 고정 비교와 기대값

grid는 `box-sizing:content-box`, 지정 content axis 120px, 네 변 2px border, padding/gap 0이다. border-box 축은 124px이므로 percentage는 border가 아니라 안쪽 120px를 기준으로 한다.

- `calc(0px + 10%)`, direct `10%`, fixed `12px`는 columns/rows × auto-fill/auto-fit의 12개 scene에서 같은 result여야 한다. 10% minimum은 12px, 반복은 10개다. auto-fill tracks는 `[12]×10`; 단일 item은 12px이다. auto-fit은 `[120,0×9]`로 collapse하며 단일 item은 120px이다.
- `calc(0px + 150%)`와 fixed `180px` control은 각 축 auto-fill의 네 overflow scene에서 같다. 150% minimum은 180px이므로 반복은 1개다. item은 content origin에서 180px를 차지해 124px outer border box를 58px 넘는다. root/item frame assertion과 screenshot에서 실제 2px border 밖으로 나오는 범위를 확인한다.
- 실제 Chrome observation, 계산 JSON, 화면과 screenshot hash는 [16-scene precomparison](../spec/internal/evidence/c11-4-auto-repeat-positive-percent-minimum-precomparison-2026-10-23/README.md)에 고정했다.

## 구현 경계

1. cascade는 `ComputedCssGridTrackSize::MinMax`의 minimum이 finite `Percentage(p)`이고 `p > 0`이며 maximum이 fixed length 또는 flex일 때만 auto-repeat fixed minimum으로 인정한다. bare Percentage track, zero/negative percentage, intrinsic maximum, 다른 함수와 calc tree는 새 입장에서 허용하지 않는다. 기존 positive `LengthPx`와 지원 calc sum은 그대로 둔다.
2. cascade tests는 `calc(0px + 10%)`와 direct `10%`가 모두 `Percentage(0.1)`로 정규화되는 것을 고정한다. authored syntax를 typed layout에 보존한다고 주장하지 않는다.
3. layout은 grid axis의 content basis가 명시적 positive fixed size로부터 계산되고 `p×basis`가 finite positive일 때만 Taffy에 전달한다. ContentBox는 fixed content dimension을, BorderBox는 fixed dimension에서 해당 축 padding/border를 뺀 값을 쓴다. 해당 축의 min-size는 0, max-size는 none이어야 하며, zero/indefinite/cyclic basis, non-finite/zero resolve와 clamp가 필요한 입력은 `InvalidGridTrackList`로 fail-closed 한다. 이 검사를 두지 않으면 Taffy zero minimum 경로가 0 분모와 track 상한으로 진행할 수 있다.
4. Rust layout tests는 10%와 150%의 columns/rows track count, auto-fit collapse, root/item frame, actual outer-border overflow, fixed controls, zero/indefinite basis 거부를 검사한다.
5. 기존 opt-in FFI fixture에 16 scene을 연결한다. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 V8-authored HTML → HostDocument → Stylo → typed input → Taffy → WGPU scene/present까지 전부 실행하고 frame/track/collapse/trace를 비교한다. simulator screenshot은 제출 이미지 증거이며 별도 surface readback을 하지 않으면 pixel equality는 주장하지 않는다.
6. 관련 Rust/FFI tests, fixture boundary, `cargo fmt --all -- --check`, workspace Clippy와 Android/iOS simulator build를 기록한다. default/opt-in static archive를 대조하며 APK/IPA 전체 제외는 별도 bundle 검사 없이는 주장하지 않는다.
7. 수정 계획과 다른 실패 관점의 구현/runtime 20개 및 PR diff 20개를 각각 검토하고, 결함 수정 후 해당 관점을 다시 검사한다. GitHub PR 본문에 simulator 화면을 넣고 Agent Browser에서 이미지 load를 확인한 후 해당 browser process를 종료한다.
8. Rebase and merge 후 actual merge SHA와 check 상태를 확인해 `spec/STATUS.md`를 갱신한다. Tailnet preview 원본을 hash manifest와 함께 동기화하고 Agent Browser에서 route·counter·이미지를 확인한다.

전체 C11.4, 공개 CSS author API, WPT conformance, Android 실기기, hardware GPU와 iOS compositor 이후 pixel은 이 계획에서 판정하지 않는다. 내부 계약 0059 숫자 버전은 `0.1.0`으로 유지하고, C11.4 체크와 공식 완료 수 `72/213`을 바꾸지 않는다.
