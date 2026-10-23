# C11.4 auto-repeat calc 최소값 계획 PR diff 적대 검토 30개

검토 대상은 계획 PR의 최종 변경 파일과 링크·fixture·oracle 자료다. 계획 자체 실패 관점 20개 검토 및 구현/runtime 20개 검토와 별개로 수행한다.

| # | 독립 실패 관점 | diff 대조와 판정 |
| --- | --- | --- |
| 1 | 새 계획이 기존 C11.4 parent의 좁은 후보와 충돌한다. | parent 최상단에 계산식 최소값을 별도 제한 후보로 연결하고 동일 경계임을 확인했다. |
| 2 | parent의 area 조합 요약이 이미 병합된 PR을 계획-only로 남긴다. | #539/#545/#549의 현재 병합 상태와 범위로 요약을 갱신했다. |
| 3 | 과거 시점의 완료 수를 현재 count로 덮어쓴다. | 새 최신 기록은 공식 72/213을 사용하고 과거 날짜의 수치는 소급 변경하지 않았다. |
| 4 | C11.4 checkbox가 하위 fixed fixture를 전체 완료로 과장한다. | checkbox를 미완료로 유지하고 새 STATUS 항목에 구현 미착수라고 적었다. |
| 5 | public roadmap 완료 수가 하위 구현 때문에 증가한다. | plan과 spec 모두 72/213 유지, public complete ID를 추가하지 않았다. |
| 6 | 내부 계약 버전이 기능 추가만으로 상승한다. | `0.1.0`을 그대로 유지하도록 명시했다. |
| 7 | 2025 CRD 대신 moving draft에 기대값을 묶는다. | 고정 W3C 2025-03-26 CRD 링크와 section을 연결했다. |
| 8 | `<fixed-size>` grammar가 잘못 인용된다. | §7.2.1과 §7.2.3.2 및 `minmax(fixed breadth, track breadth)` 허용을 대조했다. |
| 9 | `calc()`로 repeat count에 쓰일 수 없는 intrinsic minimum을 허용한다. | 구현 범위를 positive absolute-length calc minimum으로 제한했다. |
| 10 | max `1fr`를 definite repeat-count 기준으로 처리한다. | 독립 계산은 40px minimum으로 count를 정하고 남는 공간만 fr에 나누게 고정했다. |
| 11 | 184/40을 올림해 5회 반복한다고 착각한다. | definite maximum container에서 4개의 최소폭 track만 들어가는 계산과 Chrome 관찰을 기록했다. |
| 12 | 24px free space 분배가 빠지거나 잘못된다. | 네 track에 6px씩 더해 46px 배열을 독립적으로 계산했다. |
| 13 | gap이 count에 섞여 Chrome scene과 expected values가 달라진다. | 모든 geometry fixture의 gap을 0으로 명시했다. |
| 14 | Chrome 시각 페이지와 canonical CSS fixture가 서로 다른 입력으로 기록된다. | README에서 두 파일 역할을 구분하고 정확 JSON 관측 원본을 canonical fixture로 지정했다. |
| 15 | Agent Browser 버전이나 Chrome 엔진 정보가 불명확하다. | Agent Browser 0.31.1과 UA `HeadlessChrome/156.0.0.0`을 기록했다. |
| 16 | viewport/DPR 차이가 frame 결과를 바꾼다. | 관찰 JSON에 1280×633 CSS px과 DPR 1을 기록했다. |
| 17 | columns만 검사하고 rows preflight 비대칭을 놓친다. | canonical fixture와 JSON에 두 축의 여섯 scene을 포함했다. |
| 18 | row item이 행이 아니라 새 implicit column으로 배치된다. | row fixture의 `grid-auto-flow: column`과 Chrome child frame을 대조했다. |
| 19 | auto-fill control이 calc 표현과 다른 track 값으로 사용된다. | columns/rows 각 축에서 fixed 40px control이 calc scene과 동일한 46px 배열임을 기록했다. |
| 20 | auto-fit을 placement 전에 collapse한다. | 한 item이 첫 반복 track을 점유하고 세 빈 track을 접는 used tracks와 frame을 기록했다. |
| 21 | child frames를 독립 판정에서 빠뜨린다. | six scene 전부 root/child frames를 Chrome JSON과 spec-derived JSON에 저장했다. |
| 22 | JSON scene ID와 fixture ID가 불일치한다. | 여섯 이름을 정확히 일치시켜 fixture·Chrome observation·expectation에 사용했다. |
| 23 | root coordinates와 root-relative item frames를 혼합한다. | Chrome JSON은 viewport 좌표, plan·independent JSON은 root-relative frame임을 구별했다. |
| 24 | screenshot이 측정값의 출처로 오해된다. | screenshot은 시각 설명으로 한정하고 JSON과 독립 계산을 판정 자료로 구분했다. |
| 25 | screenshot hash가 이전 파일을 가리킨다. | PNG를 다시 hash하고 SHA manifest를 새 관찰 JSON에 맞춰 갱신했다. |
| 26 | precomparison을 Spinon 결과로 잘못 인용한다. | README와 `spec/0001`에서 implementation 전 oracle임을 명시했다. |
| 27 | 고정 fixture가 일반 CSS admission을 약속한다. | plan/spec/status에서 opt-in profile과 제한 범위만 주장했다. |
| 28 | percent, zero, negative, overflow 계산식이 조용히 추가 허용된다. | 모두 scope exclusion 또는 fail-closed rejection으로 명시했다. |
| 29 | 계획 검토와 계획 PR diff 검토가 같은 횟수로 합쳐진다. | plan 자체 20과 별도 PR diff 30 경로를 문서에서 분리했다. |
| 30 | 구현 전에 후속 검증·브라우저·저장소 안전 조건이 빠진다. | simulator/artifact gates, main checkout branch, Agent Browser 종료, Tailnet sync와 Rebase merge를 계획에 기록했다. |
