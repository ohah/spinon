use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
};
use spinon_text_layout::{OverflowWrapMode, WordBreakMode};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const XML: &str = "http://www.w3.org/XML/1998/namespace";

#[derive(Clone, Copy, Default)]
struct SnapshotOptions<'a> {
    language: Option<&'a str>,
    xml_language: Option<&'a str>,
    track_override: Option<&'a str>,
    duplicate_text: bool,
}

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
    word_break_css: &'static str,
    word_break: WordBreakMode,
    overflow_wrap_css: &'static str,
    overflow_wrap: OverflowWrapMode,
    track: TrackKind,
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
    fixed_line_count: usize,
}

const CASES: [Case; 18] = [
    case(
        WordBreakMode::Normal,
        OverflowWrapMode::Normal,
        TrackKind::MinContent,
    ),
    case(
        WordBreakMode::Normal,
        OverflowWrapMode::Normal,
        TrackKind::MaxContent,
    ),
    case(
        WordBreakMode::Normal,
        OverflowWrapMode::Normal,
        TrackKind::Fixed10Px,
    ),
    case(
        WordBreakMode::Normal,
        OverflowWrapMode::BreakWord,
        TrackKind::MinContent,
    ),
    case(
        WordBreakMode::Normal,
        OverflowWrapMode::BreakWord,
        TrackKind::MaxContent,
    ),
    case(
        WordBreakMode::Normal,
        OverflowWrapMode::BreakWord,
        TrackKind::Fixed10Px,
    ),
    case(
        WordBreakMode::Normal,
        OverflowWrapMode::Anywhere,
        TrackKind::MinContent,
    ),
    case(
        WordBreakMode::Normal,
        OverflowWrapMode::Anywhere,
        TrackKind::MaxContent,
    ),
    case(
        WordBreakMode::Normal,
        OverflowWrapMode::Anywhere,
        TrackKind::Fixed10Px,
    ),
    case(
        WordBreakMode::BreakWord,
        OverflowWrapMode::Normal,
        TrackKind::MinContent,
    ),
    case(
        WordBreakMode::BreakWord,
        OverflowWrapMode::Normal,
        TrackKind::MaxContent,
    ),
    case(
        WordBreakMode::BreakWord,
        OverflowWrapMode::Normal,
        TrackKind::Fixed10Px,
    ),
    case(
        WordBreakMode::BreakWord,
        OverflowWrapMode::BreakWord,
        TrackKind::MinContent,
    ),
    case(
        WordBreakMode::BreakWord,
        OverflowWrapMode::BreakWord,
        TrackKind::MaxContent,
    ),
    case(
        WordBreakMode::BreakWord,
        OverflowWrapMode::BreakWord,
        TrackKind::Fixed10Px,
    ),
    case(
        WordBreakMode::BreakWord,
        OverflowWrapMode::Anywhere,
        TrackKind::MinContent,
    ),
    case(
        WordBreakMode::BreakWord,
        OverflowWrapMode::Anywhere,
        TrackKind::MaxContent,
    ),
    case(
        WordBreakMode::BreakWord,
        OverflowWrapMode::Anywhere,
        TrackKind::Fixed10Px,
    ),
];

const fn case(
    word_break: WordBreakMode,
    overflow_wrap: OverflowWrapMode,
    track: TrackKind,
) -> Case {
    let effective_anywhere = matches!(
        (word_break, overflow_wrap),
        (WordBreakMode::BreakWord, _) | (_, OverflowWrapMode::Anywhere)
    );
    let fixed_line_count = match track {
        TrackKind::MinContent => {
            if effective_anywhere {
                4
            } else {
                1
            }
        }
        TrackKind::MaxContent => 1,
        TrackKind::Fixed10Px => match (word_break, overflow_wrap) {
            (WordBreakMode::Normal, OverflowWrapMode::Normal) => 1,
            _ => 4,
        },
    };
    Case {
        scene_id: match (word_break, overflow_wrap, track) {
            (WordBreakMode::Normal, OverflowWrapMode::Normal, TrackKind::MinContent) => {
                "word-break-normal-overflow-wrap-normal-min-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Normal, TrackKind::MaxContent) => {
                "word-break-normal-overflow-wrap-normal-max-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Normal, TrackKind::Fixed10Px) => {
                "word-break-normal-overflow-wrap-normal-10px"
            }
            (WordBreakMode::Normal, OverflowWrapMode::BreakWord, TrackKind::MinContent) => {
                "word-break-normal-overflow-wrap-break-word-min-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::BreakWord, TrackKind::MaxContent) => {
                "word-break-normal-overflow-wrap-break-word-max-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::BreakWord, TrackKind::Fixed10Px) => {
                "word-break-normal-overflow-wrap-break-word-10px"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Anywhere, TrackKind::MinContent) => {
                "word-break-normal-overflow-wrap-anywhere-min-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Anywhere, TrackKind::MaxContent) => {
                "word-break-normal-overflow-wrap-anywhere-max-content"
            }
            (WordBreakMode::Normal, OverflowWrapMode::Anywhere, TrackKind::Fixed10Px) => {
                "word-break-normal-overflow-wrap-anywhere-10px"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Normal, TrackKind::MinContent) => {
                "word-break-break-word-overflow-wrap-normal-min-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Normal, TrackKind::MaxContent) => {
                "word-break-break-word-overflow-wrap-normal-max-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Normal, TrackKind::Fixed10Px) => {
                "word-break-break-word-overflow-wrap-normal-10px"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::BreakWord, TrackKind::MinContent) => {
                "word-break-break-word-overflow-wrap-break-word-min-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::BreakWord, TrackKind::MaxContent) => {
                "word-break-break-word-overflow-wrap-break-word-max-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::BreakWord, TrackKind::Fixed10Px) => {
                "word-break-break-word-overflow-wrap-break-word-10px"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Anywhere, TrackKind::MinContent) => {
                "word-break-break-word-overflow-wrap-anywhere-min-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Anywhere, TrackKind::MaxContent) => {
                "word-break-break-word-overflow-wrap-anywhere-max-content"
            }
            (WordBreakMode::BreakWord, OverflowWrapMode::Anywhere, TrackKind::Fixed10Px) => {
                "word-break-break-word-overflow-wrap-anywhere-10px"
            }
            _ => unreachable!(),
        },
        text: "XXXX",
        word_break_css: match word_break {
            WordBreakMode::Normal => "normal",
            WordBreakMode::BreakWord => "break-word",
            _ => unreachable!(),
        },
        word_break,
        overflow_wrap_css: match overflow_wrap {
            OverflowWrapMode::Normal => "normal",
            OverflowWrapMode::BreakWord => "break-word",
            OverflowWrapMode::Anywhere => "anywhere",
        },
        overflow_wrap,
        track,
        min_content: if effective_anywhere { 10.0 } else { 40.0 },
        max_content: 40.0,
        min_content_height: if effective_anywhere { 40.0 } else { 10.0 },
        max_content_height: 10.0,
        fixed_line_count,
    }
}

#[test]
fn anonymous_grid_combined_word_break_overflow_wrap_reaches_style_intrinsics_and_tracks() {
    assert_eq!(CASES.len(), 18);
    assert_eq!(
        CASES
            .iter()
            .map(|case| case.scene_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        CASES.len(),
    );

    for (index, case) in CASES.iter().enumerate() {
        let (snapshot, root, text_node) = snapshot_for(
            case,
            13_700 + index as u64,
            case.word_break_css,
            case.overflow_wrap_css,
            SnapshotOptions {
                language: Some("en"),
                ..SnapshotOptions::default()
            },
        );
        let calculation = calculate(snapshot.clone(), case.scene_id);
        let computed_style = calculation
            .roots
            .iter()
            .flat_map(|root| root.styles.elements.iter())
            .find(|style| style.node_id == root.id())
            .expect("Grid owner computed style");
        assert_eq!(
            computed_style
                .properties
                .get("word-break")
                .map(String::as_str),
            Some(case.word_break_css),
            "{} computed word-break",
            case.scene_id
        );
        assert_eq!(
            computed_style
                .properties
                .get("overflow-wrap")
                .map(String::as_str),
            Some(case.overflow_wrap_css),
            "{} computed overflow-wrap",
            case.scene_id
        );
        assert_eq!(
            calculation.roots[0].styles.document_revision,
            snapshot.document_revision(),
            "{} computed-style revision",
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
            TrackKind::Fixed10Px => case.fixed_line_count as f32 * 10.0,
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
        assert_eq!(text_layout.input.style.word_break, case.word_break);
        assert_eq!(text_layout.input.style.overflow_wrap, case.overflow_wrap);
        assert_eq!(text_layout.input.style.locale.as_deref(), Some("en"));
        assert_eq!(text_layout.input.style.font_family, "Ahem");
        assert_eq!(text_layout.layout.overflow_wrap, case.overflow_wrap);
        assert_eq!(text_layout.layout.text, case.text);
        assert_eq!(
            text_layout.layout.lines.len(),
            case.fixed_line_count,
            "{} line count",
            case.scene_id
        );
        let expected_line_advance = match (case.track, case.word_break, case.overflow_wrap) {
            (TrackKind::Fixed10Px, WordBreakMode::Normal, OverflowWrapMode::Normal) => 40.0,
            (TrackKind::Fixed10Px, _, _) => 10.0,
            (TrackKind::MinContent, _, _) => case.min_content,
            (TrackKind::MaxContent, _, _) => case.max_content,
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
fn anonymous_grid_combined_word_break_rejects_computed_value_mismatches() {
    let case = CASES[0];
    for (owner, word_break, overflow_wrap, expected_error) in [
        (13_790, "break-word", "normal", "word-break"),
        (13_791, "normal", "anywhere", "overflow-wrap"),
    ] {
        let (snapshot, ..) = snapshot_for(
            &case,
            owner,
            word_break,
            overflow_wrap,
            SnapshotOptions {
                language: Some("en"),
                ..SnapshotOptions::default()
            },
        );
        let calculation = calculate(snapshot, case.scene_id);
        let Err(error) = calculation.layout else {
            panic!("computed property mismatch published {}", case.scene_id);
        };
        let error = format!("{error:?}");
        assert!(
            error.contains("unsupported_c114_anonymous_text_style"),
            "{error}"
        );
        assert!(error.contains(expected_error), "{error}");
    }
}

#[test]
fn anonymous_grid_combined_word_break_uses_the_cascaded_important_declaration() {
    let case = CASES[9];
    let (snapshot, root, ..) = snapshot_for(
        &case,
        13_789,
        "normal;word-break:break-word !important",
        case.overflow_wrap_css,
        SnapshotOptions {
            language: Some("en"),
            ..SnapshotOptions::default()
        },
    );
    let calculation = calculate(snapshot, case.scene_id);
    let computed_style = calculation
        .roots
        .iter()
        .flat_map(|root| root.styles.elements.iter())
        .find(|style| style.node_id == root.id())
        .expect("Grid owner computed style");
    assert_eq!(
        computed_style
            .properties
            .get("word-break")
            .map(String::as_str),
        Some("break-word")
    );
    assert!(calculation.layout.is_ok());

    let (snapshot, root, ..) = snapshot_for(
        &case,
        13_788,
        "normal !important;word-break:break-word",
        case.overflow_wrap_css,
        SnapshotOptions {
            language: Some("en"),
            ..SnapshotOptions::default()
        },
    );
    let calculation = calculate(snapshot, case.scene_id);
    let computed_style = calculation
        .roots
        .iter()
        .flat_map(|root| root.styles.elements.iter())
        .find(|style| style.node_id == root.id())
        .expect("Grid owner computed style");
    assert_eq!(
        computed_style
            .properties
            .get("word-break")
            .map(String::as_str),
        Some("normal")
    );
    assert!(format!("{:?}", calculation.layout.unwrap_err()).contains("word-break"));
}

#[test]
fn anonymous_grid_combined_word_break_rejects_wrong_owner_language_and_track() {
    let case = CASES[9];
    for (owner, options, expected_error) in [
        (13_792, SnapshotOptions::default(), "Grid style owner lang"),
        (
            13_793,
            SnapshotOptions {
                language: Some("fr"),
                ..SnapshotOptions::default()
            },
            "Grid style owner lang",
        ),
        (
            13_794,
            SnapshotOptions {
                language: Some("en"),
                xml_language: Some("en"),
                ..SnapshotOptions::default()
            },
            "xml:lang",
        ),
        (
            13_795,
            SnapshotOptions {
                language: Some("en"),
                track_override: Some("max-content"),
                ..SnapshotOptions::default()
            },
            "Grid track",
        ),
    ] {
        let (snapshot, ..) = snapshot_for(
            &case,
            owner,
            case.word_break_css,
            case.overflow_wrap_css,
            options,
        );
        let calculation = calculate(snapshot, case.scene_id);
        let Err(error) = calculation.layout else {
            panic!("unsupported owner/style published {}", case.scene_id);
        };
        let error = format!("{error:?}");
        let expected_code = if expected_error == "Grid track" {
            "unsupported_c114_anonymous_text_fixture"
        } else {
            "unsupported_c114_anonymous_text_style"
        };
        assert!(
            error.contains(expected_code),
            "expected {expected_code}: {error}"
        );
        assert!(error.contains(expected_error), "{error}");
    }
}

#[test]
fn anonymous_grid_combined_word_break_rejects_duplicate_direct_text_and_unlisted_scene() {
    let case = CASES[9];
    let (snapshot, ..) = snapshot_for(
        &case,
        13_796,
        case.word_break_css,
        case.overflow_wrap_css,
        SnapshotOptions {
            language: Some("en"),
            duplicate_text: true,
            ..SnapshotOptions::default()
        },
    );
    let calculation = calculate(snapshot, case.scene_id);
    let Err(error) = calculation.layout else {
        panic!("duplicate direct text published {}", case.scene_id);
    };
    assert!(format!("{error:?}").contains("direct child count"));

    let mut unlisted = case;
    unlisted.scene_id = "word-break-break-word-overflow-wrap-loose-min-content";
    let (snapshot, ..) = snapshot_for(
        &unlisted,
        13_797,
        unlisted.word_break_css,
        unlisted.overflow_wrap_css,
        SnapshotOptions {
            language: Some("en"),
            ..SnapshotOptions::default()
        },
    );
    let calculation = calculate(snapshot, unlisted.scene_id);
    let Err(error) = calculation.layout else {
        panic!("unlisted combined scene published");
    };
    assert!(format!("{error:?}").contains("unknown anonymous text scene"));
}

fn calculate(
    snapshot: std::sync::Arc<spinon_core::HostDocumentSnapshot>,
    scene_id: &str,
) -> crate::session::ua_cascade::calculation::RuntimeCalculation {
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
    compute_request_for_block_flex_grid_track_sizing(&request)
        .unwrap_or_else(|error| panic!("{scene_id} calculation: {error:?}"))
}

#[allow(clippy::too_many_arguments)]
fn snapshot_for(
    case: &Case,
    owner_id: u64,
    word_break: &str,
    overflow_wrap: &str,
    options: SnapshotOptions<'_>,
) -> (
    std::sync::Arc<spinon_core::HostDocumentSnapshot>,
    HostNodeHandle,
    HostNodeHandle,
) {
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
    if let Some(language) = options.language {
        set_attribute(&mut batch, root, "lang", language);
    }
    if let Some(language) = options.xml_language {
        batch.push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(Some(XML.to_owned()), "lang").unwrap(),
            value: language.into(),
        });
    }
    let columns = options.track_override.unwrap_or(match case.track {
        TrackKind::MinContent => "min-content",
        TrackKind::MaxContent => "max-content",
        TrackKind::Fixed10Px => "10px",
    });
    set_attribute(
        &mut batch,
        root,
        "style",
        &format!(
            "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:{word_break};direction:ltr;line-break:auto;text-align:start;overflow-wrap:{overflow_wrap};text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px"
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
    if options.duplicate_text {
        let duplicate = document.reserve_node_handle().unwrap();
        batch.push(DocumentOperation::CreateText {
            node: duplicate,
            data: case.text.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: duplicate,
            before: None,
        });
    }
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
