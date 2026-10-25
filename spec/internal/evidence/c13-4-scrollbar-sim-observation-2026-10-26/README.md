# C13.4 Simulator 관찰 시도 · 2026-10-26

## 결과

**플랫폼 scrollbar mode는 확인되지 않았다.** 이 실행에서는 gutter geometry, scrollbar visibility, Android/iOS 사용자 표시를 측정하지 못했으며 overlay/classic 판정으로 해석하면 안 된다.

## Android AVD

- 대상: 할당된 AVD `emulator-5560`; 물리 Samsung을 선택하지 않았다.
- 시작 전 foreground는 `dev.spinon.bootstrap/.MainActivity`였고 C11.4 Grid track sizing 화면을 캡처했다([시작 전](android-before.png)). Chrome 패키지 버전은 `149.0.7827.5`; 해당 Chrome 프로세스는 이 실행 전부터 백그라운드에 있었다.
- 임시 fixture server `127.0.0.1:8874`와 해당 AVD에 한정된 ADB reverse를 설정하고 [측정 HTML](mobile-mode-probe.html)을 Android Chrome에 열었다. Chrome은 HTML 대신 첫 실행 화면을 표시했다([Chrome 화면](android-chrome-rest.png)); Activity 확인 결과도 `FirstRunActivity`였다.
- 첫 실행 화면의 계정/약관 동의 UI를 선택하지 않았다. 따라서 HTML, computed style, scroll geometry와 scrollbar UI는 관찰하지 않았다. Chrome의 첫 실행 동의를 우회하거나 기존 프로세스를 종료하지 않았다.
- Spinon `MainActivity`를 다시 foreground로 가져오고 ADB reverse를 제거했다. 최종 foreground와 window focus가 원래 `dev.spinon.bootstrap/.MainActivity`인 것을 확인하고 캡처했다([종료 후](android-after.png)). reverse 목록은 비었다. 임시 HTTP server도 종료했다.

## iOS Simulator

- 대상: 지정된 iPhone 17 Pro Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, iOS 26.2.
- 시작 전 Spinon C11.4 화면을 캡처했다([시작 전](ios-before.png)). Safari를 foreground로 열지 않았고 앱/scene state를 바꾸지 않았다. 따라서 iOS 결과도 미관찰이다.

## 해석 및 다음 관문

Android Chrome first-run과 iOS 미실행은 플랫폼의 overlay/classic 동작에 대한 증거가 아니다. 이 자료만으로 구현을 열거나 mode를 결정하지 않는다. Safari/Chrome에서 관찰을 이어갈 때는 동의 UI를 우회하지 않는 비파괴 실행 경로가 먼저 필요하다. 플랫폼 mode가 계속 관찰 불가능하면 명세 기대값과 모바일 실측을 분리하고, CSS/geometry positive·negative control만 보존하며 scrollbar UI나 gutter를 추측해 구현하지 않는다.
