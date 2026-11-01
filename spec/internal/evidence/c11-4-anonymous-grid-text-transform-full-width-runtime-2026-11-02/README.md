# C11.4 `text-transform: full-width` 제한 fixture runtime

## 판정 범위

구현 source commit은 `2b6fec2572801a7e59db1db5c919587c65e792a9`다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 4-scene matrix를 실행해 각각 4/4 통과했다. scene별 DOM source, computed style, typed text style, transformed paragraph, intrinsic size, Grid track, row size와 trace 조각을 확인한다. 기대값은 [고정 runtime matrix](../c11-4-anonymous-grid-text-transform-full-width-precomparison-2026-11-02/runtime-matrix-expected.json), 독립 규범 mapping과 Chrome candidate 관측은 [사전 비교](../c11-4-anonymous-grid-text-transform-full-width-precomparison-2026-11-02/README.md)에 있다.

허용 scene은 `none` control, `full-width` max-content/min-content, 반복 공백 collapse control이다. 모두 Noto Sans KR 10px/400, direct `lang="ja"`, `white-space:normal`, computed `word-break:normal`, computed `line-break:auto`이며 direct anonymous Text source는 `AB 12` 또는 `AB   12`다. 기대 layout text는 full-width 세 scene에서 `ＡＢ　１２`; max/min contribution은 50px/20px, line count는 1/2다. 일반 CSS admission이나 arbitrary `full-width` 지원이 아니다.

## Android API 37 ARM64 AVD

최종 run `android-avd-pass03`은 QEMU `sdk_gphone64_arm64`, API 37, serial `emulator-5556`에서 했다. APK와 설치본 SHA-256은 모두 `7110ecbd9074d17c0205fca34fa08b77a518b050e79237971f1a75c870159bc9`다. `sourceHead`는 위 source commit과 일치한다.

| Scene | 결과 | Column | Row | Lines | Trace parts |
|---|---:|---:|---:|---:|---:|
| `text-transform-full-width-none-max-content` | PASS | 25.989998px | 10px | 1 | 8 |
| `text-transform-full-width-max-content` | PASS | 50px | 10px | 1 | 7 |
| `text-transform-full-width-min-content` | PASS | 20px | 20px | 2 | 7 |
| `text-transform-full-width-collapsed-spaces-max-content` | PASS | 50px | 10px | 1 | 7 |

전체 per-scene log와 row data는 [matrix](./android-avd-pass03/matrix.tsv), [simulator metadata](./android-avd-pass03/simulator.json), [개별 log directory](./android-avd-pass03/)에 있다. [대표 화면](./android-avd-pass03/representative-screen.png)은 runtime harness의 `layout=ready` report를 보여 준다.

## iPhone 17 Pro / iOS 26.2 Simulator

최종 run `ios-simulator-pass02`는 UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`에서 했다. 앱 executable과 설치본 SHA-256은 모두 `157122cb3569c3d13b97f7cded5c8f30ff8a1e37ed437c9caa0969a1650f4e8a`다. `sourceHead`는 같은 `2b6fec2572801a7e59db1db5c919587c65e792a9`다.

| Scene | 결과 | Column | Row | Lines | Trace parts |
|---|---:|---:|---:|---:|---:|
| `text-transform-full-width-none-max-content` | PASS | 25.989998px | 10px | 1 | 15 |
| `text-transform-full-width-max-content` | PASS | 50px | 10px | 1 | 14 |
| `text-transform-full-width-min-content` | PASS | 20px | 20px | 2 | 14 |
| `text-transform-full-width-collapsed-spaces-max-content` | PASS | 50px | 10px | 1 | 14 |

전체 per-scene log와 row data는 [matrix](./ios-simulator-pass02/matrix.tsv), [simulator metadata](./ios-simulator-pass02/simulator.json), [개별 log directory](./ios-simulator-pass02/)에 있다. [대표 화면](./ios-simulator-pass02/representative-screen.png)은 runtime harness의 `layout=ready` report를 보여 준다.

## 빌드·회귀·fixture artifact 경계

- `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-android-app.sh` — `BUILD SUCCESSFUL`.
- `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-ios-sim.sh` — `BUILD SUCCEEDED`.
- `cargo test --locked -p spinon-text-layout` — 97/97; fixture feature 사용 시 — 104/104.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test` — 395 passed, 3 ignored; integration 3/3 및 16/16.
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test` — 188 passed, 2 ignored.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs` — 32/32; `node --test tools/css-reference/c11-4-computed-text-transform-unicode.test.mjs` — 1/1.
- `cargo fmt --all -- --check`와 feature-scoped product Clippy가 통과했다. `--all-targets` Clippy는 수정 범위 밖 `anonymous_text_hyphens.rs:117`의 기존 `too_many_arguments` 경고 때문에 실패했다.
- macOS ARM64 `target/debug/libspinon_ffi.a`의 `c04-runtime-gpu` build hash는 `a563441b4f81c20934d9b3683396c57605c338350956bf73bba049ad26887885`이며 `text-transform-full-width` marker가 없다. `c04-runtime-gpu-fixture-test` positive control hash는 `91302f3d6636ddc089c4eebe7080e2e7032a4b73db588669a15f96fca084dcc3`이며 네 scene ID가 모두 들어 있다. 이는 macOS static archive 경계이며 APK/IPA 전체 bundle의 exclusion을 뜻하지 않는다.

최종 capture 후 대상 Android 앱 PID와 대상 iOS simulator 앱 process가 없음을 확인했다. Agent Browser는 이 두 app capture에 사용하지 않았고 이 run에서 새 browser process도 만들지 않았다. 별도 simulator UDID에 있던 기존 Spinon process는 이 검증과 무관해 건드리지 않았다.

| 대표 이미지 | SHA-256 |
|---|---|
| Android `android-avd-pass03/representative-screen.png` | `79bdaec6b163c47bdc55092ff20249037c8e347ab19e721d8cccafddb6f5274b` |
| iOS `ios-simulator-pass02/representative-screen.png` | `99994e38f0af491a82c37caa96c5d0b5c8b074db85a0a3f152a8b25d5937f8e4` |

## 실패·한계

`android-avd-pass01`은 첫 구현 실행 중 native host가 새 scene ID를 모르는 dispatch error를 냈다. 양쪽 native host mapping을 추가한 뒤 재실행했고, 초기 pass02/pass01 captures는 source commit이 아니라 이전 `HEAD`를 기록하므로 최종 근거로 사용하지 않는다. 각 폴더와 log는 이력용으로 보존했다. Commit-bound pass03/pass02에서 같은 네 장면이 최종 통과했다.

대표 화면은 renderer surface 안에 glyph를 칠하지 않고 `layout=ready` harness 상태를 보여 준다. 이 자료는 layout/CSSOM/runtime path 검증이며 glyph raster, pixel equality, iOS compositor 이후 pixel, Android hardware GPU, 실기기 검증이 아니다. Chrome 156은 target `text-transform:full-width`를 computed `none`으로 돌려주므로 Chrome 측 관측은 unsupported-value control과 literal output candidate에 한정한다. CSS Text Level 3 CRD at-risk 상태와 일반 CSS/WPT interoperability는 미검증이다. C11.4와 C11 완료를 뜻하지 않는다.
