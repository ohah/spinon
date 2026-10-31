# computed `word-spacing` 계획 PR diff 검토 20개

대상 diff는 계획, 24-scene precomparison fixture/evidence, 독립 기대값 validator, C11.4 상위 계획 색인 동기화다. 계획 실패 관점 검토 문서와 다른 경로를 점검한다.

| # | 변경 실패 관점 | 판정 |
|---:|---|---|
| 1 | 상위 C11.4 계획 경로가 실제 파일을 가리키는가 | 통과. `plan/c11-4-grid-track-sizing.md`와 status 대장 링크가 존재한다. |
| 2 | internal contract 버전을 올리는가 | 통과. 숫자 버전 0.1.0 유지다. |
| 3 | 제안 계획을 official checkbox로 승격하는가 | 통과. 공식 C11.4/C11 상태 및 72/213은 바꾸지 않는다. |
| 4 | CSS Text 문서 링크가 다른 속성 section을 가리키는가 | 통과. §7.1 word-spacing property 링크다. |
| 5 | CSS Sizing/Grid 규범과 oracle 구분이 빠지는가 | 통과. 서로 다른 normative sources를 연결한다. |
| 6 | font file의 hash가 observation과 맞지 않는가 | 통과. Ahem SHA가 README와 validator에 동일하다. |
| 7 | fixture의 Ahem URL이 깨지는가 | 통과. `/tests/fixtures/css/c11/`에서 `../../resources/c15/Ahem.ttf`를 가리킨다. |
| 8 | repeated-space content의 source string이 바뀌는가 | 통과. `XX   XX XX  `를 fixture·expectation에서 일치시켰다. |
| 9 | expected table을 Chrome output에서 복사하는가 | 통과. font table과 수식으로 validator가 다시 계산한다. |
| 10 | `normal` computed value를 잘못 저장하는가 | 통과. Chrome observation은 0px이며 validator가 이를 확인한다. |
| 11 | computed `0px` authored input을 빼먹는가 | 통과. normal과 별도 scene으로 둔다. |
| 12 | negative length를 positive로 뒤집는가 | 통과. observed -1px track-width 감소를 유지한다. |
| 13 | expected row/column 순서를 섞는가 | 통과. validator가 ID별 columns와 rows를 각각 검사한다. |
| 14 | viewport/DPR/browser 버전이 기록과 다른가 | 통과. 1280×633, DPR 1, Chrome 156을 고정한다. |
| 15 | screenshot을 Spinon output이라고 오인시키는가 | 통과. Chrome oracle screenshot임을 명시한다. |
| 16 | validator 실행 경로가 repo root에서 동작하지 않는가 | 통과. repo-root 상대 경로와 실제 실행 결과를 확인했다. |
| 17 | evidence가 지원 범위를 과장하는가 | 통과. fixture-only, no WPT/runtime/paint proof로 제한한다. |
| 18 | 계획의 24 scene count가 fixture generation과 어긋나는가 | 통과. 4 values × 3 text inputs × 2 intrinsic tracks다. |
| 19 | C11 CSS roadmap summary에 최근 #920 slice가 빠지는가 | 통과. C11.4 summary에 #920와 다음 계획 link를 함께 기록한다. |
| 20 | 계획 검토 표와 PR diff 표를 중복 계산하는가 | 통과. 검토 목적과 실패 경로를 분리해 기록한다. |

## 결과

계획과 oracle artefact 간 scene 수·입력·hash·expected formula를 확인했다. 이 PR은 구현 완료나 CI 통과를 주장하지 않는다.
