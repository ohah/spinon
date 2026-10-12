# C11.2.1 Android 환경 대체 응답 구현 검토

검토 대상은 Android 실기기에서 재현된 초기 C11.2.1 eval의 `status=-12`와 이후 최신 환경 갱신 결과를 수집·판정하는 코드다. Android source, 비교기, 회귀 검사, 실패·성공 run의 원본 로그와 Android/iOS 결과를 서로 대조했다.

| # | 공격 관점 | 확인 내용과 판정 |
|---:|---|---|
| 1 | `-12`를 무조건 재시도하는가 | `op=eval status=0`까지 일치할 때만 superseded로 분류한다. 통과. |
| 2 | 다른 `-12` 원인이 성공으로 둔갑하는가 | 내부 eval 성공 표식이 없으면 일반 실패 경로에 남는다. 통과. |
| 3 | `-7` 등 다른 실패를 삼키는가 | 성공 summary가 있어도 비교기는 초기 상태가 0 또는 승인된 -12인지 요구한다. negative control 통과. |
| 4 | superseded 초기 응답이 최종 실패로 남는가 | 전용 marker를 기록하고 즉시 반환해 terminal failure summary를 만들지 않는다. 통과. |
| 5 | superseded 응답 뒤 새 환경 갱신이 없을 때 거짓 성공하는가 | summary가 없으므로 수집기와 비교기가 실패한다. ready-summary 누락 negative control 통과. |
| 6 | 실패 진단에 오래된 장면 오류가 남는가 | superseded 경로는 오류 진단을 내지 않고, 실제 최신 적용 실패는 환경 적용 결과로 표시한다. 통과. |
| 7 | 오래된 surface generation을 최신으로 오인하는가 | 환경 적용 전후 generation·sequence를 검사한다. 통과. |
| 8 | 구형 sequence가 새 sequence 사이에 끼어드는가 | `isCurrentPresentation` 확인 뒤 현재 요청만 성공 처리한다. 통과. |
| 9 | 환경 적용 실패가 ready로 기록되는가 | 환경 status가 0일 때만 ready helper와 draw 요청에 진입한다. 통과. |
| 10 | layout 준비 전 summary를 쓰는가 | helper는 status 0 및 `layout=ready`를 모두 요구한다. 통과. |
| 11 | 두 경로가 summary를 중복 기록하는가 | `stateLock` 아래 단일 emitted flag를 확인한다. 통과. |
| 12 | frame과 paint order가 다른 결과에서 합쳐지는가 | helper에 전달된 동일 report에서 두 marker를 추출한다. 통과. |
| 13 | summary가 다른 앱 실행의 로그를 가져오는가 | 비교기는 run ID와 scene ID가 같은 marker 하나를 요구하며 runner는 PID를 분리한다. 통과. |
| 14 | 초기 eval 상태와 최종 summary 상태가 충돌하는가 | 이번에 상태 대조를 추가했다. 거부 fixture는 두 상태가 같아야 하고, success는 0 또는 검증된 superseded -12만 허용한다. 통과. |
| 15 | -12인데 superseded marker가 빠져도 성공하는가 | 비교기는 marker 1개와 내부 eval 성공 상태를 요구한다. 누락 marker negative control 통과. |
| 16 | marker가 summary 뒤에 기록되어도 성공하는가 | 비교기는 eval → superseded → ready summary 순서를 요구한다. 통과. |
| 17 | 평가 report 중복이 모호성을 만드는가 | summary·eval marker 중복은 `markerLine`에서 거부한다. 통과. |
| 18 | 성공 summary 뒤 실제 frame 또는 WGPU draw가 없는가 | 전체 fixture frame, 허용 오차, 기대 box 수의 후속 draw를 요구한다. 통과. |
| 19 | 거부 장면이 partial frame이나 비어 있지 않은 draw를 내는가 | 거부 비교기는 정확한 진단 및 frame·positive draw 부재를 요구한다. 통과. |
| 20 | 제한 fixture 결과를 더 넓은 적합성으로 과장하는가 | 10 scene·21 node 결과만 기록하고 WPT 적합성·전체 Grid·C11 완료를 제외한다. 통과. |

## 확인된 결함과 수정

초기 비교기는 최종 성공 summary와 draw를 확인하면서 초기 `-12`가 실제 superseded 응답인지 검증하지 않았다. 그래서 무관한 실패 상태와 성공 summary가 섞인 로그를 이론상 통과시킬 수 있었다. 비교기에 상태 일치, superseded marker 존재·내용·순서 조건을 넣고 누락 marker 및 `-7` negative control을 추가했다.

## 검증 결과와 남은 경계

- C11.2.1 비교기 전용 검사: 5/5 통과.
- CSS reference 전체: 342/342 통과.
- Android 16/API 36 실기기: 10 scene, 성공 6/6·기대 거부 4/4, geometry 최대 오차 0 CSS px, 표면 표본 20/20 exact.
- iOS 26.2 Simulator: 같은 10 scene·21 node 결과, geometry 최대 오차 0 CSS px, Metal drawable 표본 20/20 exact.
- 수정 후 플랫폼 rerun은 -12 분기를 재발생시키지 않았다. 이전 실패 로그와 comparator synthetic regression은 보존했지만, 이 특정 분기의 수정 후 실기기 재현은 미확정이다.
- 이 검토는 제한 C11.2.1 실행기 변경에 대한 것이다. C11.1/C11.2.1 통합, Spinon WPT runner·적합성, iOS 실기기 및 전체 CSS Grid 검증은 완료로 표시하지 않는다.

세부 실행 로그와 비교 입력은 [후속 실행 근거](./c11-2-1-android-environment-supersede-2026-10-13/README.md)에 있다.
