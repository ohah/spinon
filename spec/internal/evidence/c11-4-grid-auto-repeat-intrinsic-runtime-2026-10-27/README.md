# C11.4 `grid-auto-repeat-intrinsic-001.html` 제한 runtime 실행

이 실행은 고정 WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 Grid 8개를 opt-in 앱 fixture로 확인한다. WPT harness의 Spinon 통과나 C11.4 전체 지원을 뜻하지 않는다. 사전 관찰과 독립 기대값은 [비교 근거](../c11-4-grid-auto-repeat-intrinsic-precomparison-2026-10-27/README.md)에 있다.

## 실행 결과

| 플랫폼 | 대상 | scene | 결과 | 상세 근거 |
|---|---|---:|---|---|
| Android | API 37 ARM64 AVD `emulator-5554` | 8/8 | track, frame, trace, draw/present 통과 | [result](android-avd-c114-grid-auto-repeat-intrinsic-wpt-android-avd-20261027T101029040Z-44c2a0b7/result.json) · [simulator/build hash](android-avd-c114-grid-auto-repeat-intrinsic-wpt-android-avd-20261027T101029040Z-44c2a0b7/simulator.json) |
| iOS | iPhone 17 Pro / iOS 26.2 Simulator | 8/8 | track, frame, trace, draw/present 통과 | [result](ios-simulator-c114-grid-auto-repeat-intrinsic-wpt-ios-simulator-20261027T101156827Z-602592aa/result.json) · [simulator/build hash](ios-simulator-c114-grid-auto-repeat-intrinsic-wpt-ios-simulator-20261027T101156827Z-602592aa/simulator.json) |

두 플랫폼의 최종 columns는 고정 neighbor가 `[20,20,20,40]`, auto neighbor max-content가 `[10,10,10,10,60]`, auto neighbor min-content가 `[10,60]`이다. 각 run은 root/item/probe frame과 모든 Grid track-sizing trace chunk를 보존한다. 0-width probe는 frame에 남고 painted box 수에서 제외된다.

`fill-auto-min`과 `fit-auto-min`은 100px item이 74px root border box를 28px 넘는다. 두 장면 각각 Android PixelCopy에서 border·overflow item·item 바깥 control 3/3 RGBA exact를 확인했다. iOS는 compositor 제출 전 Metal drawable에서 같은 위치와 RGBA를 3/3 exact로 확인했다. [Android overflow scene 화면](android-avd-c114-grid-auto-repeat-intrinsic-wpt-android-avd-20261027T101029040Z-44c2a0b7/auto-repeat-intrinsic-fill-auto-min.png) · [iOS overflow scene 화면](ios-simulator-c114-grid-auto-repeat-intrinsic-wpt-ios-simulator-20261027T101156827Z-602592aa/auto-repeat-intrinsic-fill-auto-min.png). 이는 기기 scanout이나 iOS compositor 이후 화면 증거는 아니다.

## Feature artifact 경계

[별도 빌드·심볼 비교](feature-boundary.md)는 같은 checkout에서 기본 `spinon-ffi --lib`와 `c04-runtime-gpu-fixture-test` opt-in archive를 순서대로 빌드한 기록이다. 새 fixture의 `grid_auto_repeat_intrinsic_wpt::runtime_source` 심볼은 기본 artifact에 없고 opt-in positive control에 있다. 이 검사는 macOS ARM64 Cargo `.rlib` 범위이며 APK/IPA 또는 배포 artifact 전체를 대상으로 하지 않는다.

## 회귀 검사

- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 14/14 통과. 8개 WPT ID, catalog 516 scene, route, 양 플랫폼 검사 연결을 확인한다.
- `cargo test -p spinon-layout --features c11-4-grid-track-sizing-fixture-test`: 300 통과.
- `cargo test -p spinon-style-to-layout`: 244 통과.
- `cargo test -p spinon-ffi --features c04-runtime-gpu-fixture-test`: 138 통과, 2 ignored.
- `cargo fmt --all -- --check`, `git diff --check`, capture script `node --check` 통과.
- `cargo clippy --locked --workspace --all-targets -- -D warnings` 통과. Opt-in `spinon-layout` (C11.4 fixture feature) 및 `spinon-style-to-layout` all-target Clippy도 `-D warnings`로 통과.
- Opt-in FFI all-target Clippy는 변경하지 않은 `spinon-runtime/.../c11_4_grid_layout_auto_tracks/validation.rs:329`의 기존 `type_complexity` 경고 때문에 기본 `-D warnings` 조합에서 중단됐다. `-A clippy::type_complexity` 한 항목만 제외해 다시 실행했을 때 나머지는 통과했다.

이번 slice는 이 고정 8-case profile에 한정한다. 일반 CSS admission, Spinon WPT harness 통과, 실제 Android 기기, 임의 `auto-fill`/`auto-fit` 입력, 전체 C11.4를 검증하지 않는다. 공식 C11.4 체크와 `72/213` 집계는 미완료 상태로 유지한다.

## 변경 검토

- [구현/runtime 실패 관점 검토 20개](implementation-runtime-review-20.md)는 layout·runtime 오류 경로와 최종 simulator 실행을 다룬다.
- [PR diff 실패 관점 검토 30개](pr-diff-review-30.md)는 별도 관점으로 변경 파일 간 계약·artifact·evidence·로드맵을 대조했다. 구현 PR [#691](https://github.com/ohah/spinon/pull/691)에서 Agent Browser가 Android 1080×2400 및 iOS 1206×2622 screenshot을 실제로 불러온 것을 확인했다.
- [병합 후 상태·로드맵 검토 30개](post-merge-status-roadmap-review-30.md)는 PR #691 병합 시각·SHA, CI 보고 범위와 미완료 집계를 다시 대조한다.
- 구현 PR [#691](https://github.com/ohah/spinon/pull/691)은 2026-10-27T10:30:12Z `Rebase and merge`로 병합됐다. 최종 main merge SHA는 `8993e3f5f28f13ab707018dda52f8f619d2b8bb4`다.
- 병합 시 `gh pr checks 691`은 “no checks reported”였고 `main` branch protection의 required status checks는 `null`, required approval count는 `0`이었다. 이를 CI 통과나 독립 승인으로 표시하지 않는다.
