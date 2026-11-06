# C11.4 parsed Root HTML ASCII hyphenated-token intrinsic sizing precomparison

## Purpose and scope

The current bounded parsed Root HTML `grid-auto-rows:min-content` path accepts ASCII alphanumeric words separated by collapsible ASCII whitespace. This fixed precomparison evaluates adding only U+002D HYPHEN-MINUS between non-empty ASCII alphanumeric segments. It captures browser observation and independent expectations; it contains no Spinon implementation result.

The source grammar under review is `[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*`. The hyphen belongs to the preceding break segment. A line may wrap after U+002D; `word-break:normal`, `overflow-wrap:normal`, and `white-space:normal` remain fixed. The source and Text node must remain unchanged.

The CSS Text 3 2026 CRD §5.1 notes that a hyphen-minus before a digit can remain a break opportunity when the preceding character is a letter or digit; the fixed grammar enforces that boundary by requiring an alphanumeric segment before and after each hyphen. The pinned CSS Grid 2025 CRD §§11.4–11.7 supplies the min-content, max-content, and auto-track sizing rules. Neither observation claims WPT or general CSS support.

## Inputs and execution

- Fixed HTML fixture: [`root-html-grid-intrinsic-ascii-hyphenated-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-ascii-hyphenated-precomparison.html)
- Browser: Agent Browser 0.31.1, HeadlessChrome 157.0.0.0, viewport 1280×633 CSS px, DPR 1.
- Raw CSSOM, frame, line, and Canvas observations: [`chrome-observation.json`](./chrome-observation.json)
- Browser capture: [`chrome-agent-browser.png`](./chrome-agent-browser.png)
- Independent derivation: [`derive_expected.py`](./derive_expected.py) uses Canvas text advances, CSS break-segment boundaries, CSS Grid sizing formulas, and 1/64 CSS px upward rounding. It computes expected values without reading used Grid tracks or rows.
- Derived values: [`expected.json`](./expected.json). All nine independent scenes match Chrome within one 1/64 CSS px layout unit; line counts match exactly.

## Independent expected values

| Scene | Source | Column mode | Expected used column | Expected rows | Text lines |
| --- | --- | --- | ---: | --- | ---: |
| `hyphen-min-content` | `A-A` | `min-content` | 12.046875px | 10, 20, 10, 10px | 2 |
| `hyphen-max-content` | `A-A` | `max-content` | 18.0625px | 10, 10, 10, 10px | 1 |
| `hyphen-auto-mixed-tokens` | `AA-BB CC` | `auto` | 40px | 10, 20, 10, 10px | 2 |
| `digit-hyphen-min-content` | `12-3 45` | `min-content` | 18.0625px | 10, 30, 10, 10px | 3 |
| `mixed-hyphen-min-content` | `A1-BB2` | `min-content` | 18.0625px | 10, 20, 10, 10px | 2 |
| `multiple-hyphenated-tokens` | `A-B C-DE` | `min-content` | 12.046875px | 10, 40, 10, 10px | 4 |
| `hyphenated-reverse-order` | `AA-BB CC` | `min-content` | 18.0625px | 10, 10, 30, 10px | 3 |
| `hyphen-unbreakable-auto` | `12345678-12` | `auto` | 54.1875px | 10, 20, 10, 10px | 2 |
| `fixed-track-hyphen-break` | `AA-BB CC` | fixed 20px | 20px | 10, 30, 10, 10px | 3 |

## Limits

- The comparison covers ASCII alphanumeric runs with internal U+002D only. Leading, trailing, or repeated hyphens remain outside the candidate grammar.
- Unicode dashes/minus characters, punctuation other than U+002D, NBSP, unsupported whitespace, arbitrary fonts/styles, arbitrary HTML/CSS admission, and WPT execution are not covered.
- This is Chrome 157 desktop evidence only. It is not Android/iOS runtime, glyph-paint, physical-device, hardware-GPU, or overall C11.4 completion evidence.

## Hashes

The fixture, observation, expected JSON, derivation script, and screenshot hashes are listed in [`SHA256SUMS`](./SHA256SUMS).
