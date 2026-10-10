# 0042 · C06.6a 네이티브 viewport 길이 단위

- **문서 ID:** `0042` — 내부 문서의 고정 식별자이며 버전이 아니다.
- **내부 숫자 계약 버전:** 출시 전 `0.1.0` 고정
- **상태:** 현재 작업 브랜치에서 구현·Chromium/Rust 비교·Android/iOS Simulator 검증 완료, 미병합
- **제품 API:** 없음 · **공개 CSS 전체 지원:** 선언하지 않음
- **계획:** [C06.6 viewport 단위 계획](../../plan/c06-6-viewport-units.md)
- **계획 검토:** [실패 경로 검토](evidence/c06-6-viewport-units-plan-review-2026-10-10.md)
- **구현 검토·실행 근거:** [구현 실패 경로 검토와 플랫폼 증거](evidence/c06-viewport-units-implementation-review-2026-10-10.md)
- **Chromium 기준:** [고정 fixture 결과](../../tests/fixtures/css/references/c06-viewport-units-v1.json)

## 적용 범위

이 계약은 현재 네이티브 runtime layout profile에 한정한다. viewport 길이는 `CssViewport.width_css_px`와 `height_css_px`를 기준으로 Stylo cascade에서 CSS px로 계산하고 Taffy의 현재 지원 layout 속성에 전달한다. 화면 drawable의 실제 pixel 수나 Android dp/iOS point를 CSS viewport 값으로 쓰지 않는다. DPR은 raster 변환에만 쓰며 CSS 단위 계산에서 분리한다.

기본 `vw`, `vh`, `vi`, `vb`, `vmin`, `vmax`와 `sv*`, `lv*`, `dv*`의 width·height·inline·block·min·max 계열을 지원 범위로 둔다. 고정 Chromium fixture는 기본 단위, 세 viewport family의 여섯 변형, `calc()` 혼합, `var()` 치환, width·height·flex-basis·margin·padding·gap에 대표값을 넣어 비교한다. 현재 지원 writing mode인 `horizontal-tb`에서 `vi`/`svi`/`lvi`/`dvi`는 width 축, `vb`/`svb`/`lvb`/`dvb`는 height 축이다. 세 viewport family는 CSS 표준 단위 변형을 각각 유지하지만, 현재 native surface에는 브라우저 상단·하단 chrome처럼 접히는 viewport가 없으므로 small, large, dynamic 모두 같은 surface content viewport를 가리킨다.

길이 단위는 현재 runtime profile의 `width`, `height`, `flex-basis`, `margin`, `padding`, `gap` 지원 경계 안에서만 성공한다. viewport 단위가 포함된 `calc()`·`min()`·`max()`·`clamp()`는 기존 C06.5 typed math 범위에 한정한다. `var()` 대체 후 cascade된 Stylo typed value를 사용하며 문자열 재파싱으로 단위 의미를 바꾸지 않는다.

## 입력과 재계산

native host는 surface content bounds의 CSS px 폭·높이를 `CssViewport`에 전달한다. 실제 surface width 또는 height가 바뀌면 기존 environment revision 경로가 cascade/layout을 다시 계산한다. 동일 크기 재전달은 revision을 올리지 않는다. DPR만 바뀌면 CSS computed value와 CSS frame은 유지한다.

각 계산은 하나의 viewport·document·style·environment revision snapshot을 사용한다. 새 요청이 이전 계산보다 앞서면 이전 snapshot은 현재 scene으로 발행할 수 없다. Android에서 surface 재생성은 renderer handle만 교체하고 published scene revision을 추가로 무효화하지 않는다. viewport/style 계산을 바꾸는 요청은 별도 presentation update로 이전 scene을 무효화한 뒤 새 계산 결과를 발행한다.

## 미지원 동작과 오류

| 입력 또는 상태 | 처리 |
| --- | --- |
| `cqw`, `cqh`, `cqi`, `cqb`, `cqmin`, `cqmax` | 성공 fallback을 만들지 않고 `UnsupportedContainerRelativeUnit` 오류로 거부한다. inline style과 author stylesheet를 모두 검사한다. |
| CSS 주석·문자열 안의 단위처럼 보이는 글자 | CSS token 단위로 검사해 실제 dimension token이 아니면 거부 사유로 삼지 않는다. |
| escape된 단위 이름 | CSS tokenization 결과를 기준으로 인식한다. |
| 잘못된 CSS token stream | `InvalidContainerRelativeUnitInput`으로 거부한다. |
| invalid viewport·stale revision·지원 profile 밖 속성/값 | 기존 viewport·cascade·layout 오류 경로를 유지하며 기본값으로 조용히 바꾸지 않는다. |

Container units는 query container 선택·축별 size ownership·query cycle·stale 결과 규칙을 정의하는 C21 이후의 별도 항목이다. safe-area `env()`, visual viewport, 접히는 app chrome, viewport segments, scrollbar, OS keyboard 전용 resize 동작, vertical writing mode, CSSOM 및 외부 CSS 자원 로더는 이 계약에 포함하지 않는다.

## 검증 범위와 한계

Chrome `154.0.8037.98` 고정 실행 파일로 portrait·landscape·추가 viewport 세 가지와 DPR 1·2를 캡처한다. 15개 node의 computed CSS 값은 최대 0.02 CSS px, 각 geometry 축은 최대 0.5 CSS px 허용치로 비교한다. Rust cascade/layout tests는 3개 viewport와 DPR 1·2, environment resize를 확인한다.

Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 동일 V8 fixture의 `layout=ready`, `boxes=15`, `presented` 및 301×100 CSS px에서 341×128 CSS px로 갔다가 돌아오는 surface resize를 확인한다. Android emulator의 wgpu backend는 ANGLE/SwiftShader software path다. 실기기·하드웨어 GPU·성능·브라우저 toolbar·키보드 resize 동작은 검증하지 않았다. 캡처와 원본 로그는 [구현 증거 폴더](evidence/c06-viewport-units-implementation-review-2026-10-10.md)에 연결한다.

문서 ID `0042`, fixture ID `C06.6a`, fixture schema의 `v1`은 release/compatibility version이 아니다. 이 변경은 Spinon crate 또는 내부 숫자 계약 버전을 올리지 않는다. `0.1.0`을 유지한다.
