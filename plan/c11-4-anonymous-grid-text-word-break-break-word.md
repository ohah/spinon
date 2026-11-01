# C11.4 anonymous Grid text `word-break: break-word` interaction plan

**Parent:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [internal contract 0059](../spec/internal/0059-c11-grid-layout.md) · [official status](../spec/STATUS.md)

**Precomparison:** CSS Text 3 §5.1/§5.4, CSS Sizing 3 §5.1, CSS Grid 1 §11, pinned Chrome 156 observation, and Ahem-derived expected values in [the evidence directory](../spec/internal/evidence/c11-4-anonymous-grid-text-word-break-break-word-precomparison-2026-11-01/README.md).

## Goal and scope

Extend the existing opt-in anonymous direct-text Grid fixture so its paragraph sizing honors the deprecated computed `word-break: break-word` value. CSS Text 3 defines it as having the same effect as `word-break: normal` plus `overflow-wrap: anywhere`, regardless of the computed `overflow-wrap` value, and says `word-break` affects intrinsic sizes. The current C11.4 `word-break` fixture rejects `break-word`; the independently implemented `overflow-wrap` profile now provides the necessary text layout behavior.

The fixed profile uses one anonymous direct Text child containing `XXXX`, pinned Ahem 10px/400 with 10px line-height, horizontal-tb/LTR, `white-space: normal`, zero gap/edges, a 120px definite Grid width, one `min-content`, `max-content`, or `10px` column, and one auto row. It combines computed `word-break: normal|break-word` with computed `overflow-wrap: normal|break-word|anywhere`: 18 exact scenes. Existing 12 `word-break` and 9 `overflow-wrap` scenes remain regression gates.

For `word-break: break-word`, all three `overflow-wrap` values must produce the `normal + anywhere` result: min-content column 10px, max-content column 40px, and four 10px text lines in a fixed 10px track. For `word-break: normal`, `overflow-wrap: break-word` preserves the 40px min-content contribution while wrapping in the fixed 10px track; `overflow-wrap: anywhere` reduces min-content to 10px; `normal` does not wrap the 40px word in a 10px track. These contrasts prevent the two deprecated/current properties from being conflated.

Only the existing fixture-test opt-in runtime route changes. No public CSS admission, general HTML language behavior, arbitrary font/text combinations, other writing modes, WPT conformance, or C11.4/C11 completion is claimed. The internal contract version stays `0.1.0`; official `72/213` and C11/C11.4 checkboxes remain unchanged.

## Stylo Servo parser compatibility

The pinned Stylo 0.22 Servo build does not compile its `WordBreak::BreakWord` parser value; that value is gated behind Stylo's Gecko feature. The opt-in C11.4 route therefore rewrites only a literal `word-break: break-word` value token to an equal-byte-length `normal` token for Stylo parsing. The cascade adapter recovers the original keyword from the declaration block that won, including stylesheet source order, inline style, and `!important`; it exposes the inherited or winning alias as `break-word` in the same computed-style snapshot consumed by layout. CSS-wide `inherit`/`unset` continue from the parent, while `revert` and `revert-layer` follow Stylo's cascade-priority rollback before falling back to inherited values. Focused regressions cover a lower UA-origin value, a lower stylesheet layer exposed by inline `revert-layer`, and inherited aliases. No scene ID selects or overrides the value. The parser rewrite is compiled only with the existing C11.4 fixture-test feature. Custom-property substitution into `break-word`, arbitrary CSSOM, `@layer` author-CSS admission, and general author CSS admission remain outside this slice.

## Normative and independent expectations

- CSS Text 3 §5.1 says deprecated `word-break: break-word` has the same effect as `word-break: normal` plus `overflow-wrap: anywhere`, regardless of the actual `overflow-wrap` property, and that `word-break` participates in intrinsic-size calculation.
- CSS Text 3 §5.4 distinguishes `overflow-wrap: anywhere` from `overflow-wrap: break-word`: `anywhere` opportunities participate in min-content sizing, while `break-word` emergency opportunities do not.
- Ahem 10px gives `XXXX` a 40px unbroken advance. `anywhere`-effective min-content permits a 10px fragment and yields four lines at a 10px column; max-content remains 40px. The independent per-scene values and Chrome observations are kept in the precomparison evidence.

## Implementation and verification sequence

1. Review the plan and its independent oracle from 20 distinct failure perspectives. Revise the plan or oracle before implementation if a case is ambiguous.
2. Review the plan PR diff separately from the plan review. Merge the plan before creating the implementation branch from current `origin/main`.
3. Add an explicit `WordBreakMode::BreakWord` model value. Build Parley with `WordBreak::Normal` and effective `OverflowWrap::Anywhere` whenever this value is present, while retaining the independently computed `ParagraphStyle.overflow_wrap` for provenance and diagnostics. The override applies to final line layout and intrinsic min-content; `overflow-wrap: break-word` without `word-break: break-word` must keep its existing non-min-content behavior. Keep `Normal`, `BreakAll`, and `KeepAll` behavior unchanged.
4. Extend the existing C11.4 anonymous Grid text adapter across `spinon-runtime` scene validation and the fixture-only `spinon-ffi` source generator with the exact 18-scene allowlist. Use the fixture-only Stylo compatibility boundary above to preserve the alias from its winning declaration. Validate both computed properties from one exact Grid-owner style snapshot; do not infer either CSS value from the scene ID. Reject missing or mismatched computed properties, unlisted scenes, unsupported font/white-space/layout inputs, and stale snapshots before layout publication.
5. Run focused text-layout and FFI fixture tests, the existing `word-break`/`overflow-wrap` regression matrices, Rust formatting and scoped Clippy. Compare the exact source revision in Android API 37 AVD and iOS Simulator; retain simulator, app-hash, trace, and screenshot evidence. An emulator is not physical-device or hardware-GPU evidence.
6. Perform a new implementation/runtime review from 20 distinct failure perspectives and a separate 20-perspective review of the exact implementation PR diff. Record any failure before revising; re-review affected perspectives after fixes.
7. After merge, reconcile internal contract 0059, C11 status, the C11.4 roadmap, and the Tailnet preview. Keep official completion at `72/213` and C11/C11.4 unchecked.

## Failure controls

- `normal + overflow-wrap: break-word` must not be treated as `anywhere` for min-content sizing.
- `word-break: break-word` must override `overflow-wrap: normal|break-word|anywhere` consistently for both line layout and intrinsic min-content sizing.
- The max-content advance must remain 40px; soft emergency breaks must not change it.
- The 10px fixed-track controls must distinguish no wrap from four-line emergency wrapping.
- Property values must come from the same computed-style snapshot as the Grid owner's text source; scene ID is only an expectation selector, never a source of CSS values.
- CSS-wide inheritance and rollback keywords must preserve the actual parent/origin/layer result; unsupported source recovery must not silently preserve a parent alias.
- Missing/duplicate Text nodes, changed source text, wrong owner/locale, stale revisions, and partial frame/trace publication must fail closed.
- Fixture-only coverage must not admit general user-authored CSS or leak its runtime entrypoint into the default FFI artifact.

## Out of scope

`break-all`/`keep-all` expansion, arbitrary `word-break` values, arbitrary `overflow-wrap` combinations beyond the six fixed pairs, non-Ahem fonts, arbitrary scripts, inline descendants, `lang` inheritance changes, bidi/vertical text, all CSS Text behavior, WPT conformance, physical-device/hardware-GPU testing, and complete C11.4/C11 support.

## 구현 결과 · 2026-11-01

구현 PR [#974](https://github.com/ohah/spinon/pull/974)은 2026-11-01T07:28:34Z에 Rebase and merge됐고 merge SHA는 `2700a8bfad05c1b8fc474603c58ae68c2dde3fb3`다. opt-in C11.4 fixture에 `word-break` × `overflow-wrap` × Grid track의 18개 고정 장면을 연결했다. Android API 37.1 ARM64 16 KB SwiftShader AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 18/18을 통과했다. 집중 style/cascade·runtime·text-layout·FFI·fixture-boundary 검사, scoped Clippy, 서식 검사와 Android/iOS simulator app 빌드는 [runtime evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-word-break-break-word-runtime-2026-11-01/README.md)에 기록했다. 구현/runtime 검토와 별도 PR diff 검토는 실행 근거에, 병합 후 문서 변경의 적대 검토 20개는 [로드맵 동기화 근거](../spec/internal/evidence/roadmap-tailnet-sync-pr974-2026-11-01/roadmap-pr-diff-review-20.md)에 연결했다.

GitHub에서 PR checks가 보고되지 않아 CI 통과로 세지 않는다. glyph-paint fixture가 비활성화되어 screenshot은 글자 픽셀 증거가 아니다. 일반 CSS admission, WPT, 실기기/hardware GPU 동작, iOS compositor 이후 픽셀, 전체 C11.4/C11은 미검증이다. 이 계획의 제한 slice는 병합했지만 C11/C11.4 체크는 미완료로 두며 공식 완료 `72/213`을 유지한다.
