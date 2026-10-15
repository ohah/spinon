# C13.3 native wheel·trackpad 계획 실패 경로 검토 20개 · 2026-10-16

구현 전 계획의 독립 실패 경로 20개를 검토했다. iOS 부호 미정, AVD injected event의 물리 장치 오해, momentum 추론 위험을 반영했고 native callback이 오지 않으면 FFI enqueue로 대체하지 않는 gate를 추가했다.

1. **Chrome delta sign과 OS 축 sign 혼동** — W3C sign과 native 축을 분리하고 양/음 H/V를 둔다. 통과.
2. **Android positive V를 CSS positive Y로 복사** — Android viewport 방향에 맞춰 CSS deltaY를 반전한다. 통과.
3. **Android positive H를 Y와 같이 반전** — positive H는 viewport right이므로 deltaX는 양의 식으로 둔다. 통과.
4. **ViewConfiguration factor 누락** — 축별 factor와 원본 axis를 함께 기록한다. 통과.
5. **physical px를 CSS px로 취급** — surface physical px/CSS px scale로 나눈다. 통과.
6. **minSdk에서 factor API 부재** — minSdk 29와 API 26+ factor availability를 대조했다. 통과.
7. **touchpad ACTION_MOVE를 ACTION_SCROLL로 오인** — API34 classified-move는 이 slice 밖의 no-enqueue control이다. 통과.
8. **touchpad source를 mouse로 뭉침** — SOURCE_MOUSE와 SOURCE_TOUCHPAD를 구분하고 AVD callback을 확인한다. 통과.
9. **ADB input을 hardware 증거로 게시** — emulator/API/device id와 synthetic injection 한계를 적는다. 통과.
10. **모든 iOS pan을 trackpad로 분류** — discrete/continuous masks를 분리한다. 통과.
11. **iOS touch가 두 번 처리** — allowedTouchTypes를 비우고 finger path를 유지한다. 통과.
12. **iOS translation total을 각 callback delta로 합산** — 직전 값과의 차분만 보낸다. 통과.
13. **UIKit 문서가 sign을 보장한다고 단정** — ± Simulator preflight를 구현 전 gate로 둔다. 통과.
14. **UIKit points를 drawable pixels로 오인** — raw unit points, scale points/CSS px로 정의한다. 통과.
15. **recognizer ended를 momentum end로 사용** — explicit phase 없으면 None이다. 통과.
16. **pinch/magnification이 wheel queue에 섞임** — Android Ctrl/pinch와 iOS Control/pinch path를 차단한다. 통과.
17. **native point를 transform 없이 사용** — local point를 CSS point로 바꾸고 target을 한 번 고정한다. 통과.
18. **surface 변경 뒤 이전 baseline 사용** — generation/lifecycle change에서 scale과 baseline을 갱신한다. 통과.
19. **queue rejection 뒤 delta를 다시 전송** — last-observed 값을 전진시켜 dropped delta를 replay하지 않는다. 통과.
20. **opt-in fixture를 product support로 홍보** — artifact controls와 public API/S04.10 gate를 유지한다. 통과.

## 구현 전 보류 조건

- iPadOS Simulator에서 discrete/continuous callback, 부호, point scale을 확인한다.
- Android API 37 AVD에서 mouse/touchpad source가 ACTION_SCROLL로 도착하는지 확인한다.
- callback 부재를 직접 FFI 주입으로 대체하지 않는다.
