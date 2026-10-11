# C12.4 iOS runtime matrix 계획 · 실패 관점 검토

**기준:** [실행 계획](../../../plan/c12-4-ios-runtime-matrix.md)
**검토일:** 2026-10-11
**상태:** 계획 경계 보완 후 구현 가능

| # | 독립 실패 관점 | 판정 및 방지 계약 |
| ---: | --- | --- |
| 1 | Android와 다른 fixture 입력을 실행 | 공통 generator JSON과 inventory hash를 그대로 사용한다. |
| 2 | 앱 sandbox에 남은 이전 matrix 결과를 새 결과로 판정 | 매 실행 새 run ID, 결과 파일 제거, 결과의 run ID 일치를 요구한다. |
| 3 | Simulator가 다른 기기/OS인데 고정 결과로 기록 | UDID·기기명·iOS runtime을 고정 확인하고 기록한다. |
| 4 | 한 case의 HostDocument/cache가 다음 case에 잔류 | 각 case에 새 host, 성공·오류 모두 `defer` 해제한다. |
| 5 | host 생성 실패를 CSS의 기대 reject로 분류 | 생성 오류는 case 실패이며 CSS 거부와 분리한다. |
| 6 | 환경 설정 실패를 source 평가 결과로 오인 | viewport 설정 status가 0일 때만 evaluate한다. |
| 7 | supported case의 JS 성공만 보고 layout 성공 처리 | `status=0`과 `layout=ready`를 모두 요구한다. |
| 8 | rejected case가 예상과 다른 속성으로 거부 | `node`, 정확한 property 또는 정확한 `showModal` TypeError 진단을 검사한다. |
| 9 | 결과 수만 같고 case가 중복·누락 | 26개 ID 집합, 고정 index, 유일성, 요약을 비교한다. |
| 10 | 77개 source node 중 일부 frame 누락 | case별 fixture node ID와 runtime node ID의 완전 대응 및 frame 개수를 확인한다. |
| 11 | frame 인덱스나 node ID가 중복 | 모든 frame을 파싱하고 인덱스·node ID의 중복을 거부한다. |
| 12 | NaN/Infinity/비정상 숫자가 오차 비교를 우회 | 모든 좌표 및 크기가 유한한 수인지 확인한다. |
| 13 | 평균 오차가 단일 큰 오차를 숨김 | 각 노드의 x/y/width/height 절대 오차를 개별 검사한다. |
| 14 | root/stage/style wrapper 포함 차이로 frame 수가 오판 | source에 따라 root·stage·style wrapper를 명시적으로 계산한다. |
| 15 | stage origin이 빠져 절대 좌표가 어긋남 | Chromium fixture 기준 frame에 같은 stage x/y offset을 적용한다. |
| 16 | iOS scale factor가 CSS px 비교를 오염 | 환경 scale은 1로 고정하고 결과 단위를 CSS px로 기록한다. |
| 17 | log/JSON truncate 또는 한 case 예외가 부분 통과 | 원자 파일 기록, 정확한 case 수와 완료 summary를 모두 요구한다. |
| 18 | JSON 파일 쓰기 전 이전 run이 발견됨 | 입력·출력 파일을 분리하고 실행 전에 출력 삭제, run ID 검증을 한다. |
| 19 | iOS/Android 비교 결과를 하드웨어 GPU 또는 pixel 비교로 과장 | 결과 metadata에 Simulator 및 geometry-only 범위를 고정한다. |
| 20 | 26-case pass를 WPT/C12.4 전체 완료로 승격 | 상태 문서에서 matrix 결과만 체크하고 WPT·pixel·실기기 등 미검증 경계를 그대로 남긴다. |

## 계획 수정 후 판정

초안에서 구분이 약했던 실행 산출물 재사용, host 생성과 CSS reject 구분, Simulator 식별, CSS pixel 단위, 검증 범위 과장을 위 계약에 추가했다. 나머지는 구현/비교기 단계의 구체 테스트 조건으로 옮겼다. 계획 검토에서 구현 차단 사항은 남지 않았다.
