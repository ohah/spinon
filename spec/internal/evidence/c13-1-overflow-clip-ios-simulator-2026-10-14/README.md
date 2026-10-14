# C13.1 iOS Simulator 검증 · 2026-10-14

## 구현 전 판정 기준

Rust의 고정 C13.1 fixture host는 있었지만 iOS 빌드 flag와 앱 경로는 연결되지 않았다. 변경 전 다음 기준을 기록했다.

1. `SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST`는 기본값 0이고, C04 runtime GPU fixture와 함께 켜야 한다. 켜면 전용 Cargo feature와 ObjC++ macro만 추가한다.
2. `--spinon-c131-overflow-clip`은 고정 C13.1 app 경로를 선택한다. 다른 CSS fixture 인자와 함께 오면 host 생성 전에 거부한다.
3. iOS runtime은 `status=0`, `layout=ready`, 13개 frame을 반환한다. 고정 Chrome 155 geometry 차이는 각 축 0.5 CSS px 이하이고, 15개 hit-test 표본과 네 viewport 밖 null 표본이 모두 일치해야 한다.
4. iOS Simulator의 Metal surface를 앱 화면에서 확인하고 캡처한다. drawable readback이나 최종 compositor/scanout 판정은 별도다.
5. Cargo 기본 feature-off archive에는 C13.1 FFI symbol과 fixture source payload가 없고, C13.1 opt-in archive에는 둘 다 있어야 한다.
6. build flag 경계, runtime, workspace test/Clippy, rustfmt, diff를 확인한다. C13/C13.1 구현 완료나 제품 CSS 지원은 선언하지 않는다.

## 환경과 빌드

- 대상: iPhone 17 Pro / iOS 26.2 Simulator, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- 실기기는 연결하거나 사용하지 않았다. 이미 부팅된 Simulator를 재사용했고 erase/shutdown하지 않았다.
- 격리 worktree: `/tmp/spinon-c13-1-overflow-clip`.
- V8: 공유 checkout의 고정 revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059`, 읽기 전용으로 사용.
- [기본 feature-off build](default-feature-off-build.log)는 feature 인자 없이 `cargo build`를 실행해 `aarch64-apple-ios-sim` archive를 만들었다.
- [C04-only build](feature-off-build.log)는 `SPINON_ENABLE_C04_RUNTIME_GPU=1`, C13 flag 0으로 앱/FFI를 빌드했다. 별도 flag를 2로 주거나 C04 없이 C13을 켜면 build script가 exit 2로 거부했다([simulator negative logs](build-invalid-value-negative.log), [prerequisite log](build-prerequisite-negative.log), [native prerequisite log](native-prerequisite-negative.log)).
- [C13 opt-in build](feature-on-build-2.log)은 C04=1, C13=1로 성공했다. Cargo feature 목록에 `c13-1-overflow-clip-fixture-test`가 들어가며 Xcode ObjC++ compile command는 `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST=1`을 받았다.
- [archive isolation](feature-isolation.txt): Cargo 기본 archive는 C13.1 symbol 2개와 `c131-root`/fixture payload 문자열이 모두 없었다. C13 opt-in archive는 두 C ABI symbol과 payload를 포함했다. symbol 확인은 rustc 1.96.1과 같은 LLVM 22.1.2의 `llvm-nm`을 사용했다.
- Xcode verbose build log 복사본에서는 줄 끝 공백과 마지막 빈 줄만 정규화했다. 로그에 `<escaped-space-trimmed>`가 있으면 Xcode가 출력한 escaped trailing space를 표시한다.

## Simulator 결과

`simctl`로 opt-in 앱을 설치해 `--spinon-c131-overflow-clip`을 실행했다. [원본 unified log](ios-simulator.log), [실행기록](simulator-launch.txt), [화면 캡처](ios-simulator.png), [Chrome와 독립 대조 결과](frame-parity.txt)를 보존했다. 캡처는 C13 fixture draw 로그가 나온 직후 Simulator 화면에서 얻었다. 이미지 SHA-256은 `95021b36b3a6e76dd59c2c65b67db3cda7ef318d5c5bd9e8b1ff042b95bdd23d`다.

- runtime: `status=0`, `layout=ready`, 13 boxes.
- Chrome 155.0.8059.40와 frame geometry: 13/13, 최대 절대 오차 `0.000 CSS px`.
- 고정 hit-test: sample ID·좌표·element ID를 독립 비교해 15/15 일치. viewport 밖 네 점은 `null`.
- presentation: `backend=Metal`, `Apple iOS simulator GPU`, `Bgra8UnormSrgb`, surface `960×720`; `SPINON_C0410_DRAW`가 13 boxes와 viewport `320×240 CSS px` 제출을 기록했다.
- 이 screenshot은 UIKit 제목/상태 label과 Metal surface가 함께 보이는 Simulator 화면 캡처다. Metal drawable의 GPU readback, 정확한 RGBA 표본, compositor 완료 또는 광학적 scanout을 증명하지 않는다.
- default feature-off archive와 ObjC++ macro guard는 C13 FFI 경로를 포함하지 않는다. C13과 C12.3 인자를 함께 준 route collision은 `SPINON_C131_ARGUMENT_ERROR`로 거부되고 host를 만들지 않았다([negative log](route-conflict-negative.log)).

## 검증과 남은 범위

Rust fixture host가 embedded Chrome oracle의 15개 hit-test 결과를 계산하고, iOS Swift route는 반환된 sample ID·좌표·element ID·matched status와 13 frame을 다시 검사한다. Python 검증기([verify-ios-simulator.py](verify-ios-simulator.py))는 그 unified log를 저장된 agent-browser Chrome observation JSON과 독립 비교한다.

iOS Simulator integration gate는 통과했다. `cargo test --workspace --locked`는 24개 test-result suite에서 730 passed, 0 failed, 4 ignored였고, focused C13.1 테스트 2개도 통과했다. `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, 두 iOS build script의 `bash -n`, Xcode project `plutil -lint`, `git diff --check`도 통과했다. 각 명령의 결과는 [`workspace-test.log`](workspace-test.log), [`focused-cargo-test.log`](focused-cargo-test.log), [`workspace-clippy.log`](workspace-clippy.log), [`workspace-fmt.log`](workspace-fmt.log)에 남겼다. [독립 대조 출력](frame-parity.txt)은 13/13 frame과 15/15 hit sample 비교를 다시 통과했다.

C13.1 전체 gate는 아직 미완료다. 전체 사전 비교 matrix, broader paint/hit negative controls, WPT 후보 분류, scroll/CSSOM/event 기능, radius·transform geometry, float geometry, 실기기와 최종 display 결과는 이 실행의 범위에 포함하지 않는다. 관련 [C13.1 구현 실패 관점 20개](../c13-1-overflow-clip-ios-simulator-implementation-review-2026-10-14.md)와 [PR delta 30개](../c13-1-overflow-clip-ios-simulator-pr-review-2026-10-14.md)는 별도 검토로 보존한다.
