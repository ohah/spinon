use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
};
use spinon_text_layout::OverflowWrapMode;

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TrackKind {
    MinContent,
    MaxContent,
    Fixed10Px,
}

#[derive(Clone, Copy)]
struct Case {
    scene_id: &'static str,
    text: &'static str,
    computed_overflow_wrap: &'static str,
    overflow_wrap: OverflowWrapMode,
    track: TrackKind,
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
    fixed_line_count: usize,
}

const CASES: [Case; 9] = [
    case(OverflowWrapMode::Normal, TrackKind::MinContent),
    case(OverflowWrapMode::Normal, TrackKind::MaxContent),
    case(OverflowWrapMode::Normal, TrackKind::Fixed10Px),
    case(OverflowWrapMode::Anywhere, TrackKind::MinContent),
    case(OverflowWrapMode::Anywhere, TrackKind::MaxContent),
    case(OverflowWrapMode::Anywhere, TrackKind::Fixed10Px),
    case(OverflowWrapMode::BreakWord, TrackKind::MinContent),
    case(OverflowWrapMode::BreakWord, TrackKind::MaxContent),
    case(OverflowWrapMode::BreakWord, TrackKind::Fixed10Px),
];

const fn case(overflow_wrap: OverflowWrapMode, track: TrackKind) -> Case {
    Case {
        scene_id: match (overflow_wrap, track) {
            (OverflowWrapMode::Normal, TrackKind::MinContent) => "overflow-wrap-normal-min-content",
            (OverflowWrapMode::Normal, TrackKind::MaxContent) => "overflow-wrap-normal-max-content",
            (OverflowWrapMode::Normal, TrackKind::Fixed10Px) => "overflow-wrap-normal-fixed-10px",
            (OverflowWrapMode::Anywhere, TrackKind::MinContent) => {
                "overflow-wrap-anywhere-min-content"
            }
            (OverflowWrapMode::Anywhere, TrackKind::MaxContent) => {
                "overflow-wrap-anywhere-max-content"
            }
            (OverflowWrapMode::Anywhere, TrackKind::Fixed10Px) => {
                "overflow-wrap-anywhere-fixed-10px"
            }
            (OverflowWrapMode::BreakWord, TrackKind::MinContent) => {
                "overflow-wrap-break-word-min-content"
            }
            (OverflowWrapMode::BreakWord, TrackKind::MaxContent) => {
                "overflow-wrap-break-word-max-content"
            }
            (OverflowWrapMode::BreakWord, TrackKind::Fixed10Px) => {
                "overflow-wrap-break-word-fixed-10px"
            }
        },
        text: "XXXX",
        computed_overflow_wrap: match overflow_wrap {
            OverflowWrapMode::Normal => "normal",
            OverflowWrapMode::Anywhere => "anywhere",
            OverflowWrapMode::BreakWord => "break-word",
        },
        overflow_wrap,
        track,
        min_content: match overflow_wrap {
            OverflowWrapMode::Anywhere => 10.0,
            OverflowWrapMode::Normal | OverflowWrapMode::BreakWord => 40.0,
        },
        max_content: 40.0,
        min_content_height: match overflow_wrap {
            OverflowWrapMode::Anywhere => 40.0,
            OverflowWrapMode::Normal | OverflowWrapMode::BreakWord => 10.0,
        },
        max_content_height: 10.0,
        fixed_line_count: match (overflow_wrap, track) {
            (OverflowWrapMode::Anywhere, TrackKind::MinContent | TrackKind::Fixed10Px)
            | (OverflowWrapMode::BreakWord, TrackKind::Fixed10Px) => 4,
            _ => 1,
        },
    }
}

#[test]
fn anonymous_grid_overflow_wrap_reaches_style_paragraph_intrinsics_and_tracks() {
    assert_eq!(CASES.len(), 9);
    assert_eq!(
        CASES
            .iter()
            .map(|case| case.scene_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        CASES.len(),
    );

    for (index, case) in CASES.iter().enumerate() {
        let (snapshot, root, text_node) =
            snapshot_for(case, 13_500 + index as u64, case.computed_overflow_wrap);
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
                .get("overflow-wrap")
                .map(String::as_str),
            Some(case.computed_overflow_wrap),
            "{} computed overflow-wrap",
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
        let expected_track = match case.track {
            TrackKind::MinContent => case.min_content,
            TrackKind::MaxContent => case.max_content,
            TrackKind::Fixed10Px => 10.0,
        };
        let expected_height = match case.track {
            TrackKind::MinContent => case.min_content_height,
            TrackKind::MaxContent => case.max_content_height,
            TrackKind::Fixed10Px if case.overflow_wrap == OverflowWrapMode::Normal => 10.0,
            TrackKind::Fixed10Px => 40.0,
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
        let item = diagnostics
            .item_inputs
            .iter()
            .find(|input| input.node == text_node.id())
            .expect("anonymous text intrinsic input");
        let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
            panic!("{} item did not use text intrinsic sizes", case.scene_id);
        };
        assert_close(sizes.min_content(), case.min_content, case, "min-content");
        assert_close(sizes.max_content(), case.max_content, case, "max-content");
        assert_close(
            sizes.min_content_height(),
            case.min_content_height,
            case,
            "min-content height",
        );
        assert_close(
            sizes.max_content_height(),
            case.max_content_height,
            case,
            "max-content height",
        );

        let text_layout = diagnostics
            .item_text_layouts
            .iter()
            .find(|text| text.input.node == text_node.id())
            .expect("anonymous text layout");
        assert_eq!(text_layout.input.style.overflow_wrap, case.overflow_wrap);
        assert_eq!(text_layout.layout.overflow_wrap, case.overflow_wrap);
        assert_eq!(text_layout.layout.text, case.text);
        assert_eq!(
            text_layout.layout.lines.len(),
            case.fixed_line_count,
            "{} line count",
            case.scene_id
        );
        let expected_line_advance = match (case.track, case.overflow_wrap) {
            (TrackKind::Fixed10Px, OverflowWrapMode::Normal) => 40.0,
            (TrackKind::Fixed10Px, _) => 10.0,
            _ => expected_track,
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
        let tracks = column_pass
            .stages
            .last()
            .expect("final column sizing stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(tracks.len(), 1, "{} track count", case.scene_id);
        assert_close(tracks[0], expected_track, case, "final track");
        if case.track != TrackKind::Fixed10Px {
            let expected_contribution_kind = match case.track {
                TrackKind::MinContent => LayoutGridTrackSizingContributionKind::MinContent,
                TrackKind::MaxContent => LayoutGridTrackSizingContributionKind::MaxContent,
                TrackKind::Fixed10Px => unreachable!(),
            };
            assert!(
                column_pass.contributions.iter().any(|contribution| {
                    contribution.kind == expected_contribution_kind
                        && (contribution.value - expected_track).abs() <= 1.0 / 64.0
                }),
                "{} missing {expected_contribution_kind:?} contribution",
                case.scene_id
            );
        }
    }
}

#[test]
fn anonymous_grid_overflow_wrap_rejects_scene_value_mismatch() {
    let case = CASES[0];
    let (snapshot, _, _) = snapshot_for(&case, 13_599, "anywhere");
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
        .expect("style failure is reported with the layout result");
    let Err(error) = calculation.layout else {
        panic!("scene/value mismatch published layout");
    };
    let error = format!("{error:?}");
    assert!(
        error.contains("unsupported_c114_anonymous_text_style"),
        "{error}"
    );
    assert!(error.contains("overflow-wrap"), "{error}");
}

fn snapshot_for(
    case: &Case,
    owner_id: u64,
    overflow_wrap: &str,
) -> (
    std::sync::Arc<spinon_core::HostDocumentSnapshot>,
    HostNodeHandle,
    HostNodeHandle,
) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(owner_id).unwrap();
    let root = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
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
    let columns = match case.track {
        TrackKind::MinContent => "min-content",
        TrackKind::MaxContent => "max-content",
        TrackKind::Fixed10Px => "10px",
    };
    set_attribute(
        &mut batch,
        root,
        "style",
        &format!(
            "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:{overflow_wrap};text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px"
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
    (std::sync::Arc::new(document.snapshot()), root, text)
}

fn set_attribute(batch: &mut DocumentChangeBatch, node: HostNodeHandle, name: &str, value: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, name).unwrap(),
        value: value.into(),
    });
}

fn assert_close(actual: f32, expected: f32, case: &Case, field: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{} {field}: expected {expected}, got {actual}",
        case.scene_id
    );
}
