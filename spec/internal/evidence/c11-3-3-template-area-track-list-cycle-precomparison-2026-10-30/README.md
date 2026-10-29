# C11.3.3 template-area implicit track-list cycling · precomparison

## Source and capture

- Normative source: [CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.3, 7.5, 7.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-tracks). §7.6 defines `grid-auto-rows` and `grid-auto-columns` as `<track-size>+`; when the list has multiple sizes, it repeats the pattern. The first implicit track after the explicit grid receives the first size. The separate before-explicit direction runs backward from the last size; this proposal does not implement that direction.
- Browser: Agent Browser 0.31.1, `HeadlessChrome/156.0.0.0`, macOS ARM64, viewport `1280×633`, DPR 1.
- Fixture: [`grid-template-areas-auto-track-list-cycling-c1133-precomparison.html`](../../../../tests/fixtures/css/c11/grid-template-areas-auto-track-list-cycling-c1133-precomparison.html), SHA-256 `4790832b831ad41d4193f5d2b41f88528f6c49578619deccc2637f96bf1fd4ba`.
- Chrome observation: [`observation.json`](./chrome/observation.json), SHA-256 `17e68a7f78474aea69c9864ddaa4311927db94a06894f73d670e60bebaa977ca`.
- Independent normative expectation: [`expected.json`](./expected.json), SHA-256 `87f0f8f730c06f37c2b3183fbef894859fa489ce2605c38d8122292ba693e4e1`.
- Browser screenshot: [`fixture.png`](./chrome/fixture.png), SHA-256 `f424c8bf7fcb7a28a2d6babbde711b2b6aaaf9728af5150e28d1114aefdb693a` (`1280×633`).

## Observation

The six fixed scenes show a two-value auto-track list cycling from its first value on each positive implicit axis. `both-axis-cycle` produces rows `[20, 5, 7, 5]px` and columns `[30, 11, 13, 11]px`; one-axis scenes test the same rule with two authored tracks before the generated tracks. The two `*-continues-after-area` controls place one more item beyond an area-expanded axis and verify that auto-track list phase continues after the area-generated tracks. `authored-tracks-exceed-area` preserves all three authored tracks on both axes even though its named areas use only two.

The independent values in `expected.json` were calculated from the authored explicit prefix and the §7.6 pattern rule before looking at the Chrome output. Chrome's resolved track strings and root-relative item frames match them exactly. The screenshot is visual context; it is not Spinon pixel evidence.

## Boundary

The proposed implementation only adds non-empty, fixed-`px` auto-track lists to the existing `RuntimeBlockFlexGridTemplateV1` area-expansion route. It must preserve the list phase if explicit placement generates more positive implicit tracks after the area expansion. It does not claim percentage, intrinsic, flexible, `calc()`, negative implicit tracks, auto-repeat interaction, general CSS admission, WPT conformance, or C11 completion. The Agent Browser session was closed after capture, and `pgrep -fl agent-browser` returned no processes.
