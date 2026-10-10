# C07.2 · 테두리 폭 레이아웃 계획

## 목표

현재 모바일 우선 runtime layout profile에서 물리 방향 `border-top/right/bottom/left-width`와 `border-width`·`border` shorthand가 계산한 테두리 폭을 box geometry에 반영한다. Stylo typed width를 입력으로 받되 pinned Chromium의 CSS px 폭 정규화를 적용한 뒤 Rust layout DTO와 Taffy `Style.border`까지 전달하고, Chromium 기준 fixture와 비교한다.

이 작업은 border 장식의 GPU 페인트나 공개 CSS 지원을 구현하지 않는다. 내부 계약 숫자 버전과 Spinon crate 숫자 버전은 출시 전 `0.1.0`으로 고정한다. `0044`는 명세 문서 ID이며 버전이 아니다.

## 확정 범위

- 물리 방향 네 longhand 폭과 `border-width` shorthand를 포함한다. `border` shorthand는 Stylo가 만든 winning longhand 결과를 소비한다.
- 길이 입력은 pinned Chromium과 Stylo가 계산하는 CSS px, `thin`·`medium`·`thick`, 현재 C06 subset의 길이 `calc()`·`var()` 결과를 포함한다.
- 각 면의 `border-style`을 cascade에서 계산해 `none`·`hidden`은 used border width를 0으로 만들고, 다른 표준 border style은 정규화한 폭을 레이아웃에 반영한다. 이 작업에서 style의 시각적 선 모양은 그리지 않는다.
- 폭 입력은 문자열 재파싱 없이 Stylo typed computed `BorderSideWidth`에서 CSS px scalar로 얻는다. Stylo가 `calc()`·`var()`를 computed width로 이미 계산하며, 고정 Chrome의 Typed OM도 width를 `CSSUnitValue`로 반환하므로 border용 CSS math AST를 다시 운반하지 않는다. 단, Stylo는 fractional 폭을 DPR별 device pixel로 snap해 DPR 2에서 Chrome의 CSS px 결과와 달라질 수 있다. layout 경계는 pinned Chrome 관찰에 맞춰 0보다 큰 값을 정수 CSS px로 내리고 최소 1px을 적용한다. `none`·`hidden`과 정확한 0은 0이다. computed snapshot의 네 side width 문자열도 같은 결과로 정규화한다.
- `border-width`는 음수와 percentage를 받을 수 없다. 무효 선언은 Stylo cascade에서 탈락하며 앞선 유효 선언을 보존한다. 내부 `LayoutBorder` DTO는 면별 CSS px `f32`만 표현한다. 음수·NaN·무한대는 node와 면 이름을 포함한 오류로 전체 계산을 거부한다.
- border style 또는 width 변경은 style revision 변경으로 layout 입력에 반영된다. 색상만 바꾸면 used width DTO와 frame은 그대로다. 다만 현재 layout revision은 전역 `StyleRevision`이므로 색상만 바뀐 업데이트도 계산을 다시 수행할 수 있으며, 이 작업은 속성별 계산 생략 최적화를 약속하지 않는다.
- `content-box`는 콘텐츠 크기에 padding과 used border width를 더한다. `border-box`는 지정 크기에 padding과 used border width를 포함하며, 이 합계가 지정 크기보다 크면 Chromium 관찰대로 최소 외곽 상자 크기를 유지한다. C07.1 min/max 및 기존 percentage·math·flex 계산은 같은 used border 값을 사용한다.
- 기존 제한 runtime layout profile 및 실제 파생 profile만 확장한다. UA stylesheet, inline style, custom property와 등록된 custom property의 현재 cascade 경로에 같은 속성 표면을 적용한다. CSS `border` shorthand가 initial로 재설정하는 `border-image-*` 다섯 값은 초기값일 때만 profile 검사에서 허용한다. 비초기값의 border-image paint는 계속 거부한다. 미지원 profile은 이전 상태를 조용히 새 의미로 해석하지 않는다.

## 명시적 제외

border 색상·선 모양의 GPU 페인트, radius, border-image, outline, shadow, 논리 방향 border 속성, table layout, aspect ratio, replaced element·텍스트 intrinsic sizing, Grid track sizing, 외부 CSS URL, 전체 CSSOM·`getComputedStyle`, 전체 CSS 지원 선언은 제외한다. `border-style`은 used width의 0 여부에만 필요하며 paint capability를 의미하지 않는다. Outline은 box geometry에 영향을 주지 않는다.

## 현재 구현 상태

Stylo typed border width/style → `LayoutBorder` → Taffy, 50-node Chrome 비교, Android API 37·iOS 26.2 Simulator runtime 연결, 실행 증거 및 구현 실패 관점 검토까지 작업 브랜치에서 완료했다. 내부·crate 버전은 계속 `0.1.0`이다. 공식 완료와 미리보기 상태는 PR 병합 뒤 갱신한다. 자세한 실행 결과는 [시뮬레이터 증거](../spec/internal/evidence/c07-2-border-width-layout-simulators-2026-10-10.md), 구현 검토는 [구현 실패 관점 기록](../spec/internal/evidence/c07-2-border-width-layout-implementation-review-2026-10-10.md)을 본다.

## 자료 흐름과 오류 계약

1. Stylo cascade snapshot에서 네 면의 computed `BorderSideWidth`와 `BorderStyle`을 같은 style revision으로 읽는다. width는 Stylo typed app-unit 값이며 문자열 파싱을 하지 않는다.
2. Stylo app-unit width를 CSS px scalar로 바꾸고 pinned Chrome CSS px snap을 적용한다. 양수는 `max(1, floor(width))`, 정확한 0은 0이다. `BorderStyle::none_or_hidden()`이면 style gate가 0을 우선한다. 색상은 layout DTO로 보내지 않는다.
3. `LayoutStyle.border`는 네 면을 분리한 `LayoutBorder`로 소유한다. 각 값은 nonnegative finite CSS px `f32`이며 percentage나 CSS math AST는 DTO에 들어오지 않는다. CSS `calc()`·`var()` 결과는 Stylo computed scalar에서 이미 반영된다.
4. `to_taffy_style`은 정규화한 네 scalar를 `taffy::Style.border`의 `LengthPercentage::length`로 물리 면에 매핑한다. box-sizing 변환이나 값을 두 번 더하는 별도 보정은 adapter에 넣지 않는다.
5. `LayoutError`는 잘못된 면과 node를 식별한다. 음수·non-finite 입력은 전체 계산을 거부하고 partial frame을 반환하지 않는다.
6. 변경의 layout invalidation은 border-width와 border-style에 의존한다. border-color 단독 변경은 geometry를 바꾸지 않는다. 기존 source/style/environment revision 일치 게이트를 우회하지 않는다.

## 구현 순서와 판정

1. [구현 전 비교 모델](../spec/internal/evidence/c07-2-border-width-layout-precomparison-2026-10-10.md), 50-node HTML/inventory, 고정 Chromium 154 reference를 유지한다. 파일 hash·Chrome revision·DPR 환경이 다르면 geometry pass/fail 판정을 중지한다.
2. 계획 자체에 대한 서로 다른 실패 관점을 기록하고 지적사항을 이 계획에 반영한다. [계획 검토](../spec/internal/evidence/c07-2-border-width-layout-plan-review-2026-10-10.md)를 마친 뒤 제품 코드에 들어간다.
3. Stylo cascade snapshot에서 폭·스타일의 same-revision 관계, `none/hidden` gate, shorthand winner, CSS px scalar 변환을 구현한다. `calc()`·`var()`가 Stylo에서 계산된 값으로 도착하는지 typed value test를 둔다.
4. `LayoutStyle`에 면별 `LayoutBorder`를 더하고 음수·non-finite 검증 및 Taffy `Style.border` 투영을 구현한다. 기존 C07.1 box sizing, padding, min/max 및 flex 경로에 regression test를 추가한다.
5. inline author style allowlist와 fixture 변환기를 갱신한다. 각면 지정, shorthand 확장, longhand override, style reset, custom property 경로를 확인한다.
6. 기준 50개 node의 border side computed style·정규화 폭과 `x/y/width/height`를 node별 비교한다. 각 frame field의 최대 절대 오차는 `0.5 CSS px` 이하여야 하며, 평균으로 개별 실패를 숨기지 않는다. DPR 1·2의 CSS computed 값과 geometry는 같아야 한다. 레이아웃 전용 profile이 거부하는 paint-only `outline` 규칙은 Rust 비교 입력에서 정확히 한 선언만 제거하고, Chrome reference는 원본 HTML 전체에서 계속 캡처한다.
7. 전체 Rust workspace test, Clippy `-D warnings`, rustfmt, FFI all-features 검사와 CSS reference suite를 실행한다. 제한 profile을 통한 Android API 37 emulator 및 iOS 26.2 Simulator의 V8→Stylo→Taffy→WGPU fixture를 각각 실행하고 원본 로그·화면을 남긴다. Android emulator 결과를 hardware GPU나 실기기 결과로 표현하지 않는다. 실기기 확인은 백로그다.
8. 구현 뒤에는 계획 검토와 겹치지 않는 실패 관점으로 실제 코드·실행 경로를 검토한다. 결함 수정 시 영향을 받은 검사와 fixture를 다시 실행한다. 구현 검토를 마치고 발견 결함과 수정 결과는 [구현 검토 기록](../spec/internal/evidence/c07-2-border-width-layout-implementation-review-2026-10-10.md)에 남긴다.
9. 내부 명세, 오류 예제, `spec/STATUS.md`, `spec/internal/README.md`, evidence index와 Tailscale 미리보기를 동기화한다. 구현이 끝나도 C07 상위는 aspect ratio·기본값 등 잔여 항목 때문에 미완료로 둔다.

## 완료 조건

- 네 면의 Stylo typed computed 값과 winning `border-style`이 한 style revision에서 정확히 사용된다.
- `none`·`hidden`은 0 폭, 다른 유효 style은 폭을 geometry에 반영한다. 색상·outline은 geometry를 바꾸지 않는다.
- CSS shorthand·longhand cascade, `thin/medium/thick`, CSS px, Stylo가 scalar로 계산한 길이 `calc()`·`var()`, zero, invalid negative/percentage fallback이 fixture로 확인된다.
- content-box·border-box, padding보다 작은 지정 border-box, min/max, flex shrink가 기준 결과에 맞는다.
- 50개 node 모두의 지원 computed value와 frame이 판정 기준을 통과하며 DPR 1·2 결과가 동일하다.
- Android와 iOS Simulator runtime fixture는 실제 V8 실행과 WGPU presentation을 증명하되, 개별 simulator child frame의 수치 정확도는 별도 oracle test 결과로만 주장한다.
- 오류·검토·실행 근거가 기록되고, 새 20개 구현 검토에 미해결 결함이 없다. 공개 CSS 지원 완료로 승격하지 않는다.
