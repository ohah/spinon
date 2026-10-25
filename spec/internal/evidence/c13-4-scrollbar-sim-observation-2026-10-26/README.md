# C13.4 Simulator 관찰 · 2026-10-26

**iOS Safari 관찰 PR diff 검토:** [실패 관점 30개](../c13-4-scrollbar-ios-safari-pr-diff-review-30-2026-10-26.md)

## 결과

Android는 관찰하지 못했다. iOS Safari에서 fixture와 geometry를 읽었으며, `stable`과 `stable both-edges` 모두 `auto`와 동일한 zero-additional-gutter geometry였다. 이는 overlay/no-gutter 동작과 일치하지만, 캡처에서 활성 native indicator가 보이지 않아 transient UI mode 자체를 직접 관찰한 결과는 아니다. 상세 값과 hash는 [구조화 결과](ios-safari-result.json)에 기록했다.

## Android AVD

- 대상: 할당된 AVD `emulator-5560`; 물리 Samsung을 선택하지 않았다.
- 시작 전 foreground는 `dev.spinon.bootstrap/.MainActivity`였고 C11.4 Grid track sizing 화면을 캡처했다([시작 전](android-before.png)). Chrome 패키지 버전은 `149.0.7827.5`; 해당 Chrome 프로세스는 이 실행 전부터 백그라운드에 있었다.
- 임시 fixture server `127.0.0.1:8874`와 해당 AVD에 한정된 ADB reverse를 설정하고 [측정 HTML](mobile-mode-probe.html)을 Android Chrome에 열었다. Chrome은 HTML 대신 첫 실행 화면을 표시했다([Chrome 화면](android-chrome-rest.png)); Activity 확인 결과도 `FirstRunActivity`였다.
- 첫 실행 화면의 계정/약관 동의 UI를 선택하지 않았다. 따라서 HTML, computed style, scroll geometry와 scrollbar UI는 관찰하지 않았다. Chrome의 첫 실행 동의를 우회하거나 기존 프로세스를 종료하지 않았다.
- Spinon `MainActivity`를 다시 foreground로 가져오고 ADB reverse를 제거했다. 최종 foreground와 window focus가 원래 `dev.spinon.bootstrap/.MainActivity`인 것을 확인하고 캡처했다([종료 후](android-after.png)). reverse 목록은 비었다. 임시 HTTP server도 종료했다.

## iOS Simulator

- 대상: 지정된 iPhone 17 Pro Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, iOS 26.2.
- 이번 실행 시작 때는 Spinon 앱이 실행 중이지 않았고 iOS Home screen이었다([시작 전](ios-before-safari.png)). 이전 슬롯 기록의 C11.4 캡처 [ios-before.png](ios-before.png)는 이 실행의 pre-state가 아니다.
- 로컬 fixture server를 띄우고 Safari에 `http://127.0.0.1:8874/mobile-mode-probe.html`을 일반 URL로 열었다. Safari first-run, 약관, 로그인 prompt가 없어 동의나 우회 없이 페이지가 열렸다. 1초 캡처는 아직 빈 loading 화면이었고, 4초 캡처에서 fixture와 결과 텍스트가 나타났다([초기 loading](ios-safari-initial-loading.png), [결과 화면](ios-safari-result.png)).
- UA는 `Version/26.2 Mobile/15E148 Safari/604.1`; viewport `402×714 CSS px`; `CSS.supports('scrollbar-gutter','stable')`는 `true`였다. `auto`, `stable`, `stable both-edges`가 각각 `client=216×126`, `offset=220×130`, `scroll=380×280`으로 정확히 같았다. 세 frame의 x 좌표도 모두 91px이고 폭은 220px여서 `both-edges`가 inline 시작점을 밀지 않았다. 즉 이 fixture에서는 stable gutter가 추가 공간을 예약하지 않았다. `overflow:clip` negative control에는 scroll owner나 gutter가 없었다.
- 결과 캡처에는 native scrollbar indicator가 보이지 않았다. CSS geometry는 관찰했지만 touch 중 transient indicator는 직접 검증하지 않았으므로 “overlay와 일치하는 no-gutter 동작”으로 제한해 기록한다. Chrome 156 classic desktop 결과는 positive control이며 iOS 폭 기준은 아니다.
- Safari는 시작 전 foreground process가 없어 실행 뒤만 종료했다. 마지막으로 `dev.spinon.bootstrap`을 띄워 Spinon 화면으로 복귀했다([종료 후](ios-after-spinon.png)); 앱 화면은 `Spinon 루트 HTML`이다. iOS Simulator는 종료하지 않았고 그대로 parent에게 반환했다.

## 해석 및 다음 관문

Android Chrome first-run blocker는 남아 있다. iOS 결과는 Safari의 CSS geometry 관찰이며 Spinon 구현 동작이 아니다. 사용자에게 보이는 scrollbar UI와 mode를 직접 확인하지 못했으므로, 이 근거만으로 C13.4 구현을 시작하지 않는다. Android 관찰 경로가 안전하게 열리지 않으면 그 상태를 별도로 기록하며, scrollbar UI나 gutter를 추측해 구현하지 않는다.
