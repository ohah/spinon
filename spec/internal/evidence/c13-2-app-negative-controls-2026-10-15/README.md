# C13.2 앱 FFI 오류 원자성·stale publish 후속 검증 · 2026-10-15

## 판정 범위

C13.2 root/body 및 26-frame·9-owner app route에 deterministic failure controls를 추가했다. 일회성 scroll-state reservation failpoint가 예상 `ERR_LAYOUT`과 정확한 오류 marker를 만들고, 오류 뒤 기존 `RuntimeRenderSnapshot`이 `PartialEq` exact로 유지되는지, 같은 host가 회복하는지 앱 FFI 경로에서 확인한다. 이어 이전 presentation sequence로 publish를 시도해 거부되는지, 새 평가 뒤 같은 host가 정상 presentation을 다시 만드는지 확인한다.

양 플랫폼의 앱 gate는 `c13_2_failure_controls=passed`, `reserve_failure=atomic`, `reserve_failure_marker=matched`, `stale_publish=rejected`, `recovery=same_host`를 요구한다. 최종 runtime log와 앱 화면은 아래에 연결했다.

이 근거는 OS allocator의 실제 OOM이나 프로세스 전체 메모리 압박을 재현하지 않는다. stale control은 이미 무효화한 이전 sequence를 순차적으로 다시 publish하는 결정적 negative control이며 동시 scheduler race를 증명하지 않는다. equality는 `RuntimeGpuHost`의 저장 scroll-state snapshot을 확인한다. 일반 `eval`은 시작 때 현재 presentation sequence를 무효화하므로 실패 중 기존 화면이 계속 draw 가능한 상태로 남는다는 주장은 하지 않는다. Android PixelCopy와 iOS Metal drawable pixel readback도 실행하지 않았다. ink-only `box-shadow` overflow는 runtime에서 지원되지 않아 fail-closed이며 이 검증의 pass 범위가 아니다. C13.2와 상위 C13은 미완료다.

## 고정 판정 기준

1. root/body 장면, 9-owner matrix, offset 및 네 lifecycle 단계를 먼저 성공시킨다.
2. 현재 scroll-state snapshot을 복사하고 일회성 reservation failpoint를 켠다. 실패가 `ERR_LAYOUT`이고 report에 `c13_2_injected_reserve_failure`가 있으며 failpoint 관찰 flag도 켜져야 한다.
3. 오류 뒤 저장된 snapshot이 기존 snapshot과 exact-equal이어야 한다. marker 없이 발생한 다른 layout 오류는 pass가 아니다.
4. 동일 host에서 matrix를 다시 평가해 `layout=ready`를 얻는다.
5. 현재 sequence를 저장하고 invalidate한 다음 이전 sequence로 publish한다. 호출은 거부되어야 하며 저장 snapshot은 그대로여야 한다.
6. 동일 host에서 다시 평가하고 scroll offset을 적용한다. 9-owner gate, 최종 presentation 존재, published sequence와 scroll-state snapshot equality를 통과해야 한다.

## 실행 중 찾아 수정한 결함

- 같은 V8 global lexical scope에서 matrix를 다시 실행하자 `SyntaxError: Identifier 'nodes' has already been declared`가 발생했다. matrix source를 IIFE로 감싸 source 재평가의 lexical binding을 분리했다. 초기 Android trace (`android-api37-avd-initial-rerun.log`).
- matrix를 재실행할 때 이전 matrix root와 root/body root가 연결된 채 남아 `multiple_host_roots`가 났다. 이전 root와 lifecycle/map 참조를 먼저 정리하도록 수정했다. 다중 root trace (`android-api37-avd-multiple-roots.log`).
- layout frame의 completed key와 render snapshot을 full `PartialEq`로 비교해 scroll revision만 달라도 layout이 stale로 오인됐다. scroll revision을 제외한 generation/document/render-tree/style/environment revision tuple과 non-empty render snapshot을 확인하도록 좁혔다. 마지막 Android/iOS 앱 실행에서 matrix가 통과했다.
- 첫 실패 control은 failpoint 관찰 flag와 `ERR_LAYOUT`만 확인하고 report의 주입 marker는 확인하지 않았다. 다른 `ERR_LAYOUT`을 오인할 수 있어 marker를 정확히 검사하도록 보강했고, Android/Swift app gate도 `reserve_failure_marker=matched`를 요구한다. 이 수정 뒤 두 Simulator를 다시 빌드·실행했다.
- feature 전용 Clippy에서 기존 `unnecessary_map_or`, `needless_lifetimes`, `map_identity` 경고가 드러났다. 관용 표현으로 고치고 feature 전용 Clippy를 다시 통과시켰다. 발견 log (`ffi-feature-clippy-marker-verification.log`).

## Android API 37 ARM64 AVD

- AVD: `spinon_api37_0_applied_txn`, serial `emulator-5556`, API 37, `arm64-v8a`, ANGLE/Vulkan/SwiftShader.
- Android build: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST=1 mise exec -- bun run build:android` — build log (`android-build-final.log`), `BUILD SUCCESSFUL`.
- 실행: `adb -s emulator-5556 shell am start -W -n dev.spinon.bootstrap/.MainActivity --ez spinon_c132_scroll_fixture true` — install (`android-install-final.log`), launch (`android-launch-final.log`).
- APK SHA-256: `1431910eb44085839580e3ce77354474255b6f48d7ece690edf85a1fea72ba11` (hash file (`android-apk-final.sha256`)).
- 앱 log: Android runtime (`android-api37-avd-runtime-final.log`). 단일 assertion 행에 root/body, border escape paint/hit, lifecycle, `reserve_failure_marker=matched`, stale publish 거부, same-host recovery가 기록됐다. 이어진 draw는 `boxes=26`, `scroll_state_revision=8`, 9개 offset을 보고했다.
- Android 앱 화면 (`android-api37-avd-final.png`), 1080×2400, SHA-256 `36e5bf9d3eccd0e277592967170ca0ba6c1daa1ee7cd5f045eba25b16793e780`. 화면은 root/body의 visible border escape를 육안으로 보여 준다. 이 Android status label은 첫 root/body eval의 `boxes=5`를 표시하고, 최종 matrix의 26 boxes는 runtime draw log로 확인했다.

## iPhone 17 Pro / iOS 26.2 Simulator

- Simulator UDID: `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`; 실기기는 사용하지 않았다.
- iOS build: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim` — build log (`ios-simulator-build-final.log`), `BUILD SUCCEEDED`.
- 실행: `xcrun simctl launch ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D dev.spinon.bootstrap --spinon-c132-scroll` — launch (`ios-launch-final.log`), runtime (`ios-26-2-runtime-final.log`).
- 앱 runtime의 assertion 행이 Android와 같은 marker들을 모두 보고했고 `status=0`으로 끝났다. renderer는 iOS Simulator Metal을 사용했다.
- executable SHA-256: `c571129c15bc0c87e9aca18d34ce06dd7e90daf1ff1fb9f0d4634dd32d19f83c` (hash file (`ios-executable-final.sha256`)).
- iOS Simulator 화면 (`ios-iphone17pro-ios26_2-final.png`), 1206×2622, SHA-256 `db053cd700cc677fa12aa1e6a729c23d3b351bc48e894507e5490bde75d283ce`.

## 코드·회귀 및 빌드 경계

- source 기준: base `430634f8a1e1581a9373c549871aa601879d2bb7`; C13.2 Rust/Android/iOS/JS 변경 patch SHA-256 `a216e67caf7cfcdf66d44eab7237788f06bce9ebf1db81aae053679797ed106f`; V8 `7b50b62cb18f28617959e8452e2cd18195b38bcf` (checkout clean). source revision (`source-revision.json`).
- `cargo fmt --all -- --check` 통과; 로그 (`rustfmt-final.log`).
- `cargo test --locked -p spinon-ffi --features c13-2-scroll-fixture-test --lib`: 43 passed, 0 failed, 2 ignored; 로그 (`ffi-feature-test-final.log`).
- `cargo test --locked --workspace`: 797 passed, 0 failed, 4 ignored (26 test suites); 로그 (`workspace-test-final.log`).
- `cargo clippy --locked --workspace --all-targets -- -D warnings`와 `cargo clippy --locked -p spinon-ffi --features c13-2-scroll-fixture-test --lib -- -D warnings` 모두 통과; workspace (`workspace-clippy-final.log`), feature (`ffi-feature-clippy-final.log`).
- `node --test tools/css-reference/c13-2-scroll-precomparison.test.mjs`: 5/5, 로그 (`reference-test-final.log`). `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 5/5, 로그 (`fixture-boundary-test-final.log`). `git diff --cached --check` 통과, 로그 (`diff-check-final.log`).
- PR diff 검사를 위해 텍스트 실행 log 16개의 trailing whitespace와 끝의 빈 줄만 제거했다. 로그의 비공백 본문은 유지했다.
- `mise exec -- bun run docs:build`는 현재 저장소 문서 전반의 390개 dead-link 보고로 실패했다. 기존 내부 문서의 바이너리 evidence 링크와 `0062`의 문서 사이트 밖 `../../plan/` 링크가 포함된다. 이번 evidence README의 로그·이미지·JSON은 일반 Markdown URL로 만들지 않았으며 최종 build trace에는 새 evidence README의 dead-link 항목이 없다. Rspress log (`docs-build-final-after-link-adjustment.log`).
- default `c04-runtime-gpu` static archive 524,889,680 bytes, SHA-256 `4f7dad8b5bbc814cef63020476700801645b8e1e5f10ccdabaacd8aa46ebfc20`; C13.2 FFI symbol·payload·error marker 모두 absent. 기본 build (`default-archive-build-final.log`) · 기본 scan (`default-archive-scan-final.log`).
- opt-in `c13-2-scroll-fixture-test` static archive 534,005,544 bytes, SHA-256 `b049ed4647d0dbd3a27b333e6ebb10155ccc19f38083cbc677d2119156e9fcfc`; FFI symbols, `c132-root`, injected error와 success marker가 positive control로 present. opt-in build (`opt-in-archive-build-final.log`) · opt-in scan (`opt-in-archive-scan-final.log`).
- Apple `nm`은 관련 없는 Rust 1.96 LLVM 22 `compiler_builtins` attribute를 읽지 못했다. 격리 판정은 기본/opt-in archive `strings` 대조로만 보고한다. 정적 archive 근거를 최종 앱 번들 크기나 release artifact 판정으로 확대하지 않는다.

## PR 독립 검토

- [구현 실패 경로 20개](../c13-2-app-negative-controls-implementation-review-20-2026-10-15.md): 이 앱 failure-control 변경의 오류·상태·복구 경계를 검토했다.
- [PR 변경 적대 검토 30개](../c13-2-app-negative-controls-pr-review-30-2026-10-15.md): feature boundary, app gates, evidence, simulator output 및 게시/병합 경계를 별도로 검토했다.
- C13.2는 전체 pass가 아니다. ink-only overflow, Android PixelCopy/iOS Metal pixel readback, OS OOM, concurrent scheduler race, public CSSOM/gesture/event, scrollbar, snap, RTL/logical axes 및 상위 C13은 남아 있다.
