# C07.1 · 물리 축 최소·최대 크기 계획

## 목표

현재 runtime layout profile에 물리 축 `min-width`, `max-width`, `min-height`, `max-height`를 연결한다. 지정 크기와 최소·최대 제약이 Stylo의 computed typed value에서 Rust layout DTO와 Taffy까지 보존되고, 결과 geometry가 고정 Chromium oracle과 일치하는지 검사한다.

이 하위 작업은 CSS 전체 지원을 의미하지 않는다. 출시 전 내부 계약의 숫자 버전은 `0.1.0`으로 고정한다. 구현 항목과 문서 수가 늘어도 제품·계약 숫자 버전을 올리지 않는다.

## 결정된 구현 범위

- 지원 속성: 물리 축 `min-width`, `max-width`, `min-height`, `max-height`.
- 값: Stylo typed `auto`, `none`, CSS px, percentage, 그리고 현재 C06에서 표현·평가 가능한 `calc()`·`min()`·`max()`·`clamp()`·`var()` 결과.
- `min-* : auto`와 `max-* : none`은 내부 레이아웃 제약 없음 값으로 표현한다. 값 자체는 property 의미와 computed-style 문자열에서 계속 구분한다.
- 값은 nonnegative finite여야 한다. CSS parser가 무효 선언을 cascade에서 버리는 동작과, layout adapter가 표현할 수 없는 computed value를 오류로 거부하는 동작을 구분한다.
- containing block이 definite이면 percentage를 Taffy percentage 값으로 전달한다. indefinite basis 및 calc resolution 결과는 C06의 기존 정책을 적용하고, 새 경로가 이를 우회하지 않는다.
- 기존 `box-sizing`과 padding을 이용해 content-box·border-box 최소/최대 의미를 함께 계산한다. border 두께·border paint는 입력 fixture에서 사용하지 않는다.
- Flex shrink/grow가 min/max 경계에 도달하는 사례를 포함한다. `min-width:auto`는 빈 요소 fixture에서만 기준을 비교한다.
- runtime author inline allowlist와 Stylo property registration을 모두 확장한다. shorthand와 cascade는 Stylo가 만든 winning longhand를 사용한다.
- 기존 지원 profile에 영향을 주지 않도록 새 속성은 `RuntimeFlexLayoutV1` 계열 중 실제 runtime layout 경로에만 허용한다. profile 복제·custom property·registered property 경로에도 같은 property surface인지 확인한다.

## 명시적 제외

`min-content`, `max-content`, `fit-content`, `stretch`, anchor sizing, 논리 축 `min-inline-size`·`max-block-size`, replaced element intrinsic dimensions, 텍스트 최소 콘텐츠 크기, aspect ratio, border 두께·paint, overflow clipping, Grid intrinsic track sizing, 외부 CSS URL, 공개 CSSOM/getComputedStyle, 제품의 전체 CSS 호환 표시는 포함하지 않는다. 제외된 값을 문자열 fallback이나 0으로 조용히 바꾸지 않는다.

## 인터페이스 변경안

1. `ComputedLayoutDimensions`에 네 제약 값의 Stylo typed representation을 더한다. `min-*`의 `auto`와 `max-*`의 `none`을 기존 width `auto`와 혼동하지 않도록 내부 enum 또는 property-specific 변환을 둔다.
2. `ComputedCssMath` 수집과 cascade winner 추출에 네 속성을 추가한다. CSS 문자열을 새로 파싱하지 않는다.
3. 내부 `LayoutStyle`에 네 물리 축 제약 필드를 추가하고 Taffy `Style.min_size`·`max_size`로 투영한다. CSS math property ID도 출처 속성에 맞게 추가해 오류 진단을 보존한다.
4. style projection은 `RuntimeFlexLayoutV1`과 실제 사용 중인 runtime 파생 profile에서만 값을 수용한다. unsupported keyword·non-finite·negative DTO는 node와 property를 포함한 오류로 실패한다.
5. 기존 JSON fixtures와 테스트 fixture 변환기는 기본 `auto`/`none` 값과 네 제약 입력을 명확히 표현한다. 공개 API나 패키지 숫자 버전은 바꾸지 않는다.

## 구현 순서

1. [구현 전 비교 모델](../spec/internal/evidence/c07-1-min-max-sizing-precomparison-2026-10-10.md)과 [고정 Chromium reference](../tests/fixtures/css/references/c07-min-max-sizing-v1.json)를 유지한다. 입력 HTML·inventory·캡처기 hash가 맞지 않으면 수치 비교를 중단한다.
2. 계획 자체의 20개 실패 경로 검토를 완료하고, 지적된 누락을 이 계획에 반영한다.
3. Stylo snapshot DTO와 `ComputedValues` 변환을 구현한다. `auto`, `none`, px, percentage, calc AST 및 미지원 intrinsic/anchor 키워드를 각각 검사한다.
4. Rust layout DTO와 Taffy projection을 구현한다. min>max precedence, box-sizing+padding, zero, percent basis, flex shrink/grow, calc finite/error paths를 단위 테스트한다.
5. inline property allowlist, shorthand cascade, CSS custom property 및 registered property 경로에 속성 허용을 일관되게 추가한다. unsupported declaration이 조용히 사라지지 않는지 확인한다.
6. 고정 Chromium 결과와 HostDocument/Taffy 결과를 노드별로 비교한다. frame 네 필드 각각 최대 `0.5 CSS px`, DPR 간 CSS px 동일성을 통과해야 한다. 지원 범위 밖 reference는 pass 분모에 넣지 않고 이유를 별도 표시한다.
7. 기존 runtime app route에 필요하면 최소 fixture surface를 추가해 Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 V8→Stylo→Taffy→wgpu 표시를 확인한다. 시뮬레이터는 통합 실행만 증명하며, 실기기·hardware GPU·성능 증거로 보고하지 않는다.
8. 변경 코드와 실행 증거를 계획 표와 다른 20개 구현 실패 관점으로 검토한다. 발견된 결함을 수정하고 영향받은 검토를 다시 실행한다.
9. 내부 인터페이스 명세, API/오류 예제, `spec/STATUS.md`, 내부 evidence 인덱스, 지정된 Tailscale 미리보기를 같은 작업에서 동기화한다. 공개 지원 완료 체크는 명세와 실제 근거를 확인한 뒤 판정한다.

## 완료 판정

- Stylo typed computed 값이 네 속성 모두에서 출처와 값 의미를 보존한다.
- Taffy `min_size`·`max_size` 경로가 고정된 값과 CSS math 값을 모두 받으며 invalid/unsupported 값을 fail-closed 한다.
- fixture의 모든 지원 노드에서 Chrome CSS px frame 최대 절대 오차가 네 필드 각각 `0.5 CSS px` 이내다. 값·geometry는 DPR 1과 2에서 동일하다.
- `auto`/`none`, zero, invalid negative declaration, percentage basis, min>max, content-/border-box+padding, flex shrink/grow, calc/cascade, unsupported keyword 경계가 서로 분리된 결과를 갖는다.
- Rust workspace 테스트, Clippy, rustfmt, FFI all-features check와 필요한 Android/iOS Simulator 실행이 실제로 끝나고 원본 로그/화면이 남는다.
- 20개 구현 공격 관점에 미해결 결함이 없다. 잔여 범위는 미지원으로 문서화하며 C07 상위는 미완료로 유지한다.

## 구현 결과 · 2026-10-10

- 제한 runtime profile에서 Stylo typed `min-width`, `max-width`, `min-height`, `max-height`를 Rust layout DTO와 Taffy에 연결했다. 출시 전 crate·내부 계약 숫자 버전은 `0.1.0`으로 유지했다.
- pinned Chrome 154 fixture 35개 노드의 frame `x/y/width/height`를 DPR 1·2에서 비교했고 최대 절대 오차는 양쪽 모두 `0 CSS px`였다.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 C07.1 전용 V8 fixture를 WGPU 화면으로 표시했다. 양쪽 로그 모두 `status=0`, `layout=ready`, `boxes=10`이다. 이 실행은 mobile child frame 수치 비교, 실기기 또는 hardware GPU 성능 검증이 아니다.
- [구현 실패 경로·실행 근거](../spec/internal/evidence/c07-1-min-max-sizing-implementation-review-2026-10-10.md)에 별도 20개 점검, 수정 결함, 명령별 결과와 화면·로그를 남겼다.
- 전체 C07 box model, intrinsic keyword/text sizing, logical size axis, aspect ratio, border paint, 외부 stylesheet와 공개 CSS 지원 선언은 후속 범위다.
