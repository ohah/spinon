# C11.4 `text-transform: full-size-kana` 제한 runtime 후보

## 범위와 판정

구현 소스 기준 커밋은 `e6729ffbde7955f14f762232f188c02e845a755d`다. 기능은 C11.4 anonymous direct-text Grid fixture 경로에서만 동작하며 `c11-4-text-transform-unicode-fixture-test` opt-in이 필요하다. 계획 PR [#1081](https://github.com/ohah/spinon/pull/1081)은 병합됐고 구현 PR [#1082](https://github.com/ohah/spinon/pull/1082)는 검토 중이다. PR check rollup은 비어 있고 main required status checks도 없어 CI 통과로 세지 않는다.

허용 조건은 computed value `text-transform:full-size-kana`, Grid style owner의 직접 `lang="ja"`, Noto Sans KR 10px/400, horizontal LTR, `white-space:normal`, `word-break:normal`, `line-break:strict`와 정확한 고정 source allowlist다. CSS Text Level 3 CRD 2026-08-14 Appendix G의 Unicode 15.0 기반 58개 mapping을 normalized layout text에만 적용한다. HostDocument 원문과 UTF-16 source 범위는 유지한다. 이 결과는 일본어 일반 문장이나 임의 CSS 입력에 대한 공개 지원을 뜻하지 않는다.

Chrome 156은 `text-transform:full-size-kana`를 지원하지 않아 변환의 positive oracle로 사용하지 않았다. 브라우저 대조는 변환 결과를 literal로 둔 CSS Grid의 track/row geometry와 font 관찰에 한정했다. 변환 문자는 Appendix G에서 정했다. [사전 비교](../c11-4-anonymous-grid-text-transform-full-size-kana-precomparison-2026-11-03/README.md)와 [독립 기대값](expected-values.json)을 참고한다.

## 고정 10-scene 행렬

| Scene | 입력 → layout text | Column | Row | Lines |
| --- | --- | ---: | ---: | ---: |
| `none-max-content` | `あカンABC 12` → 동일 | 62.370003 px | 10 px | 1 |
| `hiragana-min-content` | `ぁぃ` → `あい` | 10 px | 20 px | 2 |
| `hiragana-max-content` | `ぁぃぅぇぉっゃゅょゎ` → `あいうえおつやゆよわ` | 100 px | 10 px | 1 |
| `katakana-min-content` | `ァィ` → `アイ` | 10 px | 20 px | 2 |
| `katakana-max-content` | `ァィゥェォッャュョヮ` → `アイウエオツヤユヨワ` | 99.5 px | 10 px | 1 |
| `halfwidth-min-content` | `ｧｨ` → `ｱｲ` | 5 px | 20 px | 2 |
| `halfwidth-max-content` | `ｧｨｩｪｫｯｬｭｮ` → `ｱｲｳｴｵﾂﾔﾕﾖ` | 45 px | 10 px | 1 |
| `supplementary-min-content` | `𛄲𛅐` → `こゐ` | 10 px | 20 px | 2 |
| `supplementary-max-content` | `𛄲𛅐𛅑𛅒𛅕𛅤𛅥𛅦𛅧` → `こゐゑをコヰヱヲン` | 90 px | 10 px | 1 |
| `pass-through-max-content` | `あカンABC 12` → 동일 | 62.370003 px | 10 px | 1 |

Android와 iOS에서 track 및 row 크기가 고정 비교 허용오차 `0.03125 CSS px` 이내였다. Chrome literal pass-through 폭은 `62.375px`, runtime은 `62.37000274658203px`로 허용오차 안이다.

## Android API 37 ARM64 AVD

최종 실행은 QEMU `sdk_gphone64_arm64`, serial `emulator-5556`의 [`android-avd-pass05`](./android-avd-pass05/)다. 10개 scene이 모두 통과했다. APK와 설치 APK SHA-256은 `052df9b9ef2f95d2a843a888f6b1684709e89c172da3fd74ecccbf7c6e395f99`로 같고 `sourceHead`는 `e6729ffbde7955f14f762232f188c02e845a755d`다. 전체 [행렬](./android-avd-pass05/matrix.tsv), [시뮬레이터 기록](./android-avd-pass05/simulator.json), scene 로그와 [대표 화면](./android-avd-pass05/representative-screen.png)을 보존했다.

## iPhone 17 Pro / iOS 26.2 Simulator

최종 실행은 UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`의 [`ios-simulator-pass02`](./ios-simulator-pass02/)다. 10개 scene이 모두 통과했다. 앱 executable과 설치 executable SHA-256은 `a79409d01d8673915207c48f6c86a8e1b00b37bf4ebed58193b3593a18491404`로 같고 `sourceHead`는 Android와 동일하다. 전체 [행렬](./ios-simulator-pass02/matrix.tsv), [시뮬레이터 기록](./ios-simulator-pass02/simulator.json), scene 로그와 [대표 화면](./ios-simulator-pass02/representative-screen.png)을 보존했다.

두 화면은 fixture 앱의 `layout=ready` 보고와 비어 있는 render surface만 보여준다. 이 harness에서는 glyph paint가 꺼져 있다. 따라서 텍스트 raster, GPU surface readback, compositor 출력 또는 픽셀 동등성 증거로 해석하지 않는다. 캡처 runner는 각 플랫폼 실행 뒤 앱을 종료한다.

## 실행·회귀·artifact 경계

- Android fixture 앱 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:android` — [`android-build-pass05.log`](./android-build-pass05.log), `BUILD SUCCESSFUL`.
- iOS Simulator fixture 앱 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim` — [`ios-build-pass06.log`](./ios-build-pass06.log), `BUILD SUCCEEDED`.
- `spinon-text-layout` opt-in Kana 검사 3/3, feature-off 경계 검사 1/1.
- `spinon-runtime` computed text-transform 검사 4/4, anonymous Grid HostDocument/source 보존 행렬 1/1.
- `spinon-ffi` 고정 direct-language scene catalog 1/1.
- CSS reference scene 검사 2/2, fixture module boundary 검사 32/32. Rust formatting 검사 통과.
- macOS ARM64 `target/release/libspinon_ffi.a`의 fixture 없는 `c04-runtime-gpu` 빌드 SHA-256은 `9487fae39fb19baf1a155293655d818ccedd222ed5213f4fc1289e209c249ba1`이다. 신규 scene marker 10개와 C11.4 fixture export가 모두 없다. [기본 archive scan](./ffi-archive-default-scan.json)을 참고한다.
- `c04-runtime-gpu-fixture-test` positive control archive SHA-256은 `fbcfbee018942deb6b2da631fce0eb194f6a9c0cfc31598a21151278e2e68025`다. scene marker 10개와 fixture export가 모두 들어 있다. [opt-in archive scan](./ffi-archive-fixture-scan.json)을 참고한다. 이는 macOS static archive feature 경계만 보이며 완성된 APK/IPA 전체에서 제외됨을 증명하지 않는다.

## 차이와 한계

- 후보는 at-risk로 표시된 CSS Text Level 3 CRD Appendix G mapping을 따른다. Chrome의 미지원 판정은 positive 구현 oracle로 취급하지 않는다.
- Katakana min-content의 runtime line advance는 `9.5/10px`, Chrome literal Range 관찰은 `10/10px`다. 양쪽 track은 10px이며 해당 line advance가 일치한다고 주장하지 않는다.
- 제한 runtime 검사는 ICU4X에서 `No segmentation model for complex script: Chinese/Japanese` 경고 16개를 출력한다. 고정 후보 scene은 통과했지만 일본어 일반 단어 분절을 입증하지 않는다.
- 일반 CSS author admission, 임의 source text/locale, WPT 적합성, glyph paint, 픽셀 일치, 실기기, hardware GPU 및 iOS compositor 이후 출력을 검증하지 않았다.
- 이전 실패는 별도 폴더에 유지했다. Android pass01은 scene mapping, pass02는 exact-frame 기대값, pass03은 fixture-test 비활성 상태에서 실패했다. iOS pass01은 UTF-8 byte 수와 문자 단위 trace chunk 수가 달라 실패했고, 문자 수 계산으로 수정했다. Android pass04는 수정 후 opt-in에서 10/10 통과했다. 최종 commit-bound Android pass05와 iOS pass02도 각각 10/10 통과했으며 실패 실행을 성공 cohort에 섞지 않았다.

성공·실패 실행을 포함한 모든 evidence 파일의 digest는 [`SHA256SUMS.txt`](./SHA256SUMS.txt)에 기록한다.
