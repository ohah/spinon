# C11 Grid 계획·상태 대장·공개 로드맵 독립 검토 · 2026-10-13

| # | 실패를 노린 모순/누락 | 판정 및 반영 |
| --- | --- | --- |
| 1 | C11.2.1 계획은 WPT 전체 runner를 제외하면서 상태 대장은 Spinon WPT 미완료를 해당 단계 blocker로 서술 | 모순. WPT 적합성은 C11.2.1 blocker에서 제외하고 X16 별도 추적으로 명시한다. |
| 2 | Chrome WPT 후보 PASS를 Spinon WPT PASS로 오인 | 금지 유지. Chrome oracle과 Spinon runner 결과를 분리한다. |
| 3 | pinned WPT source hash가 단계별 fixture inventory와 무관하게 바뀔 가능성 | 고정 revision/hash 및 후보 단위 결과를 유지하고 전체 적합성으로 일반화하지 않는다. |
| 4 | profile 밖 기능이 포함된 WPT 원문 전체를 단계 subset 증거로 취급 | 원문 실행은 참고로만 두고 지원 case 대응표 없는 통과 주장은 거부한다. |
| 5 | X16 공통 WPT 자동화와 C11 기능 구현의 소유권 중복 | X16은 runner/공통 적합성, C11은 Grid 동작 subset으로 분리한다. |
| 6 | X09가 C11과 동일한 Grid를 별도 구현/완료 처리 | C11 소유권을 유지하고 X09는 교차 검증으로만 둔다. |
| 7 | C11.2.2 runtime이 C11.2.1의 아직 통합되지 않은 타입/profile에 먼저 의존 | C11.2.2는 C11.2.1 별도 변경 통합과 Rust·Chrome fixture 통과 후 시작한다. |
| 8 | Android 실기기·WPT 잔여 항목 때문에 C11.2.2 계획·fixture 준비까지 멈춤 | 계획과 oracle 준비는 가능하다. runtime 구현 선행 조건은 C11.2.1 통합 및 Rust·Chrome fixture로 제한한다. |
| 9 | Android AVD/SwiftShader 결과를 실기기 결과로 승격 | 금지. 10/13 실기기 run을 별도 AVD run과 분리 기록한다. |
| 10 | Android PixelCopy를 하드웨어 GPU 성능 증거로 해석 | 금지. SM-S731N의 표면 픽셀 비교이며 성능 측정이 아니다. |
| 11 | iOS Metal drawable readback을 compositor 이후 화면/scanout 검증으로 표현 | 금지. 현재 값은 compositor 제출 전 drawable texture다. |
| 12 | iOS Simulator를 iOS 실기기 결과로 표현 | 금지. Simulator와 Android physical run을 별개 대상으로 둔다. |
| 13 | geometry 0 CSS px를 pixel 표면 비교로 오인 | 금지. node frame 비교와 PixelCopy/Metal sample 비교를 별도 결과로 기록한다. |
| 14 | 성공·거부 scene을 합쳐 전체 scene 성공으로 표기 | 각 플랫폼별 10 scene, 성공 6/6·기대 거부 4/4로 나눠 기록한다. |
| 15 | 10/12 AVD run을 10/13 현재 소스 증거로 계속 가리킴 | 10/13 source rerun summary를 만들고 세 최신 run ID를 모두 직접 연결한다. |
| 16 | working tree 전체 테스트 결과를 clean commit/PR 결과로 간주 | 증거 문서에 dirty tree와 base HEAD를 밝혀 commit-specific 주장을 금지한다. |
| 17 | Rust full suite PASS만으로 hidden incremental regression을 검증했다고 주장 | 전용 incremental test와 수정 제거 negative control을 직접 링크한다. |
| 18 | `display:none`과 `visibility:hidden`을 같은 subtree 의미로 처리 | 이번 수정은 computed `display:none` propagation만 변경하며 visibility 동작을 주장하지 않는다. |
| 19 | C11.2.1 완료를 C11 전체 또는 CSS 전반 완료로 확대 | 모두 미완료로 유지한다. |
| 20 | plan, STATUS, Tailnet HTML이 서로 다른 WPT gate/실기기 최신 상태를 표시 | 세 경로의 문구와 run 링크를 같은 변경에서 동기화한다. |

## 적용한 계약 정리

C11.2.1의 현재 남은 직접 완료 조건은 제한 profile의 통합 상태와 그 단계에 고정된 code/fixture/runtime 증거다. Android SM-S731N 10-scene runtime·PixelCopy와 iOS Simulator 10-scene runtime·Metal drawable 표본은 2026-10-13에 통과했다. C11.2.1에서 제외한 Spinon 전체 WPT runner/적합성은 X16에서 추적하며 이 제한 profile의 코드 통합이나 C11.2.2 runtime 착수 조건으로 두지 않는다. C11.2.2는 C11.2.1 통합 및 Rust·Chrome fixture 검사가 끝나야 시작한다.

아직 C11.2.1은 통합되지 않았고 C11.2.2 runtime은 시작되지 않았다. Chrome oracle PASS를 Spinon 적합성으로 표현하지 않으며, AVD·Simulator 자료를 실기기 근거로 합치지 않는다. [현재 소스·기기 증거](c11-2-1-current-source-rerun-2026-10-13/README.md).
