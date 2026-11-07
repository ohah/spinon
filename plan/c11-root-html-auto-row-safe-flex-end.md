# C11.4 parsed Root HTML `align-content:safe|unsafe flex-end`

**상태:** 계획·계획 실패 관점 검토 20개·Chrome 157 사전 비교를 마쳤고 bounded implementation 진행 중이다. 6 scene·72 item-frame coordinates는 독립 기대값과 정확히 일치해 최대 오차가 0 CSS px다. Layout·style projection·RuntimeSession focused tests는 각 2/2, 회귀 filter는 각각 9/9·6/6·9/9 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 positive·safe overflow·unsafe overflow 기본 Root HTML route 세 장면을 실행했고 기존 앱·build/bootstrap 복원을 확인했다. 구현/runtime 실패 관점 20개 검토를 마쳤으며 실제 PR diff 실패 관점 30개 검토가 남아 있다. [사전 비교 근거](../spec/internal/evidence/c11-root-html-auto-row-safe-flex-end-precomparison-2026-11-07/README.md) · [실행·Simulator 근거](../spec/internal/evidence/c11-root-html-auto-row-safe-flex-end-runtime-2026-11-07/README.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-root-html-auto-row-safe-flex-end-runtime-2026-11-07/implementation-runtime-review-20.md) · [계획 검토 20개](./c11-root-html-auto-row-safe-flex-end-plan-review-20.md) · [계획 PR diff 검토 30개](./c11-root-html-auto-row-safe-flex-end-plan-pr-diff-review-30.md). C11.4/C11 및 공식 `78/214`는 미완료·변경 없음.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [선행 `safe|unsafe flex-start`](./c11-root-html-auto-row-safe-flex-start.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 목적

기본 parsed Root HTML의 기존 `RootHtmlAutoRowStretchV1` fixed profile에서 exact computed `align-content:safe flex-end`와 `unsafe flex-end`를 제한적으로 허용한다. CSS Box Alignment Level 3 §4.1은 flex formatting context 밖의 `flex-end`가 `end`처럼 동작한다고 정의한다. 따라서 이 Grid profile에서 plain `end`와 plain `flex-end`의 배치는 같아야 한다. `safe` overflow는 시작점 쪽으로 fallback하고 `unsafe`는 요청된 end 위치를 유지한다. 해당 overflow modifier는 Working Draft에서 at-risk다.

## 기준과 독립 기대값

고정 입력은 definite 40×40px content box, 40px explicit column, 10px explicit row, 두 direct-text implicit `auto` rows, row auto-flow, `column-gap:0`, `align-items:normal`, fixed empty 40×10px item과 line-height 10px `A`/`B` direct text다. 기존 `RootHtmlAutoRowStretchV1` row sizing과 bounded row-gap 처리는 유지한다. [Chrome 157 사전 비교](../spec/internal/evidence/c11-root-html-auto-row-safe-flex-end-precomparison-2026-11-07/README.md)는 `end`, `safe flex-end`, `unsafe flex-end`를 positive 및 overflow 입력에서 대조한다.

| `row-gap` | `align-content` | Used rows (px) | Item y offsets (px) |
|---|---|---|---|
| `0px` | `end`, `safe flex-end`, `unsafe flex-end` | `[10,10,10]` | `[10,20,30]` |
| `calc(120% - 2px)` | `end`, `unsafe flex-end` | `[10,10,10]` | `[-82,-26,30]` |
| `calc(120% - 2px)` | `safe flex-end` | `[10,10,10]` | `[0,56,112]` |

Definite 40px block-size basis에서 row gap은 46px이다. Overflow alignment subject는 세 10px rows와 두 46px gutters를 합쳐 122px이며 end placement는 첫 row를 y=-82px에 둔다. Safe fallback은 첫 row를 y=0px에 둔다. `unsafe flex-end`에서는 앞 두 item이 0보다 작은 block offset을 가진다. Simulator screenshot은 화면 밖 frame을 재지 않으므로 negative frame의 판정은 Rust Layout/RuntimeSession coordinates에 두고, OS screenshot은 보이는 surface bounds만 확인한다.

## Admission과 실패 동작

- 해당 fixed profile의 exact computed `safe flex-end`와 `unsafe flex-end`만 기존 `align-content` allowlist 및 profile gate에 추가한다.
- computed serialization과 safety modifier를 보존한다.
- `end`, plain `flex-end`, safe/unsafe start·center·end/flex-start 회귀를 확인한다.
- `safe|unsafe`가 붙은 baseline·distributed value, unknown serialization, 다른 item/track shape와 부적격 Root HTML node는 계속 fail-closed다.
- Style-to-layout과 layout gate가 같은 값 집합을 허용해야 한다. Runtime rejection은 `align-content`에 귀속되고 partial snapshot을 publish하지 않아야 하며, 같은 RuntimeSession은 이후 valid request로 복구해야 한다.
- Parser, 일반 CSS admission, public API, FFI, row sizing, row-gap resolution, fixture ABI와 내부 숫자 계약 버전 `0.1.0`은 바꾸지 않는다.

## 검증 게이트

1. 고정 [Chrome 157 사전 비교](../spec/internal/evidence/c11-root-html-auto-row-safe-flex-end-precomparison-2026-11-07/README.md)와 독립 기대값으로 6개 scene·72개 item-frame 좌표를 1/64 CSS px 허용오차에서 비교한다.
2. Layout·style projection focused tests에서 computed string, positive/overflow coordinates, rows `[10,10,10]`와 gutters `[46,46]`를 확인한다.
3. RuntimeSession에서 safe fallback·unsafe negative coordinates, complete snapshot, property error attribution, atomic rejection 및 same-session recovery를 확인한다.
4. 기존 safe/unsafe start·center·end, fixed/percentage/`calc()` row-gap 회귀, scoped Clippy와 `cargo fmt --all -- --check`를 실행한다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML route에서 positive, safe overflow, unsafe overflow를 실행한다. Android/iOS OS screenshot은 visual bounds로 분류하고 off-screen negative frame의 좌표 증거로 사용하지 않는다.
6. Android APK, iOS app, bootstrap input 및 설치 앱의 실행 전·후 해시를 기록하고 복원한다. Empty PR check rollup은 CI 성공으로 기록하지 않는다.
7. 구현/runtime diff는 새로운 20개 failure-oriented 관점으로, 실제 PR diff는 별도 30개 관점으로 검토한다. 가능한 경우 PR 본문에 양쪽 Simulator screenshot을 넣는다.

## 남은 범위

다른 writing mode/direction, arbitrary Grid topology, 일반 CSS admission, WPT 적합성, physical device, hardware GPU와 iOS compositor 이후 pixel parity는 이 slice에서 다루지 않는다. C11.4/C11의 완료 check와 공식 `78/214`는 유지한다.
