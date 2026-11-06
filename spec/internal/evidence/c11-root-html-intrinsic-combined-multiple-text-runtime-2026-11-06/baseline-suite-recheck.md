# Base-commit failure recheck · 2026-11-06

Base: `3d1d4cab0c75b0905bd6952e3faee59a72dc863d`. Candidate commit: `64502bd9a797ba7adcff539a85d79195777fd84b`.

The changed production files were temporarily replaced with `git show <base>:<path>` and restored from byte-for-byte working copies on shell exit. Style-to-layout's new helper test declarations were also restored from base for its baseline runs. Each listed test still failed in the base-source run (exit 101). The candidate and base failures are not reported as passing checks.

## `spinon-runtime`: 8/8 rechecked failures

- `c11_1_root_html_default_runtime_applies_fixed_px_grid_gaps`
- `c11_1_root_html_default_runtime_rejects_out_of_scope_author_grid_inputs`
- `c11_1_root_html_unsupported_auto_minimum_track_does_not_publish_partial_layout`
- `c11_1_root_html_fixed_grid_content_alignment_reaches_default_runtime`
- `c11_1_root_html_numeric_grid_placement_rejects_unsupported_scenes_atomically`
- `c11_1_root_html_default_runtime_rejects_unsupported_implicit_row_combinations_atomically`
- `c11_1_root_html_integer_repeat_rejects_out_of_profile_and_over_budget_inputs`
- `c11_1_root_html_fixed_grid_item_alignment_matches_all_expected_frames`

The full candidate package run reported 288 passed, 8 failed and 3 ignored. All 11 tests in the changed intrinsic-auto-row runtime module passed.

## `spinon-style-to-layout`: 10/10 rechecked failures

- `aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs`
- `root_html_fixed_grid_projects_distribution_values_to_taffy`
- `root_html_fixed_grid_safe_alignment_falls_back_and_unsafe_alignment_overflows`
- `root_html_explicit_placement_rejects_mixed_partial_implicit_and_anonymous_auto_items`
- `root_html_grid_item_alignment_maps_physical_left_and_right_in_ltr`
- `root_html_grid_item_alignment_maps_positional_and_place_items_values`
- `root_html_grid_item_safe_alignment_falls_back_and_unsafe_alignment_overflows`
- `root_html_grid_self_axis_values_reach_the_grid_item_layout`
- `invalid_negative_gap_cascades_but_math_requires_a_definite_percentage_basis`
- `root_html_grid_math_gaps_compose_with_implicit_rows_column_flow_and_named_areas`

The full candidate package run reported 422 passed and 10 failed. All 7 tests in the changed intrinsic-auto-row style-to-layout module passed.

## `spinon-layout`: unchanged package

The package source and tests are unchanged by this PR. Its full run reported 453 passed and one existing `c15_text_layout_tests::host_document::host_document_collects_direct_text_and_inherited_spans_in_source_order` failure; the isolated test reproduced the failure. The 12 filtered intrinsic-track tests passed.
