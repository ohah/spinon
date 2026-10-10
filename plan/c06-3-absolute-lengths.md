# C06.3 · 절대 길이 단위의 CSS px 정규화

- **상위 계획:** [C06 값·단위 변환](c06-value-unit-conversion.md)
- **공식 상태:** [`spec/STATUS.md`의 C06.3](../spec/STATUS.md#css-구현-체크리스트)
- **내부 계약 숫자 버전:** 출시 전 `0.1.0` 고정
- **기준 구현:** Stylo `0.22.0`의 `specified::NoCalcLength::to_px_if_absolute`, `to_computed_value_with_base_size`, `computed::CSSPixelLength::px`
- **브라우저 비교:** Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- **구현 전 공격 검토:** [서로 다른 실패 경로 20개](../spec/internal/evidence/c06-3-absolute-lengths-plan-review-2026-10-10.md)
- **범위 상태:** 현재 작업 브랜치에서 구현·Chromium/Rust 비교·Android/iOS Simulator runtime 검증 완료 · PR 미병합.
- **구현 근거:** [독립 구현 실패 경로 20개와 실행 결과](../spec/internal/evidence/c06-absolute-lengths-implementation-2026-10-10.md).

## 목적과 책임 경계

CSS 절대 길이 `px`, `in`, `cm`, `mm`, `Q`, `pt`, `pc`를 해당 CSS 값 처리 단계에서 CSS px로 정규화해 기존 typed style DTO와 Taffy layout에 전달한다. CSS 기준은 `1in = 96 CSS px`이며 화면의 실제 인치, Android dp, iOS point, drawable pixel 또는 장치 DPI와 혼합하지 않는다.

Stylo가 authored absolute unit을 computed `CSSPixelLength`로 정규화한다. Spinon은 이 값을 다시 단위 변환하지 않고 `length.px()`를 통해 읽어 DTO에 저장한다. 따라서 C06.3의 제품 변경은 이 변환 경계의 명시적 테스트, fixture 기반 지원 계약, 실제 V8 → Stylo → Taffy → WGPU 화면 연결이다. 두 번째 수식 변환기를 추가하지 않는다.

지원 선언은 현재 runtime layout profile의 `width`, `height`, `flex-basis`, physical `margin`, `padding`, `row-gap`, `column-gap`에 한정한다. 새 CSS property allowlist를 추가하지 않는다. `font-size`와 텍스트 측정은 C06.4 및 텍스트 계획의 책임으로 남긴다.

## CSS px 기준 변환

| authored 단위 | CSS px 배율 | 동치 관찰값 |
|---|---:|---:|
| `px` | 1 | `96px` |
| `in` | 96 | `1in = 96px` |
| `cm` | 96 / 2.54 | `2.54cm = 96px` |
| `mm` | 96 / 25.4 | `25.4mm = 96px` |
| `Q` | 96 / 101.6 | `101.6Q = 96px` |
| `pt` | 96 / 72 | `72pt = 96px` |
| `pc` | 96 / 6 | `6pc = 96px` |

값은 기존 CSS floating-point precision과 Stylo/Taffy 표현 범위를 사용한다. 값의 문자열 표기나 원래 authored unit을 layout DTO에 보존하지 않는다. typed computed length와 geometry를 각각 검증해 serialization 결과만으로 단위 의미를 판정하지 않는다.

## 비교 기준과 fixture

고정 [26-node fixture](../tests/fixtures/css/c06/absolute-lengths.html)는 320×800 CSS px, DPR 1, `en-US`, UTC, light, coarse pointer, no-hover 환경에서 캡처한다. 7개 단위를 width·height·flex-basis·margin·padding·gap 입력에 분산하고, `2.54cm` custom-property substitution과 `-4.5pt` signed margin도 포함한다. 마지막 row까지 root 안에서 모두 보이는 높이를 사용한다.

불변 reference [`c06-absolute-lengths-v1.json`](../tests/fixtures/css/references/c06-absolute-lengths-v1.json)은 fixture·inventory·capture script·helper·Chrome executable hash와 browser revision을 묶는다. 캡처 명령은 `node tools/css-reference/capture-c06-absolute-lengths.mjs`다. reference는 자동 재생성으로 덮어쓰지 않는다.

Rust comparison은 26개 노드의 CSSOM observation과 최종 frame을 따로 대조한다. listed computed style 문자열은 고정 Chrome observation과 정확히 맞아야 하고, 각 node의 `x`, `y`, `width`, `height` 편차는 각각 0.5 CSS px 이하여야 한다. typed DTO는 단위별로 `96`, `48`, `6`, `3` CSS px와 음수 margin `-6`을 검증한다. 동일 CSS viewport에서 `device_scale_factor=1`과 `3`의 CSS frame이 같아야 한다.

실제 JavaScript fixture [`runtime-absolute-lengths.js`](../tests/fixtures/css/c06/runtime-absolute-lengths.js)는 26-node 장면을 만든다. Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 V8 evaluation 성공, `layout=ready`, `boxes=26`, WGPU presented 장면을 확인하고 캡처한다. Chrome 320×800 fixture와 모바일 301×100 또는 플랫폼 viewport의 자식 frame이 같다고 주장하지 않는다.

## 명시적 미지원 및 실패 경계

- `em`, `rem`, `ex`, `ch`, `cap`, `ic`, `lh`, `rlh` 등 글꼴 상대 단위는 C06.4에서 처리한다. `font-size`, 폰트 fallback·font metrics·텍스트 shaping을 이 항목의 근거로 주장하지 않는다.
- `vw`, `vh`, `sv*`, `lv*`, `dv*`, container-relative 단위는 C06.6에서 처리하며 absolute unit 변환기로 보내지 않는다.
- `calc()`, `min()`, `max()`, `clamp()`의 표현식 계산은 C06.5 범위다. 단 Stylo가 상수 절대식 `calc(1in)`을 computed `CSSPixelLength`로 이미 해소하면 C06.3은 그 typed 결과를 그대로 소비한다. percentage 기준이 남은 `calc(1in + 10%)` 등 `Unpacked::Calc` 값은 C06.5 전까지 실패 처리한다.
- authored unknown unit은 CSS parser 진단·invalid declaration 규칙을 따른다. 알 수 없는 단위를 px나 0으로 조용히 대체하지 않는다.
- typed 값의 NaN·무한대·DTO 표현 범위 초과, layout frame overflow는 기존 `UnsupportedComputedValue`, `InvalidStyle`, `NonFiniteFrame` 실패 경계를 유지한다.
- Web 공개 API, CSSOM, border/text/position support, screen physical measurement, 전체 CSS 적합성은 추가하지 않는다.

## 구현 순서와 완료 조건

1. 이 계획의 20개 실패 경로 검토를 마치고 지적을 계획에 반영한다.
2. fixed Chrome reference와 hash 검증 테스트를 만들고 단위별 CSSOM·geometry 수치를 고정한다.
3. Rust runtime layout fixture에서 computed `LengthPx`·margin·padding·gap DTO와 Taffy output을 Chrome oracle에 대조한다. 중복 단위 변환기를 만들지 않는다.
4. Stylo가 해소한 `calc(1in)`과 percentage가 남은 `calc(1in + 10%)`, unknown unit, 비유한 내부 값 및 화면 배율 변경의 경계가 조용히 다른 단위로 바뀌지 않는지 검사한다.
5. Android·iOS runtime fixture entrypoint를 추가하고 두 Simulator에서 실제 V8 평가와 GPU 장면을 캡처한다.
6. `cargo fmt --all -- --check`, `cargo test --locked --workspace`, CSS reference test, `cargo clippy --locked --workspace --all-targets -- -D warnings`, FFI check, Android/iOS Simulator build·launch를 실행한다.
7. 기능 구현 이후 계획 검토와 겹치지 않는 코드·fixture·실행 실패 경로 20개를 새로 검토하고, 지적을 고친 뒤 영향 검사를 재실행한다.
8. 내부 계약 문서 ID `0039`와 실행 근거를 작성한다. `0039`는 문서 식별 번호이며 제품·계약 숫자 버전이 아니다. C06.3만 완료 처리하고 C06 상위·C06.4–C06.6은 미완료로 유지한다. 출시 전 숫자 계약 버전은 `0.1.0`으로 고정한다.
