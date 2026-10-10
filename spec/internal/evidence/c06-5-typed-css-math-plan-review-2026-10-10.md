# C06.5 typed CSS math 계획 검토

**대상:** [C06.5 계획](../../../plan/c06-5-typed-css-math.md) · C06.5 기능 구현 전 계획 전용 검토

초기 검토에서 세 가지 모순을 찾아 계획에 반영했다. 기존 설명은 Taffy `calc` feature를 켜면 계산된다고 암시했고, divide-by-zero를 CSS invalid로 단정했으며, `clamp()`의 `none` 경계를 밝히지 않았다. 계획은 이제 Taffy 저수준 resolver를 요구하고, CSS Values 4 Working Draft와 고정 Chromium의 non-finite 관찰을 분리하며, `clamp(none, …)`를 첫 slice의 미지원 경계로 명시한다.

| # | 공격 경로 | 계획에서 요구하는 차단·검증 |
|---:|---|---|
| 1 | Taffy `calc` feature를 켜고 고수준 `TaffyTree`를 그대로 사용 | Taffy 0.14.0 고수준 tree resolver가 0.0 기본값임을 고정 의존성에서 확인하고 저수준 adapter를 요구한다. |
| 2 | feature가 Cargo dependency resolver에서 실제 활성화되지 않음 | `spinon-layout` dependency feature와 `cargo tree -e features`를 검사한다. |
| 3 | Stylo 비공개 calc node 포인터를 Taffy에 넘김 | Stylo 값에서 owned DTO로 변환하고 Taffy 주소에는 adapter 소유 객체만 넣는다. |
| 4 | Taffy calc 포인터의 낮은 3 bit가 tag로 쓰여 정렬이 부족하거나 owner가 drop됨 | `repr(align(8))` owner, callback 동안 Arc 소유, pointer→expression map 및 pointer를 직접 역참조하지 않는 불변식을 검증한다. |
| 5 | Taffy callback이 반환할 수 없는 평가 오류를 `0`으로 삼킴 | side channel에 첫 오류를 기록하고 전체 frame 결과를 폐기한다. |
| 6 | calc의 percentage를 항상 viewport 폭으로 계산 | width/height/flex-basis와 spacing별 기존 C06 basis checker를 연결한다. |
| 7 | parent basis가 indefinite일 때 callback에 임의 0 전달 | calc AST의 percentage 존재를 검사하고 기존 definite-basis 오류로 사전 차단한다. |
| 8 | Typed OM `50 percent`의 50을 50.0 ratio로 사용 | Stylo reify 값의 100배 표현을 확인하고 DTO에서 `100% = 1.0`으로 변환한다. |
| 9 | `Number`, `<length>`, `<percentage>`를 모두 float 하나로 취급 | Typed OM unit/dimension을 보존하며 incompatible typed nodes를 지원 처리하지 않는다. |
| 10 | `calc(1px + 1s)`를 레이아웃 adapter의 generic error로 오진 | CSS parser invalid/fallback과 adapter-unsupported 결과를 별도 fixture 상태로 기록한다. |
| 11 | `/ 0`을 무조건 CSS invalid로 거부 | CSS Values 4의 ±∞/NaN 계산과 Chrome 154 computed/used 결과를 캡처한다. |
| 12 | Infinity/NaN이 Taffy frame, cache 또는 GPU까지 유출 | callback 결과 검사, 최종 finite frame 검사, 실패 시 snapshot/frame 미커밋을 적용한다. |
| 13 | 음수 `width`와 음수 `margin`에 동일 clamp 사용 | nonnegative size/padding/gap과 signed margin을 속성별로 판정한다. |
| 14 | `clamp(100px, …, 50px)`에서 max가 우선 | CSS Values 4와 Chromium fixture에 따라 minimum bound 우선임을 검증한다. |
| 15 | `clamp(none, …)`를 세 숫자 인수처럼 해석 | typed AST가 안전하게 보존하는지 확인하고 첫 범위에서는 explicit unsupported로 둔다. |
| 16 | `min()`·`max()`의 단일/복수 인자, 동률, 중첩에서 잘못 선택 | pinned Chrome computed output과 geometry를 인자 개수·순서별로 비교한다. |
| 17 | 식 내부 우선순위·괄호·중첩 calc가 달라짐 | Stylo 제공 AST만 평가하고 곱/나눗셈 precedence와 중첩을 별도 oracle로 검사한다. |
| 18 | `round()` 등 미지원 함수가 0으로 성공 | 미지원 연산자를 구별해 오류가 되고, 기본값·0 frame으로 대체되지 않는지 검사한다. |
| 19 | 저수준 Taffy adapter에서 block/Flex, hidden subtree, cache 경로 누락 | math 없는 전체 기존 layout regression suite와 C06.1–C06.4 fixtures를 비교한다. |
| 20 | DPR, runtime FFI, renderer가 typed/used CSS px를 다시 배율 적용 | DPR 1/2의 Chrome frame과 Android/iOS simulator의 CSS viewport, boxes, WGPU present 및 화면을 독립 기록한다. |

## 판정

계획 검토 당시에는 Chromium precomparison과 Stylo Typed OM shape 확인이 선행 조건이었다. 후속 사전 비교에서 fixture를 캡처하고 Stylo Typed OM shape test를 추가했다. 이 과정에서 계획에 적힌 Stylo `0.20.0`이 실제 고정 의존성과 다름을 발견해 `0.22.0`으로 바로잡았다. CSS Values 4는 Working Draft이므로 출시 계약 전체로 간주하지 않고 고정 Chromium 154 결과를 이 runtime slice의 기준으로 삼는다. 미출시 내부/package numeric version는 `0.1.0`으로 유지한다.

## 기준 문서와 소스

- [CSS Values and Units Module Level 4 §10.1–10.2, §10.9, §10.12](https://www.w3.org/TR/css-values-4/)
- 고정 Stylo source: `stylo 0.22.0`, `values/computed/length_percentage.rs`, `typed_om/mod.rs`, `values/generics/calc.rs`
- 고정 Taffy source: `taffy 0.14.0`, `tree/traits.rs`, `tree/taffy_tree.rs`, `examples/custom_tree_vec.rs`
