# C11.4 parsed Root HTML `fit-content(percentage)` implicit rows · 2026-11-08

## 범위와 판정

기존 singleton `grid-auto-rows:fit-content(px)` 제한 profile에 direct finite non-negative percentage cap을 추가했다. 입력은 definite 40×100px border-box Grid, 40px explicit column, 10px explicit row, sparse row flow, gap 0, 세 direct item이다. Percentage는 Grid의 100px content-box block size를 기준으로 사용한다. 기존 px slice 및 다른 Root HTML Grid profile은 그대로 둔다.

Chrome 157 사전 비교 7개 scene과 독립 기대값을 기준으로 RuntimeSession이 computed typed percentage와 used frames를 확인했다. `0%`, 20%, 24%, 34% 경계, 40%, 120%, 그리고 20% cap 아래로 28px까지 올라가는 item minimum을 포함한다. 34px nested payload의 visible overflow와 다음 6px probe row도 검사한다. 계획·고정 입력·Chrome 관측은 [계획](../../../../plan/c11-4-root-html-fit-content-auto-rows-percentage.md), [사전 비교 근거](../c11-4-root-html-fit-content-percentage-auto-rows-precomparison-2026-11-08/README.md)에 있다.

`fit-content(calc(...))`, mixed list, indefinite Grid height, intrinsic explicit track, template areas, nonzero gap, column flow와 explicit placement는 거부한다. 실패 snapshot은 completed layout/render를 내지 않고 동일 session에서 dark-mode calc 거부 뒤 light-mode direct percentage로 복구한다.

이는 bounded parsed Root HTML runtime slice다. 일반 author CSS admission, 모든 percentage implicit rows, 전체 C11.4/C11 적합성, WPT 또는 공식 완료를 뜻하지 않는다. C11/C11.4 체크는 미완료이며 공식 집계 `78/214`를 유지한다.

## Rust 검증

- Style projection percentage gate: 1/1. 기존 explicit px/percentage fit-content projection 회귀: 2/2.
- Layout fit-content auto-row positive oracle와 malformed input rejection: 2/2.
- Parsed Root HTML RuntimeSession percentage matrix, atomic rejection, same-session recovery: 3/3.
- 기존 px positive/normalization과 atomic rejection/recovery: 2/2.
- 변경 crate all-target Clippy (`spinon-layout`, `spinon-style-to-layout`, `spinon-runtime`), `cargo fmt --all -- --check`, `git diff --check`: 통과.
- 모든 원본 명령과 출력은 [`verify-focused.sh`](verify-focused.sh), [`checks/`](checks/)에 있다.
- 구현/runtime 실패 관점 20개는 [별도 검토](implementation-runtime-review-20.md)에 있다.

## Android/iOS Simulator 실행

기본 Root HTML 앱 route에 이 fixture를 임시 넣고 Android API 37 ARM64 AVD `emulator-5554` 및 전용 iPhone 17 Pro / iOS 26.2 Simulator `Spinon-C114-FitContent-Pass04`에서 빌드·실행했다. 실기기는 사용하지 않았다. `page_scale=1`, `layout=ready`, `boxes=7`, `dark=false`와 앱 boot 성공을 확인했다. Android backend log는 ANGLE/SwiftShader다. Build flags에서 fixture-test feature는 비활성화했다.

| 플랫폼 | 실행 화면 | 크기 | 제한 |
| --- | --- | --- | --- |
| Android API 37 ARM64 AVD | [screenshot](android/screenshot.png) | 1080×2400 | 시각 smoke; PixelCopy·hardware GPU 아님 |
| iPhone 17 Pro / iOS 26.2 Simulator | [screenshot](ios/screenshot.png) | 1206×2622 | 시각 smoke; Metal drawable/compositor 이후 pixel readback 아님 |

정확한 7-scene geometry는 Rust tests에서 검사했다. 앱 smoke log는 track base size를 출력하지 않으므로 그 자체를 geometry oracle로 해석하지 않는다. 빌드·실행 명령은 [`build-simulators.sh`](build-simulators.sh), [`install-capture-simulators.sh`](install-capture-simulators.sh), 로그와 screenshot은 [`android/`](android/) 및 [`ios/`](ios/)에 있다. iOS 빌드는 성공했으며 dSYM 생성 시 vendored V8 object의 duplicate timestamp debug-map 경고가 기록됐다.

Fixture, APK, iOS app의 설치된 `index.html`, Info.plist, screenshots hash는 [`simulator-artifacts.sha256`](simulator-artifacts.sha256)에 연결했다. Bootstrap 원본 `index.html`/`root-app.js`는 빌드 스크립트가 backup 뒤 복구했고 복구 hash는 [`bootstrap-original/restored-source.sha256`](bootstrap-original/restored-source.sha256)에 있다.

## iOS Simulator 데이터 컨테이너 관측

iOS 설치 전 app data container는 `7D511E07-2629-4E16-887B-5C57536C6261`, 설치 뒤에는 `A2E877C5-CABD-493A-8DEE-18BB687E37B7`였다. 이전 경로는 현재 존재하지 않고 새 경로의 크기는 9.6 MB다. 스크립트는 이 경로가 같지 않음을 발견해 마지막 보존 assertion에서 중단했다. `simctl uninstall`, `simctl erase` 및 데이터 초기화 명령은 실행하지 않았으나, 설치 이전 데이터 내용은 별도로 백업하지 않아 연속성을 입증할 수 없다. 전후 경로는 [`data-container-before.txt`](ios/data-container-before.txt), [`data-container-after.txt`](ios/data-container-after.txt)에 남겼다. 이를 앱 기능 검증 성공이나 데이터 보존 증거로 사용하지 않는다.

`adb install -r`로 기존 Android package를 교체했고 `pm clear`/uninstall은 실행하지 않았다. 이 실행은 Android app-data contents를 검사하지 않았다.

## 로드맵 진행 카드 관측

구현 직전 공개 Tailnet roadmap을 Agent Browser로 확인했다. 보드에는 `진행 중 1/2`와 `C11.4 · ACTIVE`가 보였지만 세부 카드명이 `intrinsic track sizing`으로만 표시되어 현재 작업인 percentage implicit row를 식별할 수 없었다. 당시 화면은 [`roadmap-before-sync.png`](roadmap-before-sync.png)다. 병합 후 문서 PR에서 active card를 실제 순차 항목명으로 바꾸고 공개 preview를 다시 동기화한다.

실제 PR 변경분을 별도로 검토한 30개 관점은 [`pr-diff-review-30.md`](pr-diff-review-30.md)에 있다.
