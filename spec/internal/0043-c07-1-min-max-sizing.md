# 0043 · C07.1 물리 축 최소·최대 크기

**문서 ID:** `0043` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 제한 runtime profile 구현·검증 완료 · **공개 API:** 아님

문서 ID는 이 명세 파일을 찾기 위한 식별자이며 제품·호환성 버전이 아니다. 출시 전 내부 계약 숫자 버전 `0.1.0`은 구현 진행이나 문서 추가로 올리지 않는다.

## 적용 범위

이 계약은 현재 runtime layout 계산의 물리 축 `min-width`, `max-width`, `min-height`, `max-height`에 적용한다. 앱 작성자에게 CSS를 공개 지원한다고 선언하지 않는다.

최소·최대 크기는 기존 `box-sizing`, `width`·`height`, padding, percentage basis 및 제한된 typed CSS math와 함께 Taffy `Style.min_size`·`max_size`에 전달한다. percentage는 축별 definite containing block 기준을 보존한다. width 제약은 containing block 너비, height 제약은 containing block 높이를 사용한다.

## 값과 오류

| CSS 값 | 내부 계산 입력 |
| --- | --- |
| `min-width`·`min-height: auto` | 최소 자동 크기 값 `LayoutDimension::Auto` |
| `max-width`·`max-height: none` | 제한 없음 값 `LayoutDimension::Auto` |
| nonnegative finite length | CSS px `LayoutDimension::Fixed` |
| nonnegative finite percentage | 분율 `LayoutDimension::Percent`; Taffy가 부모 content size로 해석 |
| 현재 C06 subset의 typed math | property별 `LayoutDimension::Calc`와 출처가 일치하는 AST binding |

`auto`와 `none`의 CSS 차이는 computed-style 직렬화와 property 위치로 보존한다. Taffy에는 해당 최소 자동값 또는 무제한값으로 투영한다. CSS parser가 무효로 판단한 음수 literal declaration은 cascade에 들어오지 않으며 앞선 유효 선언이 남는다. Rust layout DTO의 음수·NaN·무한대는 `LayoutError::InvalidStyle`로 계산 전에 거부한다. 계산식 ID의 소유 노드·속성 바인딩이 빠지거나 다르면 결과 전체를 거부한다.

`min-content`, `max-content`, `fit-content`, `stretch`, anchor-size 및 텍스트 intrinsic minimum은 이 구현 범위에서 지원하지 않는다. 이 값들을 `0`, `auto` 또는 `none`으로 조용히 대체하지 않고 node/property가 포함된 `UnsupportedComputedValue` 오류로 반환한다.

## 계산 사례

```css
.card {
  box-sizing: border-box;
  width: 160px;
  min-width: 100px;
  max-width: 120px;
}
```

최종 frame은 기존 Flex·Block 계산의 제약을 받으며 지정 width와 min/max를 모두 만족해야 한다. `min-width`가 `max-width`보다 크면 pinned Chromium 관찰에서 최소값이 우선했다. Flex shrink에서는 min constraint를 유지하고, 명시적인 `min-width: 0`은 콘텐츠가 빈 fixture에서 공간에 맞게 줄 수 있다. text wrapping이나 min-content 측정은 보장하지 않는다.

```css
.flex-child { flex-basis: 100px; min-width: 0; }
```

## 내부 자료 흐름

1. Stylo `ComputedValues`에서 min/max 값과 출처 계산식 AST를 추출한다.
2. `ComputedLayoutDimensions`와 `ComputedCssMaxSize`가 px·percentage·auto/none·unsupported 의미를 담는다.
3. `spinon-style-to-layout`은 현재 runtime layout 계열 profile만 이 속성을 읽는다. 과거 fixture profile은 기존 기본 제약으로 계산한다.
4. `spinon-layout::LayoutStyle`은 네 물리 축의 제약을 소유한다.
5. Taffy 0.14.0 `min_size`·`max_size`에서 최종 geometry를 계산한다.

이 자료 흐름은 공개 DOM API, `getComputedStyle`, 사용자 정의 renderer API 또는 전체 CSSOM을 만들지 않는다.

## 현재 검증 상태와 남은 경계

- [구현 전 비교 모델](evidence/c07-1-min-max-sizing-precomparison-2026-10-10.md)의 pinned Chrome 154 35-node fixture에 Rust Stylo→Taffy frame을 대조한다. 현재 Rust 테스트는 모든 frame field에서 최대 절대 오차 `0 CSS px`, DPR 1·2 동일 결과를 확인했다.
- 구현의 별도 실패 경로 점검은 [구현 검토 기록](evidence/c07-1-min-max-sizing-implementation-review-2026-10-10.md)에 있다. 전체 Rust workspace tests·Clippy·format, FFI all-features check, 고정 reference 검사, Android/iOS simulator 빌드·실행이 통과했다.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 C07.1 전용 V8 fixture가 `layout=ready`, `boxes=10`으로 표시됐다. [Android 화면](evidence/c07-1-min-max-sizing/android-api37.png) · [iOS 화면](evidence/c07-1-min-max-sizing/ios-26.2.png) · [Android 실행 로그](evidence/c07-1-min-max-sizing/android-c071-log.txt) · [iOS 실행 로그](evidence/c07-1-min-max-sizing/ios-log.txt).
- simulator runtime 로그는 개별 child frame 계측을 제공하지 않으므로 노드별 수치 비교는 Chrome/Rust oracle에 한정한다. Android 실기기, hardware GPU 성능, 스크롤, 텍스트 intrinsic sizing은 확인하지 않았다.
- 이 제한 slice는 [공식 상태 대장](../STATUS.md)에서 완료할 수 있지만 C07 전체 또는 공개 CSS 지원 완료를 의미하지 않는다.
