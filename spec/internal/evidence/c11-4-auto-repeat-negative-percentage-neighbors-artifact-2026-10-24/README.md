# C11.4 signed percentage auto-repeat `calc()` artifact/build evidence

## 검증 결과

### 정적 FFI archive 경계

| Archive | 크기 | SHA-256 | 신규 ID | FFI fixture symbol |
| --- | ---: | --- | ---: | --- |
| product `c04-runtime-gpu` | 75,147,056 bytes | `5286b3979b3686a3640d06a45fbf046588401335ebd10350580efd7faf49df80` | 0/9 | 없음 |
| opt-in `c04-runtime-gpu-fixture-test` | 78,267,640 bytes | `549f185831adce554a50be9da53d24456b7dad9752c0cf4289abef25af89ed58` | 9/9 | 있음, 모듈 symbol 1개 |

`scan-ffi-archives.py`는 Rust LLVM 22.1.8 `llvm-nm`으로 symbol을 검사하고 같은 archive bytes 안에서 각 fixture ID를 검색한다. [원본 scan JSON](scan-results-llvm22.json)에는 archive hash, ID별 marker, symbol rows, 구현 diff hash 및 manifest hash가 있다. 구현 diff hash는 [source manifest](../c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/source-manifest.json)의 범위와 일치한다. 이 검사는 FFI static archive에 한정되며 최종 APK/IPA에 fixture 문자열이나 코드가 빠졌다는 근거가 아니다.

### focused checks

- Stylo computed AST, style-to-layout projection, layout final clamp/rejection, FFI exact scene catalog: 각 Rust test 1/1 통과.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 9/9 통과.
- Chrome 156 사전 oracle validator: 9/9 scene이 독립 CSS Grid 기대값과 1/64 CSS px 이내.
- `cargo fmt --all -- --check`: 통과.
- Focused Clippy: `spinon-style`, `spinon-style-to-layout`, `spinon-layout`, `spinon-ffi` 모두 `-D warnings` 통과.
- Android Gradle build와 iOS Simulator Xcode build: 성공. iOS log에는 기존 `libspinon_wgpu_r08_spike.a`/`libspinon_ffi.a` 중복 Metal symbol 및 dSYM 타임스탬프 경고가 있지만 build는 성공했다.

각 명령의 전체 출력은 `*-final.log`, `clippy.log`, Android/iOS build·matrix log에 보존했다. PR diff의 trailing whitespace만 없앴고 로그 event 내용은 바꾸지 않았다. Android/iOS 11-scene runtime matrix는 [runtime README](../c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/README.md)에 연결되어 있다.

## 재현 명령

```sh
cargo test --locked -p spinon-style track_sizing_profile_preserves_signed_percentage_terms_in_computed_ast
cargo test --locked -p spinon-style-to-layout track_sizing_projection_accepts_the_supported_auto_repeat_neighbor_shapes
cargo test --locked -p spinon-layout calc_neighbor_range_clamps_only_the_final_supported_value
cargo test --locked -p spinon-layout calc_neighbor_rejects_invalid_shapes_basis_identity_and_overflow
cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test negative_percentage_neighbor_catalog_is_exact_and_routed
node --test tools/css-reference/fixture-module-boundary.test.mjs
cargo fmt --all -- --check
```

Archive scan command:

```sh
SPINON_LLVM_NM=/opt/homebrew/opt/llvm@22/bin/llvm-nm \
  python3 spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-artifact-2026-10-24/scan-ffi-archives.py \
  /tmp/<product-archive> /tmp/<fixture-test-archive>
```

The script checks that product markers and the fixture dispatcher symbol are absent, while the opt-in archive contains all 9 IDs and the expected symbols. Temporary archive copies are removed after evidence collection; Cargo's normal build cache remains available.
