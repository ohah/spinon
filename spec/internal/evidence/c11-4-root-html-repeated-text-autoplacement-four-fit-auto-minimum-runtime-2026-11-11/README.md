# C11.4 four-fit Root HTML repeated-text auto-placement

## 범위

여섯 member의 `grid-auto-rows` 목록에 `fit-content()` 네 개와 `min-content`/`max-content` tail을 두는 제한 profile을 기본 parsed Root HTML RuntimeSession에서 확인했다. 네 fit track에 자동 배치한 text item의 `min-height:auto` content floor를 보존한다. 일반 Grid 지원이나 C11.4/C11 전체 완료를 뜻하지 않는다.

Chrome 기준 입력과 독립 검산은 [사전 비교 기록](../c11-4-root-html-repeated-text-autoplacement-four-fit-auto-min-precomparison-2026-11-11/README.md)에 있다.

## 기준과 RuntimeSession 결과

- 기준은 Agent Browser 0.31.1의 HeadlessChrome 157, DPR 1에서 얻은 CSS Grid 관측이다. Chrome 결과를 Spinon 실행 결과로 세지 않았다.
- 5,760개 scene, 2,880개 paired key, 720개 ordered track list를 비교했다. 네 fit member 위치 quadruple 15개, min/max tail 순서 2개, cap 배정 24개를 모두 포함했다.
- 독립 검산에는 eligible fit item occurrence 15,360개가 있었다. `min-height:0` baseline의 fit-row under-floor/visible-overflow occurrence 7,200개가 확인됐고, candidate의 fit-row floor 위반, non-fit tail floor 위반, item/payload Grid-end overflow는 각각 0개였다.
- RuntimeSession의 full four-fit matrix와 control/rejection 그룹은 5/5 test pass, 5,760/5,760 scene pass였다. 별도의 row track trace test도 1/1 pass였다. Rejection 뒤 같은 session의 다음 유효 revision 복구를 검사했다.
- 기존 1-, 2-, 3-fit auto-placement profile을 유지했다. 1-fit 1,440 scene, 2-fit zero/automatic minimum 각 2,880 scene, 3-fit 5,760 scene 및 3-fit row-span 11,520 scene이 통과했다. 1-/2-fit row-span automatic-minimum 회귀, repeated-text 810 scene, 잘못된 profile 거부도 확인했다. Four-fit row-span은 계속 거부된다.

## 플랫폼 화면 확인

iOS Simulator는 iPhone 17 Pro / iOS 26.2, Android는 기존 API 37.2 ARM64 AVD `spinon_api37_2_presenttime_probe_20261018`를 사용했다. 같은 Root HTML fixture를 light/dark 각각 열어 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`와 `SPINON_C0410_DRAW status=0 presented`를 확인했다.

| 대상 | light | dark | 이미지 크기 |
| --- | --- | --- | --- |
| iPhone 17 Pro / iOS 26.2 Simulator | [화면](./ios-iphone17pro-ios26_2-light.png) | [화면](./ios-iphone17pro-ios26_2-dark.png) | 1206×2622 |
| Android API 37.2 ARM64 AVD | [화면](./android-api37_2-light.png) | [화면](./android-api37_2-dark.png) | 1080×2400 |

네 screenshot과 실행·복구·정리 로그 21개는 [SHA-256 manifest](./screenshots-and-logs.sha256)로 고정했다. 텍스트 로그에서 진단 내용에 영향 없는 줄 끝 공백과 중복 EOF 빈 줄만 정리한 뒤 hash를 만들었다.

구현 PR [#1344](https://github.com/ohah/spinon/pull/1344)의 commit-pinned raw 이미지 네 장은 Agent Browser 0.31.1에서 모두 `complete=true`로 표시됐고 iOS 1206×2622, Android 1080×2400의 natural dimensions가 확인됐다. [Agent Browser 검증 기록](./pr-image-agent-browser-verification.json).

Fixture는 의도적으로 `40×220px` Grid 하나만 그린다. 따라서 전체 기기 viewport의 대부분이 비어 있는 것이 맞으며, 이 이미지는 visual smoke이지 pixel oracle이나 실제 기기 확인이 아니다. 상세 boot/draw 로그는 `ios-*.log`, `android-*.log`에 있다.

## 빌드와 정적 검사

- iOS Simulator 및 Android 기본 앱 빌드가 통과했다. 로그는 `ios-build.log`, `android-build.log`다.
- Style projection 및 layout profile four-fit unit tests는 각각 2/2 통과했다.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`와 `cargo fmt --all -- --check`가 통과했다. Fixture feature Clippy strict 실행에서는 이 변경과 무관한 두 기존 `too_many_arguments` 경고가 나왔고, 그 lint만 허용한 재실행이 통과했다.
- `git diff --check`, 캡처·빌드 스크립트의 `bash -n`, Root app JavaScript `node --check`가 통과했다.

## 앱·시뮬레이터 정리

첫 캡처의 종료 로그에서 이전 앱 복원이 실패했다. iOS 백업 경로의 `.app` 확장자가 빠져 있었고, Android에서 꺼낸 APK는 서명 digest 검증을 통과하지 못했다. 캡처 스크립트는 이제 iOS 확장자를 보존하고 Android 백업의 원격/로컬 SHA-256 및 서명을 캡처 전에 확인하며, 복원 실패 시 임시 백업을 남긴다. Android app-data 비교도 timestamp가 아닌 파일 내용 hash를 사용한다.

캡처 후 두 플랫폼 기본 앱을 다시 빌드해 update 설치했다. iOS app-data 파일 hash는 재설치 전후 같았다. Android의 최종 app-data archive hash `2e02c53a68fee54c409417a72e463d7beb3f297b148ea6c55958b5045a65a473`는 최초 캡처의 before/after 값과 일치한다. 상세값은 [복구 기록](./post-capture-app-recovery.log)과 [원본 데이터 검사](./app-data-preservation.log)에 있다. Source fixture는 [원래 bootstrap HTML/JS hash](./source-restoration.log)로 복구됐다.

공유 iPhone/AVD profile은 삭제하지 않고 종료했다. S03 Pilot profile은 기기 목록에 없었다. 작업 전용 임시 복구 사본은 데이터 확인 후 제거했다.

[구현/runtime 실패 관점 20개 검토](./implementation-runtime-review-20.md) · [실제 PR diff 실패 관점 30개 검토](./implementation-pr-diff-review-30.md).
