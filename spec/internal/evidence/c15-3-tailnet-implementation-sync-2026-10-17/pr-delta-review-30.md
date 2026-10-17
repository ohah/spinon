# Tailnet 구현 동기화 PR 변경 적대 검토 30개

각 행은 제출 diff의 한 실패 면을 검토한다. 기존 C15.3 완료 경계는 유지하고, 동기화된 문서의 도달 가능성과 복사 안전성을 확인한다.

| # | 검토 관점 | 판정 |
|---:|---|---|
| 1 | 새 evidence README의 목적이 병합된 C15.3 후속 동기화로 한정됨 | 통과 |
| 2 | C15.3 구현 PR 번호가 현재 main history와 일치함 | 통과 |
| 3 | 전체 C15 완료라고 과장하지 않음 | 통과 |
| 4 | 런타임 text admission을 구현했다고 쓰지 않음 | 통과 |
| 5 | glyph paint를 구현했다고 쓰지 않음 | 통과 |
| 6 | Block/Flex/Grid consumer sizing을 검증했다고 쓰지 않음 | 통과 |
| 7 | overflow/clip/hit-test를 통과했다고 쓰지 않음 | 통과 |
| 8 | Android/iOS Simulator를 실행했다고 주장하지 않음 | 통과 |
| 9 | preview가 repo를 직접 제공하지 않는 구조를 문서화함 | 통과 |
| 10 | listener 재시작 금지를 명시함 | 통과 |
| 11 | copy-only 정책을 명시함 | 통과 |
| 12 | preview 전체 삭제를 허용하지 않음 | 통과 |
| 13 | 대상이 다를 때만 backup한다는 규칙을 기록함 | 통과 |
| 14 | backup 검증을 overwrite보다 앞에 둠 | 통과 |
| 15 | source/target/served hash를 분리함 | 통과 |
| 16 | static 전체 트리 복사를 제외함 | 통과 |
| 17 | raw fixture와 rendered docs 경로를 분리함 | 통과 |
| 18 | 고정 글꼴 안내와 실제 폰트 파일 링크를 포함함 | 통과 |
| 19 | 기존 Chrome oracle를 구현 screenshot이라고 표시하지 않음 | 통과 |
| 20 | strict docs build 실패를 숨기지 않음 | 통과 |
| 21 | 선택 렌더링 build의 `/tmp` 보정 범위를 설명함 | 통과 |
| 22 | HTTP 200만으로 내용이 최신이라 단정하지 않음 | 통과 |
| 23 | served byte hash 확인을 요구함 | 통과 |
| 24 | visual screenshot은 Agent Browser 기록으로 남김 | 통과 |
| 25 | Terminal Browser를 사용하지 않음 | 통과 |
| 26 | review 항목을 별도 evidence로 노출함 | 통과 |
| 27 | roadmap와 status 둘 다 동기화 목록에 포함함 | 통과 |
| 28 | 내부 C15 index에서 새 동기화 기록으로 이동 가능함 | 통과 |
| 29 | 내부 로그는 선택 파일만 복사하여 링크 경계를 유지함 | 통과 |
| 30 | 문서 동기화를 제품 구현 또는 배포 완료로 혼동하지 않음 | 통과 |
