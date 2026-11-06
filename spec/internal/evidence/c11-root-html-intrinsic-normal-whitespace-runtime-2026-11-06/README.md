# C11.4 Root HTML intrinsic rows: normal whitespace runtime evidence

This candidate extends the existing default parsed Root HTML `grid-auto-rows: min-content` path to accept ASCII `SPACE`, `TAB`, `LF`, and `CR` separators around one or more standalone `A` tokens. It leaves the source text and text-node identity intact and delegates collapsing and line wrapping to the existing text layout path. The scope and Chrome 156 reference values are fixed in the [plan](../../../../plan/c11-root-html-intrinsic-normal-whitespace-multiple-text.md) and [precomparison](../c11-root-html-intrinsic-normal-whitespace-precomparison-2026-11-06/README.md).

## Rust checks

- `cargo fmt --all -- --check` — passed.
- `cargo test --locked -p spinon-style-to-layout --lib c11_1_root_html_intrinsic_auto_rows_text_tests` — 9/9 passed.
- `cargo clippy --locked -p spinon-style-to-layout --all-targets -- -D warnings` — passed.
- `cargo test --locked -p spinon-runtime --lib root_html_grid_intrinsic_auto_rows` — 13/13 passed after the runtime test module was split by acceptance and rejection behavior.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test --lib parsed_root_html_normal_whitespace` — 2/2 passed. The opt-in diagnostics verify parsed text values, text-node identity, revisions, per-item contributions, and the row sizing trace.
- `cargo clippy --locked -p spinon-runtime --all-targets -- -D warnings` — passed.
- Feature-enabled runtime Clippy is not green: it reports two existing `clippy::too_many_arguments` errors in `anonymous_text_hyphens.rs:117` and `anonymous_text_text_transform.rs:507`. Both files are unchanged from `origin/main`; this candidate does not claim that command passed.

## Android and iOS Simulator smoke

The passing run is [`pass05`](./pass05/environment.txt). It used Android API 37 ARM64 AVD `emulator-5554` and iPhone 17 Pro Simulator on iOS 26.2 (`9DCCE836-7659-4914-B6C1-13F54BFFA9DB`). The physical Samsung device was not used. Both C11 fixture-test flags were off; the apps used the default Root HTML route. The installed HTML SHA-256 on each platform is `9378b438c4398d56440a4513e700059fd335a5f38dd0c31074ee9fc88dfb6d68`, matching the committed [runtime fixture](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-normal-whitespace-runtime.html).

- Android: `layout=ready`, `boxes=8`, app evaluation `error=none`, `presented boxes=8`, and `runtime_text_count=2`. The [1080×2400 screenshot](./pass05/android/screenshot.png) shows the wrapped long text, short text, fixed controls, and content overflowing below the 40px Grid.
- iOS: `layout=ready`, `boxes=8`, app evaluation `error=none`, `presented boxes=8`, and `runtime_text_count=2`. The [1206×2622 screenshot](./pass05/ios/screenshot.png) shows the same scene.
- The app script checks DOM order, one direct text node per text item, and exact parsed `textContent`, including the CRLF produced by the carriage-return character reference followed by a line feed. It sets `data-runtime="ready"` after those assertions.
- [`run-simulators.sh`](./run-simulators.sh) restores the saved bootstrap and iOS bundle assets. `pass05` records byte-for-byte restoration as `PASS`. A post-run process check found no `agent-browser` or `HeadlessChrome` process; this run did not open a browser.
- Xcode emitted trailing spaces and blank lines at the end of `ios/build.log`; only that whitespace was normalized so the evidence passes `git diff --check`.

Screenshots are visual smoke only. No PixelCopy, Metal drawable readback, compositor pixel comparison, physical-device or hardware-GPU result, WPT run, or CI pass is claimed.

## Retained harness failures

The earlier attempts remain beside the passing run so the failure and correction are auditable:

| Attempt | Result | Cause |
| --- | --- | --- |
| [`pass01`](./pass01/environment.txt) | Failed | The app's DOM facade has no `childNodes` collection. The layout reached `ready`; the assertion raised a `TypeError`. |
| [`pass02`](./pass02/environment.txt) | Failed | The assertion expected one text node per item, but treated CRLF as LF. |
| [`pass03`](./pass03/environment.txt) | Failed | Same expectation mismatch; the diagnostic captured the actual preserved CRLF source. |
| [`pass04`](./pass04/environment.txt) | Runtime passed; runner failed | The runner looked for layout fields after the Android app-eval log, which is truncated. Separate environment and draw records contain those values. |
| [`pass05`](./pass05/environment.txt) | Passed | The assertion uses the facade's `firstChild`/`nextSibling` API and checks the captured CRLF value. Log predicates match the separate app-eval, layout, and draw records. |

Asset restoration passed after each simulator run. The only shipped implementation change remains the whitespace selector and its focused tests; the earlier failures were in the smoke harness and its assertions.

## Scope boundary

This is a bounded C11.4 candidate, not general CSS Grid or public CSS support. Form feed, vertical tab, NBSP, NEL, em-space, empty/whitespace-only sources, non-`A` tokens, and other unsupported combinations remain rejected atomically. Arbitrary Unicode, fonts, languages, nested text, WPT conformance, and C11/C11.4 completion are not included. The official roadmap remains `78/214` until its separate completion criteria are met.
