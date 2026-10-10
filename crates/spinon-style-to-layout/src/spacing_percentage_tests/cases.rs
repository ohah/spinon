use spinon_layout::LayoutError;
use spinon_style::ComputedCssSpacingValue;

use super::fixture::{
    FullChromiumSpacingFixture, RuntimeSpacingFixture, assert_absolute_frame,
    assert_relative_frame, fixture_reference, reference_node,
};
use crate::StyleLayoutError;

#[test]
fn full_chromium_fixture_matches_every_supported_runtime_node() {
    let fixture = FullChromiumSpacingFixture::new();
    let output = fixture.compute().unwrap();
    let reference = fixture_reference();
    let excluded = [
        "m-invalid-auto",
        "g-auto-column",
        "g-auto-column-a",
        "g-auto-column-b",
        "g-auto-row-inline",
        "g-auto-row-inline-a",
        "g-auto-row-inline-b",
    ];
    let mut compared = 0;
    let mut maximum_error = 0.0_f32;

    for expected in reference["observation"]["nodes"].as_array().unwrap() {
        let id = expected["id"].as_str().unwrap();
        if excluded.contains(&id) {
            continue;
        }
        let node = fixture.nodes.get(id).unwrap_or_else(|| {
            panic!("지원 대상 Chromium 노드 {id}가 HostDocument 입력에 없습니다")
        });
        let frame = output.layout.frames[&node.id()];
        maximum_error = maximum_error.max(assert_absolute_frame(&frame, &expected["rect"], id));
        compared += 1;
    }
    assert_eq!(compared, 71);
    eprintln!("C06.2 Chromium 비교: nodes={compared} max_abs_error_css_px={maximum_error}");

    let zero_node = fixture.nodes["m-zero"].id();
    let zero_style = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == zero_node)
        .unwrap();
    assert_eq!(
        zero_style.layout_spacing.margin.top,
        ComputedCssSpacingValue::Percentage(0.0)
    );
    let over_node = fixture.nodes["m-over100"].id();
    let over_style = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == over_node)
        .unwrap();
    assert_eq!(
        over_style.layout_spacing.margin.top,
        ComputedCssSpacingValue::Percentage(1.25)
    );
}

#[test]
fn runtime_spacing_percentages_match_pinned_chromium_local_geometry() {
    let fixture = RuntimeSpacingFixture::new();
    let output = fixture.compute().unwrap();
    let reference = fixture_reference();

    for (id, parent) in [
        ("m-physical", "margin-cb"),
        ("m-negative", "margin-cb"),
        ("p-physical", "padding-cb"),
        ("g-row-a", "g-row"),
        ("g-row-b", "g-row"),
        ("g-column-a", "g-column"),
        ("g-column-b", "g-column"),
        ("g-auto-width-row-a", "g-auto-width-row"),
        ("g-auto-width-row-b", "g-auto-width-row"),
        ("g-stretched-column-a", "g-stretched-column"),
        ("g-stretched-column-b", "g-stretched-column"),
    ] {
        let actual = output.layout.frames[&fixture.nodes[id].id()];
        let actual_parent = output.layout.frames[&fixture.nodes[parent].id()];
        let expected_node = reference_node(&reference, id);
        let expected_parent_node = reference_node(&reference, parent);
        let expected = &expected_node["rect"];
        let expected_parent = &expected_parent_node["rect"];
        assert_relative_frame(&actual, &actual_parent, expected, expected_parent, id);
    }

    let style = fixture.computed_style(&output, "m-physical");
    assert_eq!(
        style.layout_spacing.margin.top,
        ComputedCssSpacingValue::Percentage(0.1)
    );
    assert_eq!(
        style.layout_spacing.margin.bottom,
        ComputedCssSpacingValue::Percentage(0.1)
    );
    let style = fixture.computed_style(&output, "m-negative");
    assert_eq!(
        style.layout_spacing.margin.left,
        ComputedCssSpacingValue::Percentage(-0.05)
    );
    let style = fixture.computed_style(&output, "p-physical");
    assert_eq!(
        style.layout_spacing.padding.top,
        ComputedCssSpacingValue::Percentage(0.1)
    );
    let style = fixture.computed_style(&output, "g-row");
    assert_eq!(
        style.layout_spacing.column_gap,
        ComputedCssSpacingValue::Percentage(0.1)
    );
    assert_eq!(
        style.layout_spacing.row_gap,
        ComputedCssSpacingValue::Percentage(0.25)
    );
}

#[test]
fn cyclic_gap_root_spacing_and_unsupported_margin_fail_with_context() {
    let mut fixture = RuntimeSpacingFixture::new();
    fixture.set_style(
        "g-stretch-parent",
        "display:flex;align-items:center;width:200px;height:120px",
    );
    fixture.set_style(
        "g-stretched-column",
        "display:flex;flex-direction:column;width:100px;height:auto;row-gap:10%",
    );
    assert!(matches!(
        fixture.compute(),
        Err(StyleLayoutError::Layout(
            LayoutError::IndefinitePercentageBasis {
                property: "row-gap",
                axis: "height",
                ..
            }
        ))
    ));

    let mut fixture = RuntimeSpacingFixture::new();
    fixture.set_style("root", "display:block;width:100%;height:100%;padding:10%");
    assert!(matches!(
        fixture.compute(),
        Err(StyleLayoutError::Layout(
            LayoutError::IndefinitePercentageBasis {
                property: "padding-top",
                axis: "containing block width",
                ..
            }
        ))
    ));

    let mut fixture = RuntimeSpacingFixture::new();
    fixture.set_style("m-physical", "width:20px;height:20px;margin-left:auto");
    assert!(matches!(
        fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "margin-left",
            ..
        })
    ));
}

#[test]
fn block_auto_width_and_flex_stretch_prove_definite_gap_bases() {
    let fixture = RuntimeSpacingFixture::new();
    let output = fixture.compute().unwrap();
    let root = output.layout.frames[&fixture.nodes["root"].id()];
    let auto_first = output.layout.frames[&fixture.nodes["g-auto-width-row-a"].id()];
    let auto_second = output.layout.frames[&fixture.nodes["g-auto-width-row-b"].id()];
    assert_eq!(root.width, 320.0);
    assert_eq!(auto_second.x - auto_first.x, 52.0);

    let stretched_first = output.layout.frames[&fixture.nodes["g-stretched-column-a"].id()];
    let stretched_second = output.layout.frames[&fixture.nodes["g-stretched-column-b"].id()];
    assert_eq!(stretched_second.y - stretched_first.y, 22.0);
}

#[test]
fn custom_property_spacing_keeps_the_typed_percentage_value() {
    let mut fixture = RuntimeSpacingFixture::new();
    fixture.set_style(
        "m-physical",
        "width:20px;height:20px;--space:12.5%;margin:var(--space)",
    );
    let output = fixture.compute_custom_properties().unwrap();
    let margin = fixture
        .computed_style(&output, "m-physical")
        .layout_spacing
        .margin;
    assert_eq!(margin.top, ComputedCssSpacingValue::Percentage(0.125));
    assert_eq!(margin.left, ComputedCssSpacingValue::Percentage(0.125));
    let child = output.layout.frames[&fixture.nodes["m-physical"].id()];
    let parent = output.layout.frames[&fixture.nodes["margin-cb"].id()];
    assert_eq!(child.x - parent.x, 25.0);
    assert_eq!(child.y - parent.y, 25.0);
}
