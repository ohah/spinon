# C11.4 static archive boundary scan

Source revision (HEAD): `0c7c34b8f956a6b063a180ec68942fd0d1c8798f`
Implementation diff SHA-256: `26cff5b6335feca9476017b62c336c6081f1a1b5228435ab590e8d0e96656010`
Implementation files: 22
Implementation aggregate SHA-256: `b505d00b10e143279053e58102028fd200d74593f8873552c49f056f51fd6b42`
Source scope: the binary diff is limited to the 22 compiled implementation paths; evidence/docs changes are intentionally excluded. Every on-disk implementation file is hashed directly, including untracked files.
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

Full 22-path per-file source snapshot and all four before/after captures are in `scan-results.json`; each build log records its full before/after snapshots.

Scope: macOS ARM64 `spinon-ffi` static archives only. This scan does not inspect APK/IPA bundles or device state.
