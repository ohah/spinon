# C11.4 Tailnet 미리보기 동기화 — pass38

2026-10-22 KST, 문서 PR #455가 병합된 `main`에서 선택 문서 사이트를 만들고 Tailnet 미리보기를 갱신했다. GitHub Pages는 배포하지 않았다.

## 선택 build

- 소스 commit `571a3aa9cb894e6cb6ccd2cde1d26078f5df4b86`, tree `90e126f43048120cadf1dcbee461917e3b848487`.
- Rspress `2.0.22`로 만든 선택 site는 2,910개 파일, 121,848,508 bytes이며 tree SHA-256은 `e5135ee5865ce58238bea766acda0fabb3b897ca4fec6f2b26bb7d5012b48fa2`다.
- 선택 경로에 공개 roadmap, `STATUS`, C11.4 계획, 사전 비교 자료, runtime 근거와 Android/iOS Simulator 화면을 포함했다. C11.4 checkbox는 미완료이고 공식 완료 수 `72/213`은 유지한다.

## 안전 동기화와 원격 확인

Preflight는 2,910개 target을 비교해 신규 324개, 교체 154개, 동일 2,432개로 분류했다. 삭제는 하지 않았다. 기존 교체 파일 19,194,589 bytes를 pass38 전용 backup에 저장하고 원본 byte 수·SHA-256을 확인한 뒤 원자 교체했다.

로컬 최종 검증에서 선택 build 대상 2,910/2,910개와 백업 154/154개가 모두 SHA-256 일치했다. Tailnet origin [공개 roadmap](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html) 아래 전체 2,910개 경로는 HTTP 200을 반환했고 응답 byte 수와 SHA-256이 선택 build와 모두 일치했다. 실패는 0이다.

Agent Browser에서 공개 roadmap, 공식 status, 양 축 계획, runtime README와 두 Simulator 이미지 경로를 렌더링 확인했다. Android API 37 ARM64 AVD 화면은 1080×2400, iPhone 17 Pro/iOS 26.2 Simulator 화면은 1206×2622로 로드됐다. 화면과 실행 근거는 [C11.4 runtime README](../c11-4-nested-grid-two-axis-runtime-2026-10-22/README.md), [Android AVD](../c11-4-nested-grid-two-axis-runtime-2026-10-22/android-avd-c114-nested-grid-two-axis-depth-two-android-avd-20261021T153032389Z-894472da/nested-depth-two-min-content-two-axis.png), [iOS Simulator](../c11-4-nested-grid-two-axis-runtime-2026-10-22/ios-simulator-c114-nested-grid-two-axis-depth-two-ios-simulator-20261021T153134714Z-a13a35fb/nested-depth-two-min-content-two-axis.png)에서 확인할 수 있다.

## 남은 범위

이 기록은 정적 preview build와 전송 결과다. 이번 문서 작업에서 새 모바일 runtime 실행을 하지 않았고, Android 실기기, iOS compositor 이후 pixels, WPT 또는 전체 C11.4 적합성을 추가로 확인하지 않았다. 공식 C11.4 checkbox와 `72/213` 완료 수는 그대로다.
