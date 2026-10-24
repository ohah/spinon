# C11.4 정적 아카이브 fixture 경계

이 근거는 `spinon-ffi` Rust 정적 아카이브의 feature 경계만 확인한다. 최종 Android APK나 iOS 앱 번들에서 fixture가 제외되는지는 증명하지 않는다.

소스 스냅샷은 `feat/c11-4-cross-axis-fixed-auto-repeat-areas-20261025` 브랜치의 `96efeebd9e816110ac981b6c24eecd3e88b620a8` 커밋, tree `0692abb4f20d50f7f96f3be1f9152fa50172cf8e`다. 도구는 Rust/Cargo 1.96.1, LLVM 22.1.2, `aarch64-apple-darwin`이다. C11.4 FFI와 corpus 입력 다섯 파일의 빌드 전후 hash는 `source-hashes-before.txt`, `source-hashes-after.txt`에 있으며 서로 일치한다.

## 재현 명령

빌드 대상 디렉터리는 `/tmp/spinon-c114-staticlib-boundary-20261025T0319KST`로 분리했다.

```sh
c114_evidence_dir=spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-2026-10-25/staticlib-archive-boundary-20261025T0319KST
CARGO_TARGET_DIR=/tmp/spinon-c114-staticlib-boundary-20261025T0319KST cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu
cp /tmp/spinon-c114-staticlib-boundary-20261025T0319KST/debug/libspinon_ffi.a "$c114_evidence_dir/spinon-ffi-c04-runtime-gpu.a"
CARGO_TARGET_DIR=/tmp/spinon-c114-staticlib-boundary-20261025T0319KST cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
cp /tmp/spinon-c114-staticlib-boundary-20261025T0319KST/debug/libspinon_ffi.a "$c114_evidence_dir/spinon-ffi-c04-runtime-gpu-fixture-test.a"
```

각 아카이브의 정의된 외부 심볼은 Rust 1.96.1 toolchain의 LLVM 유틸리티로 덤프한다.

```sh
/Users/yoonhb/.rustup/toolchains/1.96.1-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-nm --defined-only --extern-only "$c114_evidence_dir/spinon-ffi-c04-runtime-gpu.a" > "$c114_evidence_dir/fixture-free-defined-symbols.txt"
/Users/yoonhb/.rustup/toolchains/1.96.1-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-nm --defined-only --extern-only "$c114_evidence_dir/spinon-ffi-c04-runtime-gpu-fixture-test.a" > "$c114_evidence_dir/fixture-test-defined-symbols.txt"
python3 "$c114_evidence_dir/verify-boundary.py"
```

검증기는 새 교차축 장면 ID 11개 전체, 대표 CSS track-list payload, C11.4 host 생성·평가 심볼의 archive bytes와 `llvm-nm` 정의 여부를 확인한다. 결과는 `fixture-free-boundary-result.json`, `fixture-test-boundary-result.json`에 저장한다.

원본 `.a` 두 파일은 각각 660 MB와 696 MB로 GitHub 파일 크기 한도를 넘으므로 이 저장소 변경에는 추가하지 않는다. 원본은 현재 작업 디렉터리에만 보존하고, build log·크기·SHA-256·심볼 목록·scan 결과는 근거로 남긴다. 새 checkout에서 검증기를 재실행하려면 위 두 build 뒤 `.a`를 evidence 디렉터리에 복사한다.

## 결과

| Feature | 크기 | SHA-256 | scene ID | 대표 payload | C11.4 host symbol |
|---|---:|---|---|---|---|
| `c04-runtime-gpu` | 660361184 bytes | `ea34c2319ffcafe850a20f5d10b4ea201fe789a9fc9dc058d83410c717ec3a98` | 0/11 포함 | 없음 | 둘 다 없음 |
| `c04-runtime-gpu-fixture-test` positive control | 696066136 bytes | `7b73bdcfebe9843143fbef80394d0907750b5ace1cbd3d6d3a3cced3ce30ce57` | 11/11 포함 | 포함 | 둘 다 포함 |

두 빌드는 성공했다. 전체 build log, 크기, SHA-256, 필터된 C11.4 host symbol 일치 결과와 검사 JSON을 PR에 포함한다. 26–27 MiB 전체 `llvm-nm` 정의 심볼 덤프와 630–664 MiB 원본 `.a`는 로컬 evidence 디렉터리에 보존하고 PR 변경에는 넣지 않았다. 재현 명령은 전체 심볼 덤프를 다시 생성한다. 정적 아카이브 검사 결과를 최종 APK/IPA 경계로 확대 해석하지 않는다.
