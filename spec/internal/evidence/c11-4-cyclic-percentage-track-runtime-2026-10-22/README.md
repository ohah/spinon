# C11.4 cyclic percentage track runtime 근거

2026-10-22에 cyclic percentage column·row와 definite percentage control을 기존 C11.4 opt-in runtime fixture에 연결했다. 계산 알고리즘은 새로 만들지 않고 `RuntimeBlockFlexGridTrackSizingV1`과 Taffy 0.15.0 경로를 사용한다.

## 기준과 고정 입력

CSS Grid Layout Module Level 1 §7.2.1의 2025-03-26 CRD를 규범 기준으로 삼는다. cyclic percentage는 intrinsic container sizing 중 auto처럼 취급하고, 결과 inner size를 기준으로 final track percentage를 resolve한다. 기존 Chrome 155 `indefinite-percent-track`은 column 50% → 12px의 pinned baseline으로 유지한다. 세 고정 scene의 추가 Chrome 관찰은 HeadlessChrome 156.0.8078.48로 따로 기록했으며 Chrome 155 baseline으로 취급하지 않는다. 독립 axis basis 계산과 fixture는 [사전 비교 근거](../c11-4-cyclic-percentage-track-runtime-plan-2026-10-22/README.md), [HTML 입력](../../../../tests/fixtures/css/c11/grid-cyclic-percent-tracks-c114-precomparison.html)에 있다.

| scene | final Columns / Rows | Grid / item frame | 표면 경계 표본 |
| --- | --- | --- | --- |
| `cyclic-percent-column` | `[12]` / `[16]` | `24×16` / `24×16` | x=11, 12에서 red item |
| `cyclic-percent-row` | `[24]` / `[8]` | `24×16` / `24×16` | y=7, 8에서 red item |
| `definite-percent-column` | `[20]` / `[16]` | `40×16` / `24×16` | x=19, 20에서 red item |

## 구현과 검증

- Rust scene adapter는 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/cyclic_percentage_tracks.rs`에 두고 `c04-runtime-gpu-fixture-test`에서만 연결했다.
- runner는 정확히 세 scene의 두 축 used track, 전체 node frame, box 수, draw/present 및 두 boundary pixel을 확인한다.
- API 37 ARM64 AVD `emulator-5558`에서 3/3 scene과 PixelCopy 표본 6/6 exact를 확인했다. local APK와 설치 APK SHA-256은 `b33090f9196784cd7a0391af0c981a8bf1dae38b63bdfbb13c5a6b52eeafa747`로 일치한다. 원본은 [Android 실행 폴더](./android-avd-c114-cyclic-percent-tracks-android-avd-20261021T193719031Z-1a84be50/)에 있다.
- iPhone 17 Pro / iOS 26.2 Simulator에서 3/3 scene과 Metal drawable 표본 6/6 exact를 확인했다. 이 표본은 compositor 제출 전 drawable 결과다. 원본은 [iOS 실행 폴더](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261021T193727291Z-2dcc3441/)에 있다.
- 화면: [Android column](./android-avd-c114-cyclic-percent-tracks-android-avd-20261021T193719031Z-1a84be50/cyclic-percent-column.png), [Android row](./android-avd-c114-cyclic-percent-tracks-android-avd-20261021T193719031Z-1a84be50/cyclic-percent-row.png), [iOS column](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261021T193727291Z-2dcc3441/cyclic-percent-column.png), [iOS row](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261021T193727291Z-2dcc3441/cyclic-percent-row.png).
- `mise exec -- cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test exposes_cyclic_percentage_track_runtime_scenes`: 1/1 통과.
- `mise exec -- bun test tools/css-reference/fixture-module-boundary.test.mjs`: 6/6 통과. `mise exec -- cargo fmt --all -- --check`, `git diff --check`, Android fixture APK build와 iOS fixture Simulator build도 통과했다.
- 기본 `spinon-ffi` release `.rlib`에서 세 scene ID와 module marker가 빠지고, opt-in fixture-test positive control에 세 ID와 fixture symbol/payload가 들어가는 것을 `strings`로 확인했다. archive SHA와 앱 산출물 hash는 [artifact 경계 JSON](./artifact-boundary.json)에 기록했다.

초기 AVD 실행은 `justify-items:start`가 CSS projection에서 지원되지 않아 거부됐다. 고정 oracle에서 필요하지 않은 `align-items`/`justify-items`를 runtime fixture 입력에서 제거해 HTML oracle의 alignment만 남긴 뒤 전체 플랫폼 행렬을 다시 실행했다. 최초 오류 로그는 [실패한 초기 시도](./android-avd-c114-cyclic-percent-tracks-android-avd-20261021T193402478Z-9b5eb25f/runtime-cyclic-percent-column.log)에 보존한다.

Android logcat 원문 중 줄 끝 padding 공백이 있는 다섯 파일은 읽기 쉬운 `.log` 사본에서 줄 끝 공백만 제거했다. 같은 디렉터리의 `.log.raw.gz`는 수정 전 byte-exact logcat 출력이다.

## 한계

이 결과는 세 고정 opt-in fixture와 simulator surface에 한정한다. Android 실기기, iOS compositor 이후 화면 픽셀, glyph, WPT, public CSS admission, `calc()` track, percentage `minmax()`/`fit-content()` limit, 여러 percentage track, 같은 Grid의 양 축 cyclic 동시 resolve, nested Grid와 전체 C11.4는 검증하지 않았다. C11.4 공식 checkbox는 미완료이며 완료 수 `72/213`을 유지한다.

## 캡처 소스

두 최종 simulator capture는 base commit `93126f9928478182be3ce0d2a8137083f53402df`에서 수행했다. runner source patch SHA-256은 `e9f5d30fc7a4c76e95a9cc9b21a8515eacc15963dee47d8703f73ca21c4c22b4`이며, staged 상태의 다섯 구현/runner source 경로를 포함한다. 전체 simulator artifacts 및 source hash 목록은 [SHA256SUMS](./SHA256SUMS)다.
