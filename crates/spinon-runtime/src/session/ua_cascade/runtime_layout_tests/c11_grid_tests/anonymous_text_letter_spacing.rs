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
    specified_value: &'static str,
    text: &'static str,
    track_kind: &'static str,
    letter_spacing: f32,
    expected_min_content: f32,
    expected_max_content: f32,
}

fn cases() -> [Case; 16] {
    [
        case(
            "normal-min-content",
            "normal",
            "XX XX",
            "min-content",
            0.0,
            20.0,
            50.0,
        ),
        case(
            "normal-max-content",
            "normal",
            "XX XX",
            "max-content",
            0.0,
            20.0,
            50.0,
        ),
        case(
            "minus-one-min-content",
            "-1px",
            "XX XX",
            "min-content",
            -1.0,
            19.0,
            46.0,
        ),
        case(
            "minus-one-max-content",
            "-1px",
            "XX XX",
            "max-content",
            -1.0,
            19.0,
            46.0,
        ),
        case(
            "zero-min-content",
            "0px",
            "XX XX",
            "min-content",
            0.0,
            20.0,
            50.0,
        ),
        case(
            "zero-max-content",
            "0px",
            "XX XX",
            "max-content",
            0.0,
            20.0,
            50.0,
        ),
        case(
            "half-min-content",
            "0.5px",
            "XX XX",
            "min-content",
            0.5,
            20.5,
            52.0,
        ),
        case(
            "half-max-content",
            "0.5px",
            "XX XX",
            "max-content",
            0.5,
            20.5,
            52.0,
        ),
        case(
            "one-min-content",
            "1px",
            "XX XX",
            "min-content",
            1.0,
            21.0,
            54.0,
        ),
        case(
            "one-max-content",
            "1px",
            "XX XX",
            "max-content",
            1.0,
            21.0,
            54.0,
        ),
        case(
            "two-min-content",
            "2px",
            "XX XX",
            "min-content",
            2.0,
            22.0,
            58.0,
        ),
        case(
            "two-max-content",
            "2px",
            "XX XX",
            "max-content",
            2.0,
            22.0,
            58.0,
        ),
        case(
            "minus-one-single-min-content",
            "-1px",
            "XX",
            "min-content",
            -1.0,
            19.0,
            19.0,
        ),
        case(
            "minus-one-single-max-content",
            "-1px",
            "XX",
            "max-content",
            -1.0,
            19.0,
            19.0,
        ),
        case(
            "one-single-min-content",
            "1px",
            "XX",
            "min-content",
            1.0,
            21.0,
            21.0,
        ),
        case(
            "one-single-max-content",
            "1px",
            "XX",
            "max-content",
            1.0,
            21.0,
            21.0,
        ),
    ]
}

fn case(
    suffix: &'static str,
    specified_value: &'static str,
    text: &'static str,
    track_kind: &'static str,
    letter_spacing: f32,
    expected_min_content: f32,
    expected_max_content: f32,
) -> Case {
    Case {
        scene_id: match suffix {
            "normal-min-content" => "anonymous-letter-spacing-normal-min-content",
            "normal-max-content" => "anonymous-letter-spacing-normal-max-content",
            "minus-one-min-content" => "anonymous-letter-spacing-minus-one-min-content",
            "minus-one-max-content" => "anonymous-letter-spacing-minus-one-max-content",
            "zero-min-content" => "anonymous-letter-spacing-zero-min-content",
            "zero-max-content" => "anonymous-letter-spacing-zero-max-content",
            "half-min-content" => "anonymous-letter-spacing-half-min-content",
            "half-max-content" => "anonymous-letter-spacing-half-max-content",
            "one-min-content" => "anonymous-letter-spacing-one-min-content",
            "one-max-content" => "anonymous-letter-spacing-one-max-content",
            "two-min-content" => "anonymous-letter-spacing-two-min-content",
            "two-max-content" => "anonymous-letter-spacing-two-max-content",
            "minus-one-single-min-content" => {
                "anonymous-letter-spacing-minus-one-single-min-content"
            }
            "minus-one-single-max-content" => {
                "anonymous-letter-spacing-minus-one-single-max-content"
            }
            "one-single-min-content" => "anonymous-letter-spacing-one-single-min-content",
            "one-single-max-content" => "anonymous-letter-spacing-one-single-max-content",
            _ => panic!("unknown letter-spacing case suffix"),
        },
        specified_value,
        text,
        track_kind,
        letter_spacing,
        expected_min_content,
        expected_max_content,
    }
}

fn expected_computed_value(specified_value: &'static str) -> &'static str {
    if specified_value == "0px" {
        "normal"
    } else {
        specified_value
    }
}

#[test]
fn anonymous_grid_computed_letter_spacing_reaches_intrinsic_tracks_and_text_layout() {
    let cases = cases();
    for (index, case) in cases.iter().enumerate() {
        let (snapshot, root, text_node) = snapshot_for(case, 11_800 + index as u64);
        let request = WorkRequest {
            key: key_for(&snapshot, EnvironmentRevision::INITIAL),
            snapshot: snapshot.clone(),
            viewport: CssViewport {
                width_css_px: 120.0,
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
            .expect("root computed style belongs to the calculation");
        assert_eq!(
            computed_style
                .properties
                .get("letter-spacing")
                .map(String::as_str),
            Some(expected_computed_value(case.specified_value)),
            "{} computed letter-spacing",
            case.scene_id
        );
        assert_eq!(
            calculation.roots[0].styles.document_revision,
            snapshot.document_revision(),
            "{} computed style revision",
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
        let expected_height = if case.text == "XX XX" && case.track_kind == "min-content" {
            20.0
        } else {
            10.0
        };
        assert_eq!(
            (frame(root).width, frame(root).height),
            (120.0, expected_height),
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
            .expect("anonymous text intrinsic input");
        let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
            panic!("{} item did not use text intrinsic sizes", case.scene_id);
        };
        assert_eq!(
            sizes.min_content(),
            case.expected_min_content,
            "{} min",
            case.scene_id
        );
        assert_eq!(
            sizes.max_content(),
            case.expected_max_content,
            "{} max",
            case.scene_id
        );

        let text_layout = diagnostics
            .item_text_layouts
            .iter()
            .find(|text| text.input.node == text_node.id())
            .expect("anonymous text layout");
        assert_eq!(text_layout.input.style.letter_spacing, case.letter_spacing);
        assert_eq!(text_layout.layout.text, case.text);
        let expected_line_count = if case.text == "XX XX" && case.track_kind == "min-content" {
            2
        } else {
            1
        };
        assert_eq!(text_layout.layout.lines.len(), expected_line_count);
        let expected_line_advance = if case.track_kind == "min-content" {
            case.expected_min_content
        } else {
            case.expected_max_content
        };
        assert!(
            text_layout.layout.lines.iter().all(|line| {
                (line.advance - expected_line_advance).abs() <= 1.0 / 64.0
                    && line.line_top.is_finite()
                    && line.baseline.is_finite()
            }),
            "{} line metrics: {:?}",
            case.scene_id,
            text_layout.layout.lines
        );

        let trace = diagnostics
            .traces
            .iter()
            .find(|trace| trace.node == root.id())
            .expect("Grid sizing trace");
        let column_pass = trace
            .passes
            .iter()
            .rfind(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .expect("column sizing pass");
        let track_sizes = column_pass
            .stages
            .last()
            .expect("final column sizing stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(
            track_sizes,
            vec![expected_line_advance],
            "{} track",
            case.scene_id
        );
        let (expected_contribution_kind, expected_contribution) =
            if case.track_kind == "min-content" {
                (
                    LayoutGridTrackSizingContributionKind::MinContent,
                    case.expected_min_content,
                )
            } else {
                (
                    LayoutGridTrackSizingContributionKind::MaxContent,
                    case.expected_max_content,
                )
            };
        assert!(
            column_pass.contributions.iter().any(|contribution| {
                contribution.kind == expected_contribution_kind
                    && (contribution.value - expected_contribution).abs() <= 1.0 / 64.0
            }),
            "{} missing {expected_contribution_kind:?} contribution",
            case.scene_id
        );
    }
}

#[test]
fn anonymous_grid_letter_spacing_mismatch_fails_before_layout_publication() {
    let mut case = cases()[2];
    case.specified_value = "2px";
    let (snapshot, _, _) = snapshot_for(&case, 11_899);
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 120.0,
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
        panic!("a mismatched computed letter-spacing published layout output");
    };
    let error = format!("{error:?}");
    assert!(
        error.contains("unsupported_c114_anonymous_text_style"),
        "{error}"
    );
    assert!(error.contains("letter-spacing"), "{error}");
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
            "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;tab-size:8;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:{};word-spacing:0px;text-indent:0px",
            case.track_kind, case.specified_value
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
