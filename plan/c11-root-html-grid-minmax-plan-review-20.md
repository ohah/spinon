# C11.1 Root HTML Grid `minmax()` 계획 실패 관점 검토 20개

검토 대상은 [계획](./c11-root-html-grid-minmax.md)과 [Chrome 156 precomparison](../spec/internal/evidence/c11-root-html-grid-minmax-precomparison-2026-11-03/README.md)이다. 구현 전에 계획을 고정하며, 이 표는 구현/runtime 검토로 재사용하지 않는다.

| # | 독립 실패 관점 | 방어·판정 |
|---:|---|---|
| 1 | Chrome computed track을 규범 기대값으로 오인한다 | **PASS** — expected는 W3C CRD 규칙과 독립 산술에서 만들고 browser JSON을 별도로 보존한다. |
| 2 | `max < min`에서 선언을 거부하거나 max를 그대로 둔다 | **PASS** — used value 단계에서 max를 min으로 바닥 처리하고 px/% 사례를 별도로 고정한다. |
| 3 | minmax의 flex 최대값을 flex 최소값처럼 허용한다 | **PASS** — max breadth만 `Fraction`을 허용하고 CSS-invalid/min Fraction injection은 거부한다. |
| 4 | percentage basis를 gap 차감 뒤 값이나 outer border-box로 계산한다 | **PASS** — 기존 zero-edge profile의 definite content axis를 쓰고 gap은 free-space 계산에만 한 번 반영한다. |
| 5 | indefinite/cyclic percentage 축을 임의의 0 또는 viewport 값으로 resolve한다 | **PASS** — 해당 축의 definite finite non-negative px size가 없으면 layout 전에 거부하고 definite zero basis는 별도 positive로 둔다. |
| 6 | percentage maximum/minimum이 track sum overflow를 만든다 | **PASS** — 각 breadth resolved range와 aggregate를 f32 범위에서 검증한 뒤 projection한다. |
| 7 | fixed max를 넘어서 track을 늘리거나 남은 공간을 잃는다 | **PASS** — 단일·복수 bound track과 available-space 경계를 fixture에 포함한다. |
| 8 | min이 container보다 커도 clamp되어 overflow가 사라진다 | **PASS** — `[40,10]` track과 x=40..48 marker로 30px root 바깥을 검증한다. |
| 9 | row percentage를 width 기준으로 resolve한다 | **PASS** — row percentage positive scene은 100px height 기준 `[40]`이며 height gate를 따로 검증한다. |
| 10 | percentage/flex 최대값에 `min-size`, `max-size`, `aspect-ratio` override가 섞인다 | **PASS** — direct track과 동일한 axis eligibility gate를 적용하고 negative matrix로 둔다. |
| 11 | named area가 sized track 리스트와 불일치해 area mapping이 바뀐다 | **PASS** — 현재 exact fixed-px named-area 계약을 유지하고 MinMax 조합은 fail-closed한다. |
| 12 | `repeat()`를 다루면서 auto-repeat/track cap/line metadata가 우회된다 | **PASS** — 기존 integer expansion만 허용하며 auto-repeat, line names, quota 우회를 거부한다. |
| 13 | `grid-auto-rows`/`grid-auto-columns`로 MinMax가 의도치 않게 새어 나간다 | **PASS** — 이번 허용은 explicit template axes에만 적용한다. |
| 14 | column-flow의 별도 placement/track contract에 MinMax를 전파한다 | **PASS** — column-flow guard와 sparse implicit sizing 계약을 그대로 유지한다. |
| 15 | 유효한 intrinsic/auto CSS 입력을 CSS syntax error로 취급한다 | **PASS** — browser control은 유효한 관측으로 남기고 product slice에서는 명시 unsupported 진단을 기대한다. |
| 16 | negative CSS breadth의 cascade drop을 성공적인 MinMax fallback으로 해석한다 | **PASS** — `minmax(20px,-1px)`는 invalid CSS control로 분리하고 observed implicit 8px track을 기록한다. |
| 17 | repeated track이나 mixed `%`/`fr` 목록에서 child origin이 달라진다 | **PASS** — fixed repeat, px/%-min flex maximum, zero-fr에서 used tracks와 item origins 모두 대조한다. |
| 18 | 하나의 Grid failure 뒤 부분 scene/frame이 publication된다 | **PASS** — runtime acceptance에 unsupported case의 diagnostic과 partial frame 부재를 요구한다. |
| 19 | stale style/layout revision이나 fixture ABI로 default runtime 결과를 흉내 낸다 | **PASS** — 기본 parsed Root HTML entry만 사용하고 source/revision identity를 고정하며 신규 ABI를 금지한다. |
| 20 | simulator screenshot을 pixel equality나 실기기/hardware GPU 증거로 과장한다 | **PASS** — screenshot은 시각 증거, AVD PixelCopy와 iOS drawable 표본은 별도 분류하고 플랫폼 제한을 적는다. |

구현 시작 전 계획 자체의 20개 관점은 모두 반영됐다. 이 계획 PR diff도 저장소 규칙에 따라 별도 30개 관점으로 검토한다.
