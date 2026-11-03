# C11.1 calc implicit rows 병합 후 문서 동기화 검토 30개

이 review는 PR #1102 병합 뒤 plan·상태 대장·내부 계약에서 candidate/pending 문구를 merged state로 바꾸는 diff만 대상으로 한다. PR merge 정보는 GitHub [PR #1102](https://github.com/ohah/spinon/pull/1102)의 실제 상태와 대조했다.

| # | 문서 동기화 실패 관점 | 결과 |
|---:|---|---|
| 1 | PR #1101의 계획 병합 기록이 사라지는가 | 유지된다. |
| 2 | 구현 PR 번호를 다른 C11 slice의 PR로 연결하는가 | #1102로 연결된다. |
| 3 | 병합 시각이 GitHub와 다른가 | `2026-11-03T02:59:45Z`로 일치한다. |
| 4 | Rebase and merge 방식이 기록과 다른가 | PR merge 방식과 같다. |
| 5 | merge SHA가 GitHub 상태와 다른가 | `7464e2ca469e5e7b7f35fc837f9adfedca6a0b3a`로 일치한다. |
| 6 | 계획 상태가 여전히 implementation pending이라고 하는가 | merged limited implementation으로 바뀌었다. |
| 7 | 상태 대장에 구현 PR pending이 남는가 | #1102 merged 기록으로 바뀌었다. |
| 8 | 상태 기록이 C11 parent가 아닌 다른 상위 항목 아래 들어가는가 | C11 하위의 날짜 기록으로 남는다. |
| 9 | C11 공식 checkbox가 실수로 체크되는가 | `[ ]` 그대로다. |
| 10 | global complete count가 바뀌는가 | `72/213` 그대로다. |
| 11 | 제한 slice를 C11.1 전체 완료로 승격하는가 | merged limited implementation으로만 표현한다. |
| 12 | 제한 slice를 상위 C11 전체 완료로 승격하는가 | 상위 C11은 미완료로 유지한다. |
| 13 | 내부 계약 숫자 버전이 임의 상승하는가 | `0.1.0` 그대로다. |
| 14 | 계약 제목이 candidate 상태로 남거나 전체 지원으로 읽히는가 | merged limited implementation으로 표시한다. |
| 15 | 계약의 Root HTML profile 경계가 PR 범위와 어긋나는가 | 같은 `RuntimeRootHtmlTextPaintV1` 경계다. |
| 16 | calc grammar가 모든 math tree로 확대되는가 | 두 항 px/% sum만 허용한다. |
| 17 | percentage basis를 height 외 축으로 잘못 기록하는가 | definite content height로 기록한다. |
| 18 | `min-height`/`max-height`/aspect ratio 거부가 누락되는가 | 세 제약을 모두 유지한다. |
| 19 | negative range clamp가 문서에서 사라지는가 | 0px clamp가 남아 있다. |
| 20 | 100% 초과 overflow가 clamp된다고 오기하는가 | overflow 보존으로 기록한다. |
| 21 | row-gap이 percentage basis에 포함된다고 오기하는가 | 별도 track gutter로 설명한다. |
| 22 | f32 aggregate 검사가 row/gap extent 일부만 대상으로 바뀌는가 | track list와 gap aggregate 경계를 유지한다. |
| 23 | `min()`/`max()`/`clamp()`가 새 지원에 포함되는가 | 제외 범위에 남아 있다. |
| 24 | named-area, implicit columns, dense/column flow가 확대 포함되는가 | 모두 제외 범위에 남아 있다. |
| 25 | 공개 CSS 지원/API 또는 WPT 적합성을 선언하는가 | 선언하지 않는다. |
| 26 | Android/iOS 결과가 pixel exact로 과장되는가 | draw/present 시각 smoke와 readback 한계를 유지한다. |
| 27 | 실행 evidence 링크가 실제 존재하지 않는 경로를 가리키는가 | README, logs, screenshots가 PR #1102에 병합된 경로와 일치한다. |
| 28 | GitHub check 기록을 CI 통과로 서술하는가 | PR check rollup 부재를 명시하고 CI 통과를 주장하지 않는다. |
| 29 | main required status checks 존재 여부가 현재 설정과 다른가 | branch protection 조회상 required checks 설정이 없다. |
| 30 | Tailnet/public roadmap가 이미 갱신된 것처럼 오해시키는가 | 이 문서 변경은 공개 사이트 동기화를 주장하지 않는다. 공개 preview는 별도 확인 후 기록한다. |

30개 문서 동기화 실패 관점에서 불일치를 찾지 못했다. 공개 Tailnet preview 업데이트와 화면 검증은 이 docs diff에 포함하지 않았다.
