use spinon_style::{ComputedCssDimension, ComputedStyleProfile};

use super::cases::VIEWPORT;
use super::test_fixture::{NodeSpec, cascade, custom_fixture, item, take_error};

const CSS: &str = r#"
.scene {
    display:grid;
    box-sizing:border-box;
    width:100px;
    height:auto;
    grid-template-columns:max-content max-content;
    grid-template-rows:auto;
    grid-auto-flow:row;
    column-gap:0;
    row-gap:0;
    align-items:normal;
    align-content:normal;
    justify-content:normal;
    justify-items:normal;
    direction:ltr;
}
.item {
    display:block;
    position:static;
    box-sizing:content-box;
    width:auto;
    height:auto;
    min-width:auto;
    max-width:none;
    min-height:auto;
    max-height:none;
    margin:0;
    border:0;
    padding:0;
    align-self:auto;
    justify-self:auto;
    font-family:Ahem;
    font-size:10px;
    font-weight:400;
    font-style:normal;
    line-height:10px;
    white-space-collapse:collapse;
    text-wrap-mode:nowrap;
    word-break:normal;
    overflow-wrap:normal;
    writing-mode:horizontal-tb;
}
.width-definite { width:60px; }
.height-definite { height:30px; }
.ratio { aspect-ratio:2 / 1; }
"#;

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    long_classes: &'static str,
    columns: [f32; 2],
    row: f32,
    long_frame: [f32; 2],
    ratio_owner: bool,
}

const SCENES: [Scene; 4] = [
    Scene {
        id: "width-ratio",
        long_classes: "item width-definite ratio",
        columns: [10.0, 60.0],
        row: 30.0,
        long_frame: [60.0, 30.0],
        ratio_owner: true,
    },
    Scene {
        id: "width-control",
        long_classes: "item width-definite",
        columns: [10.0, 60.0],
        row: 10.0,
        long_frame: [60.0, 10.0],
        ratio_owner: false,
    },
    Scene {
        id: "height-ratio",
        long_classes: "item height-definite ratio",
        columns: [10.0, 60.0],
        row: 30.0,
        long_frame: [60.0, 30.0],
        ratio_owner: true,
    },
    Scene {
        id: "height-control",
        long_classes: "item height-definite",
        columns: [10.0, 40.0],
        row: 30.0,
        long_frame: [40.0, 30.0],
        ratio_owner: false,
    },
];

#[test]
fn definite_axis_ratio_scenes_match_the_fixed_chrome_matrix() {
    for scene in SCENES {
        let fixture = custom_fixture(
            scene.id,
            "scene",
            vec![
                item("short", "item", vec![NodeSpec::text("A")]),
                item("long", scene.long_classes, vec![NodeSpec::text("AAAA")]),
            ],
            None,
        );
        let computed = cascade(&fixture.view, CSS);
        assert_eq!(
            computed.profile,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
            "{}",
            scene.id
        );
        let long_style = computed
            .elements
            .iter()
            .find(|style| style.node_id == fixture.items[1].id())
            .unwrap_or_else(|| panic!("{}: missing AAAA computed style", scene.id));
        assert_eq!(
            long_style.layout_dimensions.width,
            if scene.long_classes.contains("width-definite") {
                ComputedCssDimension::LengthPx(60.0)
            } else {
                ComputedCssDimension::Auto
            },
            "{}",
            scene.id
        );
        assert_eq!(
            long_style.layout_dimensions.height,
            if scene.long_classes.contains("height-definite") {
                ComputedCssDimension::LengthPx(30.0)
            } else {
                ComputedCssDimension::Auto
            },
            "{}",
            scene.id
        );

        let input = super::super::prepare(&fixture.snapshot, fixture.root, &computed)
            .unwrap_or_else(|error| panic!("{}: {error:?}", scene.id))
            .unwrap_or_else(|| panic!("{}: definite-axis profile fell back", scene.id));
        assert_eq!(input.paragraph_styles.len(), 2, "{}", scene.id);
        assert_eq!(
            input.definite_axis_ratio_node,
            scene.ratio_owner.then_some(fixture.items[1].id()),
            "{}",
            scene.id
        );

        let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
            &fixture.snapshot,
            fixture.root,
            computed,
            VIEWPORT,
            &input.paragraph_styles,
            &input.engine,
        )
        .unwrap_or_else(|error| panic!("{}: {error:?}", scene.id));

        let root_frame = output.layout.frames[&fixture.root.id()];
        assert_eq!([root_frame.width, root_frame.height], [100.0, scene.row]);
        assert_eq!(
            output.layout.grid_item_intrinsic_inputs.len(),
            2,
            "{}",
            scene.id
        );
        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .unwrap_or_else(|| panic!("{}: missing track sizing trace", scene.id));
        assert_eq!(
            trace.used_columns.used_track_list,
            format!("{}px {}px", scene.columns[0], scene.columns[1]),
            "{}",
            scene.id
        );
        assert_eq!(
            trace.used_rows.used_track_list,
            format!("{}px", scene.row),
            "{}",
            scene.id
        );
        assert_eq!(trace.passes.len(), 2, "{}", scene.id);
        for (index, (item, expected_width, expected_height)) in [
            (fixture.items[0], 10.0, scene.row),
            (fixture.items[1], scene.long_frame[0], scene.long_frame[1]),
        ]
        .into_iter()
        .enumerate()
        {
            let frame = output.layout.frames[&item.id()];
            assert_eq!(frame.x, if index == 0 { 0.0 } else { 10.0 }, "{}", scene.id);
            assert_eq!(frame.y, 0.0, "{}", scene.id);
            assert_eq!(frame.width, expected_width, "{}", scene.id);
            assert_eq!(frame.height, expected_height, "{}", scene.id);
            let text = &output.layout.text_layouts[&item.id()].layout;
            assert_eq!([text.width, text.height], [expected_width.min(40.0), 10.0]);
        }
    }
}

#[test]
fn definite_axis_ratio_profile_rejects_wrong_owner_mismatch_and_extra_items() {
    let wrong_owner = custom_fixture(
        "wrong-owner",
        "scene",
        vec![
            item("short", "item ratio", vec![NodeSpec::text("A")]),
            item("long", "item width-definite", vec![NodeSpec::text("AAAA")]),
        ],
        None,
    );
    let computed = cascade(&wrong_owner.view, CSS);
    let error = take_error(
        super::super::prepare(&wrong_owner.snapshot, wrong_owner.root, &computed),
        "ratio on A",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");
    assert_eq!(error.node_id, Some(wrong_owner.items[0].id().get()));

    let mismatched = custom_fixture(
        "mismatched-ratio",
        "scene",
        vec![
            item("short", "item", vec![NodeSpec::text("A")]),
            item(
                "long",
                "item width-definite ratio",
                vec![NodeSpec::text("AAAA")],
            ),
        ],
        None,
    );
    let computed = cascade(
        &mismatched.view,
        &format!("{CSS}\n.ratio {{ aspect-ratio:3 / 2; }}"),
    );
    let error = take_error(
        super::super::prepare(&mismatched.snapshot, mismatched.root, &computed),
        "numeric ratio differs from the fixed 2 / 1 scene",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");
    assert_eq!(error.node_id, Some(mismatched.items[1].id().get()));

    let extra = custom_fixture(
        "extra-empty-item",
        "scene",
        vec![
            item("short", "item", vec![NodeSpec::text("A")]),
            item(
                "long",
                "item width-definite ratio",
                vec![NodeSpec::text("AAAA")],
            ),
            item("empty", "item", Vec::new()),
        ],
        None,
    );
    let computed = cascade(&extra.view, CSS);
    let error = take_error(
        super::super::prepare(&extra.snapshot, extra.root, &computed),
        "extra empty in-flow grid item",
    );
    assert_eq!(error.code, "unsupported_c114_text_source");
}

#[test]
fn definite_axis_profile_leaves_automatic_dimension_ratio_fixtures_on_the_existing_path() {
    let fixture = custom_fixture(
        "existing-auto-ratio",
        "scene",
        vec![
            item("short", "item", vec![NodeSpec::text("A")]),
            item("long", "item ratio", vec![NodeSpec::text("AAAA")]),
        ],
        None,
    );
    let computed = cascade(&fixture.view, CSS);
    let input = super::super::prepare(&fixture.snapshot, fixture.root, &computed)
        .expect("existing automatic ratio route remains valid")
        .expect("existing direct-text ratio profile matches");
    assert_eq!(input.definite_axis_ratio_node, None);
    assert_eq!(input.paragraph_styles.len(), 2);
}

#[test]
fn definite_axis_profile_rejects_anonymous_text_and_stale_style_revision() {
    let anonymous = custom_fixture(
        "anonymous-text",
        "scene",
        vec![
            item("short", "item", vec![NodeSpec::text("A")]),
            item("long", "item width-definite", vec![NodeSpec::text("AAAA")]),
            NodeSpec::text("anonymous"),
        ],
        None,
    );
    let computed = cascade(&anonymous.view, CSS);
    let error = take_error(
        super::super::prepare(&anonymous.snapshot, anonymous.root, &computed),
        "anonymous direct Grid text",
    );
    assert_eq!(error.code, "unsupported_c114_text_source");

    let stale = custom_fixture(
        "stale-style",
        "scene",
        vec![
            item("short", "item", vec![NodeSpec::text("A")]),
            item(
                "long",
                "item width-definite ratio",
                vec![NodeSpec::text("AAAA")],
            ),
        ],
        None,
    );
    let other = custom_fixture(
        "other-snapshot",
        "scene",
        vec![
            item("short", "item", vec![NodeSpec::text("A")]),
            item(
                "long",
                "item width-definite ratio",
                vec![NodeSpec::text("AAAA")],
            ),
        ],
        None,
    );
    let computed = cascade(&other.view, CSS);
    let error = take_error(
        super::super::prepare(&stale.snapshot, stale.root, &computed),
        "computed style snapshot belongs to another document",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");
}
