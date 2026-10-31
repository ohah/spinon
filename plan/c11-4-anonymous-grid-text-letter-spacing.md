# C11.4 anonymous Grid text computed `letter-spacing` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [CSS 적합성 계약](../spec/0001-conformance.md)

**상태:** 계획 PR #919 병합 뒤 제한 구현 diff를 검토했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 16-scene runtime matrix가 각각 통과했고, 기본/fixture release static archive에서 신규 adapter·validator symbol과 16개 scene payload 경계를 확인했다. focused 회귀 검사, 구현/runtime 실패 관점 20개와 PR diff 30개 검토를 마쳤다. 구현 PR 생성·checks·merge는 남아 있으며 C11.4/C11 공식 완료 수는 올리지 않았다. 사전 비교와 구현 전 계획 검토는 각각 [비교 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-letter-spacing-precomparison-2026-10-31/README.md), [20개 실패 관점](./c11-4-anonymous-grid-text-letter-spacing-plan-review-20.md)에 둔다.

## 목적과 경계

현재 anonymous Grid text runtime fixture는 computed `letter-spacing: normal`만 허용하고 `ParagraphStyle`에는 해당 입력이 없다. 따라서 cascade 결과가 `Parley` text layout에 도달하지 않는다. 이 작업은 제한된 anonymous direct-text Grid profile에 computed px letter spacing을 전달하고 intrinsic min/max-content track 기여에 반영한다.

범위는 기존 opt-in C11.4 anonymous-text runtime profile로 제한한다: direct anonymous Grid text, Ahem 10px, font-weight 400, horizontal-tb, LTR, 기존 `white-space:normal`·`word-break:normal`, ASCII `X`와 collapsed U+0020만 사용한다. 고정 16-scene runtime allowlist는 computed `normal`, -1px, 0px(Stylo에서는 `normal`로 직렬화), 0.5px, 1px, 2px를 검사한다. 숫자 parser의 안전 범위는 -2px 이상 2px 이하로 두되 -2px scene은 allowlist에 없으므로 runtime 입력으로 허용하지 않는다. `normal`과 `0px`은 같은 0 입력이다.

일반 authored CSS admission, 공개 API/ABI, arbitrary fonts, inline spans, bidi, vertical writing, percentage/relative units, `calc()`, letter-spacing의 다른 formatting context 적용, WPT 적합성 또는 C11/C11.4 완료 승격은 포함하지 않는다. 기존의 다른 runtime profile은 계속 `letter-spacing:normal`만 받는다. 기본 제품 artifact에서 fixture code·symbol·payload가 노출되지 않아야 한다.

## 사전 비교와 판정 기준

[사전 비교 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-letter-spacing-precomparison-2026-10-31/README.md)는 동일한 16개 CSS 입력, Ahem 10px, Chrome 156 환경, 원본 JSON과 화면을 보존한다.

[CSS Text Level 3 §7.2](https://www.w3.org/TR/css-text-3/#letter-spacing-property)는 spacing을 인접한 typographic character unit 사이에 두고 줄 시작·끝에는 두지 않는다. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)의 min/max-content sizing에 이를 적용한 사전 판정값은 다음과 같다.

| letter-spacing | `XX` min/max | `XX XX` min-content | `XX XX` max-content |
| --- | ---: | ---: | ---: |
| normal / 0px | 20 | 20 | 50 |
| -1px | 19 | 19 | 46 |
| 0.5px | 20.5 | 20.5 | 52 |
| 1px | 21 | 21 | 54 |
| 2px | 22 | 22 | 58 |

Chrome 156은 같은 입력에서 각각 18/18/45, 21/21/52.5, 22/22/55, 24/24/60을 관찰했다. 일반 `width:max-content` block 대조도 동일한 두 글자 폭을 보였다. 이 차이는 Chrome 관측값으로 기록하고 규범 기대값으로 대체하지 않는다. 구현 PR과 사용자 설명에는 Chrome 156과의 이 차이를 명시하며 Chrome parity를 주장하지 않는다.

구현 중 확인한 Stylo 동작: `RuntimeBlockFlexGridTrackSizingV1`의 `ComputedElementStyle.properties["letter-spacing"]` 직렬화는 authored `0px`를 `normal`로 반환한다. runtime은 이를 0 CSS px로 해석하고 normal·0px 두 source scene을 같은 computed 결과와 기대 폭으로 검증한다. fixture source 생성 테스트는 authored serialization을 별도로 확인한다. 이 관찰은 Stylo snapshot 계약에 한정하며 CSSOM의 일반 computed-style 직렬화를 주장하지 않는다.

## 계획된 변경

1. `ParagraphStyle`에 기본값 0인 letter-spacing px 입력을 추가한다. `validate`는 finite 값을 요구한다. 모든 기존 style initializer를 명시적으로 0으로 유지해 현재 동작을 바꾸지 않는다.
2. `TextLayoutEngine::build_parley_layout`은 입력값을 고정된 Parley 0.12.0의 `StyleProperty::LetterSpacing`에 전달한다. Parley의 atom spacing만으로 CSS 줄 끝 억제를 충족한다고 가정하지 않는다.
3. anonymous Grid text adapter는 정확한 root node와 같은 `ComputedStyleSnapshot` revision에서 computed `letter-spacing`을 읽는다. `normal`은 0px로, finite `px` serialization은 지정 범위에서 f32 CSS px로 변환한다. 현재 Stylo snapshot은 authored `0px`도 `normal`로 직렬화하므로 두 입력을 동등한 computed 값으로 검사한다. 다른 단위, parse overflow, NaN/Infinity, 범위 초과 및 잘못된 node/revision은 layout publication 전 fail closed한다.
4. intrinsic layout에서 각 비어 있지 않은 line의 trailing atom spacing을 한 번 제거해 CSS Text의 줄 끝 flush 규칙을 적용한다. min-content는 줄별 보정 후 가장 넓은 line, max-content는 무제한 폭에서 계산한 각 line의 보정 폭 중 가장 큰 값을 사용한다. inline-box contribution·hanging whitespace를 letter-spacing 보정으로 잘못 빼지 않으며, 빈 line과 forced break의 block-size 계산은 유지한다. 필요하다면 helper를 분리해 line advance와 line-edge 보정을 한 곳에서 검증한다.
5. Text-layout unit tests는 한 단어 `XX`의 min/max와 두 단어 `XX XX`의 min/max를 위 16-case oracle에 대조한다. 별도 controls는 normal/0px 동일성, 음수 폭이 0 아래로 내려가지 않음, fractional px, 줄바꿈 후 각 line의 끝 억제, 모든 기존 zero-spacing 회귀를 확인한다.
6. Runtime fixture는 computed style serialization, root identity/revision, 정확한 direct-text shape, min/max-content contribution, Grid track, node frame과 unsupported-input rejection을 확인한다. CSS oracle 16 scenes를 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS Simulator에서 각각 실행하고 캡처한다. simulator surface pixel 검증은 track/frame 증거와 분리하며 실제 기기나 hardware GPU 증거로 표시하지 않는다.
7. 기본 제품 및 opt-in fixture-test artifact를 각각 빌드한다. 기본 artifact에서 변경한 fixture symbol·scene marker·payload가 빠지고 opt-in positive control에는 포함되는지 확인한다. Rust focused tests, 기존 anonymous text regression, format, Clippy와 두 simulator app builds를 실행하고 실제 로그만 결과로 기록한다.
8. 관련 내부 인터페이스 명세에는 입력 단위, finite/range 제한, unsupported 입력 오류, snapshot ownership/revision과 Chrome 차이를 기록한다. 내부 숫자 버전 `0.1.0`을 유지한다. C11/C11.4 공식 체크박스는 수정하지 않는다.

## 실패 조건과 남은 한계

- computed value를 무시하거나 장면 ID에서 상수를 고르면 실패다.
- CSS Text의 adjacent-unit model 또는 line-edge suppression을 누락해 16개 규범 기대값 중 하나라도 다르면 실패다.
- Chrome 156 관측을 규범 expected value로 사용하거나 결과에서 브라우저 차이를 숨기면 실패다.
- 음수·비유한·지원 밖 단위·stale snapshot에서 layout, partial frame 또는 trace를 발행하면 실패다.
- 빈 line, forced break, collapsed space, hanging advance 또는 기존 normal spacing 계산이 바뀌면 실패다.
- 다른 C11/C15 text profile로 computed spacing이 새거나 기본 artifact에 fixture 코드가 들어가면 실패다.
- simulator 실행은 전체 CSS Text, 임의 폰트·문자·방향, WPT 적합성, 실기기/GPU, compositor 후 화면, C11.4 전체 또는 C11 완료를 증명하지 않는다.

## 계획 검토

구현 전 서로 다른 실패 관점 20개는 [계획 적대 검토](./c11-4-anonymous-grid-text-letter-spacing-plan-review-20.md)에 기록한다. 이 검토는 구현/runtime 검토에 재사용하지 않는다.
