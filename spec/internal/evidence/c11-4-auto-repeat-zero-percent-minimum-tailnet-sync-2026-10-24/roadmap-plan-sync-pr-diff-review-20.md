# C11.4 roadmap 계획 요약 동기화 · PR 변경 검토 20개

검토 대상은 `plan/c11-4-grid-track-sizing.md`의 auto-repeat `0%` minimum 마지막 항목이다. 병합된 PR #573 코드, 집중 계획, runtime 근거, `spec/STATUS.md`, 공개 roadmap의 실제 기록과 대조했다.

| # | 적대적 질문 | 확인 결과 |
|---:|---|---|
| 1 | 아직 계획 단계인 것으로 남아 있는가? | PR #573 구현과 두 simulator의 runtime 검증을 완료 상태로 반영했다. 통과. |
| 2 | `Percentage(0)`을 모든 CSS author 입력에 지원한다고 과장하는가? | 기존 opt-in profile의 제한 gate만 설명하며 일반 CSS admission을 주장하지 않는다. 통과. |
| 3 | `calc(0px + 0%)` authored syntax를 별도 보존한다고 잘못 설명하는가? | Stylo typed `Percentage(0)` normalization 뒤 같은 경로를 탄다고 한정했다. 통과. |
| 4 | 새 calc evaluator가 추가된 것처럼 쓰는가? | 구현 계획과 코드가 evaluator를 추가하지 않았으며 문서도 이를 명시한다. 통과. |
| 5 | Taffy의 1px floor가 실제 zero minimum에 적용된다고 오해할 수 있는가? | repeat count 계산에만 적용한다고 구별했다. 통과. |
| 6 | flex sizing이나 auto-fit collapse까지 floor로 바뀐다고 쓰는가? | 실제 zero minimum, flex sizing, collapse가 유지된다고 적었다. 통과. |
| 7 | subpixel explicit neighbors도 floor 대상이라고 넓히는가? | explicit subpixel neighbors의 동작을 유지한다고 범위를 밝혔다. 통과. |
| 8 | 유한 값 조건이 빠져 있는가? | `Percentage(0)` admission의 finite 조건과 typed gate를 유지한다고 집중 계획·상태 대장에서 확인했다. 통과. |
| 9 | indefinite/cyclic axis도 검증된 것으로 확대하는가? | definite-positive-content-axis 제한을 적고 계획의 fail-closed 경계를 유지했다. 통과. |
| 10 | supported maximum 및 min/max-size gate를 누락하는가? | 상위 gate 이름을 요약에 보존했다. 통과. |
| 11 | Chrome oracle가 13 scene인지 확인했는가? | 집중 계획과 Chrome 사전 비교 README가 동일한 13 scene을 가리킨다. 통과. |
| 12 | Android 결과를 실기기 결과로 표현하는가? | API 37 ARM64 AVD라고 구체적으로 표기하고 실기기는 미검증으로 남겼다. 통과. |
| 13 | iOS Simulator의 기기·OS 범위를 정확히 적었는가? | iPhone 17 / iOS 26.2 Simulator로 runtime README와 맞췄다. 통과. |
| 14 | 각 simulator의 13/13이 node frame만 통과한 것으로 축소되는가? | tracks, collapse, trace chunks와 draw/present도 함께 적었다. 통과. |
| 15 | screenshot을 pixel equality 증거로 오해할 수 있는가? | PixelCopy/Metal pixel equality가 검사되지 않았다고 분명히 했다. 통과. |
| 16 | compiler/test 실행을 문서 밖에서 추정했는가? | Workspace tests, Clippy, fmt와 fixture-boundary 결과가 runtime evidence의 로그를 가리킨다. 통과. |
| 17 | static archive 검사를 APK/IPA 전체 fixture exclusion으로 확대하는가? | static archive marker 경계로만 표현해 artifact 한계를 보존했다. 통과. |
| 18 | PR #573 merge 시각이나 SHA가 다른가? | `gh pr view 573`와 공식 상태 기록의 `2026-10-23T17:19:38Z`, `a97324c1…`를 대조했다. 통과. |
| 19 | 빈 check rollup을 CI 통과로 읽도록 쓰는가? | rollup 공란과 main required status check 미설정을 그대로 구분했다. 통과. |
| 20 | C11.4 또는 공식 완료 수가 잘못 바뀌는가? | C11.4 상위 미완료와 공식 `72/213` 유지가 상태 대장·공개 roadmap과 일치한다. 통과. |

20개 관점에서 수정할 추가 불일치는 발견하지 않았다. 이 문서 수정은 계획 요약을 최신 코드와 맞추며 구현 범위나 공식 완료 수를 변경하지 않는다.
