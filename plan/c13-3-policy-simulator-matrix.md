# C13.3 · computed overscroll policy Simulator matrix

**상태:** simulator 행렬·현재 소스 smoke 검증 완료 · 구현 [PR #185](https://github.com/ohah/spinon/pull/185) · 상위 C13.3 미완료
**선행:** [C13.3 스크롤 제스처 계획](c13-3-scroll-gestures.md), PR #184에서 병합한 computed scroll-policy snapshot 연결
**계획 실패 관점 검토:** [20개 관점](../spec/internal/evidence/c13-3-policy-simulator-matrix-plan-review-20-2026-10-15.md)

## 목표

현재 Android/iOS fixture app은 computed 기본 `overscroll-behavior:auto auto` 입력만 실제 simulator에서 확인했다. 이 후속은 제한 C13.3 style profile에서 계산된 `overscroll-behavior-x:contain; overscroll-behavior-y:auto`가 immutable scroll snapshot을 거쳐 네이티브 touch input에 적용되는지 같은 400×240 CSS px 중첩 장면에서 확인한다.

새 경로는 `c13-3-gesture-fixture-test` opt-in adapter 전용이다. 기존 default route와 zero-offset 장면은 그대로 유지한다. 새 `auto-boundary` 대조와 `contain-x-auto-y-boundary` 대상 경로는 런타임 style cascade로 선택하며, 알 수 없는 시나리오는 기본값으로 되돌리지 않고 명시적으로 거부한다.

| 시나리오 ID | 이름 | 계산된 inner policy | 시작 offset | 선택 인수 |
| --- | --- | --- | --- | --- |
| `0` | `auto-zero` | `auto/auto` | root/outer/inner 모두 `(0,0)` | Android extra 생략 또는 `spinon_c133_policy_scenario_id=0`; iOS `--spinon-c133-policy-scenario-id=0` 또는 생략 |
| `1` | `auto-boundary` | `auto/auto` | inner `(86,106)`, root/outer `(0,0)` | Android `spinon_c133_policy_scenario_id=1`; iOS `--spinon-c133-policy-scenario-id=1` |
| `2` | `contain-x-auto-y-boundary` | `contain/auto` | inner `(86,106)`, root/outer `(0,0)` | Android `spinon_c133_policy_scenario_id=2`; iOS `--spinon-c133-policy-scenario-id=2` |

모바일 실행은 반드시 C13.3 fixture route와 함께 scenario ID를 전달한다. 중복·범위 밖 ID, fixture route 없이 전달된 ID는 실패해야 하며 `0`으로 대체하지 않는다. 모든 선택과 ID 파싱은 fixture-test 빌드에 한정한다.

## Oracle와 fixture

- 기준 geometry·색·clip은 `tests/fixtures/css/c13/c13-3-scroll-gesture.html`과 기존 Android/iOS C13.3 fixed scene이다. viewport 400×240 CSS px, DPR 1 reference, inner/outer/root의 scroll range는 각각 x/y `(86,106)`, `(106,186)`, `(158,238)`이다.
- 독립 관찰은 Chrome 155 Agent Browser precomparison JSON의 `touchAxisContainXAutoYHorizontal`, `touchAxisContainXAutoYVertical`, `touchAxisContainXAutoYDiagonal` case다. X-only에서 ancestor X는 고정되고, Y-only에서는 ancestor Y가 45 CSS px 움직인다. Chrome diagonal 결과는 두 축 모두 고정이지만 계획 계약의 축 독립 규칙과 다르다. 이 시나리오의 제품 pass oracle은 C13.3 per-axis 계약이며 Chrome의 대각선 차이는 결과에 그대로 남긴다.
- 동일한 compiled opt-in app에서 route argument만 바꿔 `auto/auto` 대조와 `contain/auto` 대상을 실행한다. computed style과 resolved input policy를 host의 ready report에서 확인한다.
- 각 입력 trace는 새 app session에서 inner owner를 `(86,106)` 경계에 준비한다. 이 초기 상태 준비는 test host가 router를 통해 적용하고 `fixture_seed`로 별도 기록한다. 측정 구간은 seed 뒤 보낸 실제 ADB/iOS Simulator swipe뿐이다. seed를 손가락 gesture나 제품 API 실행으로 보고하지 않는다.
- Android는 API 37 ARM64 `emulator-5554`를 명시하고, iOS는 iPhone 17 Pro/iOS 26.2 Simulator UDID를 명시한다. 실기기는 사용하지 않는다.

## 사전 판정 기준

각 플랫폼에서 `auto-boundary`와 `contain-x-auto-y-boundary` 각각의 fresh app run을 horizontal, vertical, diagonal swipe로 반복한다. 모든 시작점은 같은 inner-content CSS point이고 입력 방향·CSS px 길이·trace 순서는 같다. host가 기록한 실제 surface scale과 변환된 CSS delta로 비교한다.

| 입력 | `auto/auto` 대조 | `contain/auto` 대상 |
| --- | --- | --- |
| X-only, inner X=max | 남은 X가 outer X로 전달 | inner/outer/root X offset이 시작값 유지 |
| Y-only, inner Y=max | 남은 Y가 outer Y로 전달 | Y는 outer로 전달, X offset은 유지 |
| 대각선, inner X/Y=max | 두 축 남은 delta가 ancestor에 전달 | X는 막고 Y는 outer로 전달, root는 해당 sample의 남은 delta가 범위에 도달할 때만 이동 |

추가 pass 조건:

1. 준비 trace는 inner `(86,106)`, outer/root `(0,0)`을 보이고 각 owner의 range가 기존 fixture와 일치한다. 모든 measured trace는 seed 이후의 revision과 offset delta를 별도로 기록한다.
2. Android와 iOS가 동일한 per-axis 결과를 낸다. native slop으로 움직임이 없으면 그 run은 실패나 pass로 억지 분류하지 않고 입력 길이를 늘려 다시 실행한다. gesture가 시작됐다는 callback과 실제 nonzero offset commit을 구분한다.
3. ready report는 계산된 inner X=`contain`, Y=`auto` 정책을 식별한다. fixture는 값을 직접 router에 지정하지 않는다. 첫 native draw의 `scroll_state_revision`과 `scroll_offsets`도 ready report의 seed와 정확히 같아야 하며, surface snapshot이 다르면 gesture 입력을 잠근다.
4. final screenshot은 실제 native swipe 뒤 화면이다. red marker 및 nested scroll content의 모든 변경 pixel은 해당 clip/border 안에 남아야 한다. screenshot 자체는 compositor 이후 scanout 근거가 아니다.
5. 기존 기본/opt-in feature 경계를 다시 확인한다. 시나리오 선택, fixture setup과 관련 payload가 기본 앱 artifact에 나타나면 안 된다.
6. default zero-offset route, C13.3 focused Rust tests, workspace tests, Clippy, rustfmt, diff check를 실행한다. 실제 실행하지 않은 gate는 결과에서 제외한다.

## 첫 draw와 입력 준비 순서

첫 앱 실행에서 FFI ready report만으로 화면 seed가 반영됐다고 판정하지 않는다. iOS 공통 presentation refresh는 최초 환경 설정 뒤 다시 호출되므로 fixed C13.3 route는 최초 설정과 같은 `scale=1`을 사용해야 한다. Android와 iOS adapter는 첫 성공 draw의 revision/owner offsets가 scenario별 초기 snapshot과 일치한 뒤 `SPINON_C133_READY`를 내고 native swipe를 허용한다. 이 검사는 입력 state와 실제 제출한 draw snapshot의 분리를 막는다.

## 2026-10-15 simulator 실행 결과

Android API 37 ARM64 emulator `emulator-5554`와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 7개 fresh-session case를 실행했다. 모든 case가 초기 draw의 revision/offset 검사를 통과했고 `SPINON_C133_READY` 이후 native swipe가 처리됐다. root/outer/inner 경계와 screenshot 차이 분석은 [pixel-analysis.json](../spec/internal/evidence/c13-3-policy-simulator-matrix-2026-10-15/pixel-analysis.json), 전체 캡처와 명령·한계는 [실행 근거](../spec/internal/evidence/c13-3-policy-simulator-matrix-2026-10-15/README.md)에 둔다.

| 플랫폼 | case | 최종 root offset | 최종 outer offset | 최종 inner offset |
| --- | --- | --- | --- |
| Android | `auto-horizontal` | `(0,0)` | `(60.375,0)` | `(86,106)` |
| Android | `auto-vertical` | `(0,0)` | `(0,60.265)` | `(86,106)` |
| Android | `auto-diagonal` | `(0,0)` | `(58.686,58.686)` | `(86,106)` |
| Android | `contain-horizontal` | `(0,0)` | `(0,0)` | `(86,106)` |
| Android | `contain-vertical` | `(0,0)` | `(0,59.814)` | `(86,106)` |
| Android | `contain-diagonal` | `(0,0)` | `(0,60.567)` | `(86,106)` |
| iOS | `auto-horizontal` | `(0,0)` | `(50,0)` | `(86,106)` |
| iOS | `auto-vertical` | `(0,0)` | `(0,50)` | `(86,106)` |
| iOS | `auto-diagonal` | `(0,0)` | `(50,50)` | `(86,106)` |
| iOS | `contain-horizontal` | `(0,0)` | `(0,0)` | `(86,106)` |
| iOS | `contain-vertical` | `(0,0)` | `(0,50)` | `(86,106)` |
| iOS | `contain-diagonal` | `(0,0)` | `(0,50)` | `(86,106)` |

`auto-zero`는 두 플랫폼 모두 첫 draw에서 revision 0, 모든 owner `(0,0)`을 확인했다. containment 수평 case의 before/after 화면은 RGB max-channel 차이 8 초과 픽셀이 0개였다. 나머지 swipe case도 threshold 초과 변경 픽셀이 root 경계 밖에 0개였고, 캡처된 blue inner box는 green outer clip 안에, green outer box는 gold root 안에 남았다. Android/iOS 결과가 축별로 일치한다. 입력 길이만큼 outer에서 소비됐으므로 root offset은 움직이지 않았다.

전체 14-run 캡처 후 scenario/queue state를 Rust 하위 모듈로 나누었다. 그 뒤 양 플랫폼 opt-in 앱을 현재 소스에서 재빌드하고 contain-horizontal 실제 swipe를 다시 실행했다. 첫 draw seed/revision이 맞았고, 누적 입력에도 세 owner offset과 scroll revision이 유지됐으며 양쪽 before/after changed pixel은 0개였다. [최신 소스 재빌드·가로 containment smoke 로그와 화면](../spec/internal/evidence/c13-3-policy-simulator-matrix-2026-10-15/README.md#현재-소스-재빌드-확인)에서 전체 행렬 캡처와 현재 소스 smoke를 구분한다.

두 플랫폼 static archive에서 feature-off C13.3 scenario symbol/marker 0개, opt-in positive control 1개/marker 존재를 확인했다. 구현 실패 경로 검토 20개와 PR 변경 검토 30개도 작성했다. required CI와 merge만 남았다. 이 검증은 opt-in simulator fixture의 computed policy, initial draw, native touch callback과 submitted screenshot을 다룬다. 실기기, hardware GPU, compositor 이후 scanout 근거가 아니다.

## 범위 밖

이 후속은 `touch-action` non-default matrix, `none` local affordance, native glow/bounce, tap/click activation, wheel/trackpad, style invalidation, actual iOS multi-touch, root/body 특수 전파, S04.10 displayed-frame targeting, S05.2 public events, WPT, hardware GPU와 compositor/scanout을 완료하지 않는다. C13.3 및 상위 C13 상태는 미완료로 둔다.
