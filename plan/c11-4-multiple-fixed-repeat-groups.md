# C11.4 · auto-repeat과 fixed-repeat 두 그룹 조합

**상태:** 제한 구현과 runtime 검증 완료, 구현 PR [#594](https://github.com/ohah/spinon/pull/594)은 2026-10-24 10:14:36 UTC Rebase and merge로 병합됐고 merge SHA는 `1339874a04b2bc9866ce6eb6e0172e26dddf56ef`다. CSS Grid 2025 CRD와 Chrome 156의 10개 사전 비교 장면을 기준으로 Stylo cascade 변환, opt-in runtime fixture, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 검증했다. C11.4 checkbox와 공식 완료 수 `72/213`은 유지한다.

## 기존 동작과 결손

병합된 [PR #591](https://github.com/ohah/spinon/pull/591)은 한 track list에서 computed finite nonnegative `LengthPx` fixed-repeat 하나와 auto-repeat 하나를 결합하는 제한 profile을 추가했다. 현재 cascade adapter는 integer repeat을 두 번째 발견하면 `UnsupportedComputedGridValue`로 거부한다. CSS Grid 2025 CRD §7.2.3.1은 auto-repeat을 한 번만 허용하지만 같은 list의 fixed-repeat은 복수로 허용한다.

이번 slice는 fixed-repeat 두 그룹까지만 제한적으로 허용한다. 기존 한 그룹 입력의 동작은 보존하고, 두 그룹 입력에서는 authored order·auto-repeat 경계·definite axis의 반복 수·auto-fit collapse를 확인한다. 세 번째 fixed-repeat을 포함한 그 밖의 grammar를 일반 지원으로 해석하지 않는다.

## 목표와 제외

- 한 grid의 한 축에서 integer fixed-repeat 그룹 두 개와 `auto-fill` 또는 `auto-fit` 하나를 허용한다. columns와 rows를 각각 검증하지만 한 scene에서 두 축을 동시에 확장하지 않는다.
- 각 fixed-repeat count는 양의 정수이고 각 pattern은 1~2개 computed finite nonnegative `LengthPx` track이다. 기존 계산이 허용하는 상수 절대 `calc()`와 절대 단위의 computed 정규화 결과는 같은 typed 경계로 취급한다.
- 두 fixed-repeat은 auto-repeat 앞, 뒤 또는 양쪽에 나뉘어 놓일 수 있다. group 간 authored order와 각 pattern 내부 순서를 보존한다.
- auto-repeat sizing whitelist, definite-axis gate, placement, collapse, node/property 오류 연결과 neighbor validation은 그대로 둔다.
- expanded authored descriptor 합계는 `MAX_COMPUTED_GRID_TRACKS` 10,000 이내여야 한다. checked multiply/add, fallible reserve와 allocation fault-injection에서 no-partial-output을 유지한다. realized auto-repeat track 수는 기존 layout limit를 따른다.
- fixed-repeat/outer/auto-repeat line-name group은 비어 있어야 한다. named-line merge/collision, 세 번째 fixed-repeat, 두 번째 auto-repeat, nested repeat, `grid-template-areas`, percentage/unresolved `Calc`, `minmax()`, intrinsic/flex fixed-repeat breadth, arbitrary CSS admission, WPT와 전체 C11.4는 제외한다.
- 내부 계약 0059는 `0.1.0`을 유지한다. 이 opt-in runtime fixture는 공개 CSS 지원이 아니다.

## 규범·독립 기대값·Chrome 사전 비교

- 규범 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation)이다. 이 절은 auto-repeat은 한 번만 둘 수 있고 같은 track list에 fixed-repeat 그룹을 함께 둘 수 있다고 규정한다. 인접 line-name merge는 이번 범위에서 다루지 않는다.
- [Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-4-multiple-fixed-repeat-groups-precomparison-2026-10-24/README.md)는 [10-scene fixture](../tests/fixtures/css/c11/grid-multiple-fixed-repeat-auto-repeat-c114-precomparison.html), [observation JSON](../spec/internal/evidence/c11-4-multiple-fixed-repeat-groups-precomparison-2026-10-24/chrome156-observation.json), [full-page screenshot](../spec/internal/evidence/c11-4-multiple-fixed-repeat-groups-precomparison-2026-10-24/chrome156-full-page.png)를 보존한다.
- Browser은 macOS ARM64, HeadlessChrome `156.0.0.0`, viewport `1280×633`, DPR 1이다. Chrome은 교차 관찰이며 규범 기준을 대체하지 않는다.

| 장면 | authored order·axis | 독립 기대값과 Chrome 관찰 |
| --- | --- | --- |
| `groups-prefix-fill` | columns; 두 fixed 그룹 prefix | track `20,20,10,10,10`, auto-fill 5개 `30.7969`, suffix `16px` |
| `groups-split-fill` | columns; 첫 fixed prefix, 둘째 suffix | 두 fixed group과 auto-fill 순서 보존; auto-fill 5개 `30.7969px` |
| `groups-suffix-fill` | columns; 두 fixed 그룹 suffix | leading `16px`, auto-fill 5개, fixed group 순서 보존 |
| `groups-reversed-fill` | columns; fixed group 순서 반전 | 3×`10px` group이 auto-repeat 앞, 2×`20px` group이 뒤에 보존됨 |
| `groups-multitrack-fill` | columns; 두 track pattern을 포함한 첫 group | `repeat(2, 20px 12px)`는 네 fixed tracks, `repeat(3, 10px)`는 세 fixed tracks; auto-fill은 4×`36.5px` |
| `groups-prefix-fit-empty` | columns; empty auto-fit | auto-fill과 같은 5회 산정 후 다섯 auto-fit track이 `0px`로 collapse |
| `groups-prefix-fit-occupied` | columns; 첫 auto-fit track 점유 | used tracks `186px, 0px×4`; item 위치 `(110, 0)` |
| `row-groups-prefix-fill` | rows; 두 fixed 그룹 prefix | columns와 같은 높이 basis의 5회 및 `30.7969px` tracks |
| `row-groups-split-fit-empty` | rows; fixed groups가 auto-fit 주위에 분리 | authored order와 5회 collapse 보존 |
| `row-groups-prefix-fit-occupied` | rows; 첫 auto-fit row 점유 | used rows `186px, 0px×4`; item 위치 `(0, 110)` |

320px definite content axis에서 2×20px과 3×10px fixed groups, 16px ordinary neighbor, 8px gap을 쓰는 기본 matrix는 `86 + 30n + 8(5+n) = 126 + 38n`이다. `n=5`는 316px로 들어가고 `n=6`은 354px로 넘쳐서 최대 count는 5다. 남은 4px가 다섯 1fr track에 분배되어 각각 30.8px가 되며 Chrome serialization은 30.7969px다. multitrack control의 fixed breadth는 94px, fixed track 수는 7개라 `94 + 30n + 8(6+n) = 142 + 38n`; 320px에서 4회와 각 36.5px를 기대한다.

## 구현 순서

1. Stylo computed `TrackList`에서 auto-repeat 한 개와 integer fixed-repeat 두 개의 count·pattern·line-name AST를 저장된 관찰과 대조한다. 변환이 이 계획의 typed boundary와 다르면 구현을 시작하지 않고 계획/oracle을 고친다.
2. 기존 단일-group path의 동작을 보존하면서 두 integer groups만 선택적으로 수집한다. zero/negative/unrepresentable count, third group, named metadata, unsupported fixed size는 node/property를 포함한 기존 fail-closed 오류로 유지한다.
3. 두 group의 `count × pattern length`, ordinary track 수와 auto-repeat pattern 수를 checked arithmetic으로 더하고 10,000 descriptor 및 metadata 한도를 검사한다. tracks와 빈 outer line groups를 한 번 예약하고 allocation 실패는 오류로 반환한다.
4. list를 authored order로 한 차례 순회해 fixed group을 확장하고 auto-repeat `start/end`를 expanded track index로 산정한다. auto-repeat inner line metadata와 기존 auto-fill/fill, auto-fit/collapse 계약을 바꾸지 않는다.
5. `spinon-style` cascade tests에서 10개 positive oracle, one-group regression, 3-group/multiple-auto-repeat/named-line/unsupported breadth 거부, `N+1` metadata, 10,000 boundary, overflow, allocation fault-injection과 next-call recovery를 검사한다. `spinon-layout`은 expanded index, fixed/auto track 순서·gap·auto-fit collapse를 비교한다.
6. 전용 opt-in `c04-runtime-gpu-fixture-test`에 scene IDs를 추가하고 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 10 scene의 frame·track trace·revision·draw/present를 확인한다. 앱 캡처 표본은 AVD `PixelCopy`와 iOS compositor 제출 전 Metal drawable로 한정한다.
7. 기본 product artifact에는 scene IDs/symbols/payload가 빠지고 opt-in positive control에는 포함되는지 확인한다. APK/IPA 전체 exclusion, Android hardware GPU, iOS compositor 이후 표면을 이 검사로 주장하지 않는다.
8. 구현 후 새 20관점 implementation/runtime review를 하고, 별도 30관점 PR diff review 뒤에만 Rebase and merge한다. Main official checkbox와 72/213은 유지한다.

## 2026-10-24 구현 결과

- `computed_auto_repeat_tracks`가 authored order 안의 fixed-repeat 그룹을 최대 두 개까지 수집하고 각각의 computed `LengthPx` pattern을 검사·확장한다. 두 그룹의 합산 descriptor 한도, checked arithmetic, allocation failure/recovery와 rebased auto-repeat start/end를 cascade tests에서 확인했다.
- 10개 고정 scene은 columns/rows, prefix/split/suffix/reversed group, multitrack pattern, empty/occupied auto-fit을 포함한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 10/10 통과했다. PixelCopy와 compositor 제출 전 Metal drawable은 각 12/12 exact다.
- 기본과 `c04-runtime-gpu-fixture-test` positive-control macOS ARM64 debug static archive를 비교했다. 새 scene marker와 fixture ABI는 기본 archive에서 빠지고 positive control에 포함된다. 이 결과는 최종 APK/IPA exclusion을 뜻하지 않는다.
- [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md#c114-fixed-repeat-두-그룹--auto-repeat-제한-runtime-slice) · [구현/runtime 실행 근거](../spec/internal/evidence/c11-4-multiple-fixed-repeat-groups-runtime-2026-10-24/README.md) · [Chrome 사전 비교](../spec/internal/evidence/c11-4-multiple-fixed-repeat-groups-precomparison-2026-10-24/README.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-4-multiple-fixed-repeat-groups-runtime-2026-10-24/implementation-runtime-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-4-multiple-fixed-repeat-groups-runtime-2026-10-24/pr-diff-review-30.md).
- 세 번째 fixed-repeat, named-line metadata, 일반 CSS admission, WPT, Android 실기기/hardware GPU, iOS compositor 이후 pixels 및 C11.4 전체는 미완료다. 따라서 이번 slice는 공식 checkbox를 변경하지 않는다.

## 통과 조건과 남은 범위

Chrome observation과 독립 arithmetic의 반복 수·순서·used track·item offset이 일치해야 한다. Unsupported input은 부분 layout이나 잘못된 index를 만들지 않아야 한다. 기존 one-fixed-repeat controls, feature boundary, scoped tests/lints와 두 simulator scene matrix도 통과해야 한다. Plan 자체의 실패 관점 20개는 [별도 검토](./c11-4-multiple-fixed-repeat-groups-plan-review-20.md)에 둔다.

이 slice는 fixed-repeat group 두 개까지만 다룬다. 세 개 이상의 fixed-repeat, line-name merge/collision, fixed-size `minmax()`, percent/remaining computed `Calc`, simultaneous auto-repeat 양 축, template areas, 일반 CSS admission, Spinon WPT 적합성, 실기기/hardware GPU, iOS compositor 이후 pixels 및 C11.4 전체 완료는 남는다.
