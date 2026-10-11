# C12.4 iOS runtime matrix · 구현 실패 관점 검토

**계획:** [iOS 26-scene matrix](../../../plan/c12-4-ios-runtime-matrix.md) · [계획 검토](./c12-4-ios-runtime-matrix-plan-review-2026-10-11.md)
**최종 실행:** iPhone 17 Pro / iOS 26.2 Simulator, 26/26 기대 결과 일치, 지원 node 64개 최대 오차 0 CSS px
**원본:** [실행 기록](./c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/README.md) · [비교 JSON](./c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/comparison.json)

| # | 실패 관점 | 확인 결과 및 방어 |
| ---: | --- | --- |
| 1 | Android와 다른 fixture 생성 경로 | 동일한 matrix builder JSON을 앱 sandbox에 전달했고 inventory SHA-256이 일치했다. |
| 2 | inventory 변경 또는 잘못된 matrix가 조용히 실행 | 앱은 schema·26 case·77 node·21/5 기대 수·고유 ID를 확인하며 비교기는 inventory hash를 고정 builder 결과와 대조한다. |
| 3 | 오래된 산출물이 이번 실행으로 오인 | 실행마다 run ID가 다르고 앱과 host script가 해당 출력 파일을 지운 뒤 새 run ID의 결과만 수락한다. |
| 4 | run ID가 경로를 벗어나도록 조작 | 앱에서 영숫자·밑줄·하이픈 1–80자 형식으로 제한한다. |
| 5 | 잘못된 Simulator 또는 OS를 기준으로 기록 | 고정 UDID·iPhone 17 Pro·iOS 26.2를 확인하고 결과 metadata에 저장한다. |
| 6 | 둘 이상의 booted Simulator가 섞임 | capture는 대상 외 booted 기기가 있으면 실행 전 중단한다. |
| 7 | runtime GPU fixture가 꺼진 앱을 설치 | 캡처가 `SPINON_ENABLE_C04_RUNTIME_GPU=1`로 새 앱을 빌드하고 `BUILD SUCCEEDED` 및 앱 산출물을 확인한다. |
| 8 | 갱신 전 앱을 실행 | 현재 DerivedData 앱을 명시적으로 설치하고 launch log를 보존한다. |
| 9 | input 파일 일부/다른 bytes가 앱에 전달 | sandbox 복사본과 builder 출력 bytes의 SHA-256을 비교한다. |
| 10 | UI surface 초기화가 matrix에 섞임 | probe는 빈 host window에서 실행하며 WGPU surface 생성·draw·pixel readback을 호출하지 않는다. |
| 11 | matrix 작업 queue가 앱 시작과 경쟁 | 앱 delegate가 window를 보이게 한 뒤 user-initiated background queue에 probe를 실행한다. |
| 12 | case 간 HostDocument/cache 상태 누출 | 각 case마다 새 native host를 만든다. 실제 26 case 전체가 한 앱 프로세스 안에서 완료됐다. |
| 13 | host가 실패 경로에서 해제되지 않음 | 생성 이후 환경·evaluate 성공/실패 모두 함수 scope `defer`로 host를 해제한다. |
| 14 | environment가 중복 적용돼 revision이 달라짐 | 환경 설정을 case마다 한 번만 호출하고 실패 시 evaluate를 건너뛴다. |
| 15 | CSS viewport 또는 scale이 다름 | 모든 host에 320×240 CSS px, scale 1, light mode를 지정한다. |
| 16 | layout 미완료를 JS 성공으로 오인 | 지원 case는 `status=0`과 `layout=ready`를 모두 요구한다. |
| 17 | 다른 CSS 오류가 기대 reject로 통과 | visibility/opacity의 NodeId·정확한 property, pseudo-element의 content 진단, `showModal` TypeError를 각각 검사한다. 5개 기대 거부가 모두 맞았다. |
| 18 | 결과 파일의 부분 쓰기 또는 중간 상태를 통과 | JSON을 atomic write하고 capture가 완성된 run ID·schema·summary를 읽는다. |
| 19 | 행렬 전체 숫자만 맞고 frame 일부/오차를 놓침 | 공통 비교기가 case 순서·중복·77 source node 대응을 검사하고 64 지원 node의 x/y/width/height를 각각 0.5 CSS px 한계로 비교했다. 최대 오차 0이다. |
| 20 | matrix 통과를 surface/WPT/전체 CSS 완료로 과장 | 결과와 계획에 geometry-only 범위를 명시했다. Metal pixel·WPT·실기기·전체 CSS는 미완료다. |

## 비교기 실패 제어

공통 비교기 테스트 4개를 추가해 정상 결과, inventory hash 오류, 중복 case, 허용치 초과 frame, 잘못된 거부 진단을 실행했다. 네 테스트 모두 기대대로 통과/거부했다. 첫 시험 작성 때 오류 문구 assertion이 실제 비교기 진단과 달라 한 번 실패했으며, 비교기 코드는 그대로 두고 assertion을 실제 고정 진단에 맞췄다.

## 한계

이 증거는 시뮬레이터의 runtime geometry와 거부 진단만 다룬다. 21 hit-test sample 순위 비교는 Chromium `elementsFromPoint()`를 참고한 보조 비교이며 pixel oracle이 아니다. scene별 Metal readback, 공식 WPT runner, 실기기 실행은 별도 근거가 필요하다.
