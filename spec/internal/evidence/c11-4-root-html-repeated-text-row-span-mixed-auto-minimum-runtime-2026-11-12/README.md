# C11.4 repeated-text row-span mixed minimum 실행 근거

## 범위

이 근거는 여섯 개 fit-content cap을 가진 Root HTML repeated-text row-span profile에서 각 항목의 min-height 0px/auto 조합을 확인한다. 일반 Grid의 automatic minimum, 임의 track 목록, 전체 CSS 적합성 또는 C11.4/C11 완료를 뜻하지 않는다. 구현과 계획 모두 제한된 후속 slice 상태이며 공식 완료 수는 78/214로 유지한다.

## RuntimeSession 행렬

- Chrome 157 사전 관측과 동일한 720개 authored cap order, 네 가지 명시 row-span topology/source order, 두 align-content 값, topology별 모든 item mask를 RuntimeSession에서 재생했다.
- 혼합 행렬은 138,240 scenes이며 12개 chunk가 모두 종료 코드 0이다. 각 mixed-01.log부터 mixed-12.log까지는 11,520 scenes를 실행했다. 전체 표식은 mixed-matrix-final.exit-code다.
- 모든-zero/all-auto uniform control 11,520 scenes도 네 chunk가 모두 통과했다. 결과는 uniform-matrix-final.exit-code와 uniform-final-1.log부터 uniform-final-4.log에 있다.
- 별도 metadata 검사는 138,240 record, 720 cap order, 32/16 mask 집합, 양쪽 align-content, 네 topology와 uniform control 수를 확인했다.
- 행렬은 각 item의 computed min-height, per-item minimum trace source와 span, Chrome item/payload frame 및 payload floor를 확인한다. mixed scene의 raw Taffy track allocation은 Chrome의 최종 used row와 같다고 주장하지 않고, trace의 유한·비음수 형태와 item/span 결과를 확인한다.
- six-fit auto-placement 5,760-scene regression, 3-fit row-span 11,520-scene regression, 1–5-fit rejection 및 style/layout typed cap profile tests도 별도로 통과했다. 해당 이름의 로그는 mixed-matrix-logs 아래에 있다.
- 잘못된 mixed minimum/control-auto revision은 같은 session에서 atomic reject되고, 다음 valid revision에서 회복된다. 고정 minimum negative control과 수정한 41px fifth-cap rejection도 별도 기록했다.

## 빌드·정적 검사

- cargo fmt --all -- --check, git diff --check, runtime feature cargo test --no-run 통과.
- 엄격한 Clippy 실행은 이번 변경과 무관하고 diff에 없는 두 테스트 helper의 too-many-arguments 2건과 세 fit 행렬 helper의 needless-borrow 4건에서 중단됐다. 원본 결과는 mixed-matrix-logs/clippy-final.log이다. 같은 세 crate/all-targets 범위를 이 두 기존 lint 종류만 제외해 다시 실행한 결과는 mixed-matrix-logs/clippy-baseline-lints-excluded.log에 있다.
- 전체 style-to-layout / layout crate 실행에는 baseline에서도 동일하게 재현된 기존 실패가 있다. candidate/baseline 비교 로그는 mixed-matrix-logs/style-to-layout-baseline.log, style-to-layout-after-fix.log, c15-layout-baseline.log, c15-layout-failure-repro.log에 보존한다. 따라서 전체 crate suite를 모두 green으로 기록하지 않는다.

## Android/iOS Simulator smoke

양쪽 화면은 C11.4 mixed-minimum 시각 결과가 아니라 기본 Root HTML 앱 부팅 smoke다.

- Android API 37 ARM64 AVD에서 build:android APK를 설치하고 MainActivity를 직접 실행했다. SpinonBootstrap 로그는 default route, V8 bootstrap result, GPU draw status 0을 기록한다. [Android 화면](android-smoke.png), [관련 로그](android-smoke.log), [빌드 로그](android-build.log).
- iPhone 17 Pro / iOS 26.2 Simulator에서 build:ios-sim 결과를 실행했다. 화면은 Root HTML 배경과 제목을 표시하고 bootstrap result를 기록한다. [iOS 화면](ios-smoke.png), [관련 로그](ios-smoke.log), [빌드 로그](ios-build.log).
- 이 smoke는 앱 초기화와 기본 route만 확인한다. mixed min-height 행렬은 Rust RuntimeSession 자동화에서 확인했으며 앱 UI/pixel 경로의 C11.4 동작으로 확대 해석하지 않는다.

## 리소스 정리

- 테스트 당시 Android 앱은 설치 전 없었으므로 smoke 뒤 제거했고, task AVD를 종료하고 ADB server도 중지했다. 공유 API 37 AVD 프로필은 유지했다.
- iPhone 17 Pro의 기존 Spinon 설치는 보존하고 task Simulator와 앱 process만 종료했다. 공유 simulator 프로필은 삭제하지 않았다.
- S03 Pilot iOS Simulator는 이미 제거된 상태다. 다른 작업 소유의 브라우저나 세션은 사용·종료하지 않았다.

