# C13.1 viewport 밖 hit-test negative controls · Android AVD · 2026-10-14

이 실행은 agent-browser Chrome 155의 document.elementFromPoint()가 viewport 밖에서 반환하는 null을 host와 Android app runtime의 RuntimeRenderSnapshot::hit_test_css_point() 결과와 대조한다. 이 표본은 box hit query만 확인하며 DOM event dispatch나 전체 C13.1 완료를 뜻하지 않는다.

## 환경과 실행

- agent-browser 0.31.1, Chrome for Testing 155.0.8059.40, viewport 320×120 CSS px, DPR 1에서 네 경계 밖 표본을 조회했다. (-1,20), (100,-1), (320,20), (100,120) 모두 elementFromPoint()가 null이었다. 전체 oracle은 [Chrome observation JSON](../c13-1-overflow-clip-android-avd-2026-10-14/chrome155-runtime-observation.json), 새 좌표의 직접 관찰 결과는 [agent-browser JSON](agent-browser-negative-controls.json)에 있다.
- 실행 대상은 API 37 ARM64 AVD emulator-5554이며 [AVD properties](avd-properties.txt)에 ro.kernel.qemu=1, sys.boot_completed=1을 보존했다. [adb devices](adb-devices.txt) 출력에는 emulator-5554만 있다. 실기기는 연결하지 않았다.
- Android surface는 790×263 px, density 2.625, 계산 viewport 300.9524×100.1905 CSS px다. Chrome 좌표 네 점은 Android viewport에서도 모두 바깥이다. Renderer는 ANGLE/Vulkan/SwiftShader software renderer다.
- 첫 빌드는 격리 worktree에 build/v8-source/v8가 없는데 SPINON_V8_DIR를 생략해 prepareSpinonBootstrap에서 멈췄다. 고정 V8 소스가 있는 기존 checkout은 commit 7b50b62cb18f28617959e8452e2cd18195b38bcf이고 clean이었다. 이 경로를 읽기 전용으로 지정해 다시 빌드했으며 APK build는 성공했다. 저장소 checkout이나 V8 파일은 수정하지 않았다. 최종 빌드는 [build log](android-build-final.log)에 있다.
- 앱 PID는 6777, run ID는 c2952496a76이다. [원본 Android log](android-api37-arm64-hit-test-final.log), [PixelCopy PNG](android-api37-arm64-pixelcopy.png), [artifact hashes](artifact-sha256.txt), [runtime comparison](runtime-hit-test-negative-control-comparison.json)을 보존했다.

## 결과

- host Rust hit query는 기존 clip·margin·nested-axis·positioned-escape 11점과 새 viewport 밖 4점을 합쳐 15/15 Chrome 표본에 맞았다. elementId: null 표본은 실제 runtime 결과도 None이어야 통과한다. 예상 target이 있는 점에서 None이 나오거나, 예상이 null인 점에서 node가 나오면 fixture evaluation이 실패한다.
- Android 앱이 실제 publish할 runtime snapshot에서도 hit query 15/15, viewport 밖 null control 4/4가 일치했다. 로그는 표본별 JSON 행을 남긴다. Chrome oracle 순서, 좌표, target/frame index를 [comparison JSON](runtime-hit-test-negative-control-comparison.json)에서 다시 검사했다.
- 동일 실행의 PixelCopy는 원래 paint 표본 11/11 exact를 유지했다. hit query 15개와 paint 표본 11개는 독립된 inventory다. 캡처 PNG SHA-256은 2e2ad5e6ae842fa89015a05b44642e92c34a778915ef6c382dbb40a360ab7585다.
- [feature isolation](feature-isolation.txt)은 C13.1 fixture feature 빌드에서 새 viewport-left-outside oracle 문자열과 C13.1 FFI 심볼이 존재하고, 기본 FFI 정적 라이브러리에서는 둘 다 없는 것을 matching LLVM 22 llvm-nm 및 strings로 확인했다. 이 결과는 host static archive만 검사한다.

## 검증과 남은 범위

cargo test --locked --workspace, cargo test --locked -p spinon-ffi --features c13-1-overflow-clip-fixture-test, workspace Clippy, feature-on FFI Clippy, rustfmt와 git diff --check가 통과했다. 실행 로그는 [workspace test](workspace-cargo-test.log), [feature FFI tests](c13-feature-ffi-cargo-test.log), [workspace Clippy](workspace-clippy.log), [feature FFI Clippy](c13-feature-ffi-clippy.log), [rustfmt](rustfmt.log), [diff check](git-diff-check.log)에 있다.

네 좌표는 viewport 바깥에서 node가 반환되지 않는지 확인한다. clip rectangle 안팎의 모든 좌표, Android 사용자 입력 전달, DOM listener dispatch, 다른 viewport·DPR, iOS Simulator, WPT 및 같은 runtime worker의 clip ancestry allocation failure recovery는 아직 확인하지 않았다. AVD 결과는 실기기나 scanout 후 compositor 출력으로 확대하지 않는다.
