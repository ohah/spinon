# C13.3 iOS Simulator active-pan second-touch 취소

## 판정

iPhone 17 Pro / iOS 26.2 Simulator의 opt-in C13.3 fixture에서 실제 UIKit 접촉 2개가 겹치는 active-pan 경로를 확인했다. 기존 한 손가락 pan이 시작된 뒤 XCUITest가 별도 UIKit touch 하나를 추가했고, `active_contacts=2`에서 `SecondPointer` 취소가 queue에 들어갔다. 두 번째 touch가 끝나 한 손가락만 남은 동안 네 번의 추가 pan callback은 모두 `suppressed-move`로 거부됐다. 첫 touch까지 끝난 뒤 새 pan이 시작되고 scroll revision이 증가했다.

이 결과는 고정 opt-in fixture의 iOS Simulator 근거다. 실기기, Android 실제 다중 touch, 제품 입력 경로, public DOM event, wheel/trackpad, S04.10 displayed-frame target, WPT 또는 C13.3 전체 완료를 판정하지 않는다. Android의 기존 두 번째 pointer 근거는 synthetic `MotionEvent` probe에 한정된다.

## 실행 환경과 입력

- 기기: iPhone 17 Pro Simulator, iOS 26.2 (`23C54`), arm64, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- 앱: `dev.spinon.bootstrap`, `--spinon-c133-scroll-gesture` opt-in route, 400×240 CSS px surface. 앱 실행 파일과 `C0410RuntimeGpuDemo.swift` SHA-256은 [build identities](./build-identities.txt)에 있다.
- 첫 contact: `idb ui swipe 120 450 120 370 --duration 45 --udid ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`. 첫 pan은 로그에서 `pointer_id=1`, sequence 2로 시작했다.
- 두 번째 contact: 임시 R05LayerSmoke UI-test runner가 이미 실행 중인 Spinon app을 활성화한 뒤 8초 기다리고 normalized point `(0.37, 0.49)`를 한 번 탭했다. 이 점은 402×874 point 화면에서 `(148.74, 428.26)`이며 inner scrollport 안이다. [XCUITest source](./xctest-second-touch.swift), [test output](./ios-xctest.log), [xcresult summary](./xctest-summary.json)를 보라. 임시 runner는 저장소의 앱·테스트 target을 변경하지 않았다.

## 관찰값

`ios-app-touch.log`는 앱 PID 3291의 실행 로그에서 현재 run만 골랐다.

1. 21:08:45.825에 pointer 1의 `pan-began`이 발생했다. 두 번째 touch 전까지 cumulative Y translation은 `-40 CSS px`까지 수락됐다.
2. 21:09:03.973에 active pan 중 `active_contacts=2`가 기록됐고 sequence 7 cancel의 reason은 `SecondPointer`였다.
3. pointer 1이 계속 움직이는 동안 21:09:04.055, 21:09:05.865, 21:09:10.889, 21:09:15.916의 callback이 `accepted=false`인 `suppressed-move`로 끝났다. cancel 뒤 허용된 Move sequence는 없다.
4. 두 contact가 모두 끝난 뒤 pointer 2가 21:09:48.223에 새 `PanBegan`과 accepted pan을 만들고 sequence 10에서 끝났다.
5. [second-touch 직후 화면](./ios-after-second-touch.png)과 [모든 touch 종료 화면](./ios-after-all-touches-up.png)의 400×240 CSS canvas crop은 1200×720 physical pixel 전체에서 RGB 차이가 0개다. 이어진 fresh gesture는 [회복 화면](./ios-after-recovery.png)에서 marker 위치를 바꾸며 crop 안 1,080 pixel을 변경했다. 비교 범위·차이는 [pixel comparison](./canvas-pixel-comparison.txt)에 있다. 첫 화면은 cancel 전 받아들인 pan 일부를 포함하므로 second-touch 화면과 다른 것이 정상이다.

Simulator 원본 캡처는 [초기 화면](./ios-fixture-initial.png), [첫 pan 뒤](./ios-before-second-touch.png), [cancel 뒤](./ios-after-second-touch.png), [contact 종료 뒤](./ios-after-all-touches-up.png), [fresh pan 회복 뒤](./ios-after-recovery.png)다. PR 본문에는 마지막 회복 화면을 첨부한다.

## 남은 범위

iOS Simulator active-pan cancellation 및 all-contacts-up recovery를 이 fixture에서 확인했다. Android의 실제 multi-touch 입력은 별도 gate로 남는다. UIKit input arbitration 전체, OS/실기기 차이, public pointer/click event, wheel/trackpad, style/layout invalidation, compositor 이후 scanout 및 C13.3 상위 완료는 이 근거로 판정하지 않는다.
