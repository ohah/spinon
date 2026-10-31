# `spinon-ffi` C11.4 text-indent static archive boundary

Builds are release `spinon-ffi` static libraries on the current macOS host. The no-default-features archive SHA-256 is `aeb1e605352ab798ab4d156d2020ae40b6b12ca191e506ee9dc0c00a0b72e305`; searching its extracted strings found 0 matching `text-indent-*` scene IDs, 0 `computed_text_indent` adapter symbols and 0 `validate_computed_text_indent` symbols.

The positive-control archive enables only `c04-runtime-gpu-fixture-test` and has SHA-256 `c7e7a2f81e8c58a7f5e06b360f4717371666e633945ad17dc5443127ad3dcf57`. It contains all 18 fixed scene IDs and the expected FFI adapter and runtime validator symbols. Raw build logs, hashes and filtered matches are in this directory.

This checks only the `spinon-ffi` macOS static archive feature boundary; it does not establish APK, IPA or release app bundle exclusion.
