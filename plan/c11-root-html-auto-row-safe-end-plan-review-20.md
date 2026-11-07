# `align-content:safe|unsafe end` 계획 실패 관점 검토 20개

검토 대상은 [C11.4 계획](./c11-root-html-auto-row-safe-end.md)과 Agent Browser/독립 기대값 [사전 비교](../spec/internal/evidence/c11-root-html-auto-row-safe-end-precomparison-2026-11-07/README.md)다. 각 항목은 서로 다른 계약·재현 실패 경로를 확인한다.

| # | 실패 관점 | 결과 |
|---:|---|---|
| 1 | C11.4/C11 상위 완료를 부분값 추가로 오표시하는가? | 통과 — 공식 상위 체크와 `78/214`는 그대로 미완료라고 명시했다. |
| 2 | 이미 병합된 safe/unsafe center를 다시 계획·구현 대상으로 잡는가? | 통과 — `#1249`는 선행 경로와 overflow 회귀값으로만 참조한다. |
| 3 | 후보 allowlist가 `safe|unsafe end` 두 computed 값보다 넓어지는가? | 통과 — 이 두 exact serialization만 추가한다고 제한했다. |
| 4 | 일반 authored CSS 파서·admission 확장으로 오해되는가? | 통과 — 현재 fixed Root HTML profile 내부 projection만 대상이다. |
| 5 | positive free-space에서 `safe end`가 start로 잘못 동작하는가? | 통과 — 10px positive free-space의 `[10,20,30]`을 고정했다. |
| 6 | positive free-space에서 `unsafe end`가 center처럼 오해되는가? | 통과 — plain `end`와 같은 기대 좌표 및 regression 대조를 둔다. |
| 7 | overflow에서 `safe end`가 end 오프셋을 유지하는가? | 통과 — start fallback `[0,56,112]`을 기대값으로 명시했다. |
| 8 | overflow에서 `unsafe end`가 safe fallback으로 뭉개지는가? | 통과 — `[-82,-26,30]`으로 unsafe 차이를 고정했다. |
| 9 | center 회귀 컨트롤이 end 변화에 의해 깨지는가? | 통과 — safe/unsafe center 각각 기존 기대값을 포함한다. |
| 10 | alignment subject에서 gutters를 빠뜨려 overflow 크기를 틀리는가? | 통과 — 3 tracks + 2 gutters = 122px 산식을 명시했다. |
| 11 | row-gap percentage basis가 명확하지 않은가? | 통과 — definite 40px block-size와 기존 `calc(120% - 2px)`를 고정했다. |
| 12 | track sizing이 alignment modifier와 함께 바뀌는가? | 통과 — `[10,10,10]` tracks와 46px gutters 불변을 게이트로 둔다. |
| 13 | viewport, item count, order 또는 ownership가 바뀌어 다른 profile을 측정하는가? | 통과 — fixed three-item shape와 기존 root/item 조건을 그대로 사용한다. |
| 14 | `safe start` 및 `unsafe start`까지 우발 지원하는가? | 통과 — unsupported negative controls로 남겼다. |
| 15 | modifier를 분배 alignment 또는 baseline 값에 잘못 허용하는가? | 통과 — safety+distributed/baseline 조합은 거부로 고정한다. |
| 16 | Style projection과 layout이 다른 allowlist를 갖는가? | 통과 — 양 계층의 exact admission 일치를 검증한다. |
| 17 | 실패 뒤 RuntimeSession이 partial state에 갇히는가? | 통과 — no partial publication 및 동일 session 복구를 요구한다. |
| 18 | screenshot을 surface pixel parity로 과장하는가? | 통과 — screenshots는 visual-only이며 PixelCopy/Metal readback이 아니라고 썼다. |
| 19 | AVD/iOS Simulator 결과를 실기기·hardware GPU로 확대하는가? | 통과 — 대상과 검증 한계를 별도 기술했다. |
| 20 | plan, precomparison, test, merge 결과를 C11 완료와 혼동하는가? | 통과 — oracle-only 사전 자료와 미실행 구현 gate를 구분했다. |

이 검토는 계획 변경에 대한 사전 검토다. 실제 구현/runtime review 20개와 최종 PR diff 검토 30개를 대신하지 않는다.
