# C11.4 `word-break: break-word` × `overflow-wrap` simulator matrix

이 증거는 [독립 사전 비교](../c11-4-anonymous-grid-text-word-break-break-word-precomparison-2026-11-01/README.md)의 Ahem 계산과 pinned Chrome 관측에서 고정한 18개 조합을 앱의 opt-in fixture route에서 실행한 결과다. 테스트 장면은 `word-break: normal|break-word`, `overflow-wrap: normal|break-word|anywhere`, `min-content|max-content|10px` track의 곱이다.

## 실행 결과

- Android API 37.1 ARM64 16 KB AVD `spinon_api37_1_compat16k` (`emulator-5560`, SwiftShader/swangle)에서 **18/18** 통과했다. APK와 설치 APK SHA-256은 `073b4f367f0a08e3d04d8674fca3cf5fc09301a27676bd3b7cb3900db219700e`로 같다. [기기·APK 정보](./android-avd/simulator.json) · [matrix](./android-avd/matrix.tsv) · [장면별 로그](./android-avd/)
- iPhone 17 Pro (`iPhone18,1`) / iOS 26.2 Simulator에서 **18/18** 통과했다. 빌드 executable과 설치 executable SHA-256은 `0c59b77c569b33af8acbd67f1e94f0f9da5b7b7eae6960ac27f1be35fdf326ea`로 같다. [simulator·executable 정보](./ios-simulator/simulator.json) · [matrix](./ios-simulator/matrix.tsv) · [시도 원장](./ios-simulator/attempts.tsv) · [장면별 로그](./ios-simulator/)
- Capture는 독립 expected-values 파일의 장면만 실행하고, 각 run에서 computed `word-break`/`overflow-wrap`, owner node와 문서 revision, item frame, paragraph locale/font/line advances, intrinsic sizes와 최종 Grid column/row track을 검사한다. 각 matrix의 본문 18행은 모두 `PASS`다.
- Rust 검증은 style cascade/alias 12개, combined runtime 5개, 기존 word-break 2개, overflow-wrap 2개, text-layout 8개, FFI dispatcher 1개 테스트가 통과했다. `spinon-style` fixture-feature Clippy와 `cargo fmt --all -- --check`도 통과했다. Fixture-module boundary suite는 31/31 통과했다.
- 기본 `c04-runtime-gpu` FFI rlib에서 새 scene ID 문자열이 없고, opt-in Android/iOS 앱 executable에는 positive-control 문자열이 포함됐다. 이는 해당 fixture source isolation을 확인하는 문자열 검사다. Android/iOS release bundle 전체 누출 또는 크기를 측정한 근거로 확대하지 않는다.

## 캡처 한계와 정리

두 simulator build는 `C15.3 WGPU glyph paint fixture=0`이었다. 캡처 화면은 viewport와 fixture 실행 상태를 확인하는 보조 기록이며 glyph painting이나 픽셀 동일성 증거가 아니므로 PR 본문에는 동작 화면으로 첨부하지 않는다. 동작 판정은 위 trace 로그로 한다.

처음 Android scene dispatch 한 건은 host allowlist 누락으로 layout 전 거부됐다. 해당 진단은 [보존 로그](./android-avd/first-host-dispatch-rejected-word-break-normal-overflow-wrap-normal-min-content.log)에 남겼고, Android/iOS host scene catalog를 보정한 뒤 현재 두 matrix를 다시 실행했다. 이 진단은 layout assertion 실패가 아니다.

이 실행에서 직접 만든 Android AVD `emulator-5560`과 앱 프로세스는 종료했다. 기존 `emulator-5556`과 iOS simulator boot 상태는 보존했고, 사용한 iOS app 프로세스는 종료했다. Agent Browser를 띄우지 않았다.

이 slice는 고정 Ahem fixture와 simulator 실행만 다룬다. 공개/general author CSS admission, author `@layer` syntax, custom-property로 생성한 alias, 임의 text/font/language/writing-mode, WPT, 실기기/hardware GPU, glyph pixels, iOS compositor 이후 surface와 전체 C11.4/C11은 포함하지 않는다. 공식 상태의 C11/C11.4 체크와 `72/213`은 그대로다. [구현/runtime 검토 20개](./implementation-runtime-review-20.md) · [후보 소스 revision](./source-revision.json)
