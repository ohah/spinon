# C11.4 fixed-repeat `minmax(px, fr)` fixture artifact boundary

## 최신 iOS source snapshot · archive 경계

로드맵 HTML 수정이 포함된 tracked diff에서 iPhone 17 Pro / iOS 26.2 Simulator의
여섯 C11.4 장면을 새 run으로 캡처했다. `build:ios-sim`은 성공했고, 여섯 장면 모두
layout status 0, frame/track 관측, draw/present 완료였으며 superseded retry는 장면별
0회였다. 캡처와 archive build 전후의 입력 manifest는 모두 26/26 SHA-256 일치했고,
tracked diff SHA-256 `9a79a11be6cc3cd88f6fcb5d49db10eebf66c4fe4407563edbb88cba5aa0a391`
도 해당 캡처·빌드의 manifest 검사 시점에 앱 evidence와 일치했다. archive run 뒤 이
README와 `latest-run.json`을 갱신했으므로 현재 diff에는 이 evidence index 갱신이 더해져
있다. Simulator app SHA-256은
`15dc61e151fb8544ad309f04a3dbd37410298ac7113388bfd073cf13d469075e`다. 장면별 결과와
입력 hash는 [iOS run verification](../ios-simulator-c114-fixed-repeat-minmax-flex-ios-simulator-20261024T235205801Z-de685760/run-verification.json)
및 [simulator.json](../ios-simulator-c114-fixed-repeat-minmax-flex-ios-simulator-20261024T235205801Z-de685760/simulator.json)에 있다.

동일 앱 evidence snapshot에 대해 locked release Rust static archive를 각각 빌드했다.
byte-safe `/usr/bin/strings -a` scan과 Rust 1.96.1 toolchain의 `llvm-nm`으로 새 scene
marker 6개, 두 host ABI 문자열·정의 심볼, 고유 CSS payload를 검사했다.

| Archive | Feature | 크기 | SHA-256 | scene marker | host ABI 문자열/심볼 | 고유 CSS payload |
| --- | --- | ---: | --- | ---: | ---: | ---: |
| product | `c04-runtime-gpu` | 75,186,128 bytes | `9384a0d07444ecb98a6c0e69d76a5358292d7d4a95d52a743e6fc68cb4bf6d95` | 0/6 | 0/2 · 0/2 | 없음 |
| positive control | `c04-runtime-gpu-fixture-test` | 78,357,536 bytes | `c48e6742f311befa4349e51a709cfa757683c390f29ce9216b9cc38df0f8d853` | 6/6 | 2/2 · 2/2 | 포함 |

positive-control payload marker는 `repeat(1, minmax(12px, 0.5fr)) repeat(auto-fill,
minmax(24px, 0fr))`다. [최종 archive run JSON](final-source/runs/final-artifact-boundary-20261024T235409Z-94df3151/final-artifact-boundary.json),
[빌드 전후 26개 입력 hash](final-source/runs/final-artifact-boundary-20261024T235409Z-94df3151/app-source-manifest-before.json),
[원본 logs와 toolchain](final-source/runs/final-artifact-boundary-20261024T235409Z-94df3151/)에 세부 결과가 있다. 이 검사는 Rust static archive 경계만 보여주며 Android APK나 iOS IPA의 최종 exclusion을 증명하지 않는다.

이 run은 verifier 소스를 바꾸지 않고 실행 시점에 최신 simulator.json 경로만 메모리에서 선택했다. 정확한 호출은 [실행 기록](final-source/runs/final-artifact-boundary-20261024T235409Z-94df3151/verify-command.txt)에 남겼다. 이전 final-source 및 pre-provenance run도 보존했다.

이 최신 결과가 아래 pre-provenance scan보다 우선한다. 아래 이전 run과 로그는
이력으로 보존했다.

## 이전 pre-provenance run · 이력

## 판정 범위

Rust `spinon-ffi` release static archive를 `c04-runtime-gpu` product feature와
`c04-runtime-gpu-fixture-test` positive control로 각각 빌드했다. archive bytes에서
새 scene ID 6개와 C11.4 fixture host ABI 문자열을 검사하고, Rust toolchain의
`llvm-nm --demangle --defined-only`로 정의 심볼을 확인했다. 두 feature 조합은
같은 전용 Cargo target directory를 사용해 순서대로 별도 빌드했다. 각 archive는
다음 빌드가 덮어쓰기 전에 scan·hash했다.

| Archive | Feature | 크기 | SHA-256 | 새 scene marker | C11.4 host ABI 심볼 | 새 runtime source 심볼 |
| --- | --- | ---: | --- | ---: | ---: | --- |
| product | `c04-runtime-gpu` | 75,185,984 bytes | `ae1c608e89d13e2b9725d0c7a147e655053fcb7d8130a1c421854a9afc5e37fe` | 0/6 | 0/2 | 없음 |
| positive control | `c04-runtime-gpu-fixture-test` | 78,357,320 bytes | `10f790096faf0bb07ff2f780cc71d8ae2cdcedcb1401e5be128961746f5708f1` | 6/6 | 2/2 | 있음 |

검사한 scene payload marker는 `fixed-repeat-minmax-flex-columns-fill`,
`fixed-repeat-minmax-flex-columns-fit`, `fixed-repeat-minmax-flex-rows-fill`,
`fixed-repeat-minmax-flex-rows-fit`, `fixed-repeat-minmax-flex-columns-groups`,
`fixed-repeat-minmax-flex-columns-flex-factor-low`다. host 심볼은
`spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`와
`spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture`다. positive control에는
`fixed_repeat_minmax_flex::runtime_source` 정의 심볼도 포함됐다.

## 재현 및 원본

저장소 루트에서 다음 명령을 실행한다.

```sh
python3 spec/internal/evidence/c11-4-fixed-repeat-minmax-flex-runtime-2026-10-25/artifact-boundary/verify-artifact-boundary.py
```

검증기가 실행한 두 build 명령은 다음과 같다.

```sh
mise exec -- cargo build --locked --release --no-default-features --target-dir build/spinon/c11-4-fixed-repeat-minmax-flex-artifact-boundary/target -p spinon-ffi --lib --features c04-runtime-gpu
mise exec -- cargo build --locked --release --no-default-features --target-dir build/spinon/c11-4-fixed-repeat-minmax-flex-artifact-boundary/target -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
```

재현 스크립트는 product와 positive control을 각각 `cargo build --locked --release
--no-default-features`로 빌드한다. 명령·전체 build output, LLVM symbol scan 요약,
일치 심볼, archive hashes와 source input SHA-256은 [실행별 JSON](runs/artifact-boundary-20261024T232747Z-b0fcf186/artifact-boundary.json),
[run 로그](runs/artifact-boundary-20261024T232747Z-b0fcf186/) 및
[host/toolchain 정보](runs/artifact-boundary-20261024T232747Z-b0fcf186/host-toolchain.txt)에
보존했다. 빌드 입력은 branch `feat/c11-4-fixed-repeat-minmax-flex-20261025`, HEAD
`d8a9fd96d7de6d7234aca705bde1bff91a3133df`의 uncommitted working tree이며, 검사 입력의
개별 SHA-256은 JSON에 고정했다.

이 결과는 macOS ARM64에서 빌드한 Rust FFI static archive 두 개의 feature 경계만
보여준다. Android APK나 iOS IPA의 최종 fixture exclusion, 앱 크기, 실기기 또는
hardware GPU 결과를 증명하지 않는다.
