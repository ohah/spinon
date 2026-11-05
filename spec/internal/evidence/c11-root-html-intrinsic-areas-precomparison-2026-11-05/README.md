# C11.1 intrinsic tracks with named areas precomparison

This fixed fixture composes two Root HTML Grid behaviors that are implemented separately: explicit intrinsic tracks and `grid-template-areas`. The current Root HTML intrinsic-track contract explicitly rejects this combination, so the comparison records the behavior needed before extending the default route.

## Browser evidence

- Agent Browser opened the checked-in fixture in HeadlessChrome 156.0.0.0 on macOS, DPR 1, viewport 1280×633.
- [Chrome observation](chrome156-observation.json) records computed track sizes and item frames relative to each grid root.
- The fixture fixes item text at 10px monospace with an explicit 11px line height so the Grid comparison does not inherit the runtime's separate normal line-height behavior.
- [Independent structural expectations](expected-invariants.json) record the area map, area spans, and track-size relationships. Text advances in the Chrome capture remain browser/font observations and use a 0.5 CSS px cross-runtime tolerance.
- [Screenshot](chrome156-observation.png) is a visual record of the same fixture; it is not product-runtime or paint-parity evidence.
- Fixture SHA-256 is recorded in [SHA256SUMS.txt](SHA256SUMS.txt).

## Scope for the follow-up implementation

The follow-up should let the default `RuntimeRootHtmlTextPaintV1` route retain a valid rectangular area map while applying the already-admitted explicit intrinsic track sizes. It must cover columns, rows, a named area spanning multiple tracks, and a spanning area item. Existing independent restrictions for auto-repeat, implicit tracks, named-line metadata, column/dense flow, direct-text aspect-ratio mode, and invalid/non-rectangular areas remain in force unless separately verified. The generic Grid and existing area-only profiles must keep their current behavior.

The browser observation is a reference, not a reason to expand unsupported combinations silently. Invalid area maps, unsupported mixed profiles, overflow, and revision mismatch must reject atomically.
