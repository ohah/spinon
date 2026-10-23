# C11.4 0% auto-repeat 병합 후 roadmap 동기화 PR 변경 검토 30개

검토 대상은 PR #573 병합 메타데이터를 반영한 `spec/STATUS.md`, `spec/public/roadmap.html`, 집중 계획과 이 폴더의 Tailnet 동기화 근거다. 구현/runtime 20개와 구현 PR diff 30개 검토는 별도 산출물이며 여기서 재사용하지 않았다.

| # | 독립 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 병합 SHA 또는 시간이 추정값으로 기록될 수 있는가? | `gh pr view 573`의 `MERGED`, `2026-10-23T17:19:38Z`, `a97324c1b208e1f9ab58de96e0d9885164365532`와 문서가 일치한다. 통과. |
| 2 | check rollup이 비었는데 CI 통과로 서술하는가? | 문서는 rollup이 비었다고 정확히 적는다. 통과. |
| 3 | required checks 부재를 checks 성공으로 확대하는가? | `main`에 required status checks가 설정되지 않았음을 별도 서술한다. 통과. |
| 4 | 코드 요약이 변경 경계와 어긋나는가? | 제한된 Stylo cascade, layout preflight와 vendored Taffy count 계산 경계를 가리킨다. 통과. |
| 5 | authored `0%`를 유한성·범위 확인 없이 받는가? | 유한 `Percentage(0)`과 기존 nonnegative validation 문맥을 유지한다고 명시한다. 통과. |
| 6 | definite positive axis 요건이 빠졌는가? | definite-axis gate가 계속 적용됨을 계획·로드맵에 기록했다. 통과. |
| 7 | 허용되지 않은 maximum까지 `0%` minimum을 일반 허용하는가? | 기존 supported-maximum gate 안에서 처리한다고 제한했다. 통과. |
| 8 | `calc(0px + 0%)`의 처리 위치를 틀리게 설명하는가? | Stylo 정규화 후 typed `Percentage(0)` 경로라고 기록했다. 통과. |
| 9 | 새 CSS calc evaluator를 추가했다고 과장하는가? | 새 evaluator가 없음을 명시한다. 통과. |
| 10 | UA 1px floor가 실제 track sizing에도 전달되는가? | auto-repeat 개수 계산 전용이라고 명시했고 Taffy 주석·코드와 맞는다. 통과. |
| 11 | 반복 묶음의 각 track과 고정 이웃에 floor를 잘못 적용하는가? | floor 단위가 반복 track별임을 기록한다. Taffy의 고정 이웃 분리 테스트와 맞는다. 통과. |
| 12 | 음수·비유한 track 값이 floor에 의해 유효해지는가? | invalid value는 floor로 치유하지 않는다고 구현 및 근거와 대조했다. 통과. |
| 13 | 원래 zero minimum 또는 flex sizing을 바꾸는가? | 실제 zero minimum과 flex sizing은 변경하지 않는다고 명시했다. 통과. |
| 14 | auto-fit collapse 동작을 바꿨다고 오해하게 하는가? | 기존 collapse 유지 및 runtime matrix 확인 범위를 적었다. 통과. |
| 15 | scene 수 또는 플랫폼 이름이 실행 기록과 다른가? | Android API 37 ARM64 AVD와 iPhone 17/iOS 26.2 Simulator 각각 13/13으로 기록했다. 통과. |
| 16 | frame 외 검사 항목을 빠뜨리거나 완료 범위를 넓히는가? | tracks, collapse, trace, draw/present를 기록하고 scene별로 한정했다. 통과. |
| 17 | Simulator 화면을 pixel parity 증거로 잘못 제시하는가? | pixel equality를 검사하지 않았다고 명시했다. 통과. |
| 18 | 실기기·hardware GPU·WPT·일반 CSS 지원을 주장하는가? | 각각 미검증/범위 밖으로 유지한다. 통과. |
| 19 | 부분 slice가 C11.4 완료로 바뀌는가? | C11.4 상위 항목은 미완료라고 유지한다. 통과. |
| 20 | 공식 완료 합계가 task board와 어긋나는가? | Agent Browser 동적 board에서 213개 중 72개 checked를 확인했다. 통과. |
| 21 | C13이 여전히 현재 작업 lane으로 보이는가? | `inProgress`는 C11.4이고 C13 상위 체크는 미완료·비활성이라고 표시한다. 통과. |
| 22 | C13.3 follow-up PR 상태를 잘못 기술하는가? | PR #185의 live 상태는 `MERGED`이며 로드맵에 낡은 진행 중 표시는 없다. 통과. |
| 23 | 문서가 구현을 포함하지 않은 다른 main revision을 기준 삼는가? | 선택 빌드의 `HEAD`와 `origin/main`이 PR #573 merge SHA와 같음을 manifest에서 확인했다. 통과. |
| 24 | RSPress 산출물이 선택 `public/` 자산을 빠뜨리는가? | public hardlink manifest에서 10,773개 파일을 처리하고 기존 충돌은 hash 비교했다. 통과. |
| 25 | 동기화 경로 수 합계가 맞지 않는가? | 985 changed + 93 unchanged = 선택 1,078 routes다. 통과. |
| 26 | 기존 미리보기 파일을 백업 없이 덮거나 삭제하는가? | 교체 7개를 3,623,232 bytes 백업했고 신규 978개, 삭제 0개다. 통과. |
| 27 | 파일 복사 중 부분 응답 또는 다른 listener로 쓰는가? | 같은 listener PID를 재확인하고 임시 파일 후 rename, 복사 hash 확인을 쓴다. 통과. |
| 28 | 12개 원격 route 확인을 전체 사이트 검증으로 확대하는가? | 보고서와 README가 HTTP 200·hash 확인을 지정된 12개 핵심 route로 한정한다. 통과. |
| 29 | Agent Browser가 stale page나 예전 집계를 보여주는가? | live roadmap/status route에서 #573 SHA, 72/213, C11.4/C13 unchecked를 확인했다. 통과. |
| 30 | simulator image 링크가 깨졌거나 PR 본문에서 플랫폼 캡처를 혼동하는가? | Tailnet 원본과 PR #574 본문에서 세 이미지가 모두 `complete=true`로 표시됐다. roadmap 1280×633, Android 1080×2400, iOS 1206×2622다. 통과. |

30개 관점에서 수정이 필요한 불일치는 발견하지 않았다. 다만 원격 전체 1,078개 route, WPT, pixel equality, 실기기 및 hardware GPU는 검증하지 않았다.
