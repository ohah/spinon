# 현재 코드·로드맵 동기화 PR diff · 독립 실패 관점 20개

검토 대상은 2026-10-23 문서 변경 diff다. 각 관점은 별도 실패 경로/불변 조건을 확인하며, 기능 구현·런타임을 재검증했다고 주장하지 않는다.

| # | 실패 관점 | 확인 내용 | 판정 |
| --- | --- | --- | --- |
| 1 | 오래된 집계가 현재 수로 남는다 | #553/legacy C13 `74/207` 문구에 실행 당시 스냅샷이라고 표시하고 현재 `72/213`을 함께 둔다. | 통과 |
| 2 | 날짜별 기록의 당시 수치를 삭제해 이력을 훼손한다 | 기존 `74/207`, `77/213`은 dated snapshot으로 보존한다. | 통과 |
| 3 | 상태 페이지 raw checkbox 수를 공식 roadmap 집계로 오해한다 | Agent Browser raw DOM은 226/87이지만 official parser는 78+133+2 작업만 골라 72/213을 표시한다. C11.4/C13 상위 체크는 그대로 미완료다. | 통과 |
| 4 | 전체 C11.4를 완료로 과장한다 | 전체 Grid 지원 미완료·활성 C11.4 판정을 유지한다. | 통과 |
| 5 | 비활성 C13을 현재 작업 lane으로 보인다 | C13 미완료와 inactive 상태, C11.4 active lane을 구분한다. | 통과 |
| 6 | #539 merge SHA가 오타로 실제 PR commit을 가리키지 않는다 | 잘못된 ID가 Git object로 존재하지 않음을 확인하고 GitHub PR SHA `0f46437c299df6f0c069b694a3f2184ac464ad3d`로 고쳤다. | 통과 |
| 7 | #540의 상태 문서 반영 여부를 누락한다 | #540 병합 SHA와 역할을 표에 기록했다. | 통과 |
| 8 | #545를 #539와 같은 변경으로 합산한다 | 후속 양축 조합을 별도 PR·별도 제한 slice로 표시한다. | 통과 |
| 9 | #549 auto-fit 후속을 누락한다 | 별도 계획과 구현 PR을 독립 행으로 남겼다. | 통과 |
| 10 | #553 calc 처리 설명이 새 evaluator 구현으로 바뀐다 | 기존 `LengthPx` gate가 정규화 값을 허용한 범위로 유지한다. | 통과 |
| 11 | #553 simulator 화면을 pixel equality로 오해한다 | PixelCopy/Metal pixel equality를 검사하지 않았다고 명시한다. | 통과 |
| 12 | #561 구현이 일반 percentage/CSS support로 확대된다 | definite axis의 positive percentage subset 및 fail-closed 경계를 남긴다. | 통과 |
| 13 | #561 구현 계획이 계속 구현 전으로 표시된다 | 병합 SHA·16/16 simulator evidence를 연결하고 C11.4 parent는 미완료로 둔다. | 통과 |
| 14 | #568 software renderer 결과를 hardware GPU로 확대한다 | Android ANGLE/SwiftShader 한계를 유지한다. | 통과 |
| 15 | `72/213`을 PR rollup/CI 통과 수로 설명한다 | 상태 집계와 PR checks를 별개 사실로 적었다. | 통과 |
| 16 | check rollup이 비어 있는데 CI 통과를 주장한다 | GitHub 응답의 빈 rollup과 `UNKNOWN`을 기록하고 통과 문구를 쓰지 않는다. | 통과 |
| 17 | 내부 계약 숫자 버전을 올린다 | `0059` numeric version `0.1.0`을 유지한다. | 통과 |
| 18 | evidence 링크가 존재하지 않는 파일을 가리킨다 | 선택 build에서 13개 필수 route를 확인했고 Agent Browser에서 새 evidence route를 열었다. | 통과 |
| 19 | docs build 결과와 Tailnet 제공본이 다르다 | 11개 주 route와 2개 상대경로 alias route에서 HTTPS 200 및 source SHA-256 일치를 확인했다. | 통과 |
| 20 | Agent Browser 확인 뒤 브라우저가 남는다 | pass01 세션을 닫고 `active=false`, `pid=null` 및 영속 session process 부재를 확인했다. | 통과 |

close 결과는 `browser-close-pass01.json`에 기록한다.
