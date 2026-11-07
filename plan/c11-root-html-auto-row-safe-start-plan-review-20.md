# `align-content:safe|unsafe start` 계획 실패 관점 검토 20개

**대상:** [C11.4 parsed Root HTML `align-content:safe|unsafe start` 계획](./c11-root-html-auto-row-safe-start.md) 및 [Chrome 157 사전 비교](../spec/internal/evidence/c11-root-html-auto-row-safe-start-precomparison-2026-11-07/README.md).

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | CSS Box Alignment의 안전 정렬 정의가 독립 기대값과 맞지 않는가 | 통과: start alignment의 computed value와 geometry를 별도로 기록했다. |
| 2 | computed `safe start`가 `start`로 정규화되어 value identity가 사라지는가 | 통과: browser observation은 `safe start`를 그대로 보존한다. |
| 3 | computed `unsafe start`가 `start`로 정규화되어 value identity가 사라지는가 | 통과: browser observation은 `unsafe start`를 그대로 보존한다. |
| 4 | positive free-space에서 start와 safe/unsafe start가 갈리는가 | 통과: 세 값의 frames가 `[0,10,20]px`로 exact 비교됐다. |
| 5 | overflow에서 unsafe start가 끝 정렬로 잘못 배치되는가 | 통과: 세 start 계열이 `[0,56,112]px`로 관측됐다. |
| 6 | `safe start` fallback이 본래 start placement와 달라지는가 | 통과: positive/overflow 모두 plain start와 동일하다. |
| 7 | alignment가 intrinsic auto-row track sizes를 변형하는가 | 통과: 세 scene에서 used rows `[10,10,10]px`를 확인한다. |
| 8 | 계산식 row-gap의 percentage basis가 달라지는가 | 통과: definite 40px basis와 두 46px gutters를 고정했다. |
| 9 | item source order 또는 identity가 변하는가 | 통과: fixed item, text A, text B 순서를 고정하고 각 frame을 비교한다. |
| 10 | supported value 허용이 generic CSS admission으로 퍼지는가 | 통과: exact fixed profile의 두 computed value만 허용한다. |
| 11 | style projection과 layout matcher의 허용값이 어긋나는가 | 통과: 계획에 두 경계의 동일 allowlist를 명시했다. |
| 12 | alignment direction에 따른 writing-mode semantics를 과장하는가 | 통과: horizontal-tb/LTR fixed profile 밖은 제외한다. |
| 13 | baseline 또는 distributed alignment가 우발 포함되는가 | 통과: 해당 값을 negative controls로 유지한다. |
| 14 | unsupported error가 다른 property에 귀속되는가 | 통과: `align-content` attribution과 computed value를 명시적으로 검증한다. |
| 15 | unsupported failure 뒤 partial frame/snapshot이 publish되는가 | 통과: RuntimeSession atomic rejection을 gate로 둔다. |
| 16 | 오류 뒤 같은 session의 회복을 검사하지 않는가 | 통과: 다음 supported alignment로 same-session recovery를 포함한다. |
| 17 | 앞선 safe/unsafe center·end 구현이 회귀하는가 | 통과: 두 bounded feature의 focused regressions를 요구한다. |
| 18 | OS screenshot을 표면 readback이나 glyph pixel 검증으로 확대 해석하는가 | 통과: screenshot은 visual bounds에 한정한다. |
| 19 | 앱 설치물이나 빌드 산출물이 검증 뒤 변경된 채 남는가 | 통과: app/build restoration hashes를 gate에 포함한다. |
| 20 | 하위 profile 구현을 C11.4/C11 완료로 승격하는가 | 통과: 체크박스와 공식 `78/214`를 그대로 둔다. |

Chrome 비교는 계획 전 oracle 근거다. 이 리뷰는 아직 구현·RuntimeSession·시뮬레이터 검증을 수행했다는 주장이 아니다.
