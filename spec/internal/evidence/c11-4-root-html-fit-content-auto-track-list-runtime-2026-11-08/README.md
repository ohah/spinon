# C11.4 parsed Root HTML fit-content implicit-row list의 `auto` member 실행 근거

## 후보 범위

이 미병합 후보는 PR #1270의 `RootHtmlFitContentAutoRowsV1`에 `auto`를 목록 member로 추가한다. 목록은 non-empty이며 finite non-negative `fit-content(px|%)`를 하나 이상 포함해야 하고, 나머지 값은 `auto`, finite non-negative px 또는 definite percentage다. 명시 track 40px column·10px row, definite 40×100px Grid content box, sparse row auto-placement, zero gap, `align-content:start`, visible overflow와 동일한 direct-item/fixed-height payload topology를 사용한다. `auto` 단독 목록·fit-content 없는 pattern·intrinsic keyword·`minmax()`·`fr`·`calc()`·음수/비유한 값·다른 owner/style/placement/topology는 거부한다.

- 브랜치: `feat/c11-4-root-html-fit-content-auto-track-list-20261108`
- Chrome 157 사전 비교와 독립 기대값: [precomparison](../c11-4-root-html-fit-content-auto-track-list-precomparison-2026-11-08/README.md)
- 계획 및 사전 검토: [계획](../../../../plan/c11-4-root-html-fit-content-auto-track-list.md) · [계획 실패 관점 20개](../../../../plan/c11-4-root-html-fit-content-auto-track-list-plan-review-20.md)
- 구현/runtime 검토: [실패 관점 20개](./implementation-runtime-review-20.md)
- fixture SHA-256: `c206e7aef5b82129dce5b4f79599da7a58a4dc1d6fe5343bc49c03552e37300d`

## Rust 검증

Layout은 2/2, style projection은 1/1, 신규 RuntimeSession matrix와 실패/회복은 2/2, 이전 singleton 회귀는 2/2 통과했다. 변경 crate 3개 all-target Clippy `-D warnings`와 `cargo fmt --all -- --check`도 통과했다. 실행 로그는 [test-logs](./test-logs/)에 있다.

Chrome/독립 기대값의 세 장면은 `[10,20,30,8,20]`, `[10,30,20,8,25]`, `[10,0,30,8,0]` CSS px row list다. RuntimeSession은 typed track 순서, computed serialization, Grid/item/payload frame과 zero-cap row의 visible child overflow를 확인한다. dark media의 unsupported calc revision은 `grid-auto-rows`로 원자적 거부하고 같은 session에서 light revision으로 복구한다.

## Android API 37 ARM64 AVD

`sdk_gphone64_arm64` API 37 AVD (`emulator-5554`)의 기본 Root HTML route가 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass layout=ready boxes=12`를 기록했다. APK 내부 `assets/index.html` SHA-256은 고정 fixture와 같고 설치된 candidate APK hash는 build APK와 일치한다.

- [화면](./simulator/android/screenshot.png) · [boot log](./simulator/android/current-source-run.log) · [build log](./simulator/android/build.log) · [device/run](./simulator/android/device-and-run.txt)

## iPhone 17 Pro / iOS 26.2 Simulator

`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D` Simulator의 기본 Root HTML route도 `status=pass layout=ready boxes=12`를 기록했다. 설치된 app bundle의 executable·HTML·script hash가 build bundle과 일치한다.

- [화면](./simulator/ios/screenshot.png) · [boot log](./simulator/ios/root-html-boot.log) · [build log](./simulator/ios/build.log) · [device/run](./simulator/ios/device-and-run.txt)

양 플랫폼 screenshot은 좌상단 고정 Grid를 보이는 visual smoke다. PixelCopy/Metal drawable readback, compositor 이후 output 또는 pixel parity를 측정하지 않았다. Geometry와 used track 판정은 Rust Layout/RuntimeSession focused assertions가 담당한다. 실기기와 hardware GPU는 검증하지 않았다.

## 복원·상태

실행 전에 `examples/bootstrap` 입력과 generated bootstrap, Android build/installed APK, iOS build/installed app bundle을 별도 경로에 보존했다. 실행 후 bootstrap SHA, Android build 및 설치 APK hash, iOS build와 설치 app executable hash가 보관본과 일치하도록 복원했다. Android AVD는 같은 기존 앱을 다시 설치·실행해 전면 상태로 복귀했고, iOS target app은 원래와 같이 종료된 상태로 뒀다. 저장 데이터는 조회·변경하지 않았다. 보관본은 [backup location](./simulator/backup-location.txt), 상세 hash/복원 기록은 [run manifest](./simulator/run-manifest.txt)와 simulator 하위 로그에 있다.

PR diff 실패 관점 30개 검토를 마쳤다. PR 생성·CI/check 결과 확인·merge는 남아 있다. iOS Simulator build log에는 duplicate `RawWindowMetalLayer` symbol/debug-map warnings가 있었으나 Xcode는 `BUILD SUCCEEDED`로 끝났다. C11.4/C11 공식 체크, 완료 수 `78/214`, 내부 contract version `0.1.0`은 바꾸지 않았다.
