# C13.2 root/body app 검증 후속 · 2026-10-15

## 판정 범위

PR #172 뒤에 빠져 있던 root/body 실제 앱 장면을 Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 실행했다. visible root/body viewport 전파와 `html { overflow:hidden }` 아래 body의 독립 scroll owner가 Chrome 155 값과 일치했다. 두 simulator route는 이 두 장면 다음에 기존 C13.2 26-frame·9-owner 장면과 lifecycle을 실행한다.

이 근거는 C13.2 완료 판정이 아니다. Android `PixelCopy`와 iOS Metal drawable pixel readback은 연결하지 않았다. 스크린샷은 육안 확인용이며, border 밖 빨간 자식은 frame assertion과 escaped position의 hit-test identity assertion으로 확인했다. ink-only overflow와 app route의 stale/failure injection도 미실행 상태였다. 이 문단은 이 root/body 실행 당시의 범위를 기록한다. 이후 deterministic reservation failure, stale presentation rejection과 same-host recovery는 [별도 앱 오류 원자성 후속 근거](../c13-2-app-negative-controls-2026-10-15/README.md)에서 Android/iOS Simulator로 확인했다. OS OOM, concurrent scheduler race, ink-only overflow와 simulator pixel readback은 여전히 남아 있다.

## Chrome 155 기준

`agent-browser` HeadlessChrome 155에서 viewport `400×240`으로 [visible root/body fixture](../../../../tests/fixtures/css/c13/c13-2-root-body-400x240-visible.html)와 [hidden root/body fixture](../../../../tests/fixtures/css/c13/c13-2-root-body-400x240-hidden.html)를 열고 programmatic scroll 뒤의 값을 기록했다. [고정 observation JSON](chrome-root-body-400x240.json)에는 user agent, computed overflow, scrolling element, scroll/client size, root/body offset과 fixed rect를 보존했다.

| 장면 | Chrome 결과 |
| --- | --- |
| html visible, body auto | HTML/viewport owner `64×84`; body offset `0×0`; fixed rect `(340,204) 52×28` |
| html hidden, body auto | HTML range `0×0`; body owner `64×84`; window/html offset `0×0`; fixed rect `(340,204) 52×28` |

## Android API 37 ARM64 AVD

- AVD `spinon_api37_0_applied_txn` / serial `emulator-5556`, Android API 37.0, `arm64-v8a`에 `-read-only` mode로 설치·실행했다. renderer는 SwiftShader였다. 실기기에는 ADB 명령을 보내지 않았다. 검증 뒤 이 worktree가 시작한 AVD만 종료했다.
- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST=1 mise exec -- bun run build:android`.
- 격리 worktree에는 ignore된 V8 checkout이 없어 고정된 clean V8 cache를 `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8`로 지정했다. V8 revision은 `7b50b62cb18f28617959e8452e2cd18195b38bcf`이며 cache 파일은 수정하지 않았다.
- 실행: `adb -s emulator-5556 shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_c132_scroll_fixture true`.
- 앱 assertion: `root_body_propagation=passed`, `root_body_hidden=passed`, `border_escape_outside_parent=true`, `border_escape_hit=matched`, 네 lifecycle 경계 모두 통과.
- 최종 WGPU draw는 runtime log에 기록했다. 화면의 작은 status label은 첫 root/body 평가의 `boxes=5`를 표시할 수 있으므로 최종 scene frame 수 판정은 draw log와 별도 assertion을 사용한다.
- [AVD 화면](android-api37-avd-root-body.png), 1080×2400, SHA-256 `703c76cbf4c2c200930cd4715803d6a6d3eb119402c0e8de56c28705e1164b5f`.
- APK SHA-256 `32480367459471fa20ac62abaebabecb58d683dcb021b7b14862f236eec3424a`.
- [runtime log](android-api37-avd-runtime.log).

### ADB 설치 지연 조사

- 첫 시도는 이미 실행 중이던 `emulator-5554`에 `adb install`과 `pm path`를 요청했으나 두 명령이 1분 이상 끝나지 않았다. ADB server는 응답했고 `adb devices -l`은 해당 serial을 `device`로 표시했다.
- QEMU PID `86401`의 `lsof` 작업 디렉터리는 `/private/tmp/spinon-c14-intrinsic-sizing`이어서 이 C13.2 worktree 소유의 전용 AVD가 아니었다. 우리 쪽 install·조회 프로세스만 종료하고 그 AVD는 그대로 두었다.
- AVD `spinon_api37_0_applied_txn`을 별도 serial `emulator-5556`에 read-only로 부팅하자 동일 APK가 `Success`로 설치되고 앱 assertion과 draw가 통과했다. 따라서 실패는 APK/build 문제가 아니라 첫 공유 emulator endpoint에서 발생한 지연으로 범위를 좁혔다. emulator 내부 hang과 ADB transport contention 중 어느 것이 직접 원인인지는 증거만으로 확정하지 않는다.

## iPhone 17 Pro / iOS 26.2 Simulator

- Simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, arm64 simulator. 실기기는 사용하지 않았다.
- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim`.
- Android와 같은 `SPINON_V8_DIR` 고정 checkout을 읽기 전용으로 사용했다.
- 실행: `xcrun simctl launch ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D dev.spinon.bootstrap --spinon-c132-scroll`.
- 앱 assertion과 final draw에서 Android와 동일한 root/body·border escape·lifecycle 표지를 확인했다. renderer는 iOS Simulator GPU Metal, drawable `1200×720`, viewport `400×240`으로 보고했다.
- [Simulator 화면](ios-iphone17pro-ios26_2-root-body.png), 1206×2622, SHA-256 `35502a6de76fb2be1fe55d65df639c78cab5f0fe5f0a4ecd45ca9837a42d6ecc`.
- 앱 executable SHA-256 `23ead0f7b6c83bde29cd7ab3210c3bbdc40272e5c535512843d6b405d73a7c44`.
- [runtime log](ios-26-2-simulator-runtime.log).

## 회귀·경계 검증

- `cargo test --locked --workspace`: 797 passed, 0 failed, 4 ignored. [전체 log](workspace-test.log).
- `cargo test --locked -p spinon-ffi --features c13-2-scroll-fixture-test --lib`: 43 passed, 0 failed, 2 ignored. [feature FFI log](ffi-feature-test.log).
- `cargo clippy --locked --workspace --all-targets -- -D warnings` 통과. [Clippy log](workspace-clippy.log).
- `cargo fmt --all -- --check`, C13.2 Chrome reference test 5/5, fixture boundary test 5/5, `git diff --check` 통과. 각각 [rustfmt](rustfmt.log), [reference](reference-test.log), [boundary](fixture-boundary-test.log) log.
- 저장소 diff 검사가 로그의 줄 끝 공백에 걸리지 않도록 `ffi-feature-test.log`, `ios-simulator-build.log`, `workspace-test.log`의 줄 끝 공백과 파일 끝 빈 줄만 제거했다. 출력 본문은 유지했다.
- 기본 `spinon-ffi` static archive에서 C13.2 FFI symbol과 `c132-root` payload가 없고, `c13-2-scroll-fixture-test` opt-in archive에는 둘 다 있다. [archive/hash 요약](feature-isolation-followup.txt) · [기본 빌드](default-archive-build.log) · [기본 검사](default-archive-scan.log) · [opt-in 빌드](opt-in-archive-build.log) · [opt-in 검사](opt-in-archive-scan.log).

## 실행 중 수정한 fixture 결함

- V8 host는 여러 `eval`을 같은 global lexical scope에서 수행한다. root/body fixture 상단의 `const` 재선언으로 발생한 `SyntaxError`를 각 고정 장면의 함수 scope로 바꿔 해결했다.
- DOM node map은 fixture ID로 저장하면서 장면 cleanup은 `.html` alias를 읽고 있어 기존 root가 남았다. `html`/`body` alias를 명시해 다음 장면 전에 이전 root를 실제로 제거했다.
- layout/render 연결은 snapshot 전체 equality가 아니라 completed layout key와 render snapshot key가 일치하는지 검증하도록 했다. helper가 양쪽 snapshot의 generation·document·render-tree·style·environment revision을 함께 비교한다.
- [root/body 구현 실패 경로 검토 20개](../c13-2-scroll-root-body-followup-implementation-review-20-2026-10-15.md)와 [이번 PR 변경 적대 검토 30개](../c13-2-scroll-root-body-followup-pr-review-30-2026-10-15.md)를 별도 파일로 연결한다. 둘 다 기존 계획/PR review를 재사용하지 않는다.

재현 후 Android와 iOS Simulator에서 두 root/body scene, 기존 scroll matrix, scroll offset lifecycle까지 모두 `status=0`으로 끝났다. C13.2 전체 pass, app route stale/failure injection, ink-only overflow 및 simulator pixel readback은 남아 있다.
