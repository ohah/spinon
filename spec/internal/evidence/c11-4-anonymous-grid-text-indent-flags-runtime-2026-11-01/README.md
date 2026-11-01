# C11.4 anonymous Grid `text-indent` flags 실행 근거

이 근거는 anonymous direct-text Grid의 고정 opt-in fixture에 `text-indent: <length> hanging each-line`을 연결한 제한 slice만 다룬다. 일반 CSS admission이나 C11.4/C11 완료를 뜻하지 않는다. 기준 Chromium 156 관측과 독립 Ahem 계산은 [사전 비교](../c11-4-anonymous-grid-text-indent-flags-precomparison-2026-11-01/README.md)에 있다.

## 구현 PR

구현 PR [#959](https://github.com/ohah/spinon/pull/959)은 2026-11-01T02:06:32Z Rebase and merge로 병합됐고 merge SHA는 `590f5946e97e0252113abf34b23ae31b13819acf`다. GitHub check rollup은 비어 있고 required checks도 보고되지 않아 CI 통과로 기록하지 않는다. C11.4/C11 완료 확인란과 공식 `72/213`은 미완료로 유지한다.

## 최종 결과

최종 수정 코드로 Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator의 각 행렬을 다시 실행했다.

| 행렬 | Android API 37 ARM64 AVD | iPhone 17 Pro / iOS 26.2 Simulator |
| --- | ---: | ---: |
| `hanging`·`each-line` flags | [20/20 draw/present](./android-avd-c114-computed-text-indent-flags-android-avd-1793498160267-3506f280/matrix.tsv) | [20/20 draw/present](./ios-simulator-c114-computed-text-indent-flags-ios-simulator-1793498188616-f37e43f9/matrix.tsv) |
| 양수 회귀 | [18/18 draw/present](../c11-4-anonymous-grid-text-indent-runtime-2026-11-01/android-avd-c114-computed-text-indent-positive-android-avd-1793498215176-fa585dfe/matrix.tsv) | [18/18 draw/present](../c11-4-anonymous-grid-text-indent-runtime-2026-11-01/ios-simulator-c114-computed-text-indent-positive-ios-simulator-1793498240700-24ba943b/matrix.tsv) |
| 양수 포함 음수 profile | [30/30 draw/present](../c11-4-anonymous-grid-text-negative-indent-runtime-2026-11-01/android-avd-c114-computed-text-indent-negative-android-avd-1793498266019-85db63d2/matrix.tsv) | [30/30 draw/present](../c11-4-anonymous-grid-text-negative-indent-runtime-2026-11-01/ios-simulator-c114-computed-text-indent-negative-ios-simulator-1793498306331-56c5719e/matrix.tsv) |

총 136개 scene에서 기대한 root frame의 draw/present와 양수 surface texture를 확인했고 superseded retry는 0이었다. Android AVD는 `emulator-5554`, `sdk_gphone16k_arm64`, API 37, `arm64-v8a`다. iOS Simulator는 `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`, 모델 `iPhone18,1`이다. 설치된 APK SHA-256은 `997f08b094ba06f8443ec945e626e053a093916046fc4a6e2169dfbe57487128`, 설치된 iOS executable SHA-256은 `d1999a7a51d1287ddad9f002138c73b68609c04f8a1b44cf7c9d9fcc63dba43d`이며 각 capture receipt에서도 빌드/설치 hash가 일치한다. [Android build log](android-build-final.log); iOS build succeeded and its installed hash is recorded by the simulator receipts.

검증기는 computed flags, 줄 수, 줄별 advance/indent, intrinsic min/max, Grid track 및 root/item frame을 비교한다. [대표 Android 화면](./android-avd-c114-computed-text-indent-flags-android-avd-1793498160267-3506f280/representative-screen.png)은 실행 뒤 fixture harness의 `layout-ready` 상태를 보여준다. 검은 renderer 영역은 glyph raster나 text pixel 검증이 아니다.

## focused tests와 lint

- [text-layout flags test](text-layout-test-final.log): 1/1 통과.
- [runtime text-indent tests](runtime-test-final.log): 9/9 통과.
- [FFI fixture source tests](ffi-test-final.log): 2/2 통과.
- [CSS reference Node contract test](node-test-final.log) 통과, `cargo fmt --all -- --check` 통과.
- [text-layout Clippy](text-layout-clippy-final.log), [fixture runtime Clippy](runtime-clippy-final.log), [fixture FFI Clippy](ffi-clippy-final.log)이 모두 `-D warnings`로 통과.

## capture runner 보완

초기 runner는 layout summary에서 수집을 끝내 draw/present를 확인하지 못했다. 보완한 runner는 scene별 프로세스에서 `SPINON_C0410_DRAW status=0 presented boxes=1`, 기대한 `root_css_px=120×height`, 양수 surface texture 크기를 요구하고 generation/retry 수도 matrix에 기록한다. iOS log는 현재 process identifier에 한정해 수집한다.

첫 iOS forced-break 실행에서 두 줄 paragraph의 `maxContentHeight` 기대값이 10px인 오류와 첫 Android negative matrix에서 max-content line count에 min-content 줄 수를 쓴 오류가 드러나 수정했다. Stylo 0.22 fallback도 예상 diagnostic이 없을 때 authored flags를 수용하던 경계를 발견해 정확한 단일 diagnostic·source·node·위치·메시지가 일치할 때만 허용하도록 좁혔다. 수정 후 focused tests와 양 플랫폼 전체 행렬을 다시 통과했다.

## static archive feature 경계

macOS ARM64 Rust 1.96.1 기본 `c04-runtime-gpu` archive에는 flags scene ID 0/20개와 C11.4 host export 0/2개, `c04-runtime-gpu-fixture-test` positive control에는 각각 20/20개와 2/2개가 있다. [빌드·hash·scanner 근거](./artifact-boundary/README.md)는 macOS static archive 범위다. 완성된 APK/IPA 전체 bundle에서 fixture가 제외됨을 증명하지 않는다.

이 결과는 Android AVD와 iOS Simulator 확인이다. Android 실기기/hardware GPU, iOS compositor 이후 화면, glyph pixels, 전체 WPT, 일반 author CSS admission, 임의 값·글꼴·언어·writing mode 및 전체 C11.4/C11 지원은 검증하지 않았다. 공식 C11/C11.4 완료 확인란과 `72/213` 집계는 유지한다. [계획](../../../../plan/c11-4-anonymous-grid-text-indent-flags.md) · [구현 실패 관점 20개](../../../../plan/c11-4-anonymous-grid-text-indent-flags-implementation-review-20.md) · [PR diff 실패 관점 30개](./pr-diff-review-30.md)
