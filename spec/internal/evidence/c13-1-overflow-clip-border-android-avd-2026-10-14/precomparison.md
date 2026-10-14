# Android C13.1 border paint 재현 기준 · 2026-10-14

## 고정 기준

- 독립 oracle: `../c13-1-overflow-clip-paint-hit-border-2026-10-14/oracle-fixture-observation.json` 및 `screenshot-pixels.txt`에 고정한 agent-browser / Chrome 155 fixture.
- 실행 입력: Android API 37 ARM64 AVD `emulator-5554`, 400×160 CSS px C13.1 overflow-clip border route, `PixelCopy` SurfaceView capture.
- 재현 산출물: `android-border-runtime.log`, `android-border-pixelcopy.png`, `android-border-build.log`.

## 사전 판정

1. Chrome 색상은 `(125,40)=(87,166,217,255)`, `(128,40)=(240,198,116,255)`, `(131,40)=(16,24,39,255)`, `(315,40)=(87,166,217,255)`, `(318,40)=(240,198,116,255)`, `(321,40)=(16,24,39,255)`이다.
2. AVD의 같은 CSS 좌표 표본은 각 RGBA 채널이 고정 Chrome 값과 모두 같아야 한다.
3. fixture의 5개 layout frame과 6개 runtime hit query는 이미 통과해 paint 표본만 실패했다. 이 분리를 유지한다.
4. 수정 뒤 기존 Android overflow route 13 frame/15 hit query와 route 충돌 거부도 재확인한다.

## 최초 관찰

기존 PixelCopy는 hit query 6/6을 통과했지만 색상은 3/6만 일치했다. 두 border strip은 부모 배경색으로 보였고 absolute target 내부도 positioned 부모 배경에 덮였다. PNG는 1050×420 물리 픽셀이고 400×160 CSS viewport를 2.625배로 캡처했다. 원본 RGBA와 hash는 Android route 로그/PNG에 보존한다.
