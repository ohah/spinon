# PR #1313 Tailnet 동기화 증거 diff · 실패 관점 20개

검토 대상은 이번 동기화 evidence directory 전체다. live Tailnet 게시 작업 자체의 검토는 [postmerge-review-20.md](./postmerge-review-20.md)에 별도로 기록했다.

| # | 실패 관점 | 검토 내용 | 판정 |
|---:|---|---|---|
| 1 | merge commit 오기록 | PR #1312 merge SHA와 source commit 일치 | 통과 |
| 2 | merge time 혼동 | GitHub UTC `merged_at`을 기록 | 통과 |
| 3 | CI 결과 과장 | check rollup이 비어 있어 green을 주장하지 않음 | 통과 |
| 4 | 완료 수 변경 암시 | `78/214` 유지와 상위 미완료를 기록 | 통과 |
| 5 | 계획을 구현으로 격상 | 새 slice를 구현 전으로 명시 | 통과 |
| 6 | Chrome 사전 비교를 runtime으로 표현 | README와 PR 설명에서 browser-only 범위를 분리 | 통과 |
| 7 | selected build를 전체 docs build로 표현 | selected root와 dead-link 검사 제외를 명시 | 통과 |
| 8 | 전역 dead-link 검사 성공으로 읽힘 | 전체 docs build failure와 제한을 별도 서술 | 통과 |
| 9 | Tailnet 서버 재시작 누락 | PID 28040 유지와 restart false를 기록 | 통과 |
| 10 | 변경 전 파일 복구 근거 부재 | 교체 11개 경로의 backup hash를 보존 | 통과 |
| 11 | 게시 중 파일 삭제 | no-delete 상태와 route 수를 기록 | 통과 |
| 12 | preflight와 result 경로 수 불일치 | 53 route, 29 add/11 replace/13 same 대조 | 통과 |
| 13 | local hash를 Tailnet hash로 혼동 | local·Tailnet 제공 검증을 같은 JSON에서 구분 | 통과 |
| 14 | 새 plan 링크가 빠짐 | Tailnet roadmap DOM의 exact href를 기록 | 통과 |
| 15 | mobile page overflow 숨김 | documentWidth `390`을 저장 | 통과 |
| 16 | 화면 이미지가 손상됐는데 성공 처리 | broken image 목록과 Chrome sample natural size 저장 | 통과 |
| 17 | screenshot이 main merge와 분리 | desktop/mobile 캡처와 source commit을 연결 | 통과 |
| 18 | simulator가 이번 검증에 사용된 것으로 오해 | simulatorStarted false와 부팅 장치 없음 기록 | 통과 |
| 19 | 이 검증에서 만든 browser process 방치 | 세 session ID와 closed state를 JSON에 기록 | 통과 |
| 20 | temporary builder files를 불필요하게 남김 | build staging과 one-off config 삭제 확인 | 통과 |

**결과:** 20개 diff 실패 관점을 확인했다. 이 evidence PR은 동기화 증거와 캡처만 추가하며 제품 동작·공식 완료 상태를 변경하지 않는다.
