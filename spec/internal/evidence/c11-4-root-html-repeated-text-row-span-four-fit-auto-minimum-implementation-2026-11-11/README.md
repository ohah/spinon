# C11.4 four-fit row-span `min-height:auto` 구현 근거

## 범위

이 자료는 기존 parsed Root HTML repeated-text row-span profile에서 정확히 네 `fit-content()` track과 `min-content`·`auto` 각 하나만 갖는 제한 입력을 검증한다. 제품 구현 전 Chrome 157 비교와 plan PR [#1356](https://github.com/ohah/spinon/pull/1356) 병합을 마쳤다. 공식 C11.4/C11 완료, 일반 CSS Grid 적합성, public API/FFI 및 internal numeric contract 버전 상승을 뜻하지 않는다. 공식 완료 수 `78/214`는 유지한다.

## 기준 구현과 RuntimeSession 비교

독립 기준 자료는 [Chrome 157 사전 비교](../c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum-precomparison-2026-11-11/README.md)다. RuntimeSession 테스트는 같은 고정 observation corpus의 baseline `min-height:0`과 candidate `min-height:auto`를 11,520 scene에서 비교하고, computed typed track order, row trace, item/payload frame, text lines, automatic-minimum eligibility/floor, source order, `start|stretch`를 확인한다.

```sh
cargo test --locked -p spinon-runtime \
  --features c11-4-grid-track-sizing-fixture-test \
  parsed_root_html_four_fit_row_span_matches_all_11520_chrome_scenes --lib
```

반복 실행 원본은 [`runtime-four-fit-matrix.log`](./runtime-four-fit-matrix.log)에 있다. 재실행은 716.26초에 11,520/11,520 scene으로 끝났다. 독립 비교의 총 11,520 scene 중 candidate eligible item occurrence는 25,920개이며 content floor 위반은 0이다. 같은 paired keys의 baseline은 3,648 occurrence가 content floor 아래다. span/frame의 최대 차이는 0.000225 CSS px, forward/reverse source-order geometry 차이는 0 CSS px다. Matrix 각 item·payload·track trace 검증이 끝난 뒤 고정된 scene 집계를 확인한다.

기존 four-fit auto-placement rounding/control regression도 Chrome 157의 5,760 scene 전체를 재실행했다. [`runtime-four-fit-autoplacement-regression.log`](./runtime-four-fit-autoplacement-regression.log)은 5,760/5,760 scene, 1 passed를 기록한다(302.95초).

추가 focused test는 아래를 확인한다.

- Style projection과 layout profile에서 fit 위치 15개 × cap 순서 24개 × tail 순서 2개, 총 720 조합을 각각 확인한다.
- 자동 최소 contribution trace와 같은 revision의 owner·used row·final track base를 확인한다.
- duplicate/missing/unsupported cap, 다섯 fit, tail 누락, unresolved percentage `calc()`, 일곱 번째 row, dense flow, scrollable item, unsupported alignment, indefinite percentage basis 입력에서 부분 layout을 게시하지 않는다.
- Mixed item minimum을 거부한 뒤 같은 RuntimeSession의 valid 환경 revision에서 다시 layout을 게시한다.
- 기존 one-/two-/three-fit row-span, four-fit auto-placement 및 rounding controls를 별도 regression으로 확인한다.

Rust formatting과 workspace Clippy 결과는 [`format-check.log`](./format-check.log), [`clippy.log`](./clippy.log)에 기록한다. Focused projection/layout/runtime 결과는 [projection](./style-projection-focused.log), [layout](./layout-profile-focused.log), [trace](./runtime-four-fit-trace-focused.log), [rejection/recovery](./runtime-four-fit-rejections-focused.log) 로그로 보존한다.

## iOS·Android Simulator smoke

아래 이미지는 C11.4 four-fit fixture를 로드하지 않는 **기본 Root HTML 앱 시작 경로**다. 로그는 `layout=ready`와 `SPINON_C0410_DRAW status=0 presented`를 기록한다. 이 결과는 앱 build·시작·기본 layout/present smoke만 증명하고 four-fit feature geometry를 증명하지 않는다.

| 플랫폼 | OS·기기 | 모드 | 캡처 | 실행 로그 |
| --- | --- | --- | --- | --- |
| iOS | iOS 26.2 · iPhone 17 Pro · 402×778 CSS px · 1206×2334 surface · 1206×2622 capture | light | [화면](./ios-light.png) | [로그](./ios-light.log) |
| iOS | iOS 26.2 · iPhone 17 Pro · 402×778 CSS px · 1206×2334 surface · 1206×2622 capture | dark | [화면](./ios-dark.png) | [로그](./ios-dark.log) |
| Android | API 37.0 · ARM64 AVD · 411.42856×841.52374 CSS px · 1080×2209 surface · 1080×2400 capture | light | [화면](./android-light.png) | [로그](./android-light.log) |
| Android | API 37.0 · ARM64 AVD · 411.42856×841.52374 CSS px · 1080×2209 surface · 1080×2400 capture | dark | [화면](./android-dark.png) | [로그](./android-dark.log) |

iOS 앱은 `mise exec -- bun run build:ios-sim`, Android 앱은 `mise exec -- bun run build:android`로 빌드했다. 원본 build 출력은 [iOS](./ios-build.log)와 [Android](./android-build.log)에 보존했다. iOS는 V8 static archive의 중복 timestamp dSYM 경고가 있었지만 `BUILD SUCCEEDED`였다. Android는 compile SDK/Gradle plugin 및 `ANDROID_NDK_HOME/ROOT` 차이 경고가 있었지만 `BUILD SUCCESSFUL`이며 SDK NDK `27.1.12297006`을 사용했다. Android renderer는 SwiftShader다. cold boot를 시도한 별도 API 37 AVD는 host `hvf` 미지원으로 실행되지 않아 사용하지 않았고, 기존 `default_boot` snapshot이 있는 `spinon_c114_cyclic_image_span_20261023`를 `-no-snapshot-save`로 실행해 smoke 후 종료했다. Android guest night mode를 원래 `no`로 복구했다.

PixelCopy/Metal drawable readback, compositor 이후 pixel parity, hardware GPU, 실기기, full WPT 및 앱 화면에서의 four-fit fixture geometry는 검증하지 않았다.

## Tailnet 로드맵 미리보기

`https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`에서 HTTP 200을 확인했고 Agent Browser 화면에 공식 `78/214`와 `C11.4 four-fit row-span · 진행 중` 카드가 표시됐다. PR 번호가 반영된 저장소 원본과 `/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html`의 SHA-256은 모두 `bbe44944d40dd9168c94172e7879efea0e430cb2cc1b2c2c7857bd75e51ea8c1`이었다. 직전 preview는 `roadmap.html.before-c11-4-four-fit-pr1357-20261111T1140Z`에 보존했고, 그 전 파일도 `roadmap.html.before-c11-4-four-fit-20261111T1116Z`에 남아 있다. 페이지 캡처는 [Tailnet roadmap preview](./tailnet-roadmap-preview.png)다.

Roadmap 확인 session `spinon-roadmap-pr1357-e00ea27987c0`와 PR 이미지 확인 session `spinon-pr1357-e00ea27987c0`를 사용 후 닫았다. 두 session의 최종 diagnostics는 `active=false`, PID `null`이고 `session list`는 비어 있다. PR 이미지 네 장은 `complete=true`로 로드됐다. 기존 Terminal Browser는 이 작업에서 열거나 조작하지 않았다.

## 프로세스·시뮬레이터 정리

실행 종료 후 `xcrun simctl list devices booted`와 `adb devices -l`에 실행 중 기기가 없고, Agent Browser session도 없다. S03 이름의 iOS Simulator profile은 등록되어 있지 않았다. 다른 프로젝트 소유로 표시된 `Hwpjs-RN-Owned-20261025`와 공유 iPhone/iPad·Android AVD profile은 삭제하지 않았다. 기존 Terminal Browser 프로세스는 이 작업에서 열거나 조작하지 않았다.

## 화면 SHA-256

```text
ef062a0fe4c881f1a353ad4e6e70ffcf60030a7be39cde27fa28e5da6165959d  android-dark.png
c74cdf36443e7a3d1a2762c0ed5779753651f1720726a8f9f53f13c590b9d610  android-light.png
bac0e27693fd2581e7f3e480f0cae399d814ba034c1adf2ca151ae14cc48774e  ios-dark.png
b4f8f631b16c9274d8dbd81c8a5d61b6e9e1548a289a3bb78ab37fbbc556966f  ios-light.png
eb701d1b4803f809cdd16025c34878645edaecd1d00e5e36756aa047c6e9c281  tailnet-roadmap-preview.png
```

별도 implementation/runtime 20개와 PR diff 30개 검토는 각각 [구현 검토](./implementation-runtime-review-20.md), [PR diff 검토](./pr-diff-review-30.md)에 기록했다. 최종 실행 명령과 scene 집계는 [`verification.json`](./verification.json)에, 각 로그·화면 파일의 SHA-256은 [`SHA256SUMS`](./SHA256SUMS)에 기록했다.
