# PR #611 post-merge main focused verification

Tested source commit: `2c4c278f0facd31ac3e0cc60dce1f27575628b35` (PR #611 merge commit). The rerun happened before the documentation-only follow-up changed this checkout; the product source tree is the merge commit tree.

| Command target | Result |
| --- | --- |
| `spinon-runtime` `exposes_four_direct_nonspanning_text_image_intrinsic_scenes` with `c11-4-grid-track-sizing-fixture-test` | 1 passed, 0 failed |
| `spinon-ffi` `exposes_only_four_direct_nonspanning_mixed_intrinsic_scenes` with `c04-runtime-gpu-fixture-test` | 1 passed, 0 failed |

Exact commands and complete stdout/stderr are in [runtime-focused.log](runtime-focused.log) and [ffi-focused.log](ffi-focused.log). Their exit-code files are both `0`; [manifest](manifest.json) records the tested commit and command arguments.
