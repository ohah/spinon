# C11.4 fixed-repeat named-line fixture artifact boundary

The scanner builds the default `c04-runtime-gpu` release static archive and the
`c04-runtime-gpu-fixture-test` positive control, then checks the three added
scene IDs, the shared C11.4 fixture ABI, and the candidate Rust runtime source
symbol. The complete archive bytes and defined symbols are inspected. The
source revision and staged implementation diff hash are recorded in
[`artifact-boundary.json`](artifact-boundary.json).

Run from the repository root:

```sh
python3 spec/internal/evidence/c11-4-fixed-repeat-line-name-merge-runtime-2026-10-24/artifact-boundary/scan-ffi-archives.py
```

The evidence is limited to the macOS ARM64 release `spinon-ffi` static archive.
It does not establish fixture exclusion from the complete Android APK or iOS
IPA, release package size, hardware GPU behavior, or device compositor output.
