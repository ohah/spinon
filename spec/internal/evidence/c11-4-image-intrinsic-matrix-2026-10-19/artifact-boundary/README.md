# Fixture artifact feature boundary

Rust 1.96.1 release `spinon-ffi` staticlib 두 개를 같은 worktree와 dependency lock에서 빌드했다. 기본 variant는 `--no-default-features`, positive control은 `c11-grid-metal-surface-pixels-test` feature를 켰다. 산출물 복사본은 ignored `build/c11-4-artifact-boundary/`에 두며 이 문서와 로그만 PR evidence에 포함한다.

| Variant | Size | SHA-256 | C11.4 FFI symbol / scene string / image URL | Exact 97-byte PNG payload |
| --- | ---: | --- | --- | --- |
| Default | 59,914,184 bytes | `f6780e771d059dbdaa5d3cba01befcce1c21d7440637c8acca5655da9a69d4b8` | absent | absent |
| Fixture positive control | 76,303,832 bytes | `80be8997446cb63f349cd5171308a5bf47b55fe804d17cf599dd839520f03dfc` | present | present |

검사 도구는 Rust sysroot의 `llvm-nm`, macOS `/usr/bin/strings`, 고정 PNG bytes 비교다. default archive에 `spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture`, `auto-image-definite`, `c114-auto-image-grid`, `https://spinon.invalid/c14-1-png-basic.png`가 없는지 확인했다. positive control에서 동일 심볼과 scene 문자열·URL을 확인하고 저장소 PNG 전체 97 bytes가 staticlib 내부에 포함되는지 byte 비교했다.

재현: [검증 스크립트](./verify-artifact-boundary.sh), [default build log](./default-build.log), [fixture build log](./fixture-build.log), [symbol](./fixture-symbol.txt), [fixture strings](./fixture-strings.txt), [payload 결과](./png-payload-check.txt), [archive size·hash](./archive-hashes.txt). 이는 macOS ARM64 release staticlib의 경계 확인이다. Android/iOS release 앱 bundle 크기나 다른 feature 조합의 artifact를 증명하지 않는다.
