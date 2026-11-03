# C11.1 `grid-auto-columns: calc()` 병합 후 계획·로드맵 변경 검토 30개

검토 대상은 구현 PR #1097 병합 후 변경한 C11 전용 계획, 상위 C11 요약, 공개 roadmap source와 이 검토 기록이다. 구현 계약이나 공식 완료 상태는 수정하지 않았다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR이 실제 `MERGED` 상태인가 | GitHub PR #1097 상태와 병합 시각을 확인했다. |
| 2 | merge SHA가 실제 main tip 계보와 맞는가 | `21d7ed0eb0c5a124133db3b75cb4504c229cf14f`를 GitHub merge result와 origin/main에서 대조했다. |
| 3 | 병합 날짜와 UTC 시각이 일치하는가 | PR `mergedAt`의 `2026-11-03T01:56:55Z`를 source entry에 반영했다. |
| 4 | 공식 완료 수가 오르지 않았는가 | 계획과 roadmap 모두 `72/213`을 유지한다. |
| 5 | C11.1 checkbox를 완료로 바꾸지 않았는가 | `spec/STATUS.md`를 변경하지 않았고 계획은 상위 미완료를 명시한다. |
| 6 | C11 parent를 완료로 과장했는가 | 제한 parsed Root HTML slice라고 쓰고 C11 전체 미완료를 함께 표시한다. |
| 7 | 지원 profile을 일반 CSS admission으로 확대했는가 | sparse column-flow + definite-width 조건을 명시했다. |
| 8 | calc shape를 임의 CSS math로 확대했는가 | 유한 px 하나와 percentage 하나의 typed 2항 sum으로 제한했다. |
| 9 | `grid-auto-rows`까지 지원된다고 암시하는가 | 계획과 roadmap 모두 `grid-auto-columns`만 표기한다. |
| 10 | dense/row/nested flow가 범위에 들어간 것처럼 보이는가 | 계획의 남은 범위와 지원 profile에서 제외한다. |
| 11 | Browser oracle를 규범 기대값으로 혼동하는가 | 계획은 독립 산술/명세와 Chrome 관측을 분리한다. |
| 12 | runtime simulator 결과를 CI로 부르는가 | GitHub check 부재를 명시한다. |
| 13 | PixelCopy 또는 Metal readback을 주장하는가 | roadmap은 visual smoke와 surface pixel readback을 구분한다. |
| 14 | iOS 결과를 hardware device 결과로 읽게 되는가 | iPhone Simulator 명칭을 정확히 썼다. |
| 15 | Android AVD를 실기기로 오해할 수 있는가 | API 37 ARM64 AVD라고 적었다. |
| 16 | 15-box는 layout인지 pixel sample인지 모호한가 | layout/draw/present로 기록하고 pixel sample은 별도로 부인한다. |
| 17 | PR evidence README 링크 route가 올바른가 | source 경로를 public `.html` route로 연결했다. |
| 18 | 전용 plan 링크의 이름과 route가 맞는가 | `c11-root-html-grid-column-flow-calc-auto-columns.html`을 실제 source와 대조했다. |
| 19 | runtime 30-review 링크가 존재하는가 | 구현 diff review 파일 경로를 repository source와 대조했다. |
| 20 | screenshot 링크가 올바른 플랫폼 파일을 가리키는가 | Android AVD와 iPhone Simulator 각 파일명을 대조했다. |
| 21 | roadmap 최신 항목의 순서가 병합 시각을 반영하는가 | 01:56Z PR #1097 entry를 같은 날짜의 이전 #1093 entry 앞에 뒀다. |
| 22 | #1093의 과거 병합 상태를 덮어썼는가 | #1093은 별도의 직전 항목으로 보존한다. |
| 23 | 상위 C11 계획이 최신 PR을 최신 slice로 가리키는가 | #1097을 최신 제한 구현으로 추가하고 #1093을 직전으로 내렸다. |
| 24 | 전용 calc 계획이 checkout 후보 상태로 남아 있는가 | 실제 merge SHA와 PR 링크를 반영했다. |
| 25 | source roadmap 항목이 C11.1 전체 지원을 선언하는가 | 제목과 본문에 제한 구현이며 C11.1/C11 미완료임을 적었다. |
| 26 | internal contract 0059를 무단으로 완성 계약으로 수정했는가 | contract 파일은 변경하지 않았다. |
| 27 | 공식 STATUS 근거가 source와 모순되는가 | 공식 상태 파일은 untouched이고 집계 `72/213`과 일치한다. |
| 28 | PR check rollup의 빈 상태를 성공으로 간주하는가 | source entry는 check 부재만 말하고 CI 통과를 주장하지 않는다. |
| 29 | root HTML demo toggle을 지원 계약으로 노출했는가 | 계획/roadmap은 fixture-test 경로를 제품 API로 기술하지 않는다. |
| 30 | 로컬 source 편집만으로 Tailnet 제공 완료를 선언하는가 | 이 검토는 source diff만 판정하며 Tailnet 동기화는 별도 served-hash 검증을 요구한다. |

## 결과

이 변경에서 해결되지 않은 문서/범위 모순은 발견하지 못했다. Tailnet preview가 제공하는 실제 파일은 별도 동기화 단계에서 HTTP status, byte count, SHA-256과 Agent Browser 화면으로 확인해야 한다.
