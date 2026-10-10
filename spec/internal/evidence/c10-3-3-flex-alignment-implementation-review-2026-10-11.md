# C10.3.3 구현 실패 관점 검토 및 실행 결과 · 2026-10-11

**대상:** `feat/css-c10-3-3-alignment` 변경분 · [계약 0052](../0052-c10-3-3-flex-box-alignment.md) · [구현 계획](../../../plan/c10-3-flex-order-alignment.md)

| # | 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | `normal`을 computed keyword가 아니라 layout enum 기본값으로 잃는가 | 50-case Chrome reference의 computed value를 여섯 profile에서 비교했다. `align-items:normal`, `align-content:normal`, `justify-content:normal`이 보존되고 used frame도 일치했다. |
| 2 | `align-self:auto`를 상속 또는 고정 기본값으로 잘못 처리하는가 | 부모 `align-items` 변경 후 child `align-self:auto`의 computed 값은 유지되고 새 위치가 inline incremental 결과·full cascade와 일치했다. |
| 3 | `start/end`를 `flex-start/flex-end`와 동일시하는가 | logical·flex-relative start/end, `self-start/end` 및 `wrap-reverse` reference case를 비교했다. 검증된 LTR 환경의 좌표가 일치했다. |
| 4 | `safe` overflow가 `unsafe`와 같게 움직이는가 | 양·음수 여유 공간에서 `safe center`, `safe flex-end`, `unsafe center`와 safety 생략 case를 고정 Chrome과 비교했다. |
| 5 | safe modifier 누락·지원 밖 조합을 안전 정렬로 오인하는가 | Taffy 변환은 명시 `safe`만 `Safe`, 생략/명시 `unsafe`는 `Unsafe`로 보낸다. `safe space-between`, baseline, `align-content:left` 같은 adapter 입력은 node/property 문맥의 오류로 거부한다. |
| 6 | `align-self:auto`가 부모 `align-items`를 참조하지 않는가 | 직접값, author stylesheet, custom property와 등록 속성 case의 computed/style-to-layout 결과를 대조했다. 부모 정렬 변경 incremental 회귀도 통과했다. |
| 7 | `place-items`가 Flex main axis의 `justify-items` 효과를 잘못 추가하는가 | shorthand 뒤 두 computed 성분을 보존하면서 `justify-items`가 Flex item frame을 바꾸지 않는 Chrome geometry를 비교했다. |
| 8 | `place-self` 단일/두 값이 `align-self` 계산을 덮는가 | `place-self`와 뒤따르는 longhand override, 안전 값 포함 case의 computed 값과 item frame을 대조했다. |
| 9 | `place-content`가 주축 `justify-content`를 빠뜨리거나 shorthand 기본값을 다르게 복사하는가 | 한 값·두 값 shorthand에서 `align-content`와 `justify-content` computed 결과 및 두 축 geometry를 비교했다. |
| 10 | `justify-content:normal|stretch`가 임의로 공간을 분배하는가 | single item, row/column, 양·음수 여유 공간에서 Chrome used position을 비교했다. Flex main-axis used start fallback과 일치했다. |
| 11 | `left/right`가 direction 또는 flex 축에 관계없이 같은 논리 정렬로 해석되는가 | Chrome fixture의 row, row-reverse, column에서 physical `left/right` case를 별도로 비교했다. 이 계약의 기준은 LTR이며 RTL 전체 지원을 주장하지 않는다. |
| 12 | item auto margin과 `align-self`가 동시에 공간을 이중 소비하는가 | positive·zero·negative cross-space case와 `align-self:flex-end` 결합을 비교했다. auto margin이 있는 경우 `align-self`로 남은 공간을 다시 배치하지 않는다. |
| 13 | 음수 cross-start auto margin으로 overflow item이 시작 edge 바깥으로 밀리는가 | 50-case Rust fixture 실행 중 Taffy 0.14.0의 negative margin 결과가 Chrome과 다르게 나오는 입력을 찾았다. Flex frame 수집에서 LTR row/column cross-start의 음수 resolved auto margin만 보정했고 회귀 case가 통과한다. |
| 14 | 양쪽 cross-axis auto margin의 음수 여유가 한쪽 정렬로 잘못 가는가 | 양쪽 margin auto와 overflow의 full layout test를 추가했다. item은 Chrome처럼 cross-start에서 시작하고 반대 방향으로 넘친다. |
| 15 | cross-end-only auto margin·main-axis margin·일반 Block까지 교정되는가 | helper의 음성 검사에서 cross-end-only, main-axis auto, 비-Flex parent는 보정하지 않음을 확인했다. positive margin도 보정 대상이 아니다. |
| 16 | stretch가 fixed size 또는 auto margin을 무시해 크기를 늘리는가 | `auto`·고정 cross-size, margin eligibility와 single/multi-line stretch case를 비교했다. |
| 17 | min/max·padding·border·box-sizing이 stretch 후 누락되는가 | clamp 및 content/border-box 입력에서 최종 box geometry를 frame별로 비교했다. 최대 오차 기준은 각 field `0.5 CSS px`다. |
| 18 | `nowrap`와 실제 한 line이 생성된 `wrap`을 같은 `align-content` 경로로 처리하는가 | nowrap 단일 line 음성 대조와 wrap 단일 실제 line, 두 줄 이상 case를 분리했다. line 위치·item 크기·gap 결과가 Chrome reference와 일치했다. |
| 19 | runtime 경계가 Rust fixture 결과와 다른 스타일/좌표를 내거나 GPU 제출 전에 실패하는가 | Android API 36 실기기에서 실제 V8 report의 8/8 frame과 Chrome runtime reference를 대조했다. Vulkan/Xclipse 940에서 WGPU 8 boxes 제출을 확인했다. |
| 20 | iOS 경로가 별도 projection·surface 때문에 Android/Chrome 결과와 달라지는가 | iPhone 17 Pro / iOS 26.2 Simulator에서 실제 V8 8/8 frame 및 WGPU 8 boxes를 확인했다. build warning은 기록했고 simulator를 실기기 성능 증거로 부르지 않는다. |

## 증거 범위 보완

Android 전체 logcat에는 C10.3.3 판정과 무관한 시스템·앱 로그도 포함되어 있었다. 공개 PR과 Tailscale 미리보기에는 Spinon runtime report와 WGPU 제출을 담은 필터 로그만 포함한다. 전체 logcat은 제품 근거로 배포하지 않는다.

## 검증 명령 결과

- `mise exec -- cargo test --locked --workspace --all-features` — 통과
- `mise exec -- cargo fmt --all -- --check` — 통과
- `mise exec -- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` — 통과
- `node --test tools/css-reference/c10-3-3-flex-alignment.test.mjs` — 6/6 통과
- `cargo test --locked -p spinon-style-to-layout c10_flex_alignment` — six profile 비교·runtime fixture 검사 통과
- Android 실기기 debug build/install/launch 및 iOS Simulator build/install/launch — 통과; 원본 자료는 [Android](./c10-3-3-android-2026-10-11/c1033-android-physical-2026-10-11.md)와 [iOS](./c10-3-3-ios-2026-10-11/c1033-ios-simulator-2026-10-11.md)에 있다.

## 명시적 미검증 경계

- WPT 경로 inventory는 저장했지만 upstream WPT 실행은 하지 않았다.
- 50-case 전체 모바일 행렬은 실행하지 않았다. 두 플랫폼은 8-node V8 fixture를 실제 실행했다.
- RTL·다른 writing mode, baseline·text, positioned child, scroll container, hit-test·접근성, Chrome screenshot과 pixel equality, 제품 성능은 여기서 확인하지 않았다.
- Android 빌드는 compile SDK 37.2/AGP 8.13.2 compatibility warning을 남겼다. iOS 빌드에는 deprecated Swift API, `RawWindowMetalLayer` 중복 symbol 및 dSYM map warning이 있다.
