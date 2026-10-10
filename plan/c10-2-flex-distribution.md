# C10.2 · Flex 크기 배분과 min/max freeze 계획

**상태:** 구현·검증·실패 경로 검토 완료(작업 브랜치) · PR 생성 준비 완료 · 공식 병합 전

**상위 항목:** [C10 Flexbox](./c10-flexbox.md) · [공식 상태 대장](../spec/STATUS.md)

**선행:** C10.1 `flex-wrap`은 [PR #114](https://github.com/ohah/spinon/pull/114)로 병합했다.

**계획 실패 경로 검토:** [서로 다른 20개 관점](../spec/internal/evidence/c10-2-flex-distribution-plan-review-2026-10-10.md)

**내부 계약 숫자 버전:** 출시 전 `0.1.0` 고정. 이 계획 PR은 계약 문서 ID를 새로 발급하거나 버전을 올리지 않는다.

## 목표

각 Flex line에 모인 항목의 최종 주축 크기가 CSS Flexbox 알고리즘대로 계산되도록 검증하고, 필요한 구현 차이를 고친다. 현재 코드에는 `flex-basis`, `flex-grow`, `flex-shrink`, 물리 min/max 값의 computed-style 추출과 Taffy 변환 경로가 이미 있다. 그 매핑이 존재한다는 사실만으로 알고리즘 호환을 가정하지 않는다. 고정 Chromium을 최종 수치 oracle로 삼고 Taffy 0.14.0은 후보 계산기로 취급한다.

계약 기준은 flex base size와 hypothetical main size를 구분하고, line의 hypothetical outer main size 합으로 grow/shrink 알고리즘을 고르며, 초기 free space를 계산하고, inflexible item을 freeze한 뒤 남은 요소에 공간을 배분하는 것이다. 양수 여유 공간은 grow factor에 따라, 음수 여유 공간은 `flex-shrink × inner flex base size`인 scaled shrink factor에 따라 분배한다. min/max clamp 뒤 위반 부호에 맞는 요소를 freeze하고 재분배한다. 합이 1보다 작은 grow factor의 부분 채움 규칙과 남는 공간도 관찰한다.

## 성공 범위

- 기존 `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1`의 동일한 typed style projection을 유지한다. 앱 기본 경로인 `RuntimeFlexCustomPropertiesPaintV1`을 포함한다.
- C10.1의 DOM-order line collection 결과에 대해 **line마다 독립적으로** 배분한다. 각 case의 container 주축 크기는 definite하고, writing mode는 `horizontal-tb`, direction은 `ltr`, 방향은 `row|column`이다.
- `flex-basis`는 유한한 px 길이, definite main-size container에서의 %, 그리고 definite `width`/`height`를 가진 item의 `auto`를 검증한다. `flex-basis`는 해당 item의 width/height와 충돌할 때 basis가 주축 크기를 결정하는 경우를 확인한다.
- CSS `flex` shorthand의 대표 입력 `1`, `auto`, `none`, 세 성분 형식과 뒤따르는 longhand override를 비교한다. Stylo가 확장한 computed `flex-grow`, `flex-shrink`, `flex-basis`를 typed 입력의 source of truth로 사용한다.
- grow factor 0, 1보다 작은 합, 1 이상 합, 서로 다른 비율을 확인한다. shrink factor 0, 서로 다른 factor와 서로 다른 basis의 scaled shrink를 확인한다.
- 명시 `min-width`/`max-width` 또는 `min-height`/`max-height`가 분배 결과를 제한할 때 추가 freeze·재분배가 반복되는지 확인한다. min이 max보다 큰 입력에서 CSS의 min 우선 clamp 동작도 별도 case로 고정한다.
- 주축 gap과 고정 비자동 margin이 free space에서 빠지고, cross-axis 값이 주축 배분에 섞이지 않는지 확인한다. 자동 margin, `justify-content`의 비기본 분배는 이 단계에서 사용하지 않는다.
- `nowrap` line과 C10.1 `wrap` line 모두에서 해당 line에만 free space를 배분한다. wrapped container의 남는 공간을 다른 line과 합산하지 않는다.
- container와 item은 비텍스트 빈 box로 만들어 text/replaced intrinsic measurement를 사용하지 않는다. 지원되는 `align-items:flex-start`, `justify-content:flex-start`와 0 cross-axis gap을 고정한다. `align-content`는 현재 runtime profile에서 요청하지 않고, 각 container의 cross size를 line 크기 합과 같게 정해 기본 `stretch`가 남는 공간을 분배하지 않도록 한다. 별도 gap case만 주축 gap을 바꾼다.
- 기존 cascade와 revision 계약을 유지한다. style 변경으로 grow/basis/min/max가 바뀌면 새 계산값만 현재 scene으로 공개하고, 계산 실패 때 이전 frame을 새 revision 성공 결과로 취급하지 않는다.

## 이번 단계 밖

- `flex-basis: content`, `auto` main size에서 content 기반 basis 산정, indefinite container에 대한 percentage basis resolution은 intrinsic sizing을 요구하므로 C10.4/C14 이후로 둔다.
- 자동 최소 크기 `min-width:auto`/`min-height:auto`, text·replaced item, aspect-ratio가 basis에 미치는 상호작용은 이 계획의 범위가 아니다. 명시 min/max 제약은 C10.2에서 다루되 자동 최소 크기 규칙을 완료 처리하지 않는다.
- `row-reverse`, `column-reverse`, `wrap-reverse`, non-default `order`, `align-self`, 비기본 `align-content`, baseline, auto margin과 비기본 `justify-content`는 후속 C10 항목이다.
- writing mode·RTL, absolute/fixed/float child, fragmentation, Grid, GPU 배분·성능 최적화, 실기기 성능도 포함하지 않는다.
- CSS 문법상 invalid declaration은 Stylo/Chromium의 CSS cascade 규칙대로 무시하거나 앞선 유효 선언을 보존하는지 비교한다. 이를 layout adapter 오류로 바꾸지 않는다. 반대로 CSS cascade를 우회해 adapter에 직접 주입된 음수·비유한 factor나 표현할 수 없는 computed input은 기본값으로 성공시키지 않고 node와 property를 식별해 전체 layout 계산을 실패시킨다.

## 입력 계약과 고정 비교 환경

- Oracle: Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`. 해당 실행 파일·fixture·capture 도구의 SHA-256을 reference에 저장한다. reference를 테스트 실행 중 자동 덮어쓰지 않는다.
- Viewport: `320×240 CSS px`, locale `en-US`, timezone `UTC`, DPR 1과 2를 별도로 캡처한다. 두 DPR에서 CSS frame은 같아야 한다.
- Layout baseline: item/container의 불필요한 margin, padding, border, gap, intrinsic content를 0으로 둔다. 해당 변수를 보는 별도 case에서만 하나씩 바꾼다. `box-sizing`과 nonzero padding/border는 paired case로 flex-basis box semantics를 검증한다.
- 비교 필드: 지원 노드별 computed `flex`, `flex-grow`, `flex-shrink`, `flex-basis`, 주축 size, `x`, `y`, `width`, `height`. computed CSS serialization 차이는 `getComputedStyle()`이 보고하는 reference 값과 typed Stylo 값을 혼동하지 않고 기록한다.
- 기하 허용치: 각 node의 각 frame field 최대 절대 오차 `0.5 CSS px`. 평균 오차로 한 항목의 실패를 숨기지 않는다. DPR별 결과도 각각 통과해야 한다.
- Chrome 결과가 아래 손계산 불변식과 다르면 reference를 갱신하지 않고 먼저 fixture, CSS 표준 해석, Chromium 버전·환경을 조사한다. Taffy 출력이나 화면 픽셀을 정답으로 삼지 않는다.

## Fixture 계획

전용 입력은 `tests/fixtures/css/c10/flex-distribution.html`, 목록은 `flex-distribution-inventory.json`, capture 도구는 `tools/css-reference/capture-c10-flex-distribution.mjs`, 회귀 검사는 `tools/css-reference/c10-flex-distribution.test.mjs`, 고정 결과는 `tests/fixtures/css/references/c10-flex-distribution-v1.json`으로 둔다. 실제 V8 앱 실행 입력은 `tests/fixtures/css/c10/runtime-flex-distribution.js`로 분리한다.

| 관찰 ID | 입력과 목적 | 독립 예상값 또는 판정 |
| --- | --- | --- |
| `grow-equal` | 폭 600, basis 100×2, grow 1/1 | 각 item 폭 300 |
| `grow-weighted` | 폭 600, basis 100×3, grow 1/2/1 | 폭 175/250/175 |
| `grow-subunit` | 폭 300, basis 100×2, grow 0.25/0.25 | 초기 여유 100의 절반인 50을 분배해 폭 125/125, 주축 끝에 50 남음 |
| `grow-zero` | 폭 300, basis 100×2, grow 0/1 | 폭 100/200 |
| `shrink-scaled` | 폭 240, basis 200/100, shrink 1/1 | 폭 160/80 |
| `shrink-weighted` | 폭 200, basis 200/100, shrink 1/2 | scaled factor 200/200, 폭 150/50 |
| `shrink-zero` | 폭 240, basis 200/100, shrink 0/1 | 폭 200/40 |
| `basis-overrides-size` | item width 200, basis 100, grow/shrink 0 | used main size 100 |
| `basis-auto-explicit-size` | basis auto와 명시 주축 width 또는 height | 명시 main size를 basis로 사용 |
| `basis-percent-row-column` | definite row width 400 및 column height 240, 25%/50% | 해당 main size 기준 100/200 및 60/120 |
| `flex-shorthand` | `flex:1`, `flex:auto`, `flex:none`, `flex:2 1 80px`와 longhand override | Chrome computed longhand와 frame을 각각 대조 |
| `stylesheet-grow` | ID 선택자 author stylesheet의 basis 100/100, grow 1/3, 폭 300 | stylesheet cascade에서 폭 125/175 |
| `grow-max-freeze` | 폭 500, basis 100×3, grow 1, 첫 item max 120 | 폭 120/190/190, max freeze 후 재분배 |
| `grow-min-freeze` | 폭 300, basis 100×2, grow 1, 첫 item min 180 | 폭 180/120, min freeze 후 재분배 |
| `shrink-min-freeze` | 폭 200, basis 200/100, shrink 1, 첫 item min 150 | 폭 150/50 |
| `shrink-max-freeze` | 폭 200, basis 200/100, shrink 1, 첫 item max 120 | 폭 120/80 |
| `shrink-min-overflow` | 폭 200, basis 200/100, shrink 1/1, 두 item min 150 | 각 폭 150, min을 깨지 않고 주축 overflow 100 |
| `mixed-violation-freeze` | 폭 500, basis 100×3, grow 1, 첫 item max 120·둘째 min 200 | 폭 120/200/180, 위반 합 부호에 따라 반복 freeze |
| `min-over-max` | basis 100, min 150, max 120, factor 0 | used main size 150; min wins |
| `hypothetical-factor-choice` | 폭 240, bases 50/150, 첫 item min 100, 두 item shrink 1 | min 때문에 hypothetical 합이 250이 되어 shrink 분기를 택하고 폭 100/140 |
| `gap-accounting` | 폭 300, basis 100×2, grow 1, 주축 gap 20 | 폭 140/140, 두 번째 item x=160 |
| `fixed-margin-accounting` | row 폭 300, basis 100×2, grow 1, 각 item에 좌우 margin 10 | border-box frame 폭 130/130, x=10/160 |
| `wrapped-per-line` | wrap 폭 250, basis 150/150/80, grow 1/1/2 | 첫 line item 폭 250; 둘째 line 폭 156.667/93.333, 서로 재분배하지 않음 |
| `box-sizing-basis` | content-box/border-box pair, 100px basis, 좌우 padding 10px, 좌우 border 5px | border-box frame 폭 130/100; paint는 판정에 쓰지 않음 |
| `fractional-and-large-factors` | 폭 500.5, basis 100.5/100, grow `1e20`/`3e20` | 폭 175.5/325, 두 번째 item의 x=175.5 |
| `invalid-css-factor` | 유효 grow 선언 뒤 `flex-grow:-1` 선언 | computed style과 frame이 Chrome의 invalid declaration cascade와 일치 |

Fixture 합계는 구현 전에 확정 reference에서 세며, row/column, inline full/incremental cascade, author stylesheet full cascade 및 실제 앱 입력을 별도 matrix 축으로 기록한다. 현재 incremental API는 author stylesheet가 포함되면 안전한 재사용을 거절하고 전체 계산 fallback을 요구하므로, stylesheet incremental reuse를 완료 조건으로 암묵적으로 추가하지 않는다. 테스트 case 수를 계획에서 미리 고정하지 않는다. capture harness는 비활성 case를 `display:none`으로 두고 선택한 case만 정상 문서 흐름에 표시한다. harness 밖으로 flex node를 위치시켜 결과를 바꾸지 않는다. adapter 직접 입력의 음수·NaN·무한대는 Rust 단위 시험으로 분리한다. 자동화가 없는 값은 계획 완료로 간주하지 않는다.

## 구현 순서

1. **코드 변경 전 비교 입력 확정:** fixture·inventory를 먼저 만들고 고정 Chromium으로 computed style과 모든 지원 node frame을 캡처한다. 위 손계산 기대값 및 알고리즘 분기와 비교하고 discrepancy를 해결한다. source tree와 capture tool digest를 reference에 넣는다.
2. **현재 경로 감사:** 여섯 runtime Flex profile의 full/incremental cascade, inline/author stylesheet allowlist, typed style snapshot, `LayoutStyle`, Taffy projection, error conversion, runtime V8 host의 profile 선택을 각각 따라간다. 기존 구현이 정확하면 코드를 바꾸지 않고 근거만 보강한다.
3. **최소 수정 구현:** 누락이나 차이가 확인된 단계만 고친다. line collection과 grow/shrink 계산을 섞어 새 algorithm을 중복 구현하지 않는다. Taffy 0.14.0이 동작을 제공하면 adapter 연결·제약 검증에 집중한다. Taffy와 Chrome이 다른 경우에만 좁은 호환 보정을 별도 함수/테스트로 추가한다.
4. **Rust 비교:** fixture의 computed typed values→`LayoutStyle`→Taffy→frame을 대조하고 node/property context가 유지되는 음성 입력을 확인한다. 각 case의 최종 CSS frame을 Chrome reference와 비교한다.
5. **변경·오류 검증:** grow, shrink, basis, min, max를 한 번에 하나씩 바꾸고 revision과 frame 갱신을 확인한다. unsupported intrinsic input, 잘못된 factor/비유한 값, Taffy 오류에서 stale frame이나 partial success가 노출되지 않는지 확인한다.
6. **실행 앱 확인:** `runtime-flex-distribution.js`를 Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 기본 V8→Stylo→Taffy→WGPU 경로로 실행한다. runtime report에 node별 CSS frame을 남기고 Chrome과 비교한다. 화면 캡처는 실제 기본 앱이 제출되었는지 보조 증거로만 쓰며, 실기기·hardware GPU 성능 근거로 확대하지 않는다.
7. **동일 PR 문서화:** 내부 인터페이스 문서(새 ID, 숫자 버전 `0.1.0` 고정), 상태 대장, fixture inventory, 실패 경계, 실행 원본과 사용자 미리보기(`allthatnba/spinon/`)를 함께 갱신한다. PR은 구현 전 비교 모델, 테스트 결과, 플랫폼 한계와 발견·수정 내용을 한글로 설명한다.
8. **구현 뒤 새 검토:** 계획 검토 표를 재사용하지 않고 실제 코드·fixture·오류·platform path를 대상으로 새로 20개 서로 다른 실패 관점을 검토한다. 발견 항목은 수정하고 관련 관점을 다시 확인한다.

## 현재 구현 진행

- 고정 Chromium `154.0.8037.98`의 27개 case·92개 node를 DPR 1·2로 캡처했다. 입력, capture 도구, helper, 실행 파일 digest를 저장하고 테스트 실행 중 reference 덮어쓰기를 막았다.
- Rust 비교 시험은 각 case의 모든 node frame field에서 Taffy 0.14.0 결과가 Chrome 기준 오차 `0.5 CSS px` 이하임을 확인한다. 잘못된 음수·NaN·무한대 factor는 node와 property 문맥을 포함해 실패해야 한다.
- Stylo cascade 시험은 여섯 runtime Flex profile의 basis·grow·shrink·min/max typed projection, inline full/incremental 결과, author stylesheet의 사용자 지정 속성 및 등록 shorthand 경로를 확인한다.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 실제 V8 fixture를 실행했다. 두 플랫폼의 root·flex·세 item frame을 Chromium 기준과 각각 대조해 일치했고 WGPU에 다섯 box가 제출됐다. Android는 ANGLE/SwiftShader software backend다. 실기기·hardware GPU 성능 검증은 아니다.
- 서로 다른 구현 실패 관점 20개 검토에서 수정이 필요한 동작 결함은 남지 않았다. Clippy가 찾은 테스트 보조 함수의 불필요한 lifetime은 제거하고 다시 검사했다. 구체 결과는 [구현 검토와 Simulator 근거](../spec/internal/evidence/c10-2-flex-distribution-implementation-review-2026-10-10.md)에 둔다.

## 완료 조건

- Chromium reference와 일치하는 Rust computed-style·layout output을 fixture의 모든 지원 입력에서 얻는다. 모든 node/frame field가 DPR 1·2 각각 `0.5 CSS px` 허용치 이내여야 한다.
- grow factor 부분 합, scaled shrink, 초기 freeze 조건, min/max 위반 방향과 재분배 반복이 각각 독립 관찰값을 가진다. 내부 Taffy output만 비교하는 test로 대체하지 않는다.
- 여섯 runtime Flex profile의 typed projection을 확인하고, inline-style 변경에서는 incremental 결과가 full cascade와 같아야 한다. author stylesheet는 지원 profile의 full cascade에서 검사하고, incremental 경로가 재사용을 거절하면 full cascade fallback을 확인한다. 실제 V8 앱 사용자 지정 속성 paint path도 통과한다.
- Android API 37 emulator와 iOS 26.2 Simulator에서 같은 runtime fixture의 node별 CSS frames가 Chrome 기준과 허용치 내에서 일치하고 WGPU 제출·오류 로그를 저장한다. 시뮬레이터를 실제 기기 성능 증거로 주장하지 않는다.
- 새 내부 계약·사용 예제·지원 범위·오류·revision behavior를 기록하고, `spec/STATUS.md`의 C10.2만 완료한다. C10 상위와 Flexbox 전체 지원은 미완료로 남긴다.
- 구현 시 crate나 앱 버전, 내부 계약 숫자 버전을 변경하지 않는다. 필요한 경우 fixture/reference schema 버전만 이름 공간에서 독립 관리한다.

## 기준 자료

- [W3C CSS Flexible Box Layout Module Level 1 §7.1–7.2](https://www.w3.org/TR/css-flexbox-1/#flexibility), [§9.2 line length](https://www.w3.org/TR/css-flexbox-1/#line-sizing), [§9.7 resolving flexible lengths](https://www.w3.org/TR/css-flexbox-1/#resolve-flexible-lengths), [§9.8 definite/indefinite sizes](https://www.w3.org/TR/css-flexbox-1/#definite-and-indefinite-sizes). 현재 published document는 2025-10-14 Candidate Recommendation Draft이며, 구현 전에 고정 revision과 해당 절을 재확인한다.
- 저장소 고정 Taffy `0.14.0`의 `compute/flexbox.rs`와 mapped style types. Taffy는 비교 후보이며 browser compatibility의 최종 oracle이 아니다.
- C10.1의 [Chromium reference](../tests/fixtures/css/references/c10-flex-wrap-v1.json), C06.1 percentage [계약](../spec/internal/0037-c06-percentage-dimensions.md), C07.1 min/max [계약](../spec/internal/0043-c07-1-min-max-sizing.md), C07.2 border width [계약](../spec/internal/0044-c07-2-border-width-layout.md).
