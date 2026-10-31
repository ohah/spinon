use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy)]
struct Case {
    scene_id: &'static str,
    source_value: &'static str,
    computed_value: &'static str,
    text: &'static str,
    white_space_collapse: &'static str,
    line_count: usize,
    expected_track: f32,
    expected_used_line_height: f32,
}

const CASES: [Case; 16] = [
    Case {
        scene_id: "line-height-normal-two-lines",
        source_value: "normal",
        computed_value: "normal",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 20.0,
        expected_used_line_height: 10.0,
    },
    Case {
        scene_id: "line-height-zero-number",
        source_value: "0",
        computed_value: "0px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 0.0,
        expected_used_line_height: 0.0,
    },
    Case {
        scene_id: "line-height-half-number",
        source_value: "0.5",
        computed_value: "5px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 10.0,
        expected_used_line_height: 5.0,
    },
    Case {
        scene_id: "line-height-one-number",
        source_value: "1",
        computed_value: "10px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 20.0,
        expected_used_line_height: 10.0,
    },
    Case {
        scene_id: "line-height-one-point-five-number",
        source_value: "1.5",
        computed_value: "15px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 30.0,
        expected_used_line_height: 15.0,
    },
    Case {
        scene_id: "line-height-two-number",
        source_value: "2",
        computed_value: "20px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 40.0,
        expected_used_line_height: 20.0,
    },
    Case {
        scene_id: "line-height-zero-length",
        source_value: "0px",
        computed_value: "0px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 0.0,
        expected_used_line_height: 0.0,
    },
    Case {
        scene_id: "line-height-five-length",
        source_value: "5px",
        computed_value: "5px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 10.0,
        expected_used_line_height: 5.0,
    },
    Case {
        scene_id: "line-height-twelve-point-five-length",
        source_value: "12.5px",
        computed_value: "12.5px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 25.0,
        expected_used_line_height: 12.5,
    },
    Case {
        scene_id: "line-height-one-hundred-twenty-five-percent",
        source_value: "125%",
        computed_value: "12.5px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 25.0,
        expected_used_line_height: 12.5,
    },
    Case {
        scene_id: "line-height-one-point-two-five-em",
        source_value: "1.25em",
        computed_value: "12.5px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 25.0,
        expected_used_line_height: 12.5,
    },
    Case {
        scene_id: "line-height-calc-length",
        source_value: "calc(10px + 2.5px)",
        computed_value: "12.5px",
        text: "XX\nXX",
        white_space_collapse: "preserve-breaks",
        line_count: 2,
        expected_track: 25.0,
        expected_used_line_height: 12.5,
    },
    Case {
        scene_id: "line-height-normal-single-line",
        source_value: "normal",
        computed_value: "normal",
        text: "XX",
        white_space_collapse: "preserve-breaks",
        line_count: 1,
        expected_track: 10.0,
        expected_used_line_height: 10.0,
    },
    Case {
        scene_id: "line-height-half-number-single-line",
        source_value: "0.5",
        computed_value: "5px",
        text: "XX",
        white_space_collapse: "preserve-breaks",
        line_count: 1,
        expected_track: 5.0,
        expected_used_line_height: 5.0,
    },
    Case {
        scene_id: "line-height-one-point-five-soft-wrap",
        source_value: "1.5",
        computed_value: "15px",
        text: "XX XX",
        white_space_collapse: "collapse",
        line_count: 2,
        expected_track: 30.0,
        expected_used_line_height: 15.0,
    },
    Case {
        scene_id: "line-height-zero-number-soft-wrap",
        source_value: "0",
        computed_value: "0px",
        text: "XX XX",
        white_space_collapse: "collapse",
        line_count: 2,
        expected_track: 0.0,
        expected_used_line_height: 0.0,
    },
];

#[test]
fn anonymous_grid_computed_line_height_matrix_reaches_text_and_auto_row_sizing() {
    for (index, case) in CASES.iter().enumerate() {
        let (snapshot, root, text_node) = snapshot_for(case, 11_700 + index as u64);
        let request = WorkRequest {
            key: key_for(&snapshot, EnvironmentRevision::INITIAL),
            snapshot: snapshot.clone(),
            viewport: CssViewport {
                width_css_px: 20.0,
                height_css_px: 100.0,
                device_scale_factor: 1.0,
                environment_revision: EnvironmentRevision::INITIAL,
                media_environment: CssMediaEnvironment::MOBILE,
            },
            previous_snapshot: None,
            force_full: true,
            invalidation: None,
            previous_styles: None,
        };

        let calculation = compute_request_for_block_flex_grid_track_sizing(&request)
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.scene_id));
        let computed_style = calculation
            .roots
            .iter()
            .flat_map(|root| root.styles.elements.iter())
            .find(|style| style.node_id == root.id())
            .expect("the root's computed style is part of this calculation");
        assert_eq!(
            computed_style
                .properties
                .get("line-height")
                .map(String::as_str),
            Some(case.computed_value),
            "{} computed value",
            case.scene_id
        );
        assert_eq!(
            calculation.roots[0].styles.document_revision,
            snapshot.document_revision(),
            "{} style snapshot revision",
            case.scene_id
        );

        let layout = calculation
            .layout
            .unwrap_or_else(|error| panic!("{} layout: {error:?}", case.scene_id));
        let frame = |node: HostNodeHandle| {
            layout
                .frames
                .iter()
                .find(|frame| frame.node_id == node.id().get())
                .expect("node frame")
        };
        assert_eq!(
            (frame(root).width, frame(root).height),
            (20.0, case.expected_track),
            "{} root frame",
            case.scene_id
        );

        let diagnostics = layout
            .grid_track_sizing_diagnostics
            .as_ref()
            .expect("opt-in C11.4 diagnostics");
        assert!(matches!(
            diagnostics.revision.source(),
            spinon_layout::LayoutSourceRevision::HostDocument { document, .. }
                if document == snapshot.document_revision()
        ));
        let item = diagnostics
            .item_inputs
            .iter()
            .find(|input| input.node == text_node.id())
            .expect("anonymous text item intrinsic input");
        assert_eq!(
            item.grid_container,
            root.id(),
            "{} item owner",
            case.scene_id
        );
        assert_eq!(
            item.revision, diagnostics.revision,
            "{} revision",
            case.scene_id
        );
        let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
            panic!("{} item did not use text intrinsic sizes", case.scene_id);
        };
        assert_eq!(
            sizes.min_content_height(),
            case.expected_track,
            "{} min block",
            case.scene_id
        );
        let expected_max_content_height = if case.scene_id == "line-height-one-point-five-soft-wrap"
        {
            case.expected_used_line_height
        } else {
            case.expected_track
        };
        assert_eq!(
            sizes.max_content_height(),
            expected_max_content_height,
            "{} max block",
            case.scene_id
        );

        let text_layout = diagnostics
            .item_text_layouts
            .iter()
            .find(|text| text.input.node == text_node.id())
            .expect("anonymous text layout");
        assert_eq!(
            text_layout.layout.text, case.text,
            "{} normalized text",
            case.scene_id
        );
        assert_eq!(
            text_layout.layout.lines.len(),
            case.line_count,
            "{} line count",
            case.scene_id
        );
        assert!(
            text_layout.layout.lines.iter().all(|line| {
                line.line_height == case.expected_used_line_height
                    && line.line_top.is_finite()
                    && line.baseline.is_finite()
                    && line.advance.is_finite()
            }),
            "{} line metrics: {:?}",
            case.scene_id,
            text_layout.layout.lines
        );
        assert_eq!(
            text_layout.layout.height, case.expected_track,
            "{} text layout height",
            case.scene_id
        );

        let trace = diagnostics
            .traces
            .iter()
            .find(|trace| trace.node == root.id())
            .expect("Grid sizing trace");
        let row_pass = trace
            .passes
            .iter()
            .rfind(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .expect("row sizing pass");
        let row_sizes = row_pass
            .stages
            .last()
            .expect("final row sizing stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(
            row_sizes,
            vec![case.expected_track],
            "{} final row track",
            case.scene_id
        );
        assert!(
            row_pass.contributions.iter().any(|contribution| {
                contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent
                    && (contribution.value - case.expected_track).abs() <= 1.0 / 64.0
            }),
            "{} missing expected max-content row contribution: {:?}",
            case.scene_id,
            row_pass.contributions
        );
    }
}

#[test]
fn anonymous_grid_computed_line_height_mismatch_fails_before_layout_publication() {
    let mut case = CASES[0];
    case.source_value = "0";
    let (snapshot, _, _) = snapshot_for(&case, 11_799);
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot: snapshot.clone(),
        viewport: CssViewport {
            width_css_px: 20.0,
            height_css_px: 100.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };

    let calculation = compute_request_for_block_flex_grid_track_sizing(&request)
        .unwrap_or_else(|error| panic!("runtime calculation: {error:?}"));
    let Err(error) = calculation.layout else {
        panic!("a mismatched computed line-height published layout output");
    };
    let error = format!("{error:?}");
    assert!(
        error.contains("unsupported_c114_anonymous_text_style"),
        "{error}"
    );
    assert!(error.contains("line-height"), "{error}");
}

fn snapshot_for(
    case: &Case,
    owner_id: u64,
) -> (Arc<HostDocumentSnapshot>, HostNodeHandle, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(owner_id).unwrap(),
        document.document_revision(),
    );
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    set_attribute(
        &mut batch,
        root,
        "id",
        &format!("c114-{}-grid", case.scene_id),
    );
    set_attribute(
        &mut batch,
        root,
        "style",
        &format!(
            "box-sizing:border-box;display:grid;width:20px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:20px;grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:{};tab-size:8;white-space-collapse:{};text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px",
            case.source_value, case.white_space_collapse
        ),
    );
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: case.text.into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(root),
        node: text,
        before: None,
    });
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), root, text)
}

fn set_attribute(batch: &mut DocumentChangeBatch, node: HostNodeHandle, name: &str, value: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, name).unwrap(),
        value: value.into(),
    });
}
