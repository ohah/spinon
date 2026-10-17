# C15.3 overflow text-owner hit-test probe

## 고정 기준과 범위

- Chrome 155 C15.3 Unicode shaping oracle의 `visible-overflow-hit`/`hidden-overflow-clip`을 사용한다. `Ahem 10px XX`는 20 CSS px이고 overflow parent는 content width 15 CSS px다. Chrome은 visible 점에서 `visible-text`, 같은 상대 점의 hidden 점에서 id가 없는 underlying element를 돌려준다. [고정 Chrome DOM geometry·hit 값](../c15-3-unicode-shaping-precomparison-2026-10-17/README.md)
- 이 probe는 기존 opt-in C15.3 앱 scene에서 PixelCopy/Metal 표본과 동일한 CSS 위치의 `RuntimeRenderSnapshot::hit_test_css_point()` 반환 NodeId를 검사한다. visible parent 경계 밖은 40px text-owner box 안이고, hidden 점은 조상 clip 바깥이다. hidden 밖에서 underlying anonymous box가 선택될 수 있으므로 `None` 대신 hidden text-owner가 선택되지 않는 것을 기대값으로 둔다.
- 고정 fixture의 DOM 전체, glyph 별 hit region, text range와 event dispatch를 구현하거나 검증하지 않는다. 계약은 [0073](../../0073-c15-3-overflow-text-owner-hit-test.md)에 있다.

## 구현 및 실행

`crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3.rs`는 visible sample에서 visible owner, hidden 내부 sample에서 hidden owner가 선택되는지 확인한다. hidden 밖 sample은 hidden owner를 배제해야 한다. owner에 visible clip이 잘못 걸린 negative control은 실패해야 하고, 나중에 그려지는 overlay의 hit-test order가 text owner보다 높으면 overlay NodeId가 선택되어야 한다. assert는 C15 fixture-test feature에서만 사용하며 renderer 공개 API나 DOM 이벤트 route는 추가하지 않는다.

- Rust: `cargo test --locked -p spinon-ffi --features c15-3-wgpu-glyph-paint-fixture-test` — 47 passed, 0 failed, 2 ignored. 이 실행에 포함된 C15.3 fixture 테스트 3개도 전부 통과했다. [전체 테스트 근거](./rust-tests.txt)
- Feature 경계: 기본 `spinon-ffi` static archive 빌드와 printable-string scan은 fixture report/FFI symbol name이 없음을 확인했다. source `cfg` gate와 opt-in feature test가 positive evidence다. macOS `nm` LLVM attribute incompatibility로 symbol table scan은 미완료이며 [상세 결과와 한계](./default-artifact-scan.txt)에 기록했다. [기본 build log](./default-build.txt)
- Rust lint/format: `cargo clippy --locked -p spinon-ffi --all-targets --features c15-3-wgpu-glyph-paint-fixture-test -- -D warnings`와 `cargo fmt --all -- --check` 통과. [Clippy](./clippy.txt) · [format](./format.txt)
- Android: API 37 ARM64 AVD `emulator-5554`에만 APK를 설치하고 `spinon_c153_glyph_paint` fixture route를 실행했다. layout report는 CSS sample `(29,81)`, `(13,121)`, `(29,121)`에서 기대 owner 관계를 기록했다. SurfaceView PixelCopy는 visible 바깥 잉크 `(77,213)=[9,33,112,255]`, hidden 안 잉크 `(35,318)=[9,33,112,255]`, hidden 바깥 배경 `(77,318)=[247,249,252,255]`로 `3/3` 일치했다. [Android build](./android-build.txt) · [실행 log](./android-run.txt) · [PixelCopy PNG](./android-pixelcopy.png) · [화면 screenshot](./android-screen.png)
- iOS: iPhone 17 Pro / iOS 26.2 Simulator에서 `--spinon-c153-glyph-paint`를 재빌드 후 다시 실행했다. 최종 build는 `BUILD SUCCEEDED`, PID 81811의 전용 `SPINON_C153_HITTEST` log는 `visible_owner;hidden_inside_owner;hidden_outside_excludes_hidden_owner` 전체를 기록하며 fixture 기대값도 확인한다. Metal drawable before compositor는 visible 바깥 잉크 `(88,244)=[9,33,112,255]`, hidden 안 잉크 `(40,364)=[9,33,112,255]`, hidden 바깥 배경 `(88,364)=[247,249,252,255]`로 `3/3` 일치했다. 첫 dSYM 생성 실패는 용량 오류 artifact로 보존했고 최종 성공 로그와 구분한다. [최종 iOS build](./ios-build.txt) · [이전 실패 로그](./ios-build-dsym-disk-failure.txt) · [실행 log](./ios-run.txt) · [화면 screenshot](./ios-screen.png)
- PR 화면: PR #258의 처음 iOS screenshot URL은 branch 경로 오류로 404였다. direct raw URL로 수정한 뒤 Agent Browser에서 Android screen 1080×2400, PixelCopy 945×630, iOS screen 1206×2622를 모두 확인했다. [검증 자료](./pr-images-agent-browser.json)

Android PixelCopy와 iOS Metal drawable 표본은 paint visibility를 입증하는 증거다. hit-test 결과는 같은 app evaluation의 별도 Rust report에서 왔으며 화면 이미지에서 이벤트 동작을 추론하지 않는다. Simulator 결과는 실기기, Android hardware GPU 또는 iOS compositor-after 표면을 증명하지 않는다.

## 검토와 남은 경계

- [구현 실패 관점 20개](./implementation-review-20.md)
- [PR diff 실패 관점 30개](./pr-delta-review-30.md)
- text hit result는 glyph mask의 alpha로 계산되지 않고 owner box geometry와 hit-test clip을 따른다. DOM `elementFromPoint()`와 CSSOM 전체 적합성, glyph/source-range target, DOM event 전달, scroll offset, 일반 text admission/style projection, Block/Flex/Grid sizing, font fallback은 미검증이다. C15.3·C15·C16·C19 상태는 미완료로 유지한다.

저장된 build, simulator, test와 review artifact의 SHA-256 목록: [sha256.txt](./sha256.txt).
