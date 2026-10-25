# C11.4 static archive boundary scan

Candidate source HEAD: `8f825a10ce51403a7838fe564c0253483be24b16`
Candidate base HEAD: `0c7c34b8f956a6b063a180ec68942fd0d1c8798f`
Candidate source diff SHA-256: `d3912cbe66106f5848ce99dd7aa25733284422be1c4fc5d06b1aa33cab295f79`
Changed candidate source paths: 25
Candidate source aggregate SHA-256: `82fccfc0f54eabfc6444fc33424b05cde2e315134c5611328e04bd4eaac92eeb`
Source scope: the candidate diff and per-file hashes cover all 25 changed paths under `crates/` and `platforms/`, including production Rust, test-only Rust, and Android/iOS fixture drivers. This is not a claim that every path is linked into both archives. Evidence/docs changes are excluded.
Build source snapshots matched before/after both archives: `True`
Command: `python3 spec/internal/evidence/c11-4-orthogonal-text-generalization-runtime-2026-10-25/artifact-boundary/build-and-scan.py`
Scanner: `llvm-nm, compatible with GNU nm`

| Archive | Bytes | SHA-256 | Scene IDs | DOM prefix | DOM suffix | DOM template parts | Expanded IDs | Fixture FFI symbol | llvm-nm |
|---|---:|---|---:|---|---|---|---:|---|---:|
| product | 75131768 | `0eb62fd8e73253e5f0cacb50777efea04a90aa6ed29616bdb8414885344d0d07` | 0/10 | False (0) | True (5) | False | 0/10 | False | 0 |
| fixture-test | 78484008 | `240aa82b4d00a003654476261cb40e392843a0da143612c0072f1d4675fac024` | 10/10 | True (5) | True (1182) | True | 0/10 | True | 0 |

Product negative controls: no C11.4 scene ID, unique DOM prefix/template parts, or fixture FFI symbol. Fixture-test positive controls: all ten scene IDs, the DOM template prefix/suffix parts, and the fixture FFI symbol are present.

The ten fully expanded DOM root IDs are not expected as literal archive strings: runtime code formats `c114-orthogonal-text-{scene-id}-grid`. The `-grid` suffix is generic and also occurs in the product archive, so it is reported but is not used alone as a negative-control assertion.

Matching `llvm-nm --demangle --defined-only` rows:

```text
(product): no C11.4 fixture or module symbols
0000000000001828 T spinon_ffi::runtime_gpu::fixtures::c11_4::orthogonal_text_intrinsic::runtime_source::h4db52aa35f7ea7ed
fixture FFI export: spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture
```

Full 25-path changed-source snapshot and all four before/after captures are in `scan-results.json`; each build log records its full before/after snapshots.

Scope: macOS ARM64 `spinon-ffi` static archives only. This scan does not inspect APK/IPA bundles or device state.
