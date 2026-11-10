# PR #1327 병합 후 Tailnet 재동기화 PR diff 검토 30개

대상은 PR #1327 Rebase and merge 뒤에 merged `main` 기준으로 공개 Tailnet 문서 site를 다시 빌드·동기화하고 pass04 근거를 추가하는 문서 PR이다. pass03는 병합 전 source를 사용했으므로 최종 배포로 주장하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | pass04 source SHA가 병합된 main과 다른가 | 통과. source HEAD는 `e0c904ce35f302847dd90567f68c3f45dc015272`다. |
| 2 | source tree hash가 main과 다른가 | 통과. pass04 tree는 `0f8dc428f093ddacfd3e84d7c37eee39d4ec2a58`다. |
| 3 | pass03를 최종 결과로 오인하는가 | 통과. pass03는 이전 배포 이력, pass04는 최종 배포다. |
| 4 | 선택 source file 수나 크기가 잘못 적히는가 | 통과. 121 files, 31,722,466 bytes를 manifest와 대조했다. |
| 5 | 빌드 출력 route 수·크기가 다른가 | 통과. RSPress 2.0.22 결과 260 routes, 48,060,483 bytes다. |
| 6 | 전체 dead-link 검사를 했다고 과장하는가 | 통과. 전체 문서 link validation을 하지 않았다고 적었다. |
| 7 | PR #1327 merge SHA·시각이 잘못됐는가 | 통과. `e0c904ce35f302847dd90567f68c3f45dc015272`, `06:18:10Z`와 맞다. |
| 8 | PR check 부재를 CI green으로 표시하는가 | 통과. checks/status/reviews 0개이며 CI green으로 세지 않았다. |
| 9 | official completion count가 바뀌는가 | 통과. `78/214`를 유지한다. |
| 10 | C11/C11.4를 완료로 승격하는가 | 통과. 둘 다 미완료로 유지한다. |
| 11 | internal contract version을 변경하는가 | 통과. `0.1.0`을 유지한다. |
| 12 | Tailnet backend 상태가 다르게 보고되는가 | 통과. pass04 preflight은 backend Running이다. |
| 13 | 기존 server를 새로 띄운 것으로 기록하는가 | 통과. PID `90711` 유지, preview server restart 없음이다. |
| 14 | preflight create/replace/same 숫자가 다른가 | 통과. 65/66/129로 manifests와 맞는다. |
| 15 | 변경 route에 open handle이 남는가 | 통과. preflight open handle은 0이다. |
| 16 | route deletion이 생겼는데 숨기는가 | 통과. preflight/copy/verify 모두 no deletes다. |
| 17 | 교체 전 파일 backup 수와 경로가 틀리는가 | 통과. pass04 backup 66개와 정확한 위치를 기록했다. |
| 18 | local 결과에서 260 route hash를 전부 검증했는가 | 통과. local 260/260이다. |
| 19 | HTTPS 결과를 local로 대신했는가 | 통과. HTTPS HTTP 200·크기·hash 260/260이다. |
| 20 | 기존 backup hash 검증을 누락하는가 | 통과. 66/66 backup 검증 결과가 있다. |
| 21 | Agent Browser가 아닌 Terminal Browser로 확인하는가 | 통과. Agent Browser 0.31.1로만 화면을 확인했다. |
| 22 | desktop viewport/page content가 다른가 | 통과. 1440×1000에서 #1326, merge SHA, 78/214 및 미완료 상태를 확인했다. |
| 23 | mobile width가 viewport를 넘는가 | 통과. 390×844, document width 390px, overflow false다. |
| 24 | 화면 캡처가 다른 pass에서 왔는가 | 통과. pass04 desktop/mobile 새 캡처를 저장했다. |
| 25 | browser session과 Chrome을 남겨 두는가 | 통과. Chrome 종료, 남은 daemon 종료, 임시 profile 부재를 확인했다. |
| 26 | S03 simulator가 존재하는데 누락하는가 | 통과. S03 profile은 없고 iOS/Android simulator는 shutdown이다. |
| 27 | 공유 simulator profile 데이터를 지우는가 | 통과. 재사용 profile을 삭제하지 않고 보존한다. |
| 28 | 임시 build output을 쌓아두는가 | 통과. pass01–pass04 build roots는 open handle 0 확인 후 제거했다. |
| 29 | hash manifest가 새 문서·캡처를 검증하지 못하는가 | 통과. pass04 evidence SHA256SUMS를 생성하고 재검증했다. |
| 30 | merged main과 pass04 selected source hash가 어긋나는가 | 통과. source tree hash가 같고 58 tracked source files가 일치한다. `generated:index.md`는 generated input으로 분리했다. |

## 결과

30개 관점에서 병합 후 source mismatch, route/hash 검증 누락, simulator cleanup 손실 및 CI·완료 범위 과장을 찾지 못했다. 공개 Tailnet의 final source는 pass04다.
