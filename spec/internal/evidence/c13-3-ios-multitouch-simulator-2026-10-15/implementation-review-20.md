# iOS active-pan second-touch 구현·런타임 적대 검토 20개

범위는 `C0410RuntimeGpuDemo.swift`의 iOS opt-in fixture contact tracking/cancel path와 이 실행 증거다. 제품 UIKit 입력 경로, Android 구현, public Pointer Events는 범위 밖이다. 코드는 변경하지 않았다. 각 행은 다른 실패 경계를 본다.

| # | 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 두 번째 contact가 pan 시작 전에 들어왔을 때 기존 pan을 잘못 취소하는가 | 이 실행은 첫 pointer의 `PanBegan`이 두 번째 접촉보다 앞선 로그 순서를 증명한다. pan 전 경로 자체는 이 run에서 별도로 판정하지 않는다. |
| 2 | 이미 진행 중인 pan 중 second-touch cancel이 누락되는가 | `PanBegan` 후 `active_contacts=2`와 reason `SecondPointer`인 sequence 7 cancel이 기록됐다. |
| 3 | 같은 touch를 여러 callback에서 중복 집계하는가 | 구현은 `ObjectIdentifier(UITouch)` 집합을 사용한다. XCUITest 접촉이 추가된 시점에 count가 2로 바뀌었다. |
| 4 | pointer count 2 도달과 cancel 사이에 move가 수락되는가 | 로그에서 cancel sequence 7 뒤 move callback 네 개 모두 `accepted=false`다. 비동기 queue 내부의 모든 가능한 경쟁 순서는 이 단일 run으로 증명하지 않는다. |
| 5 | 두 번째 touch가 끝나자 첫 touch의 나머지 움직임을 다시 허용하는가 | suppression은 active count가 0이 될 때까지 유지된다. 첫 contact가 남은 동안 네 callback이 거부됐다. |
| 6 | 마지막 contact 종료 전에 cancellation state가 조기 초기화되는가 | 마지막 접촉 전후의 callback log에서 suppression은 계속 적용됐고, fresh pan은 all-up 뒤에만 기록됐다. |
| 7 | 모든 touch 종료 뒤 suppression이 영구 고착되는가 | all-up 뒤 pointer 2가 새 `PanBegan`과 accepted pan(sequence 8/9)을 만들고 sequence 10에서 정상 종료했다. |
| 8 | cancel 뒤 이미 반영된 offset까지 되돌려 사용자 이동을 잃는가 | 첫 pan의 accepted delta는 cancel 전 로그에 남는다. 검증은 이전 accepted offset의 보존을 주장하지 않고 후속 정지 상태만 대조한다. |
| 9 | cancel 이후 늦게 도착한 move가 화면을 추가 이동시키는가 | cancel 직후와 all-contacts-up의 CSS canvas crop 사이 RGB 차이가 0이다. 로그상 후속 callback 네 개도 모두 거부됐다. |
| 10 | 캡처 시점 차이·애니메이션을 scroll drift로 오인하는가 | 비교 대상은 동일 400×240 CSS canvas crop의 1200×720 physical pixels다. 바깥 시스템 영역은 비교에서 제외했다. |
| 11 | recovery 입력 자체가 고장나 영구 정지처럼 보이는가 | all-up 뒤 별도 fresh pan이 accepted delta와 revision을 만들었고 recovery 이미지의 marker 1,080 pixels가 변했다. |
| 12 | 테스트 탭이 두 번째 UIKit contact 대신 동일 contact를 재사용하는가 | 기존 `idb` swipe가 진행 중인 동안 XCUITest runner가 다른 UIKit touch를 주입했고 앱 log count가 2가 됐다. |
| 13 | XCTest가 테스트 대상 앱을 열지 못해 빈 runner만 통과하는가 | test output은 `dev.spinon.bootstrap`을 활성화하고 실제 앱 log에는 해당 시간의 second-pointer/cancel 이벤트가 남았다. |
| 14 | 접촉 위치가 canvas 밖이어서 count만 늘고 fixture 경로를 건너뛰는가 | XCUITest 좌표는 normalized `(0.37, 0.49)`이고 README에 Simulator point 변환값과 fixture 위치를 기록했다. 제품 hit-target 일반성은 주장하지 않는다. |
| 15 | 잘못된 앱 route나 feature-off 앱을 테스트하는가 | build identities는 `dev.spinon.bootstrap`의 opt-in C13.3 route와 실행 파일 hash를 기록한다. feature-off artifact 검사는 이번 slice에서 하지 않았다. |
| 16 | 시뮬레이터 정보가 누락돼 재실행 환경을 알 수 없는가 | 기기명, iOS build, architecture, UDID 및 XCTest summary를 기록했다. 실기기 결과로 확대하지 않는다. |
| 17 | pixel 비교에서 native status bar 등 fixture 외 변화를 포함하는가 | `canvas-pixel-comparison.txt`는 접근성 frame을 바탕으로 surface canvas만 crop했다. full-screen screenshot은 별도 원본으로 보존했다. |
| 18 | second-touch 캡처 전 움직임을 cancel 후 drift로 오인하는가 | before-second-touch와 after-second-touch는 다를 수 있으며 README가 cancel 전 accepted `-40 CSS px` 이동을 명시한다. 정지 비교는 after-second-touch와 all-up 사이만 사용했다. |
| 19 | third pointer나 반복적인 pointer churn까지 확인한 것처럼 과장하는가 | 이 실행은 두 active contact를 확인했다. 세 번째 contact, 연속 추가/제거 및 OS cancellation churn은 미검증으로 남겼다. |
| 20 | iOS Simulator 결과를 Android·제품 입력 완료로 승격하는가 | README와 roadmap은 Android 실제 multi-touch, 실기기, public events, wheel/trackpad, S04.10 및 C13.3 전체를 미완료로 유지한다. |

미해결 구현 결함은 이 로그와 코드 대조에서 발견되지 않았다. 1, 12, 14행의 추가 OS/입력 조합과 feature-off 정적 경계는 이 follow-up의 검증 항목이 아니며, 완료 판정에 사용하지 않는다.
