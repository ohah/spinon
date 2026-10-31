# C11.4 `tab-size` 전체 computed value 계획 실패 관점 검토 20개

**검토일:** 2026-10-31

**검토 대상:** [계획 초안](./c11-4-anonymous-grid-text-pre-tab-size-values.md), [Chrome 사전 비교](../spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/README.md)

**경계:** 제품 코드와 official status는 바꾸지 않았다. 아래 검토는 계획 자체의 실패 경로이며 PR diff 검토는 별도 문서로 기록한다.

| # | 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | `<number>`와 `<length>` 중 한쪽 값 문법이 빠질 수 있음 | **PASS** — 계획은 전체 non-negative number와 computed absolute length를 목표로 하고 fixture 입력은 두 종류를 분리한다. |
| 2 | author 문자열을 직접 파싱해 `em`·`calc()`를 잘못 처리할 수 있음 | **PASS** — 구현 입력을 computed-style snapshot으로 한정하고 upstream resolved CSS px만 받는다. Chrome에서 `1.5em`·`calc()` 계산값을 고정했다. |
| 3 | computed length가 px 외 단위로 전달된다고 오해할 수 있음 | **PASS** — typed value는 absolute CSS px이며 잘못된/미계산 단위는 fail-closed 경계로 보낸다. |
| 4 | unitless number가 px 길이로 오해되거나 U+0020 advance와 분리될 수 있음 | **PASS** — number는 가까운 block container의 space advance 배수로 정의하고 Ahem 10px metrics를 별도 고정했다. |
| 5 | 이진 부동소수점에 정확한 수만 시험해 fractional precision 결함을 놓칠 수 있음 | **PASS** — `.3`과 `.3px`을 matrix에 추가하고 ideal CSS advance와 Chrome geometry 반올림을 분리했다. |
| 6 | zero tab에 양수 공간 또는 glyph가 남을 수 있음 | **PASS** — `0`·`0px` leading/mixed/trailing case를 포함하고 zero는 glyph·양수 advance가 없어야 한다. |
| 7 | `0.5ch` 경계에서 `<`, `=`, `>` 비교가 틀릴 수 있음 | **PASS** — `.25`/`2px`, `.5`/`5px`, `12px` 입력이 threshold 아래·정확·위 조건을 만든다. |
| 8 | 연속 tab마다 독립 재계산해 Chrome과 달라질 수 있음 | **PASS** — `X\t\tX`를 고정하고 Blink 동작을 비교 oracle로만 사용한다. mainline source가 Chrome 156과 동일 revision이라고 확대 주장하지 않는다. |
| 9 | LF 뒤 tab origin이 이전 줄 column을 이어받을 수 있음 | **PASS** — LF reset이 모든 value에 포함된다. |
| 10 | CRLF를 한 번 또는 두 번 reset해 두 번째 줄 위치가 달라질 수 있음 | **PASS** — CRLF case를 LF와 따로 비교한다. |
| 11 | whitespace-only Grid child를 text-layout 양성으로 잘못 셀 수 있음 | **PASS** — W3C Grid §6 규칙과 Chrome 관측에 따라 tab-only row는 0×0 억제 대조군으로 두고 runtime matrix에서 제외했다. |
| 12 | tab anchor가 원본 node/source range를 잃거나 글리프 provenance를 가져갈 수 있음 | **PASS** — 계획은 zero-length position anchor에 원본 node·UTF-16 range를 보존하고 glyph는 원래 source에 귀속한다. |
| 13 | tab만 정규화된 빈 문자열에서 Parley builder가 조기 종료할 수 있음 | **PASS** — Grid가 억제하는 경계와 일반 text layout을 구별하고, 런타임 양성 입력은 보존되는 비공백 text를 포함한다. |
| 14 | min-content가 줄 내 tab stop 또는 여러 줄 reset을 무시할 수 있음 | **PASS** — min-content와 forced-line case를 같은 independent advance model에 포함한다. |
| 15 | trailing tab이 advance 또는 intrinsic contribution에서 빠질 수 있음 | **PASS** — non-whitespace text 뒤 trailing tab을 각 값으로 비교한다. |
| 16 | `white-space:pre`가 soft-wrap되어 고정 track overflow가 감춰질 수 있음 | **PASS** — 25px fixed track의 no-soft-wrap scene을 포함하고 item overflow를 검사한다. |
| 17 | 큰 유한 입력·곱셈 overflow가 잘못된 partial geometry를 publication할 수 있음 | **PASS** — finite/layout-representable 경계를 명시하고 overflow는 기존 error boundary에서 publication 전에 거부하며 별도 negative control로 확인한다. |
| 18 | 누락·stale computed snapshot이나 scene ID가 실값을 바꿀 수 있음 | **PASS** — 누락/revision 불일치는 거부하고 scene ID는 computed value 기대값 검증에만 쓴다. |
| 19 | 기존 `Pre`·정수 `PreWithTabSize`·다른 whitespace mode가 회귀할 수 있음 | **PASS** — `Pre`, 1/2/4/8, `PreWrap`, `BreakSpaces` 및 기존 55-scene regression을 검증한다. |
| 20 | fixture 검증을 일반 CSS·기기·GPU·공식 C11 완료로 과장할 수 있음 | **PASS** — opt-in fixture, 두 simulator, software-rendered Android와 iOS drawable 경계를 적고 C11.4/C11/`72/213`은 유지한다. |

**결과:** 최초 검토 중 tab-only Grid 입력과 `.3px` browser geometry의 해석 문제를 발견했다. 계획과 fixture를 수정한 뒤 20개 관점 모두 PASS다. `compare-observation.py`는 154행, 168개 line advance, Ahem metrics, computed serialization 및 1/64px geometry 보정을 비교해 mismatch 0을 보고한다. 이는 구현/runtime 검토 또는 계획 PR diff 검토를 대신하지 않는다.
