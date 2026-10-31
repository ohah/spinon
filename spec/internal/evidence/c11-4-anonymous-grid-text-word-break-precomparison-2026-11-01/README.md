# C11.4 익명 Grid 텍스트 `word-break` 사전 비교

이 자료는 기존 익명 direct-text Grid fixture가 computed `word-break: normal`만 허용하는 경계를 조사한다. CSS Text Level 3의 `normal`, `break-all`, `keep-all` 동작을 고정 Ahem 및 Noto Sans KR 입력으로 비교했다. 이는 Chrome oracle과 독립 기대값이며 Spinon 구현 결과는 아니다.

## 환경과 입력

- 기준 브라우저: Agent Browser로 구동한 Headless Chrome 156.0.8078.80, DPR 1. 사용자 에이전트 문자열은 [관측 JSON](./chrome156-observations.json)에 보존한다.
- [고정 HTML fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-word-break.html)는 Grid root의 직계 Text 노드 하나만 둔다. 바깥 `<html lang="en">`에 의존하지 않고 각 scene의 Grid root에 `lang="en"` 또는 `lang="ko"`를 직접 둬 익명 Text의 언어/style owner를 분명히 한다. `width:120px`, 0 gap/edge, `grid-template-rows:auto`, Ahem/Noto Sans KR 10px, line-height 10px, `line-break:auto`, `overflow-wrap:normal`을 고정하고 `min-content`와 `max-content` track을 관측한다.
- Ahem 400의 U+0058 `X` 및 U+0020 space advance는 각 1000 units/em, units/em은 1000이다. CSS 10px에서 각 advance는 10px이다. SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다.
- Noto Sans KR 2.004는 고정된 [C15.3 font fixture](../../../../tests/fixtures/resources/c15/c15-3/README.md)에서 로드했다. 원본 commit은 `f8d157532fbfaeda587e826d4cd5b21a49186f7c`, SHA-256은 `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76`이다. `wght=400`에 고정한 U+AC00..U+B77C `가나다라` 각 glyph advance는 920/1000 em, 즉 CSS 10px에서 9.2px이다.

## 규범과 독립 기대값

[CSS Text Level 3 §5.1](https://www.w3.org/TR/css-text-3/#word-break-property)은 `word-break`가 상속 속성이고, `break-all`은 문자 단위의 추가 soft-wrap 기회를 허용하며, `keep-all`은 문자 사이의 통상적 줄바꿈을 억제한다고 정의한다. 이 속성 효과는 intrinsic size 계산에도 반영한다. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)의 min-/max-content 기준 및 [CSS Grid Level 1 §11](https://www.w3.org/TR/css-grid-1/#algo-track-sizing)에 따라 기대값은 Chrome 관측값이 아니라 font advance와 허용 줄바꿈 지점으로 계산했다.

| 입력 | word-break | track | 독립 track 폭 | 독립 row 높이 | Chrome 156 관측 |
| --- | --- | --- | ---: | ---: | --- |
| `XXXX` / Ahem | `normal` | min-content | 40px | 10px | 40px / 10px |
| `XXXX` / Ahem | `normal` | max-content | 40px | 10px | 40px / 10px |
| `XXXX` / Ahem | `break-all` | min-content | 10px | 40px | 10px / 40px |
| `XXXX` / Ahem | `break-all` | max-content | 40px | 10px | 40px / 10px |
| `XX XX` / Ahem | `normal` | min-content | 20px | 20px | 20px / 20px |
| `XX XX` / Ahem | `normal` | max-content | 50px | 10px | 50px / 10px |
| `XX XX` / Ahem | `break-all` | min-content | 10px | 40px | 10px / 40px |
| `XX XX` / Ahem | `break-all` | max-content | 50px | 10px | 50px / 10px |
| `가나다라` / Noto Sans KR, `lang=ko` | `normal` | min-content | 9.2px | 40px | 9.203125px / 40px |
| `가나다라` / Noto Sans KR, `lang=ko` | `normal` | max-content | 36.8px | 10px | 36.8125px / 10px |
| `가나다라` / Noto Sans KR, `lang=ko` | `keep-all` | min-content | 36.8px | 10px | 36.8125px / 10px |
| `가나다라` / Noto Sans KR, `lang=ko` | `keep-all` | max-content | 36.8px | 10px | 36.8125px / 10px |

Chrome의 track 직렬화는 Noto 값을 1/64px layout unit으로 반올림한다. font-table 산식과의 최대 차이는 0.0125 CSS px다. 두 글꼴은 모두 로드됐고, 12개 장면 각각의 computed keyword는 authored 값과 일치했다. 관측 JSON에는 각 장면의 computed `word-break`, `line-break`, font-weight, Grid owner `lang`, track·row 값이 들어 있다. 정확한 측정값은 [JSON](./chrome156-observations.json)에 있고, [Agent Browser 캡처](./chrome156-word-break-precomparison.png)에 전체 장면이 보인다. [계획 PR 변경 검토 30개](./plan-pr-diff-review-30.md)도 함께 보존한다.

## 판정 범위와 한계

이 비교는 고정 글꼴, direct text node 하나, horizontal writing, LTR, normal whitespace, `overflow-wrap:normal`과 위 입력값에만 해당한다. Spinon 동작, 모바일 runtime, glyph painting, 일반 font fallback 또는 CSS Text 전체 지원을 입증하지 않는다. CSS Text Level 3은 `keep-all`과 deprecated `break-word`도 열거한다. 제안하는 fixture path는 엔진이 표현하는 `normal`, `break-all`, `keep-all`을 연결하고, paragraph model이 `overflow-wrap:anywhere` 결합 동작을 표현하지 못하므로 `break-word`는 거부한다. 일반 author CSS admission은 범위 밖이다.
