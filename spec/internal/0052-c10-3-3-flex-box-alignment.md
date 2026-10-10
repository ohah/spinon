# C10.3.3 · Flex 비-baseline Box Alignment

**문서 ID:** `0052` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 제한 구현·검증 완료, PR 검토 대기 · **공개 CSS/API 전체 완료:** 아님

이 계약은 여섯 runtime Flex profile에서 비-baseline Flex 정렬 속성을 Stylo computed style부터 Taffy frame까지 연결한다. Taffy 0.14.0은 계산기이고, 판정 기준은 고정 Chrome 154 reference다. 지원 목록 밖의 값을 기본값으로 바꿔 성공시키지 않는다.

## 지원 profile과 계산값

- 대상은 `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1`이다. legacy Flex·Block profile에 새 `align-self`·`align-content` 값을 전달하지 않는다.
- `align-items`는 computed `normal`, `stretch`, `center`, `flex-start`, `flex-end`, 그리고 `start`, `end`, `self-start`, `self-end`와 선택적 `safe|unsafe`를 받는다. `baseline` 계열은 C10.3.4에 맡긴다.
- `align-self`는 `auto`, `normal`, `stretch`, 위와 같은 비-baseline 위치 및 safety modifier를 받는다. `auto`는 computed 값으로 보존하고 layout에서 부모의 computed `align-items`를 사용한다. 부모 속성의 CSS 상속으로 대체하지 않는다.
- `align-content`는 `normal`, `stretch`, `space-between`, `space-around`, `space-evenly`, `center`, `start`, `end`, `flex-start`, `flex-end`와 선택적 `safe|unsafe`를 받는다. `baseline` 계열은 C10.3.4 범위다.
- `justify-content`는 기존 runtime Flex 지원값에 `normal`, `stretch`, `start`, `end`, `left`, `right`, 그리고 positional `safe|unsafe`를 더한다. `normal`과 `stretch`의 used 동작은 Flex 규칙대로 `flex-start`다. `place-content` 지원에 필요한 계산값이다.
- `place-items`, `place-self`, `place-content`는 Stylo cascade가 계산한 longhand 결과를 사용한다. `place-content`가 설정하는 `align-content`와 `justify-content`를 모두 layout에 전달한다. Flex에서 `justify-items`·`justify-self`가 frame을 바꾸지 않는 것도 기준과 대조한다.
- `safe` overflow modifier는 Taffy의 `Safe`로 전달하고, 생략 및 명시 `unsafe`는 `Unsafe`로 전달한다. computed 문자열은 별도로 보존한다.
- `safe|unsafe`는 positional 값에만 허용한다. invalid authored declaration은 CSS cascade에서 무시되는 동작과, adapter가 잘못된 computed 값을 받아 `UnsupportedComputedValue { node, property, value }`로 실패하는 동작을 구분한다.

## Auto margin 및 Taffy 보정

CSS Flexbox는 교차축 auto margin이 있는 item에 `align-self`를 적용하지 않는다. 교차축 여유 공간이 음수이면 cross-start auto margin을 0으로 두고 넘침을 cross-end 쪽으로 보낸다. [CSS Flexbox Level 1 §8.1·§9.6](https://www.w3.org/TR/css-flexbox-1/#algo-cross-margins).

잠긴 Taffy 0.14.0은 음수 여유 공간을 cross-start auto margin에 음수로 기록한다. `spinon-layout`은 Flex 부모와 item의 cross-start auto margin, Taffy가 계산한 해당 물리 여백이 음수인 경우에 한해 최종 item frame을 그 여백의 반대 방향으로 이동한다. 검증된 교차축은 `row`·`row-reverse`의 위쪽과 `column`·`column-reverse`의 왼쪽이며 환경은 `horizontal-tb`·LTR다. positive/zero free space, cross-end-only margin, main-axis margin, 비-Flex 부모는 보정하지 않는다. row·column overflow 사례를 별도 layout 시험으로 둔다.

RTL·다른 writing mode는 이 계약의 비교 범위에서 제외한다. 특히 RTL column의 cross-start auto margin 동작은 여기서 보증하지 않으며 C17 계약과 비교 fixture가 연결되기 전까지 지원 완료로 주장하지 않는다. 이를 일반적인 RTL 지원으로 확대 해석하지 않는다.

## 비교 기준과 통과 조건

- Oracle: macOS arm64 Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`. 원본 실행 파일과 입력 fixture·inventory·capture helper의 digest는 [고정 reference](../../tests/fixtures/css/references/c10-3-3-flex-alignment-v1.json)에 저장한다.
- CSS fixture는 `320×240 CSS px`, DPR 1·2, `horizontal-tb`, LTR다. `flex-alignment.html`은 50개 case·134개 unique element node이며, computed longhand와 각 node의 `x`, `y`, `width`, `height`를 보존한다. 각 좌표 필드의 최대 오차 기준은 독립적으로 `0.5 CSS px`다.
- Rust 검사는 위 50개 case 모두를 여섯 profile에서 비교한다. inline full/incremental cascade, author stylesheet, `var()`·등록 속성 cascade와 malformed adapter 입력을 별도 검사한다. DPR 간 Chrome CSS px frame 동일성도 확인한다.
- 앱 runtime smoke는 별도 8-node fixture다. Android와 iOS의 V8 report에서 node별 CSS frame, `layout=ready`, WGPU 제출 box 수를 고정 Chrome runtime reference와 비교한다. 이 8-node 실행은 50-case 전체 모바일 실행을 뜻하지 않는다.
- upstream WPT inventory는 commit `d5a765f1089ce6d3f72300281481edf3dddff7f3`의 관련 경로를 fixture case에 연결한다. WPT 자체 실행은 `not-run`이며 WPT 전체 적합성은 주장하지 않는다.

## 내부 실행 경로

`V8 fixture → Stylo cascade → typed style snapshot → Taffy 0.14.0 → runtime frame → WGPU`.

- Android 내부 실행 표식: `spinon_c1033_flex_alignment=true`.
- iOS 내부 실행 표식: `--spinon-c1033-flex-alignment`.
- 이 probe와 C ABI는 개발 fixture용이며 공개 JavaScript API가 아니다. 호출 실패나 layout 오류를 성공 화면으로 대체하지 않는다.
- 문서 계약 숫자 버전은 출시 전 정책에 따라 `0.1.0`으로 고정한다.

## 검증 결과와 남은 경계

- Rust workspace test, Clippy, rustfmt 및 CSS reference 검사가 통과했다. 상세한 실패 관점과 수정 이력은 [구현 검토](./evidence/c10-3-3-flex-alignment-implementation-review-2026-10-11.md)에 기록한다.
- Android 16/API 36 Samsung SM-S731N 실기기에서는 WGPU Vulkan backend / Samsung Xclipse 940을 확인했다. 8/8 node frame이 Chrome runtime과 일치했고 WGPU가 8개 box를 제출했다. [build·APK digest·원시 log·화면](./evidence/c10-3-3-android-2026-10-11/c1033-android-physical-2026-10-11.md).
- iPhone 17 Pro / iOS 26.2 Simulator에서는 V8 frame 8/8과 WGPU 제출 8개를 확인했다. 이 결과는 실기기 성능 증거가 아니다. [build·원시 log·화면](./evidence/c10-3-3-ios-2026-10-11/c1033-ios-simulator-2026-10-11.md).
- Android Gradle은 AGP 8.13.2가 compile SDK 37.2까지 검증되지 않았다는 warning과 함께 성공했다. iOS 빌드에는 기존 deprecated Swift API, 중복 `RawWindowMetalLayer` symbol 및 dSYM map warning이 남아 있다.
- RTL 및 다른 writing mode, C04 legacy profile의 새 문법, baseline/text, positioned child, scroll overflow, 전체 WPT 실행, 50-case 앱 전체 행렬, Chrome과 pixel 단위 이미지 비교, 제품 성능은 이 계약으로 완료 처리하지 않는다.

## 근거

- [C10.3 구현 계획](../../plan/c10-3-flex-order-alignment.md) · [공식 상태 대장](../STATUS.md) · [Chromium fixture](../../tests/fixtures/css/c10/flex-alignment.html) · [fixture inventory](../../tests/fixtures/css/c10/flex-alignment-inventory.json) · [Chrome reference](../../tests/fixtures/css/references/c10-3-3-flex-alignment-v1.json)
- [구현 검토](./evidence/c10-3-3-flex-alignment-implementation-review-2026-10-11.md) · [사전 비교](./evidence/c10-3-3-flex-alignment-precomparison-2026-10-10.md) · [계획 검토](./evidence/c10-3-3-align-self-content-plan-review-2026-10-10.md)
- [Android 실기기 증거](./evidence/c10-3-3-android-2026-10-11/c1033-android-physical-2026-10-11.md) · [iOS Simulator 증거](./evidence/c10-3-3-ios-2026-10-11/c1033-ios-simulator-2026-10-11.md)
