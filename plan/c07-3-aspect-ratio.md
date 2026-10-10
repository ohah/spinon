# C07.3 · 종횡비 계획

## 목표

현재 제한 runtime layout profile에서 CSS `aspect-ratio`의 초기값·명시적 `auto`와 양수 finite `<ratio>`를 Stylo의 typed computed value에서 `LayoutStyle`과 Taffy 0.14.0으로 전달한다. pinned Chromium 154와 Block·Flex 결과를 비교한다. 종횡비와 non-default `min/max-*`가 함께 오면 Taffy 0.14의 제약 전이가 Chromium과 달라 현재 slice에서는 node/property를 포함해 fail closed한다. Android·iOS Simulator의 V8→Stylo→Taffy→WGPU fixture가 표시되는지 확인한다.

이 작업은 CSS 전체 또는 공개 CSS 지원 완료가 아니다. 앱 릴리스 버전, Spinon crate 버전, 내부 계약 숫자 버전은 모두 `0.1.0`으로 유지한다. `0045`는 명세 문서 ID일 뿐 버전이 아니다.

## 구현 전 기준과 범위

- 기준은 Chromium `154.0.8037.98`, 고정 revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`이다. Chrome은 이번 고정 비교 oracle이지 CSS 표준 전체의 완전한 대체물이 아니다. 현재 CSS Sizing Level 4 문서는 Editor’s Draft이므로 규범 의미를 참고하되 초안 상태를 기록한다.
- 기존 `RUNTIME_FLEX_LAYOUT_PROPERTIES`와 author allowlist로 제한되는 모바일 runtime layout profile만 확장한다. 지원 요소는 현재 그 profile에서 layout을 수행하는 block/flex item 범위로 한정한다. text·replaced element·table·Grid·inline formatting은 이 작업으로 지원하지 않는다.
- 초기값과 `aspect-ratio:auto`는 선호 비율이 없는 상태로 표현하고 frame에 영향이 없음을 확인한다. 속성은 상속되지 않는다.
- bare 양수 finite `<ratio>`를 `width / height`인 `f32`로 변환한다. 구문 기본 분모 1, ratio가 적용되는 auto 크기, 지정 크기 양쪽 모두 definite인 경우, Block 및 Flex row/column을 검증한다. 다섯 개 min/max 조합의 Chrome 결과는 reference에 남기되, positive geometry 경로에는 넣지 않고 해당 조합을 fail closed하는 negative runtime 시험으로 확인한다. Taffy와 Chrome의 제약 전이가 일치할 때까지 지원하지 않는다.
- ratio의 분자 또는 분모가 0인 CSS degenerate ratio는 CSS의 auto 동작처럼 preferred ratio가 없는 것으로 처리한다. 계산된 비율이 0·음수·NaN·무한대가 되거나 유한한 CSS 입력이 `f32`에서 유한한 양수로 표현되지 않으면 node와 속성을 포함해 전체 layout 계산을 거부한다. 잘못된 부분 frame을 반환하지 않는다.
- `auto <ratio>`는 유효 CSS지만 bare ratio와 다른 content-box 의미를 가진다. Taffy의 `Option<f32>`만으로 두 계약을 보존할 수 없으므로 이 범위에서는 성공 경로로 받아들이지 않는다. 해당 값은 node/property가 보이는 unsupported 오류로 거부하고 bare ratio로 조용히 축약하지 않는다.
- `calc()`와 `var()`는 Stylo가 typed computed ratio로 resolve한 값만 다룬다. 원문 문자열을 다시 파싱하지 않는다. cascade에서 무효 선언을 탈락시키고 이전 유효 winner가 보존되는지 확인한다. 미등록 custom property의 C05 계약을 확장하거나 CSSOM을 새로 만들지는 않는다.
- `box-sizing`, width/height, padding, C07.2 border와 조합한다. definite width+height에서는 ratio가 그 크기를 덮어쓰지 않는지 확인한다. 현재 Taffy leaf 경로가 두 definite 크기에도 ratio를 최소 높이처럼 다시 적용하므로, leaf 양 축이 definite면 해당 layout 호출에서 Taffy ratio를 끄는 회귀 수정과 독립 검사를 둔다. non-default min/max와 ratio 조합은 layout 전에 거부한다. percentage 및 viewport basis는 기존 계약을 사용하며 새 단위 의미를 추가하지 않는다.
- `display:none`, 빈 block의 기본 높이, 자식에게 상속되지 않음, style revision 변경 후 stale ratio 재사용 방지, 반복 재계산 cache 무효화와 invalidation을 확인한다.

## 명세와 내부 자료 흐름

1. Stylo computed `GenericAspectRatio`에서 `auto` flag 및 `PreferredRatio`를 같은 cascade revision으로 읽는다. 문자열 직렬화를 입력 계약으로 사용하지 않는다.
2. 내부 `ComputedElementStyle`은 auto/degenerate는 `None`, 유효 bare ratio는 finite 양수 `Some(width_over_height)`로 소유한다. `auto`와 ratio가 함께 온 값은 명시적 unsupported error다.
3. `spinon-style-to-layout`은 지원 runtime profile에서만 typed 값을 전달한다. 다른 profile의 기존 default는 유지한다. 새 CSS 속성이 inline/stylesheet allowlist 및 profile 검사에 모두 연결되어야 한다.
4. `LayoutStyle.aspect_ratio: Option<f32>`를 추가하고 `LayoutStyle::validate`에서 `Some`의 finite 양수 불변 조건을 보장한다. 오류는 기존 `LayoutError` 경로로 node를 포함해 반환한다.
5. Taffy adapter는 검증된 값을 그대로 `Style.aspect_ratio`에 전달한다. aspect ratio를 별도로 계산해 width/height에 합산하지 않는다.

## 비교 입력과 판정

- 전용 HTML·inventory·고정 Chrome reference를 작성한다. node ID의 누락·중복, HTML·inventory·capture 도구의 digest, Chrome product/revision, viewport, locale, DPR 1·2 환경이 일치하지 않으면 비교를 무효 처리한다.
- 각 node의 `aspect-ratio`, `box-sizing`, width/height/min/max, padding/border와 `x/y/width/height`를 Chromium에서 기록한다. CSS computed value 대조는 Typed OM `computedStyleMap()`의 `aspect-ratio`·`box-sizing`만 Stylo computed snapshot과 비교한다. Chrome `getComputedStyle(width/height/...)`의 resolved px 문자열을 Stylo computed 문자열과 비교하지 않는다. 프레임은 used geometry oracle이며 지원 node·field별로 판정한다.
- 합성 fixture는 초기값·auto·1/1·4/3·16/9·정수 생략 분모·fractional ratio·`calc()`·width auto/height auto·두 definite size·box-sizing·padding/border·Flex 기본 stretch 및 start 정렬·row/column·`var()`·invalid declaration fallback·degenerate zero·inheritance·`display:none`·ratio 변경을 분리한다. min/max transfer 다섯 사례는 Chromium 관찰값으로 남기고 별도 negative 시험에서 거부를 확인한다. positive fixture의 동일 위치에는 expected-size spacer style을 사용해 뒤쪽 node의 y 비교가 빠지지 않게 하며 spacer geometry는 지원 근거로 세지 않는다.
- unsupported `auto <ratio>`와 표현 불가 범위는 positive reference 계산에 섞지 않고 독립 실패 fixture로 정확한 오류·무부분 결과를 확인한다.
- 두 DPR의 CSS pixel computed 값과 frame이 같아야 한다. 지원 프레임의 node별 각 축 최대 절대 오차는 `0.5 CSS px` 이하다. Chrome capture 실패·reference digest 불일치·fixture node 누락은 성공 판정이 아니다.
- 시뮬레이터에서는 실제 V8 fixture의 `layout=ready`, 예상 box 수, WGPU `presented`, 화면 및 로그를 확인한다. child별 geometry oracle은 Chromium/Rust fixture test의 별도 결과로만 주장한다. Android emulator software renderer를 hardware GPU나 실기기로 표현하지 않는다.

## 구현 순서

1. Chrome 고정 fixture와 비교 기준을 만들고 원본 hash 및 사전 관찰을 기록한다.
2. 이 계획을 기능 코드와 별도로 서로 다른 20개 실패 관점에서 공격 검토하고 발견 문제를 반영한다.
3. 내부 계약 `0045`와 외부 공개가 아닌 상태를 `spec/STATUS.md`, `spec/internal/README.md` 및 문서 preview에 연결한다. 숫자 버전은 올리지 않는다.
4. Stylo typed aspect-ratio 추출과 property/profile allowlist를 추가하고 auto, bare ratio, degenerate ratio, combined auto-ratio, min/max constraint rejection, non-finite 및 cascade fallback을 검사한다.
5. layout DTO와 Taffy projection, 오류·부정 입력 시험을 추가한다. 기존 C07.1/C07.2, C06 unit/math 및 Flex regression을 보존한다.
6. Chrome reference의 전체 지원 node/frame을 비교하고, 캐시·revision·반복 style update 및 미지원 실패 경계를 검증한다.
7. Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 별도 V8→WGPU 실행·화면·로그를 남긴다.
8. 기능 구현 뒤 실제 변경 코드와 실패 입력을 대상으로 새 20개 adversarial review를 수행한다. 계획 리뷰 내용을 재사용하지 않는다. 수정 후 영향받은 시험을 다시 실행한다.
9. 명세·상태 대장·내부 인덱스·reference·evidence·Tailscale preview source를 함께 동기화한다. GitHub Pages/RSPress 배포는 하지 않는다.

## 완료 조건과 제외

- bare supported ratio, auto/degenerate default, typed cascade/fallback, definite-size leaf 보정, min/max fail-closed 오류 계약이 문서·코드·시험에서 일치한다.
- Chromium 고정 reference에서 모든 supported node/frame과 DPR 1·2 비교가 기준 안에 든다. 각 실패 노드는 원인을 해결하거나 지원 범위를 좁혀 문서화한다.
- Android/iOS Simulator 모두 같은 수직 fixture를 V8과 WGPU로 실행해 정상 표시한다.
- 계획 검토와 별개인 구현 실패 경로 검토, 회귀 검사, 화면·로그 근거를 남긴다.
- `auto <ratio>`, replaced element의 intrinsic ratio, SVG·이미지·비디오 intrinsic sizing, text intrinsic sizing, Grid, table, inline, CSS animation/transition, `object-fit`, aspect-ratio 전반의 공개 지원 선언은 제외한다. 구현 slice를 끝내도 C07 상위는 남은 항목과 기본값 적합성 검증이 닫히기 전까지 미완료다.

## 현재 브랜치 결과

- 30 supported node의 Chrome 154 geometry 비교, DPR 1·2와 전체 Rust/CSS reference 회귀 검사를 통과했다. 다섯 min/max+ratio 조합은 supported result로 세지 않고 명시적으로 fail closed한다.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 V8 fixture를 실행해 세 ratio 사각형, `layout=ready`, WGPU 5-box 제출을 확인했다. 자식별 frame의 Chrome 대조는 고정 Rust fixture 결과이며 simulator가 별도 수치 계측한 것은 아니다.
- 계획 수정 범위는 [수정 계획 재검토](../spec/internal/evidence/c07-3-aspect-ratio-plan-review-scope-followup-2026-10-10.md), 코드·플랫폼 실패 경로는 [구현 검토](../spec/internal/evidence/c07-3-aspect-ratio-implementation-review-2026-10-10.md), 차이와 실행 근거는 [Taffy·시뮬레이터 evidence](../spec/internal/evidence/c07-3-taffy-differential-2026-10-10.md)에 기록했다.
- 구현·검증은 끝났으며 [PR #104](https://github.com/ohah/spinon/pull/104)가 2026-10-10에 rebase merge로 병합됐다. merge commit은 `286d11f79ee288b7cca2e2f1799b1c4634e6d372`다. 내부 계약·crate 숫자 버전은 `0.1.0`이다.
