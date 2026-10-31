use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone)]
struct Case {
    scene_id: String,
    specified_value: &'static str,
    computed_value: &'static str,
    text: &'static str,
    is_min_content: bool,
    text_indent: f32,
    expected_min_content: f32,
    expected_max_content: f32,
    expected_min_height: f32,
    unindented_line_advance: f32,
}

fn cases() -> Vec<Case> {
    let values = [
        ("zero", "0px", 0.0),
        ("five", "5px", 5.0),
        ("ten", "10px", 10.0),
    ];
    let text_cases = [
        ("two-words", "XX XX"),
        ("collapsed-three-words", "XX   XX XX  "),
        ("single-word", "XX"),
    ];
    let tracks = [("min-content", true), ("max-content", false)];
    let mut cases = Vec::with_capacity(values.len() * text_cases.len() * tracks.len());
    for (value_name, computed_value, text_indent) in values {
        for (text_name, text) in text_cases {
            let words = text.split_ascii_whitespace().collect::<Vec<_>>();
            let x_count = text.chars().filter(|character| *character == 'X').count();
            let unindented_line_advance =
                10.0 * x_count as f32 + 10.0 * words.len().saturating_sub(1) as f32;
            let expected_min_content = 10.0
                * words
                    .iter()
                    .map(|word| word.len())
                    .max()
                    .unwrap_or_default() as f32
                + text_indent;
            let expected_max_content = unindented_line_advance + text_indent;
            for (track_name, is_min_content) in tracks {
                cases.push(Case {
                    scene_id: format!("text-indent-{value_name}-{text_name}-{track_name}"),
                    specified_value: computed_value,
                    computed_value,
                    text,
                    is_min_content,
                    text_indent,
                    expected_min_content,
                    expected_max_content,
                    expected_min_height: 10.0 * words.len() as f32,
                    unindented_line_advance: if is_min_content {
                        expected_min_content - text_indent
                    } else {
                        unindented_line_advance
                    },
                });
            }
        }
    }
    cases
}

#[test]
fn anonymous_grid_computed_text_indent_reaches_intrinsic_tracks_and_first_line_position() {
    let cases = cases();
    assert_eq!(cases.len(), 18);
    assert_eq!(
        cases
            .iter()
            .map(|case| case.scene_id.as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        cases.len(),
    );

    for (index, case) in cases.iter().enumerate() {
        let (snapshot, root, text_node) = snapshot_for(case, 12_300 + index as u64);
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
                .get("text-indent")
                .map(String::as_str),
            Some(case.computed_value),
            "{} computed text-indent",
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
        let expected_track = if case.is_min_content {
            case.expected_min_content
        } else {
            case.expected_max_content
        };
        let expected_height = if case.is_min_content {
            case.expected_min_height
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
            "{} min-content",
            case.scene_id
        );
        assert_eq!(
            sizes.max_content(),
            case.expected_max_content,
            "{} max-content",
            case.scene_id
        );
        assert_eq!(
            sizes.min_content_height(),
            case.expected_min_height,
            "{} min-content height",
            case.scene_id
        );
        assert_eq!(
            sizes.max_content_height(),
            10.0,
            "{} max height",
            case.scene_id
        );

        let text_layout = diagnostics
            .item_text_layouts
            .iter()
            .find(|text| text.input.node == text_node.id())
            .expect("anonymous text layout");
        assert_eq!(text_layout.input.style.text_indent, case.text_indent);
        assert_eq!(
            text_layout.layout.text,
            case.text
                .split_ascii_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        );
        let expected_line_count = if case.is_min_content {
            case.text.split_ascii_whitespace().count()
        } else {
            1
        };
        assert_eq!(
            text_layout.layout.lines.len(),
            expected_line_count,
            "{} line count",
            case.scene_id
        );
        assert!(
            text_layout.layout.lines.iter().all(|line| {
                (line.advance - case.unindented_line_advance).abs() <= 1.0 / 64.0
                    && line.line_top.is_finite()
                    && line.baseline.is_finite()
            }),
            "{} shaped line metrics: {:?}",
            case.scene_id,
            text_layout.layout.lines
        );
        assert_eq!(
            text_layout.layout.runs[0].glyphs[0].x, case.text_indent,
            "{} first formatted line position",
            case.scene_id
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
        assert_eq!(track_sizes, vec![expected_track], "{} track", case.scene_id);
        let expected_contribution_kind = if case.is_min_content {
            LayoutGridTrackSizingContributionKind::MinContent
        } else {
            LayoutGridTrackSizingContributionKind::MaxContent
        };
        let expected_contribution = if case.is_min_content {
            case.expected_min_content
        } else {
            case.expected_max_content
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
fn anonymous_grid_text_indent_out_of_profile_and_mismatch_fail_before_layout_publication() {
    for (index, specified_value) in ["-5px", "5%", "10px"].into_iter().enumerate() {
        let mut case = cases()[6].clone();
        case.specified_value = specified_value;
        let (snapshot, _, _) = snapshot_for(&case, 12_399 + index as u64);
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
            panic!("out-of-profile or mismatched text-indent published layout output");
        };
        let error = format!("{error:?}");
        assert!(
            error.contains("unsupported_c114_anonymous_text_style"),
            "{error}"
        );
        assert!(error.contains("text-indent"), "{error}");
    }
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
            "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;tab-size:8;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:{}",
            if case.is_min_content {
                "min-content"
            } else {
                "max-content"
            },
            case.specified_value
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
