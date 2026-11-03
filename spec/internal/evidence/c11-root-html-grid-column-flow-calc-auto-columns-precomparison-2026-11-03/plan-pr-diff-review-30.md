# C11.1 `calc()` implicit columns 계획 PR 변경분 실패 관점 30개

PR 변경분 범위는 C11 master-plan 갱신, 새 구현 계획, 계획 실패 관점 20개, Chrome fixture, 사전 비교 JSON·scripts·screenshot이다. 각 항목은 변경 내용을 독립적으로 대조했다.

| # | 실패 관점 | 판정 | 확인 |
|---|---|---|---|
| 1 | C11 master plan이 최신 merged slice를 더 오래된 PR로 남기는가? | 통과 | Latest 항목이 PR #1093 `fit-content(px)`다. |
| 2 | PR #1093 merge timestamp가 기록과 다른가? | 통과 | `2026-11-03T00:25:34Z`가 fit-content plan 및 main history와 일치한다. |
| 3 | PR #1093 merge SHA가 잘렸거나 잘못 연결됐는가? | 통과 | `24610c5731b66f076908b2ecf7cd5c4104729369` 전체를 plan에서 대조했다. |
| 4 | Root HTML fit-content scope가 일반 CSS 지원으로 과장되는가? | 통과 | 제한 px 인자, named-area/repeat 조건 및 C11 미완료 경계를 적었다. |
| 5 | Android AVD를 실기기로 부르거나 iOS sample 시점을 숨기는가? | 통과 | AVD/Simulator를 구별하고 plan evidence의 한계를 연결했다. |
| 6 | 미보고 CI를 통과로 표시하는가? | 통과 | #1093 check rollup과 required checks 부재를 명시했다. |
| 7 | 새로운 calc 계획이 이미 구현된 것처럼 master plan에 놓이는가? | 통과 | `다음 계획 (구현 전)` 및 아직 계획 후보임을 표시했다. |
| 8 | C11.1/C11/72-of-213을 앞당겨 체크하는가? | 통과 | master plan, 새 plan 양쪽에서 그대로 미완료로 둔다. |
| 9 | PR #1086이 fit-content보다 새 slice로 보이는가? | 통과 | #1093 latest, #1086 직전 순서를 고쳤다. |
| 10 | #1072/#1078 gap 기록이 삭제되거나 C11 완료로 승격되는가? | 통과 | 두 계획을 이전 slice로 연결하고 미승격 상태를 남겼다. |
| 11 | 새 계획의 기존 direct px/%/fr baseline이 근거 없이 확장되는가? | 통과 | 지원 범위를 기존 sparse column-flow subset에 종속시켰다. |
| 12 | CSS math track grammar가 unsupported feature를 근거로 쓰는가? | 통과 | W3C Grid CRD §7.2.1/§7.6 및 Values 4 §§10.9/10.12를 링크했다. |
| 13 | CSS 표준 링크가 draft인지 최종 표준인지 감춘다? | 통과 | Grid CRD와 Values 4 WD 상태를 각각 제목/날짜에 적었다. |
| 14 | implicit track pattern 시작 인덱스가 명확하지 않은가? | 통과 | 두 explicit columns 뒤 첫 implicit track부터 pattern을 적용한다. |
| 15 | 첫 calc 산술이 잘못됐는가? | 통과 | `10% × 240px + 5px = 29px`다. |
| 16 | 두 번째 calc의 소수 결과를 반올림해 기대값을 바꾸는가? | 통과 | 독립 기대값은 `6.8px` 그대로다. |
| 17 | Pattern cycling 결과에서 네 implicit track 순서가 바뀌는가? | 통과 | `[29,6.8,29,6.8]px`와 전체 6개 used columns를 고정했다. |
| 18 | Column gap을 percentage basis에서 두 번 빼거나 누락하는가? | 통과 | expected starts가 track 폭에 gap을 한 번씩 더한다. |
| 19 | 행 위치/세로 간격이 열 계산에 잘못 묶이는가? | 통과 | rows `[20,40]px`와 starts `[0,23]px`를 독립 기록했다. |
| 20 | Blink subpixel 양자화를 mismatch로 잘못 판정하는가? | 통과 | 실제 `6.796875px`와 `0.003125px` 차이를 `0.02px` 허용치와 비교한다. |
| 21 | negative calc 결과를 unsupported declaration으로만 처리하는가? | 통과 | CSS Values range clamp와 Chrome의 zero track 관측을 별도 고정했다. |
| 22 | 120% 계산 결과를 container 폭으로 자르는가? | 통과 | Chrome used track `289px`와 독립 결과를 기록했다. |
| 23 | definite zero-width basis를 indefinite로 취급하는가? | 통과 | `0px × 10% + 5px`의 used track이 `5px`다. |
| 24 | Calc와 direct `fr`가 같은 list에 있을 때 남는 폭 계산을 놓치는가? | 통과 | calc+fr control과 `[29,43.5,29,43.5]px` 결과를 고정했다. |
| 25 | `%`가 포함된 calc의 width/min/max/aspect-ratio basis 경계를 숨기는가? | 통과 | definite direct px width 및 size constraint rejection을 plan gate로 명시했다. |
| 26 | Calc admission이 임의 tree shape로 넓어지는가? | 통과 | 두 항 signed `Sum`으로 범위를 고정하고 다른 operators를 제외한다. |
| 27 | CSS syntax-invalid fallback과 valid-but-unsupported tree가 혼동되는가? | 통과 | 둘을 서로 다른 projection/layout cases로 요구한다. |
| 28 | Math ID를 다른 element/property에서 재사용해도 성공하는가? | 통과 | node, property, ID owner 및 duplicate/missing checks를 고정했다. |
| 29 | fixture source와 oracle JSON/screenshot의 hash 연결이 빠지는가? | 통과 | source, raw observation, expected, range control 및 screenshot SHA-256을 기록했다. |
| 30 | 계획 PR의 screenshot을 product pixel parity 또는 process leak 근거로 오인하는가? | 통과 | screenshot을 Chrome oracle로만 표시하고 `close --all` 뒤 daemon 종료를 기록했다. |

결함은 발견하지 못했다. `git diff --cached --check`와 JSON syntax validation은 별도 pre-PR check로 확인했다. 이 문서는 plan diff review이며 구현/runtime review와 구현 PR review를 대신하지 않는다.
