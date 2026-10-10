# C06.6a 네이티브 viewport 길이 단위

## 목적과 상태

- **상태:** 구현·비교·Simulator 검증 완료 · 현재 작업 브랜치 미병합
- **범위:** 네이티브 런타임 CSS cascade에서 viewport-relative length를 실제 CSS px로 계산하고 지원하는 Taffy layout property에 전달한다.
- **기준 구현:** 저장소가 고정한 Chromium `154.0.8037.98`과 Stylo `0.22.0`의 실제 source/runtime 동작.
- **숫자 버전:** 출시 전 내부 계약과 Spinon crate 버전은 `0.1.0` 유지. 이후 문서 ID는 순번일 뿐 버전 숫자가 아니다.

## 구현 완료 상태

- Stylo viewport 단위 계산을 C06.5 typed math 및 현재 runtime layout property profile에 연결했다. 기본·small·large·dynamic 계열의 `w`, `h`, `i`, `b`, `min`, `max` 변형을 fixture와 함께 검사한다.
- container-relative units는 inline style과 author stylesheet에서 CSS token 단위로 찾아 C21 전 명시적 오류로 차단한다. 문자열·주석 오검출, nested function, escaped unit, malformed token 경계를 테스트한다.
- 고정 Chromium fixture는 세 viewport × DPR 1·2, 15 nodes를 보존한다. runtime V8 fixture는 Android·iOS host에서 실행한다.
- renderer 교체가 최신 published scene을 다시 무효화하던 Android resize 경합과 iOS 실행 argument routing 누락을 수정했다. 양 플랫폼에서 301×100 ↔ 341×128 resize 뒤 최신 `boxes=15` scene 표시를 확인했다.
- [내부 계약 문서 0042](../spec/internal/0042-c06-viewport-units.md), [구현 실패 경로 검토·실행 근거](../spec/internal/evidence/c06-viewport-units-implementation-review-2026-10-10.md), 상태 대장을 연결한다. 숫자 계약과 crate 버전은 `0.1.0` 그대로다.
- container units, 접히는 UA chrome, visual viewport, keyboard/safe-area 정책은 미구현이다. C06 상위 항목은 계속 미완료다.

CSS Values and Units Level 4는 기본 `vw`/`vh`/`vi`/`vb`/`vmin`/`vmax`를 large viewport 기준으로 정의하고, `sv*`, `lv*`, `dv*` 계열에 각각 small, large, dynamic viewport 기준을 둔다. 세 크기는 같을 수도 있다. 현재 Spinon 네이티브 surface에는 브라우저 주소창처럼 펼쳐졌다 접히는 UA chrome이 없다. 따라서 이 작업의 호스트 계약은 **surface content viewport 하나를 CSS px로 전달하고, small/large/dynamic viewport를 모두 그 크기로 정의**한다. 방향 전환·창 크기 변경 등으로 surface content viewport가 실제로 바뀌면 세 종류가 함께 바뀐다. OS 키보드가 surface를 overlay하면 viewport는 유지하고, OS가 surface 자체를 resize한 경우에만 새 surface 크기를 전달한다. 이는 keyboard resize 정책을 새로 만들지 않고 현재 native surface 경계를 따른다.

Stylo `0.22.0`의 Servo `Device` 구현은 `ViewportVariant`를 무시하고 단일 viewport를 사용한다. 이는 위의 현재 호스트 계약과 일치한다. 이 작업은 Stylo fork/vendor를 추가하지 않는다. 추후 접히는 app chrome을 도입할 경우, 서로 다른 small/large/dynamic 크기를 요구하기 전에 해당 호스트 환경 입력과 Stylo adapter를 별도 설계한다.

## CSS 지원 범위

현재 지원 layout profile에서 길이로 받는 속성에 아래 단위를 허용한다.

| 단위 집합 | 계산 기준 | 허용 경계 |
| --- | --- | --- |
| `vw`, `vh`, `vmin`, `vmax` | large viewport. 현재 native profile에선 surface viewport | CSS Values의 default-unit 정의를 따른다. |
| `svw`, `svh`, `svmin`, `svmax` | small viewport | 현재 호스트에선 surface viewport와 동일하다. |
| `lvw`, `lvh`, `lvmin`, `lvmax` | large viewport | 현재 호스트에선 surface viewport와 동일하다. |
| `dvw`, `dvh`, `dvmin`, `dvmax` | dynamic viewport | 현재 호스트에선 surface viewport와 동일하며 surface resize revision으로 바뀐다. |
| `vi`, `svi`, `lvi`, `dvi` | 해당 box writing mode의 inline axis | 현재 지원 writing mode가 `horizontal-tb`인 선언만 성공 경로다. |
| `vb`, `svb`, `lvb`, `dvb` | 해당 box writing mode의 block axis | 현재 지원 writing mode가 `horizontal-tb`인 선언만 성공 경로다. |

- unit은 cascade 중 Stylo typed value에서 CSS px로 해석한다. DPR을 CSS viewport dimension에 곱하거나 viewport 단위를 device pixel로 취급하지 않는다.
- 기존 C06.5 typed math에서 허용한 `calc()`·`min()`·`max()`·`clamp()` 안의 viewport 단위는 각 함수 인자 단위 의미를 보존해 적용한다. 지원하지 않는 속성·함수·값은 기존 fail-closed 계약을 따른다.
- `var()` 치환 결과, custom property 상속, shorthand/longhand cascade에서 viewport 단위가 유실되거나 source 문자열 재해석으로 계산 결과가 달라지지 않도록 한다.
- viewport 단위가 연결되지 않은 문서, detached node, hidden node에서 layout 결과로 노출되지 않는 값은 기존 DOM/cascade 경계를 따른다.
- 모든 `cqw`, `cqh`, `cqi`, `cqb`, `cqmin`, `cqmax`는 이 하위 항목에서 미지원이다. Stylo 기본 fallback이 값을 만들어내더라도 성공으로 노출하지 않고 `unsupported_container_unit` 계열의 명시적 오류로 닫는다. container 단위는 C21이 정의할 query-container owner, 축별 eligible 기준, 사용 크기 전달, 순환·stale 결과 정책이 준비된 뒤 별도 구현한다.
- safe-area `env()` 값, viewport segments, scrollbars, visual viewport API, WebView UI chrome, viewport-unit media query 재계산은 지원 범위가 아니다.

## 런타임·revision 계약

- 유일한 viewport source는 현재 `RuntimeCssEnvironment`에 전달되는 native surface content bounds의 CSS px 크기다. WGPU drawable pixel 크기와 CSS px 크기를 혼동하지 않는다.
- width/height 입력, DPR, media environment는 기존 `CssViewport`와 environment revision 정책을 유지한다. viewport unit은 기존 width/height로부터 계산하며 별도의 viewport variant 입력이나 계약 버전 변경을 만들지 않는다.
- width 또는 height가 이전과 다르면 기존 환경 변경 경로가 revision을 한 번 증가시켜 cascade/layout 최신 결과를 다시 만든다. 같은 값의 재전달은 revision을 증가시키지 않는다. revision exhaustion·invalid finite/range 입력은 기존 오류 정책을 유지한다.
- resize가 연속해서 들어오면 각 계산은 immutable viewport/revision snapshot을 사용한다. 더 최신 revision이 이미 등록된 뒤 늦게 끝난 이전 계산 결과는 commit/render할 수 없다.
- DPI 변화만으로 CSS viewport units의 길이는 바뀌지 않는다. DPR은 CSS-to-device rasterization 입력이며, viewport CSS-pixel length 계산과 분리한다.

## 구현 전 비교 모델

- Chromium `154.0.8037.98`, 고정한 browser binary hash/revision, fixture hash, 실행 도구 hash 및 CSS viewport metadata를 evidence에 보존한다.
- Desktop/headless Chromium의 viewport units, CSS Typed/Computed style, element geometry를 기준으로 삼는다. 기본 case는 `320×800 CSS px`, landscape `800×320 CSS px`, orientation/resize 변경 뒤 다시 측정한다. 모바일 profile과 같은 light/coarse-pointer 환경은 기존 fixture metadata를 명시한다.
- DPR `1`과 `2`에서 computed CSS values와 CSS frame이 동일하고 raster pixel 크기만 DPR에 따라 달라지는 negative control을 둔다.
- reference에는 단위별 1%, 25%, 100% probe를 넣는다. `vmin`/`vmax`, inline/block axis, `calc()` 혼합, `var()` 대체값, child inheritance, Flex width/height/flex-basis 및 spacing의 현재 지원 경로를 나눈다.
- 각 지원 node의 computed value 및 `x`, `y`, `width`, `height`를 개별 비교한다. 축별 최대 절대 frame 오차는 `0.5 CSS px` 이하이며 평균값으로 개별 node 오류를 감추지 않는다.
- viewport 값이 하나인 Chromium/headless host에선 small/large/dynamic 결과가 같은 것이 기준이다. 실제 브라우저 chrome의 수축/확장 동작을 검증했다고 주장하지 않는다.
- 미지원 container unit, vertical writing mode, zero/negative/non-finite viewport, unsupported property, unsupported CSS math, layout revision 불일치는 명시 오류가 기준이다. 거절 case를 성공 개수에 포함하지 않는다.

## 구현 순서와 완료 관문

1. [완료] 계획을 CSS Values 4·CSS Containment 3, Stylo `0.22.0` source, pinned Chromium 환경에 대조해 별도 계획 검토 근거로 기록했다.
2. [완료] 고정 Chromium capture fixture/reference와 도구 metadata를 저장하고 C06.1–C06.5 reference 회귀를 확인했다.
3. [완료] Stylo의 typed value와 C06.5 typed math 연결을 확인했다. viewport 값을 CSS px로 변환하고 DPR과 분리했다.
4. [완료] 지원되는 CSS→layout property만 통과시키고 container units를 명시적으로 거절한다. CSSOM 또는 일반 CSS 지원을 추가하지 않았다.
5. [완료] environment resize, DPR 불변성, stale result 차단을 Rust fixture에서 검사했다. 양 플랫폼의 301×100 ↔ 341×128 resize도 실행했다.
6. [완료] Rust workspace tests·Clippy·rustfmt·FFI all-features·CSS reference verifier와 Android API 37 emulator/iPhone 17 Pro iOS 26.2 Simulator 빌드·실행을 확인했다. 실기기·하드웨어 GPU 주장은 하지 않는다.
7. [완료] 구현 코드를 계획 검토와 겹치지 않는 별도 실패 관점 검토에 기록했다. Android resize 장면 무효화와 iOS 실행 argument 누락을 수정하고 해당 경로를 다시 실행했다.
8. [완료] [내부 계약 0042](../spec/internal/0042-c06-viewport-units.md)에 단위, revision, 오류, unsupported 경계, 플랫폼 한계와 실행 근거를 기록했다. `0042`는 문서 ID이며 숫자 계약/크레이트 버전은 `0.1.0`이다.
9. [완료] 상태 대장에 C06.6a만 연결한다. C06 상위와 C06.6b container-unit 하위는 미완료로 유지한다.

## 의도적으로 제외한 후속 항목

- container-relative length: C21 `@container` query-container size ownership와 cycle/stale policy 확정 뒤 별도 계획·fixture·20회 계획 검토로 진행한다. 단위 fallback은 nearest eligible container가 없을 때 small viewport이므로, C21 전에는 `cqw` 등을 viewport 값으로 잘못 대체해서는 안 된다.
- non-equal small/large/dynamic viewport size: native runtime에 접히는 UA chrome이 생기면 API 입력, environment/cache key, cascade invalidation 및 platform source를 새로 계약한 뒤 도입한다.
- vertical writing modes, root layout/style behavior beyond the current profile, scrollbars, WebView chrome, safe area, keyboard-specific resize policy, visual viewport.

## 근거

- [CSS Values and Units Level 4 §6.1.2](https://www.w3.org/TR/css-values-4/#viewport-relative-lengths) — default/large/small/dynamic viewport 정의와 축별 단위.
- [CSS Containment Level 3 §6](https://www.w3.org/TR/css-contain-3/#container-lengths) — 축별 가장 가까운 eligible container와 container 부재 시 small viewport fallback.
- 저장소 고정 의존성: `Cargo.lock`의 Stylo `0.22.0`, Taffy `0.14.0`; source 확인은 `style::device::servo::Device::au_viewport_size_for_viewport_unit_resolution`의 variant 처리와 `style::values::specified::length`의 unit 변환을 대상으로 한다.
