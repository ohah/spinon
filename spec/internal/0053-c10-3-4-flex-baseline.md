# C10.3.4 · Flex baseline 내부 계약

**문서 ID:** `0053` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 제한 구현·검증 완료, [PR #125 리베이스 병합](https://github.com/ohah/spinon/pull/125) · **공개 CSS/API 전체 완료:** 아님

이 계약은 고정 Chrome 154에서 관찰한 빈 고정 크기 element box의 Flex baseline 정렬을 내부 cascade-to-layout 경로에 연결한다. 전체 Flexbox baseline, 텍스트 shaping, 공개 CSS 지원 완료를 뜻하지 않는다.

## 프로필과 typed 값

- 대상 runtime profile은 `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1`이다.
- `align-items`는 computed `baseline`·`first baseline`을 `FirstBaseline`, `last baseline`을 `LastBaseline`으로 보존한다. `align-self`는 같은 두 그룹과 `auto`를 보존하며, `auto`는 부모의 computed `align-items`를 사용한다.
- `place-items`·`place-self`가 만든 longhand도 이 계약의 같은 typed 값을 사용한다. computed 문자열은 각 node의 `ComputedStyleSnapshot`에 보존한다.
- legacy Flex·Block projection profile에 baseline 값을 새로 허용하지 않는다. 해당 profile에 baseline computed value가 들어오면 `UnsupportedComputedValue { node, property, value }`로 거부한다.
- `align-content:first baseline`은 `LayoutAlignContent::FirstBaseline`으로 보존한다. 현재 성공 범위는 visible descendant가 없고 width·height가 고정된 Flex item뿐이다. 빈 고정 상자의 pinned Chrome geometry만 비교하며 일반 텍스트 content-alignment나 Flex line baseline distribution으로 확대하지 않는다.
- 고정 Chrome 154에서 `align-content:last baseline`은 invalid declaration으로 처리한다. stylesheet와 inline style의 직접 선언 및 `place-content` 성분을 CSS parser 단계에서 무효화해 앞선 cascade 선언을 보존한다. 파서는 comment·string·함수 내부 토큰을 수정하지 않고 중첩 rule 깊이가 64를 넘으면 원문 전체를 보존한다. 이후 Stylo가 이 경계를 넘는 computed 값을 내면 layout projection에서 성공으로 치환하지 않고 거부한다.

## baseline과 line 계산

- 성공 입력은 `horizontal-tb`·LTR의 row 계열 Flex container와 빈 fixed-size element box다. `align-self:auto`를 포함해 first·last 참여자를 서로 다른 baseline-sharing group으로 모은다. 다른 그룹과 비-baseline 참여자는 그 그룹 계산에 섞지 않는다.
- 비-Flex 빈 item의 first·last baseline은 border-box block-end edge에서 합성한다. padding·border·사용 cross margin은 Taffy의 최종 layout 값에서 가져온다.
- Taffy 0.14.0은 Flex row first-baseline 결과를 제공하지만 last-baseline set은 제공하지 않는다. Spinon은 first·last group을 따로 계산하고 최종 layout tree의 y 위치를 보정한다. first group은 cross-start, last group은 cross-end에 놓고, 참여자가 하나인 group에는 반대편 edge fallback을 적용한다.
- Flex item은 computed `order`와 원래 DOM 순서의 안정 정렬을 따른다. line은 Taffy 위치 reset을 기본으로 재구성한다. 같은 main-axis 좌표는 이전 item의 사용 크기·margin과 부모 gap을 함께 보고 판정하며, 음수 main margin처럼 좌표만으로 line 경계를 확정할 수 없는 입력은 `UnsupportedBaseline { node, reason }`으로 실패한다.
- 여러 wrapped line의 cross bounds는 사용 cross margin으로 계산하고, `align-content:normal|stretch`는 content cross 크기와 `row-gap`을 반영해 각 line의 stretch 분배를 계산한다. `nowrap` 및 실제 한 line은 부모의 inner cross box를 사용한다.
- column 계열은 horizontal writing mode에서 baseline 축과 cross axis가 다르므로 별도 보정하지 않는다. computed value와 고정 Chrome cross-start fallback geometry만 fixture로 비교한다.
- RTL, 다른 writing mode, text/replaced intrinsic size, line box, pseudo-element, out-of-flow/flex item, inline formatting, 제품 pixel 일치는 이 계약 밖이다. 텍스트 baseline이 필요한 입력을 fixed box baseline으로 대체하지 않는다.

## 비교 범위와 실행 근거

- 고정 Google Chrome `154.0.8037.98`에서 18개 case·73개 node를 DPR 1·2로 관찰했다. Rust 통합 비교는 여섯 runtime profile 각각에서 computed `align-items`·`align-self`·`align-content`와 각 node의 `x`·`y`·`width`·`height`를 비교하며 frame 허용 오차는 필드마다 `0.5 CSS px`다.
- WPT revision과 mapped subset은 [fixture inventory](../../tests/fixtures/css/c10/flex-baseline-inventory.json)에 고정했다. WPT 원본 suite는 실행하지 않았다. 텍스트 baseline WPT는 C15 선행 범위라 제외했다.
- Android SM-S731N 실기기와 iPhone 17 Pro / iOS 26.2 Simulator에서 12-node V8 → Stylo → Taffy → WGPU smoke를 실행했다. 이 플랫폼 실행은 18-case 전체 행렬이 아니라 실제 runtime 호출·frame report·scene 제출 확인이다.
- 전체 증거와 명시적 미검증 경계는 [구현 검토 및 실행 결과](./evidence/c10-3-4-baseline-implementation-review-2026-10-11.md)에 기록한다.

## 근거

- [구현 계획 C10.3.4](../../plan/c10-3-flex-order-alignment.md#c1034--범위와-baseline-모델)
- [공식 상태 대장](../STATUS.md)
- [Chromium fixture](../../tests/fixtures/css/c10/flex-baseline.html) · [inventory](../../tests/fixtures/css/c10/flex-baseline-inventory.json) · [reference](../../tests/fixtures/css/references/c10-3-4-flex-baseline-v1.json)
- [Android·iOS runtime evidence](./evidence/c10-3-4-baseline-runtime-2026-10-11/)
