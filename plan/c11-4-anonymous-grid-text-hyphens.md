# C11.4 익명 Grid 텍스트 `hyphens` 제한 profile 계획

**상태:** Chrome 156 사전 비교와 Ahem 독립 계산을 고정했다. 제품 구현은 시작하지 않았다. C11.4/C11은 미완료이며 공식 완료 `72/213`은 변경하지 않는다.

## 규범 동작과 문제

CSS Text 3 §5.3에서 `hyphens`는 `none | manual | auto`, 초기값 `manual`, 상속 속성이다. 하이픈 기회는 min-content intrinsic size 계산에 반영된다. `manual`은 U+00AD SOFT HYPHEN 같은 명시적 기회에서만 단어를 나누고 줄이 바뀌면 하이픈을 표시한다. `none`은 U+00AD 기회도 막지만, 눈에 보이는 U+002D HYPHEN-MINUS의 일반 줄바꿈은 막지 않는다. `auto`는 언어별 사전이 필요하다.

현재 C11.4 direct-text adapter는 computed `hyphens`를 Paragraph 입력으로 전달하지 않는다. Stylo 0.22가 `hyphens`를 알 수 없는 속성으로 무시하는 현상은 기존 negative fixture가 기록한다. 사전 비교의 Chrome geometry는 이 격차가 실제 min-content track과 고정 너비 줄바꿈에 영향을 주는 것을 보여준다.

## 제한 구현 범위

- C11.4 `RuntimeBlockFlexGridTrackSizingV1` fixture-test 경로의 익명 direct-text Grid item만 대상으로 한다. 일반 작성 CSS admission이나 공개 CSS 지원으로 승격하지 않는다.
- 고정 Ahem 10px, `lang="en"`, 가로쓰기, ASCII `A`–`F` 및 내부 U+00AD 한두 개만 허용한다. 다른 font·script·language, styled inline run과 미지원 source shape는 계산 전에 거부한다.
- 입력 CSS의 `hyphens` 선언은 fixture-test 전용 parser adapter가 Stylo가 cascade할 수 있는 예약 custom property로 연결한다. Stylo가 실제 적용한 custom property의 computed 결과를 읽어 `none | manual | auto`를 결정하며, DOM scene ID나 author 원문 검색만으로 스타일을 추측하지 않는다. 초기값은 `manual`; `initial`은 `manual`로 매핑한다. inherited/`unset`과 `revert`/`revert-layer`, stylesheet·inline source order, selector specificity와 `!important`는 cascade 경계를 검증한다.
- 어댑터는 Stylo parsing 전에 authored `hyphens` 선언을 검사한다. `balanced`처럼 잘못된 keyword가 CSS parser에서 조용히 버려져 미선언 상태와 구별되지 않는 경우 그 scene을 `manual`로 처리하지 말고 layout 전에 거부한다.
- 예약 property 이름의 author 충돌, 그 이름에 대한 `@property` 등록, `all` shorthand는 이 profile에서 fail closed한다. `auto`와 유효하지 않거나 미지원된 computed value도 dictionary/값 경계가 닫히지 않으면 layout 전에 거부한다.
- `none` layout input에서 U+00AD를 같은 UTF-16 길이의 U+2060 WORD JOINER로 바꾸어 그 명시적 break를 제거한다. `manual` definite-width layout은 U+00AD 기회를 유지하되 실제 줄이 그 지점에서 끝나면 line advance에 U+2010 HYPHEN 폭을 포함한다. `manual` min-content pass는 측정용 복사본에서 U+00AD를 U+2010으로 바꾸어 break 위치의 glyph advance를 contribution에 포함한다. max-content pass와 HostDocument 원문은 U+00AD를 그대로 둔다. Parley가 줄 끝의 discretionary hyphen advance를 제공하지 않으면 명시적인 line-metric adjustment를 구현하고 Chrome line-advance 값을 대조한다.
- `hyphens`는 상속되므로 Grid owner의 조상 선언도 테스트한다. normalization과 변환 뒤 source NodeId, UTF-8 byte·UTF-16 범위, document/style revision이 보존되는지 확인한다.

이 구현은 layout geometry·intrinsic contribution을 검증한다. 현재 C11.4 runtime text path는 glyph를 그리지 않으므로 화면 캡처는 문자 모양이나 하이픈 glyph paint의 pixel 증거가 아니다. glyph painting 및 일반 CSS 적합성은 별도 미완료 범위다.

## 고정 fixture와 Chrome 기대값

[CSS Text 3 §5.3](https://www.w3.org/TR/css-text-3/#hyphenation)과 [Agent Browser Chrome 156 사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-hyphens-precomparison-2026-11-01/README.md)를 기준으로 한다. [HTML fixture](../spec/internal/evidence/c11-4-anonymous-grid-text-hyphens-precomparison-2026-11-01/anonymous-grid-text-hyphens.html)는 Grid의 직접 text child를 측정한다. Ahem `A`–`F`, `-`, U+2010의 hmtx advance는 모두 1000/1000 em이다.

고정 21-case runtime matrix는 아래 19개 Chrome positive observation에 `auto`와 잘못된 value의 기대 거부 두 장면을 더한다.

| 입력 | `hyphens:none` | `hyphens:manual` |
| --- | --- | --- |
| `AB&shy;CD`, min-content | column 40px, row 10px, 1 line | column 30px, row 20px, 2 lines |
| `AB&shy;CD`, max-content | 40×10px, 1 line | 40×10px, 1 line |
| `AB&shy;CD`, fixed 30px | 30px track, 40px overflow, row 10px, 1 line | 30px track, row 20px, 2 lines; line advances 30/20px |
| declaration 없음 (initial `manual`), fixed 30px | — | 30px track, row 20px, 2 lines |
| `AB-CD`, min-content/fixed 30px control | min 30px; fixed 30px에서 2 lines | none과 동일 |
| `ABCD`, min-content control | 40px | 40px |
| `AB&shy;CD&shy;EF`, min-content | 60px, 1 line | 30px, 3 lines |
| `AB&shy;CD&shy;EF`, fixed 30px | 60px overflow, 1 line | 30px track, 3 lines |
| 조상에 선언한 `none|manual`, fixed 30px | 40px overflow, 1 line | 30px 안에서 2 lines |

나머지 두 scene은 computed `hyphens:auto`와 잘못된 keyword `hyphens:balanced`를 각각 layout 전에 거부하는 negative control이다.

## 검증 관문

1. 실제 Stylo cascade가 예약 custom property의 상속, `!important`, selector/source order, CSS-wide keyword와 revision 변경을 의도대로 유지한다는 style tests를 추가한다. alias collision, `@property`, `all`, stylesheet·inline 경계도 positive/negative control을 둔다. Stylo computed value를 확실히 읽지 못하거나 정상 cascade 의미가 보존되지 않으면 implementation을 진행시키지 않는다.
2. `spinon-text-layout`에서 `none`/`manual`의 max/min-content, single/multiple soft hyphen, visible-hyphen control, UTF-16 source map, finite metrics와 rejection을 검증한다. U+00AD의 ordinary line-break 동작이 Parley pinned revision에서 Chrome과 다르면 그 차이를 숨기지 말고 explicit break handling을 구현한다.
3. opt-in C11.4 runtime fixture에서 21 scene 모두 computed property, exact owner, source/style revision, line count/advance, min/max contribution, Columns/Rows track, overflow와 expected rejection을 검사한다. 일반 CSS projection negative gate와 기존 C11.4 regression을 유지한다.
4. 기본/product artifact에서 fixture-only alias/FFI/payload가 빠지고 fixture-test positive control에만 포함되는지 확인한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 pinned matrix와 회귀를 실행하고 설치 artifact hash를 근거에 묶는다. 실기기/hardware GPU와 glyph paint는 판정 범위 밖이다.
5. 구현/runtime 실패 관점 20개와 PR 변경 diff 20개는 서로 다른 항목으로 남긴다. 제한 profile만 병합해도 C11.4/C11 체크와 공식 `72/213`은 유지한다.
