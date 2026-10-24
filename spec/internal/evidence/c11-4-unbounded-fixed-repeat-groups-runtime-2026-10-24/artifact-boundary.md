# C11.4 fixed-repeat 그룹 수 제한 제거 · FFI archive 경계

## 검사 범위

호스트 macOS ARM64 debug-profile `spinon-ffi` static archive만 비교했다. 기본 feature 조합과 `c04-runtime-gpu-fixture-test` opt-in positive control을 각각 빌드했다. 문자열은 `/usr/bin/strings`, 심볼은 Rust 1.96.1에 포함된 `llvm-nm`으로 조사했다.

검사 marker는 `fixed-repeat-groups-four-prefix-fill`, fixture ABI는 `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`다.

## 결과

| 구성 | SHA-256 | archive bytes | 새 scene marker | fixture ABI |
| --- | --- | ---: | --- | --- |
| 기본 feature | `289edccb47fde2a323b155b59a1c8612481db53183e04a6a588278eddb496d05` | 513,086,216 | 없음 | 없음 |
| `c04-runtime-gpu-fixture-test` | `b0cadf37b74ba629efefa07f85ef31e2e9574c63ec8b1c5839c00de5be049ca1` | 696,139,056 | 있음 | 있음 |

## 한계

이 비교는 macOS ARM64 debug static archive의 fixture 경계만 입증한다. Android APK 또는 iOS app bundle의 release dead stripping·크기, Android 실기기/hardware GPU, iOS compositor 이후 pixel 결과는 검사하지 않았다.
