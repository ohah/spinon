# C11.4 six-fit Root HTML repeated-text auto-placement runtime evidence

**Status:** implementation PR [#1354](https://github.com/ohah/spinon/pull/1354) was rebase-merged at `2026-11-11T07:55:23Z`, merge SHA `714194ac7123a431296e8268b1e9aa487f7c48e9`. GitHub reported no status checks. This bounded six-fit slice does not change the official C11.4/C11 completion count.

## Scope

The change admits one exact six-member `grid-auto-rows` profile in parsed Root HTML repeated-text auto-placement. Every member must be `fit-content()` and the typed cap multiset must be `{0px, 10px, 20%, 30px, 40px, 50px}`. All 720 authored cap orders are supported. The new automatic minimum keeps each of the eight direct text items at or above its payload floor, including the zero-cap track. This does not add arbitrary CSS Grid behavior, public API/FFI, full WPT support, hardware/compositor proof, or C11.4 completion.

## RuntimeSession and regression results

The Chrome 157 reference data is in the sibling `c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11` directory. It covers 5,760 scenes / 2,880 paired baseline and candidate keys. The RuntimeSession matrix compares computed typed and serialized track order, alignment, same-revision Grid/item/payload frames, line and glyph counts, overflow and automatic-minimum floors.

Expected matrix totals:

| Measure | Result |
| --- | ---: |
| Scenes / cap orders | 5,760 / 720 |
| Baseline under-floor / payload overflow occurrences | 7,200 / 7,200 |
| Candidate automatic-minimum item occurrences | 23,040 |
| Candidate floor violations / visible payload overflow | 0 / 0 |
| Grid-end overflow (items / payloads) | 0 / 0 |
| Paired row profiles changed by auto minimum | 2,880 / 2,880 |
| `start` versus `stretch` row geometry differences | 0 |

The focused six-fit gates passed: 4 controls/rejection/recovery tests, 1 fixture-feature trace test, and 2 style/layout exact-profile unit tests. The trace checks one explicit plus eight implicit rows, one same-revision Grid owner, eight automatic-minimum contributions, each payload floor, and each final row base size.

Existing profile regression logs record: one-fit 4/4, two-fit 6/6, three-fit 5/5, four-fit focused controls/rejections 4/4 plus the corrected rejection test 1/1, five-fit 6/6, repeated-text row-span 4/4, feature-gated row-span trace/regression 17/17, and track-list regressions 20/20. Initial one-, two-, and four-fit rejection runs are retained because they exposed obsolete tests that incorrectly expected an already-supported five-fit input to fail. The rejection inputs were corrected to use a genuinely unlisted six-fit cap; subsequent corrected runs passed. No five-fit behavior was removed.

`cargo fmt --all -- --check`, the source/documentation `git diff --check`, and default affected-crate Clippy (`cargo clippy --locked --all-targets -p spinon-layout -p spinon-style-to-layout -p spinon-runtime -- -D warnings`) passed. Captured tool stdout logs are preserved verbatim; several contain tool-generated trailing whitespace or a final blank line. Feature-enabled Clippy also found two existing `too_many_arguments` violations in untouched `anonymous_text_hyphens.rs:117` and `anonymous_text_text_transform.rs:507`; the log is retained and those unrelated functions were not changed.

## Platform smoke and limits

The iOS Simulator app build succeeded. The iPhone 17 Pro / iOS 26.2 task clone rendered the default Root HTML `index.html` route in light and dark appearance. Logs show the Root HTML render host ready, layout ready, and draw/present successful; the screenshots are `ios-light-root-html.png` and `ios-dark-root-html.png`. The fixture uses fixed colors, so the screenshots have the same palette even though the dark appearance was applied. This default app route does not load the six-fit fixture, so these screenshots are app smoke evidence, not six-fit geometry evidence.

The Android debug APK build succeeded (`app-debug.apk`, SHA-256 `3a65dcc07d55816c41deff145f1f7421d4ff08716ce711c9051b0404d580ae42`). The task-owned API 37 ARM64 AVD did not boot on this host: QEMU reported `hvf is not enabled`, repeated `qemu_mprotect__osdep: mprotect failed: Permission denied`, and hung main/CPU threads. Therefore there is no Android install, launch, screenshot, or runtime validation in this evidence. `android-emulator-retry.log` and `android-emulator-gui-retry.log` preserve the failure.

Resource cleanup was verified after the smoke: the task-owned iOS clone and Android AVD were deleted; `xcrun simctl list devices booted` showed no booted device, `Simulator.app` was not running, and `adb devices -l` showed no device. Shared iPhone/iPad simulator entries remain shutdown and untouched. The older S03 C9 task clone is not present in the current simulator inventory. Existing unrelated Agent Browser sessions were preserved.

## App and screenshot identities

| Artifact | SHA-256 |
| --- | --- |
| iOS Simulator app executable | `1c1cb58e5d387b35bcb69077ff01a2145d8cd769c02ddbf92bf7076beb800113` |
| bundled `index.html` | `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9` |
| light appearance screenshot | `b0ab3338ed8439c22c1722806f0fe4722edd86eeed227762e066d072cf84712f` |
| dark appearance screenshot | `e76e1bb7f6df7c695a4d17757e1bf33782b4cd710374d817a50d000318f4d2e7` |
| six-fit runtime fixture | `b40525ddafea32a4e757db67b85444ffc69a288d48ee2782e67d815f5d1dce38` |

`SHA256SUMS` records the hashes of the evidence files in this directory. Failed diagnostic attempts are deliberately preserved alongside corrected passing runs so their cause remains reviewable.

The implementation diff review is in `implementation-runtime-review-20.md`; the separate review of the opened PR diff is in `implementation-pr-review-30.md`.

## Logs

- `six-fit-runtime-matrix.log`, `six-fit-focused-final.log`, `six-fit-trace-final-rerun.log`: 5,760-scene RuntimeSession matrix, focused controls/rejections/recovery, and trace.
- `style-layout-focused.log`: exact profile unit tests.
- `one-fit-regression-corrected.log`, `two-fit-regression-corrected.log`, `three-fit-regression.log`, `four-fit-focused-corrected.log`, `four-fit-rejection-corrected.log`, `five-fit-regression.log`: placement regressions.
- `repeated-text-row-span-regression.log`, `repeated-text-row-span-trace-regression.log`, `track-list-regression.log`: existing grid regressions.
- `clippy-final.log`, `clippy-fixture-feature.log`, `clippy.log`: Clippy runs and the unrelated feature-enabled findings.
- `ios-build.log`, `ios-boot.log`, `ios-clone-boot.log`, `ios-light-bootstrap.log`, `ios-light-render.log`, `ios-dark-render.log`: iOS build and app smoke.
- `android-build.log`, `android-emulator-retry.log`, `android-emulator-gui-retry.log`: Android build and failed AVD startup.
- Initial failure logs ending in obsolete five-fit expectations, plus initial trace attempts, remain in this directory with the corrected logs.
