# C11.4 anonymous Grid text break-spaces 계획 실패 관점 검토 20개

**대상:** [계획](./c11-4-anonymous-grid-text-break-spaces.md), [독립 기대값](../spec/internal/evidence/c11-4-anonymous-grid-text-break-spaces-precomparison-2026-10-31/expected-values.json), [Chrome 관측](../spec/internal/evidence/c11-4-anonymous-grid-text-break-spaces-precomparison-2026-10-31/chrome-observation-agent-browser.json).

| # | 독립 실패 관점 | 판정과 계획의 대응 |
| ---: | --- | --- |
| 1 | CSS white-space:break-spaces와 longhand white-space-collapse:break-spaces가 서로 다른 의미로 처리될 수 있는가 | 통과 — 현재 계획은 shorthand parser를 추가하지 않고 typed longhand 값을 validator에서 확인한다. |
| 2 | BreakSpaces가 PreWrap의 hanging semantics를 그대로 물려받을 수 있는가 | 통과 — Parley WhiteSpaceCollapse::BreakSpaces를 명시 사용하며 trailing-space wrapping을 별도 scene으로 검증한다. |
| 3 | Normal의 공백 collapse가 새 mode 때문에 바뀔 수 있는가 | 통과 — Normal regression control과 기존 16-scene matrix를 유지한다. |
| 4 | XX  XX max-content에서 반복 공백을 하나 누락할 수 있는가 | 통과 — independent Ahem 합 20+10+10+20=60px을 고정한다. |
| 5 | min-content 계산이 break-after-each-space와 pre-wrap hanging을 같은 값으로 처리하는가 | 통과 — BreakSpaces 30px과 PreWrap control 20px을 같은 input에서 분리한다. |
| 6 | 가장 긴 unbreakable segment가 단어 XX만으로 잘못 계산될 수 있는가 | 통과 — XX와 trailing preserved space를 합친 30px 조각을 고정한다. |
| 7 | 25px soft-wrap scene이 overflow와 hanging을 혼동할 수 있는가 | 통과 — 30px line fragment 두 개, 25px track, zero hanging advance를 분리 검사한다. |
| 8 | max-content line 끝 trailing spaces가 줄어들거나 사라질 수 있는가 | 통과 — XX와 두 trailing spaces를 합친 40px scene을 둔다. |
| 9 | 25px definite track 끝 trailing spaces가 hang해 item height를 줄일 수 있는가 | 통과 — 같은 trailing source의 fixed-width wrap scene이 20px height를 요구한다. |
| 10 | forced LF가 ordinary whitespace로 collapse될 수 있는가 | 통과 — LF forced-break max-content scene을 soft-wrap와 별도로 둔다. |
| 11 | forced break 앞 공백이 조건부 hanging으로 잘못 처리될 수 있는가 | 통과 — XX 두 공백 LF XX max-content 40px 및 min-content 30px/30px 결과로 양쪽 sizing을 검사한다. |
| 12 | comment-split Text source가 여러 anonymous items로 분리될 수 있는가 | 통과 — 두 TextSource identity를 보존하는 단일 60px item을 확인한다. |
| 13 | CRLF split provenance가 공통 segment-break normalizer 변경으로 손상될 수 있는가 | 통과 — 기존 PreWrap CRLF source-range 회귀를 유지하고 새 unit module에서 같은 contract를 재검사한다. |
| 14 | tab-size를 Parley 기본값으로 추측해 잘못 허용할 수 있는가 | 통과 — 계획은 tab 입력을 fail-closed하고 author tab-size를 제외한다. |
| 15 | WhiteSpaceMode의 exhaustive match가 새 variant를 빠뜨려 일부 path에서 panic 또는 fallback할 수 있는가 | 통과 — model, normalization, Parley mapping, Grid profile gate의 모든 mapping을 갱신하고 compile/focused tests를 요구한다. |
| 16 | PreWrap tab expansion이 BreakSpaces에 적용되어 각 tab을 여러 independent spaces로 만들 수 있는가 | 통과 — tab-specific PreWrap branch에서 BreakSpaces를 분리하며 tab은 error로 거부한다. |
| 17 | anonymous Grid path 밖의 arbitrary CSS/root HTML admission이 넓어질 수 있는가 | 통과 — 기존 opt-in C11.4 fixture allowlist만 확장하고 root HTML/general CSS는 제외한다. |
| 18 | 공백-only direct Text가 Grid item 또는 auto-placement cell을 만들 수 있는가 | 통과 — whitespace-only와 following-block marker 두 negative control을 둔다. |
| 19 | fixture FFI marker/source가 default product archive로 유출될 수 있는가 | 통과 — default negative와 fixture-test positive archive comparison을 요구한다. |
| 20 | 제한된 fixture 결과가 C11.4/C11 완료나 공식 72/213 증가로 과장될 수 있는가 | 통과 — 계획은 상위 checkbox와 집계를 유지하고 미지원 범위를 반복해서 명시한다. |

## 판정

20개 계획 실패 경로를 검토했다. 기준 입력·계산·예상 결과·fail-closed 경계가 분리돼 있다. 이 검토는 계획을 평가할 뿐 구현, 앱 runtime, WPT 또는 제품 CSS 지원을 입증하지 않는다. 구현 뒤에는 별도의 새로운 20개 runtime 관점 검토와 PR diff 30개 검토가 필요하다.
