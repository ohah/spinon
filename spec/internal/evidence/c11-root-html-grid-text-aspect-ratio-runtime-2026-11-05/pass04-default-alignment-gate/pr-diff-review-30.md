# PR diff adversarial review · #1161 · 2026-11-05

Reviewed PR diff base `54d05c424a3e7b296b10264c8e4b3407f3db26a0` → head `6b9212b9d42f2b47ed42487e38e1a2321ce5d0e9`. The following checks cover distinct failure paths in the implementation, tests, evidence, and status changes. The follow-up commit that adds this review record does not change product code.

| # | Failure path checked | Result |
|---:|---|---|
| 1 | Aspect-ratio allowlist leakage into other computed-style profiles | The cascade enables typed ratio output only for `RuntimeRootHtmlTextPaintV1`; other profile gates remain unchanged. |
| 2 | Computed ratio text diverging from its typed value | Projection requires serialized `2 / 1` and a matching typed ratio/sizing box before admitting the item. |
| 3 | `auto <ratio>`, zero, or another numeric ratio entering this fixed slice | Cascade and projection reject unsupported forms; focused negative controls cover `auto <ratio>`, `0 / 1`, and `3 / 2`. |
| 4 | Default `aspect-ratio:auto` changing unrelated item geometry | Default/absent ratio does not enter the ratio-item set; ordinary Root HTML nodes project no ratio. |
| 5 | Ratio on the Grid owner, a non-Grid node, or an unrelated descendant | Ownership lookup requires a direct Grid item; unsupported owner/root ratios fail before layout. |
| 6 | Two ratio items on one Grid owner | The owner group is limited to one ratio item and rejects larger groups. |
| 7 | Ratio attached to the wrong item or attached to both items | The candidate requires the second source-order item to be the ratio target; the first item must remain auto-sized and ratio-free. |
| 8 | Width-definite transfer using a different target width | Only `width:60px; height:auto` reaches the supported width-ratio scene; the focused geometry expects a `60×30` target. |
| 9 | Height-definite transfer using a different target height | Only `width:auto; height:30px` reaches the height-ratio scene; the focused geometry expects a `60×30` target. |
| 10 | Min/max constraints altering the transferred axis | Owner and item min/max values must be their initial auto/none forms; a `max-height` control is rejected. |
| 11 | Indefinite or changed owner width affecting the ratio basis | The owner must have a definite `70px` content width and auto height; altered width fails the candidate gate. |
| 12 | Extra tracks or different track functions changing contribution sizing | Columns must be exactly `[10px,60px]` and rows exactly one `auto`; only that row receives the bounded intrinsic-track exception. |
| 13 | Implicit track, repeat, line-name, or named-area expansion | Auto-repeat, nonempty line names, areas, non-auto implicit tracks, and other explicit track forms are excluded by the projection/layout gates. |
| 14 | Different auto-placement order changing item-to-track association | Only row sparse flow and auto placement are accepted; item order is required to stay zero and DOM/paint order is preserved. |
| 15 | A third rendered Grid item or anonymous text item entering the scene | The owner must expose exactly two supported direct Grid items; additional mapped items fail the candidate check. |
| 16 | Similar tags or foreign-namespace elements being mistaken for the HTML scene | Owner and item checks require HTML namespace `div` elements. |
| 17 | Nested Grid ownership accidentally selecting this profile | Projection rejects owners under another Grid ancestor; nested owners cannot acquire the new placement mode. |
| 18 | Nested spans, multiple text nodes, or altered text entering the intrinsic control | Each item must have one direct text child with exact `A`/`AAAA` source text and no element descendants. |
| 19 | Wrapping, transforms, spacing, or shaping style changing the measured text contribution | Paragraph validation fixes font size/line-height, nowrap, transform, spacing, weight, style, and run/inline ancestors; unsupported paragraphs fail closed. |
| 20 | Vertical writing or RTL changing ratio-axis interpretation | The owner and projected paragraph must remain horizontal-tb/LTR; other direction/writing-mode states are rejected. |
| 21 | Box sizing, margin, padding, border, or overflow changing the sizing box | The layout scene requires content-box, zero box edges, and visible overflow; the projection gate also checks zero edges/visible overflow. |
| 22 | Item/container alignment changing auto-size behavior | Root item alignment is fixed to start, content alignment to normal, and item self-alignment to auto; negative controls exercise non-default values. |
| 23 | Alignment support leaking to unrelated Root HTML nodes | Alignment properties are projected only for candidate owners/items; other nodes retain default layout alignment. |
| 24 | Missing text-layout cache entry causing stale-width geometry | The recomputation fallback is limited to an item of the new mode and uses the current content width; missing engine/text errors remain explicit. |
| 25 | Cached paragraph accepted for the wrong width | The available-width bit key is checked again against the returned snapshot before using it. |
| 26 | New placement mode bypassing generic Grid validation | The new mode still passes normal track, placement, sizing, and direct-scene validation before Taffy layout. |
| 27 | Stale owner/item identity crossing a snapshot or style revision | Projection derives ownership from the same tree/snapshot and uses stable `NodeId` maps; runtime evidence records the fixture/source hashes. |
| 28 | Ratio-bearing unsupported scene partially projecting before rejection | Ratio candidates are validated before `LayoutStyle` publication; invalid ratio values return `UnsupportedComputedValue`. |
| 29 | Simulator run testing a different asset or an opt-in profile | Android/iOS packaged fixture hashes match the pinned HTML input; both runs use the default app route and record build/runtime identifiers. |
| 30 | Evidence or roadmap text overstating completion/platform parity | Candidate status stays unchecked; `78/214` is identified as the live derived roadmap count. Logs distinguish SwiftShader/iOS Simulator, no PixelCopy/Metal post-compositor readback, and baseline package failures. |

## Review result

No product-code defect was found in these checks. Focused tests, formatting, Clippy, and both simulator runs are linked from this directory's [README](./README.md). Full package suites are not reported green: the same 10 `spinon-style-to-layout` failures and the same single C15 `spinon-layout` assertion reproduce at base HEAD. GitHub reported no PR checks and `main` has no required status checks configured; this is not a CI-pass claim.
