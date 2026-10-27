use std::collections::BTreeMap;

use spinon_layout::{ParagraphInput, TextSource};

use super::super::{TEXT_CASES, TextCase, TextDimension, TextTrack, generic_direct_text};
use super::test_fixture::{Fixture, NodeSpec, cascade, custom_fixture, direct_text, item};

#[test]
fn all_one_item_text_cases_reach_the_legacy_fallback_with_ahem_intrinsics() {
    assert_eq!(TEXT_CASES.len(), 13);

    for case in &TEXT_CASES {
        let fixture = custom_fixture(
            case.root_id,
            "scene",
            vec![item(
                case.item_id,
                "item",
                vec![NodeSpec::text(case.content)],
            )],
            None,
        );
        let css = legacy_stylesheet(case);
        let computed = cascade(&fixture.view, &css);

        macro_rules! assert_does_not_claim {
            ($label:literal, $result:expr) => {{
                let result = $result.unwrap_or_else(|error| {
                    panic!("{} failed for {}: {error:?}", $label, case.scene_id)
                });
                assert!(
                    result.is_none(),
                    "{} unexpectedly claimed {}",
                    $label,
                    case.scene_id
                );
            }};
        }

        assert_does_not_claim!(
            "nested grid text multispan adapter",
            super::super::super::c11_4_nested_grid_text_multispan::prepare(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "nested grid depth-two adapter",
            super::super::super::c11_4_nested_grid_depth_two::prepare(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "nested grid intrinsic adapter",
            super::super::super::c11_4_nested_grid_intrinsic::prepare(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "implicit intrinsic text adapter",
            super::super::super::c11_4_implicit_intrinsic_text::prepare(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "typed auto-repeat adapter",
            super::super::prepare_typed_auto_repeat_text(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "multispan adapter",
            super::super::prepare_multispan_text(&fixture.snapshot, fixture.root, &computed)
        );
        assert_does_not_claim!(
            "flex span adapter",
            super::super::flex_span::prepare(&fixture.snapshot, fixture.root, &computed)
        );
        assert_does_not_claim!(
            "overlapping spans adapter",
            super::super::overlapping_spans::prepare(&fixture.snapshot, fixture.root, &computed)
        );
        assert_does_not_claim!(
            "cyclic minmax two-axis span adapter",
            super::super::cyclic_minmax_two_axis_span_text::prepare(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "cyclic minmax cross-axis mixed span adapter",
            super::super::cyclic_minmax_calc_cross_axis_mixed_span_groups::prepare(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "cyclic minmax cross-axis span adapter",
            super::super::cyclic_minmax_calc_cross_axis_span_groups::prepare(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "cyclic fit-content calc adapter",
            super::super::cyclic_fit_content_calc::prepare(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "cyclic two-axis multitrack adapter",
            super::super::cyclic_two_axis_multitrack::prepare(
                &fixture.snapshot,
                fixture.root,
                &computed
            )
        );
        assert_does_not_claim!(
            "cyclic calc adapter",
            super::super::cyclic_calc::prepare(&fixture.snapshot, fixture.root, &computed)
        );
        assert_does_not_claim!(
            "generic direct-text collector",
            generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed)
        );

        let mut input = super::super::prepare(&fixture.snapshot, fixture.root, &computed)
            .unwrap_or_else(|error| {
                panic!(
                    "legacy TEXT_CASES fallback failed for {}: {error:?}",
                    case.scene_id
                )
            })
            .unwrap_or_else(|| panic!("legacy TEXT_CASES fallback skipped {}", case.scene_id));
        let [item_node] = fixture.items.as_slice() else {
            panic!("{} fixture must have one Grid item", case.scene_id);
        };
        let paragraph_style = input
            .paragraph_styles
            .get(&item_node.id())
            .unwrap_or_else(|| panic!("{} paragraph style is missing", case.scene_id));
        assert_eq!(input.paragraph_styles.len(), 1, "{}", case.scene_id);
        assert_eq!(paragraph_style.font_family, "Ahem", "{}", case.scene_id);
        assert_eq!(paragraph_style.font_size, 10.0, "{}", case.scene_id);
        assert_eq!(paragraph_style.line_height, 10.0, "{}", case.scene_id);
        assert_eq!(
            paragraph_style.white_space, case.white_space,
            "{}",
            case.scene_id
        );
        assert_eq!(
            paragraph_style.word_break, case.word_break,
            "{}",
            case.scene_id
        );
        assert!(!input.two_axis_nested_grid, "{}", case.scene_id);

        let paragraph = ParagraphInput {
            node: item_node.id(),
            sources: vec![TextSource {
                node: direct_text(&fixture.snapshot, *item_node).id(),
                inline_ancestors: Vec::new(),
                text: case.content.to_owned(),
            }],
            style: paragraph_style.clone(),
            inline_styles: BTreeMap::new(),
            inline_run_styles: BTreeMap::new(),
        };
        let intrinsic = input
            .engine
            .intrinsic_inline_sizes(&paragraph)
            .unwrap_or_else(|error| {
                panic!("{} Ahem intrinsic sizing failed: {error:?}", case.scene_id)
            });
        let (expected_min_content, expected_max_content) = expected_intrinsic_sizes(case);
        assert_eq!(
            intrinsic.min_content(),
            expected_min_content,
            "{} min-content",
            case.scene_id
        );
        assert_eq!(
            intrinsic.max_content(),
            expected_max_content,
            "{} max-content",
            case.scene_id
        );
    }
}

#[test]
fn legacy_fallback_rejects_unknown_identity_topology_and_style() {
    let case = &TEXT_CASES[0];
    let stylesheet = legacy_stylesheet(case);

    expect_legacy_error(
        custom_fixture(
            "unknown-legacy-root",
            "scene",
            vec![item(
                case.item_id,
                "item",
                vec![NodeSpec::text(case.content)],
            )],
            None,
        ),
        &stylesheet,
        "unsupported_c114_text_fixture_root",
        "unknown root ID",
    );
    expect_legacy_error(
        custom_fixture(
            case.root_id,
            "scene",
            vec![item(
                case.item_id,
                "item",
                vec![NodeSpec::text("changed source")],
            )],
            None,
        ),
        &stylesheet,
        "unsupported_c114_text_fixture_case",
        "changed source text",
    );
    expect_legacy_error(
        custom_fixture(
            case.root_id,
            "scene",
            vec![item(
                "unknown-legacy-item",
                "item",
                vec![NodeSpec::text(case.content)],
            )],
            None,
        ),
        &stylesheet,
        "unsupported_c114_text_fixture_case",
        "unknown item ID",
    );
    expect_legacy_error(
        custom_fixture(
            case.root_id,
            "scene",
            vec![
                item(case.item_id, "item", vec![NodeSpec::text(case.content)]),
                item("unlisted-empty-sibling", "item", Vec::new()),
            ],
            None,
        ),
        &stylesheet,
        "invalid_c114_text_grid_owner",
        "extra direct grid item",
    );

    let wrong_track_stylesheet = stylesheet.replace(
        "grid-template-columns: auto;",
        "grid-template-columns: min-content;",
    );
    assert_ne!(wrong_track_stylesheet, stylesheet);
    expect_legacy_error(
        custom_fixture(
            case.root_id,
            "scene",
            vec![item(
                case.item_id,
                "item",
                vec![NodeSpec::text(case.content)],
            )],
            None,
        ),
        &wrong_track_stylesheet,
        "unsupported_c114_text_grid_style",
        "wrong typed Grid track",
    );
}

fn expect_legacy_error(fixture: Fixture, css: &str, expected_code: &str, label: &str) {
    let computed = cascade(&fixture.view, css);
    assert!(
        generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed)
            .unwrap_or_else(|error| panic!("{label}: generic collector failed: {error:?}"))
            .is_none(),
        "{label}: generic collector should leave a one-item legacy shape untouched"
    );
    let error = match super::super::prepare(&fixture.snapshot, fixture.root, &computed) {
        Err(error) => error,
        Ok(Some(_)) => panic!("{label}: unsupported legacy input unexpectedly succeeded"),
        Ok(None) => panic!("{label}: unsupported legacy input was silently ignored"),
    };
    assert_eq!(error.code, expected_code, "{label}");
}

fn legacy_stylesheet(case: &TextCase) -> String {
    let width = dimension_css(case.grid_width);
    let height = dimension_css(case.grid_height);
    let columns = track_css(case.columns);
    let rows = track_css(case.rows);
    let align_items = if matches!(case.columns, TextTrack::FitContentPercentage)
        || matches!(case.rows, TextTrack::FitContentPercentage)
    {
        "normal"
    } else {
        "start"
    };
    let min_width = case
        .min_width
        .map(|value| format!("{value}px"))
        .unwrap_or_else(|| "auto".to_owned());
    let max_width = case
        .max_width
        .map(|value| format!("{value}px"))
        .unwrap_or_else(|| "none".to_owned());
    let min_height = case
        .min_height
        .map(|value| format!("{value}px"))
        .unwrap_or_else(|| "auto".to_owned());

    format!(
        r#"
.scene {{
  display: grid;
  box-sizing: border-box;
  width: {width};
  height: {height};
  grid-template-columns: {columns};
  grid-template-rows: {rows};
  align-items: {align_items};
  align-content: normal;
  row-gap: 0px;
  column-gap: 0px;
}}
.item {{
  display: block;
  box-sizing: {box_sizing};
  font-family: Ahem;
  font-size: 10px;
  font-weight: 400;
  font-style: normal;
  line-height: 10px;
  white-space-collapse: collapse;
  text-wrap-mode: {white_space};
  word-break: {word_break};
  direction: ltr;
  line-break: auto;
  text-align: start;
  overflow-wrap: normal;
  text-transform: none;
  writing-mode: horizontal-tb;
  letter-spacing: normal;
  word-spacing: 0px;
  text-indent: 0px;
  min-width: {min_width};
  max-width: {max_width};
  min-height: {min_height};
  padding-left: {inline_padding}px;
  padding-right: {inline_padding}px;
  border-left: {inline_border}px solid;
  border-right: {inline_border}px solid;
}}
"#,
        box_sizing = case.box_sizing,
        white_space = case.white_space_mode,
        word_break = case.word_break_mode,
        inline_padding = case.inline_padding,
        inline_border = case.inline_border,
    )
}

fn dimension_css(dimension: TextDimension) -> String {
    match dimension {
        TextDimension::Px(value) => format!("{value}px"),
        TextDimension::Auto => "auto".to_owned(),
        TextDimension::MaxContent => "max-content".to_owned(),
    }
}

fn track_css(track: TextTrack) -> String {
    match track {
        TextTrack::Length20 => "20px".to_owned(),
        TextTrack::Auto => "auto".to_owned(),
        TextTrack::MinContent => "min-content".to_owned(),
        TextTrack::MaxContent => "max-content".to_owned(),
        TextTrack::FitContent40 => "fit-content(40px)".to_owned(),
        TextTrack::FitContentPercentage => "fit-content(50%)".to_owned(),
        TextTrack::FitContentCalc => "fit-content(calc(10% + 5px))".to_owned(),
    }
}

fn expected_intrinsic_sizes(case: &TextCase) -> (f32, f32) {
    match case.scene_id {
        "auto-text-max-content"
        | "min-content-text-normal"
        | "max-content-text-normal"
        | "fit-content-text-minimum"
        | "min-content-text-edge"
        | "min-content-text-min-width"
        | "min-content-text-max-width" => (70.0, 100.0),
        "fit-content-calc-limit" | "min-content-text-break-all" => (10.0, 80.0),
        "min-content-text-nowrap" => (100.0, 100.0),
        "cyclic-fit-content-column" | "definite-fit-content-column" => (20.0, 50.0),
        "cyclic-fit-content-row" => (10.0, 70.0),
        scene_id => panic!("unexpected legacy text scene {scene_id}"),
    }
}
