use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, TextTransformMode,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy)]
struct Case {
    scene_id: &'static str,
    computed_value: &'static str,
    mode: TextTransformMode,
    text: &'static str,
    expected_text: &'static str,
    is_min_content: bool,
    min_content: f32,
    max_content: f32,
    first_line_advance: f32,
    second_line_advance: f32,
}

#[derive(Clone, Copy)]
struct UnicodeCase {
    scene_id: &'static str,
    computed_value: &'static str,
    mode: TextTransformMode,
    source_nodes: Option<&'static [&'static str]>,
    source_text: &'static str,
    expected_text: &'static str,
    is_min_content: bool,
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
    line_advances: &'static [f32],
}

const UNICODE_CASES: [UnicodeCase; 11] = [
    unicode_case(
        "text-transform-unicode-sharp-s-min-content",
        "ßeta",
        "Sseta",
        true,
        (25.1875, 25.1875),
        (10, 10),
        &[25.1875],
    ),
    unicode_case(
        "text-transform-unicode-sharp-s-max-content",
        "ßeta",
        "Sseta",
        false,
        (25.1875, 25.1875),
        (10, 10),
        &[25.1875],
    ),
    UnicodeCase {
        source_nodes: Some(&["ß", "eta"]),
        ..unicode_case(
            "text-transform-unicode-source-split-max-content",
            "ßeta",
            "Sseta",
            false,
            (25.1875, 25.1875),
            (10, 10),
            &[25.1875],
        )
    },
    unicode_case(
        "text-transform-unicode-accented-max-content",
        "éclair",
        "Éclair",
        false,
        (25.953125, 25.953125),
        (10, 10),
        &[25.953125],
    ),
    unicode_case(
        "text-transform-unicode-combining-max-content",
        "e\u{0301}lan",
        "E\u{0301}lan",
        false,
        (20.46875, 20.46875),
        (10, 10),
        &[20.46875],
    ),
    unicode_case(
        "text-transform-unicode-hyphen-boundary-max-content",
        "well-known",
        "Well-Known",
        false,
        (32.671875, 56.165863),
        (20, 10),
        &[56.165863],
    ),
    unicode_case(
        "text-transform-unicode-apostrophe-word-max-content",
        "o'neill",
        "O'neill",
        false,
        (30.28125, 30.28125),
        (10, 10),
        &[30.28125],
    ),
    unicode_case(
        "text-transform-unicode-opening-punctuation-max-content",
        "«élan»",
        "«Élan»",
        false,
        (30.046875, 30.046875),
        (10, 10),
        &[30.046875],
    ),
    unicode_case(
        "text-transform-unicode-number-first-none",
        "49ers",
        "49ers",
        false,
        (24.984375, 24.984375),
        (10, 10),
        &[24.984375],
    ),
    UnicodeCase {
        mode: TextTransformMode::None,
        computed_value: "none",
        ..unicode_case(
            "text-transform-unicode-number-first-control",
            "49ers",
            "49ers",
            false,
            (24.984375, 24.984375),
            (10, 10),
            &[24.984375],
        )
    },
    unicode_case(
        "text-transform-unicode-collapsed-space-max-content",
        "ßeta   éclair",
        "Sseta Éclair",
        false,
        (25.953125, 53.359375),
        (20, 10),
        &[53.359375],
    ),
];

const fn unicode_case(
    scene_id: &'static str,
    source_text: &'static str,
    expected_text: &'static str,
    is_min_content: bool,
    intrinsic_sizes: (f32, f32),
    intrinsic_heights: (u8, u8),
    line_advances: &'static [f32],
) -> UnicodeCase {
    UnicodeCase {
        scene_id,
        computed_value: "capitalize",
        mode: TextTransformMode::Capitalize,
        source_nodes: None,
        source_text,
        expected_text,
        is_min_content,
        min_content: intrinsic_sizes.0,
        max_content: intrinsic_sizes.1,
        min_content_height: intrinsic_heights.0 as f32,
        max_content_height: intrinsic_heights.1 as f32,
        line_advances,
    }
}

const CASES: [Case; 12] = [
    case(TextTransformMode::None, true),
    case(TextTransformMode::None, false),
    case(TextTransformMode::Uppercase, true),
    case(TextTransformMode::Uppercase, false),
    case(TextTransformMode::Lowercase, true),
    case(TextTransformMode::Lowercase, false),
    Case {
        scene_id: "text-transform-capitalize-none-min-content",
        computed_value: "none",
        mode: TextTransformMode::None,
        text: "iiii ii",
        expected_text: "iiii ii",
        is_min_content: true,
        min_content: 11.0,
        max_content: 18.71875,
        first_line_advance: 11.0,
        second_line_advance: 5.5,
    },
    Case {
        scene_id: "text-transform-capitalize-none-max-content",
        computed_value: "none",
        mode: TextTransformMode::None,
        text: "iiii ii",
        expected_text: "iiii ii",
        is_min_content: false,
        min_content: 11.0,
        max_content: 18.71875,
        first_line_advance: 18.71875,
        second_line_advance: 0.0,
    },
    Case {
        scene_id: "text-transform-capitalize-min-content",
        computed_value: "capitalize",
        mode: TextTransformMode::Capitalize,
        text: "iiii ii",
        expected_text: "Iiii Ii",
        is_min_content: true,
        min_content: 11.1875,
        max_content: 19.09375,
        first_line_advance: 11.1875,
        second_line_advance: 5.6875,
    },
    Case {
        scene_id: "text-transform-capitalize-max-content",
        computed_value: "capitalize",
        mode: TextTransformMode::Capitalize,
        text: "iiii ii",
        expected_text: "Iiii Ii",
        is_min_content: false,
        min_content: 11.1875,
        max_content: 19.09375,
        first_line_advance: 19.09375,
        second_line_advance: 0.0,
    },
    Case {
        scene_id: "text-transform-capitalize-mixed-case",
        computed_value: "capitalize",
        mode: TextTransformMode::Capitalize,
        text: "iIII ii",
        expected_text: "IIII Ii",
        is_min_content: false,
        min_content: 11.734375,
        max_content: 19.65625,
        first_line_advance: 19.65625,
        second_line_advance: 0.0,
    },
    Case {
        scene_id: "text-transform-capitalize-collapsed-spaces",
        computed_value: "capitalize",
        mode: TextTransformMode::Capitalize,
        text: "ii   ii",
        expected_text: "Ii Ii",
        is_min_content: false,
        min_content: 5.6875,
        max_content: 13.59375,
        first_line_advance: 13.59375,
        second_line_advance: 0.0,
    },
];

const fn case(mode: TextTransformMode, is_min_content: bool) -> Case {
    let (value, min_scene_id, max_scene_id, expected_text, min_content, max_content, first, second) =
        match mode {
            TextTransformMode::None => (
                "none",
                "text-transform-none-min-content",
                "text-transform-none-max-content",
                "IIII ii",
                11.734375,
                19.46875,
                11.72,
                5.5,
            ),
            TextTransformMode::Uppercase => (
                "uppercase",
                "text-transform-uppercase-min-content",
                "text-transform-uppercase-max-content",
                "IIII II",
                11.734375,
                19.84375,
                11.72,
                5.86,
            ),
            TextTransformMode::Lowercase => (
                "lowercase",
                "text-transform-lowercase-min-content",
                "text-transform-lowercase-max-content",
                "iiii ii",
                11.0,
                18.71875,
                11.0,
                5.5,
            ),
            TextTransformMode::Capitalize => (
                "capitalize",
                "text-transform-capitalize-min-content",
                "text-transform-capitalize-max-content",
                "Iiii Ii",
                11.1875,
                19.09375,
                11.1875,
                5.6875,
            ),
        };
    Case {
        scene_id: if is_min_content {
            min_scene_id
        } else {
            max_scene_id
        },
        computed_value: value,
        mode,
        text: "IIII ii",
        expected_text,
        is_min_content,
        min_content,
        max_content,
        first_line_advance: first,
        second_line_advance: second,
    }
}

#[test]
fn anonymous_grid_computed_text_transform_reaches_intrinsic_tracks_and_keeps_dom_text() {
    assert_eq!(CASES.len(), 12);
    assert_eq!(
        CASES
            .iter()
            .map(|case| case.scene_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        CASES.len()
    );

    for (index, case) in CASES.iter().enumerate() {
        let (snapshot, root, text_node) = snapshot_for(case, 12_910 + index as u64, Some("en"));
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
                .get("text-transform")
                .map(String::as_str),
            Some(case.computed_value),
            "{} computed text-transform",
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
        let root_frame = layout
            .frames
            .iter()
            .find(|frame| frame.node_id == root.id().get())
            .expect("Grid root frame");
        let expected_track = if case.is_min_content {
            case.min_content
        } else {
            case.max_content
        };
        let expected_height = if case.is_min_content { 20.0 } else { 10.0 };
        assert_eq!(
            (root_frame.width, root_frame.height),
            (120.0, expected_height)
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
        close_to(sizes.min_content(), case.min_content, case.scene_id);
        close_to(sizes.max_content(), case.max_content, case.scene_id);
        close_to(sizes.min_content_height(), 20.0, case.scene_id);
        close_to(sizes.max_content_height(), 10.0, case.scene_id);

        let text_layout = diagnostics
            .item_text_layouts
            .iter()
            .find(|text| text.input.node == text_node.id())
            .expect("anonymous text layout");
        assert_eq!(text_layout.input.style.text_transform, case.mode);
        assert_eq!(text_layout.input.sources[0].text, case.text);
        assert_eq!(text_layout.layout.text_transform, case.mode);
        assert_eq!(text_layout.layout.text, case.expected_text);
        assert_eq!(text_layout.layout.sources.len(), 7);
        for (source_index, source) in text_layout.layout.sources.iter().enumerate() {
            assert_eq!(source.node, text_node.id());
            assert_eq!(source.source_utf16, source_index..source_index + 1);
            let expected_paragraph_range =
                if case.scene_id == "text-transform-capitalize-collapsed-spaces" {
                    match source_index {
                        0 => 0..1,
                        1 => 1..2,
                        2..=4 => 2..3,
                        5 => 3..4,
                        6 => 4..5,
                        _ => unreachable!(),
                    }
                } else {
                    source_index..source_index + 1
                };
            assert_eq!(source.paragraph_utf8, expected_paragraph_range);
        }
        let expected_line_count = if case.is_min_content { 2 } else { 1 };
        assert_eq!(text_layout.layout.lines.len(), expected_line_count);
        close_to(
            text_layout.layout.lines[0].advance,
            if case.is_min_content {
                case.first_line_advance
            } else {
                case.max_content
            },
            case.scene_id,
        );
        if case.is_min_content {
            close_to(
                text_layout.layout.lines[1].advance,
                case.second_line_advance,
                case.scene_id,
            );
        }

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
        assert_eq!(track_sizes.len(), 1, "{} track count", case.scene_id);
        close_to(track_sizes[0], expected_track, case.scene_id);
        let expected_contribution_kind = if case.is_min_content {
            LayoutGridTrackSizingContributionKind::MinContent
        } else {
            LayoutGridTrackSizingContributionKind::MaxContent
        };
        let expected_contribution = if case.is_min_content {
            case.first_line_advance
        } else {
            text_layout.layout.lines[0].advance
        };
        assert!(
            column_pass.contributions.iter().any(|contribution| {
                contribution.kind == expected_contribution_kind
                    && (contribution.value - expected_contribution).abs() <= 1.0 / 64.0
            }),
            "{} expected {:?} contribution near {}, got {:?}",
            case.scene_id,
            expected_contribution_kind,
            expected_contribution,
            column_pass.contributions
        );
    }
}

#[test]
fn anonymous_grid_unicode_text_transform_reaches_tracks_and_keeps_utf16_provenance() {
    assert_eq!(UNICODE_CASES.len(), 11);
    assert_eq!(
        UNICODE_CASES
            .iter()
            .map(|case| case.scene_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        UNICODE_CASES.len()
    );

    for (index, case) in UNICODE_CASES.iter().enumerate() {
        let fallback_source_nodes = [case.source_text];
        let source_nodes = case.source_nodes.unwrap_or(&fallback_source_nodes);
        let (snapshot, root, text_nodes) = snapshot_for_sources(
            case.scene_id,
            case.source_text,
            case.computed_value,
            source_nodes,
            12_940 + index as u64,
            Some("en"),
            case.is_min_content,
        );
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
        let layout = calculation
            .layout
            .unwrap_or_else(|error| panic!("{} layout: {error:?}", case.scene_id));
        let diagnostics = layout
            .grid_track_sizing_diagnostics
            .as_ref()
            .expect("opt-in C11.4 diagnostics");
        let text_node = text_nodes[0];
        let item = diagnostics
            .item_inputs
            .iter()
            .find(|input| input.node == text_node.id())
            .expect("anonymous text intrinsic input");
        let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
            panic!("{} item did not use text intrinsic sizes", case.scene_id);
        };
        close_to(sizes.min_content(), case.min_content, case.scene_id);
        close_to(sizes.max_content(), case.max_content, case.scene_id);
        close_to(
            sizes.min_content_height(),
            case.min_content_height,
            case.scene_id,
        );
        close_to(
            sizes.max_content_height(),
            case.max_content_height,
            case.scene_id,
        );

        let text_layout = diagnostics
            .item_text_layouts
            .iter()
            .find(|text| text.input.node == text_node.id())
            .expect("anonymous text layout");
        assert_eq!(text_layout.input.style.text_transform, case.mode);
        assert_eq!(
            text_layout
                .input
                .sources
                .iter()
                .map(|source| source.text.as_str())
                .collect::<Vec<_>>(),
            source_nodes,
            "{} direct text nodes",
            case.scene_id
        );
        assert_eq!(
            text_layout.layout.text, case.expected_text,
            "{}",
            case.scene_id
        );
        assert_eq!(
            text_layout.layout.lines.len(),
            case.line_advances.len(),
            "{} line count",
            case.scene_id
        );
        for (line, expected) in text_layout.layout.lines.iter().zip(case.line_advances) {
            close_to(line.advance, *expected, case.scene_id);
        }
        assert_unicode_source_mapping(case, &text_nodes, &text_layout.layout.sources);

        let expected_track = if case.is_min_content {
            case.min_content
        } else {
            case.max_content
        };
        let expected_height = if case.is_min_content {
            case.min_content_height
        } else {
            case.max_content_height
        };
        let root_frame = layout
            .frames
            .iter()
            .find(|frame| frame.node_id == root.id().get())
            .expect("Grid root frame");
        close_to(root_frame.width, 120.0, case.scene_id);
        close_to(root_frame.height, expected_height, case.scene_id);

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
        assert_eq!(track_sizes.len(), 1, "{} track count", case.scene_id);
        close_to(track_sizes[0], expected_track, case.scene_id);
        let expected_contribution_kind = if case.is_min_content {
            LayoutGridTrackSizingContributionKind::MinContent
        } else {
            LayoutGridTrackSizingContributionKind::MaxContent
        };
        let expected_contribution = if case.is_min_content {
            case.min_content
        } else {
            case.max_content
        };
        assert!(
            column_pass.contributions.iter().any(|contribution| {
                contribution.kind == expected_contribution_kind
                    && (contribution.value - expected_contribution).abs() <= 0.03125
            }),
            "{} {:?} contribution near {expected_contribution}, got {:?}",
            case.scene_id,
            expected_contribution_kind,
            column_pass.contributions
        );
    }
}

fn assert_unicode_source_mapping(
    case: &UnicodeCase,
    text_nodes: &[HostNodeHandle],
    sources: &[spinon_layout::TextSourceRange],
) {
    if case.scene_id.ends_with("sharp-s-min-content")
        || case.scene_id.ends_with("sharp-s-max-content")
    {
        assert_eq!(sources.len(), 5, "{}", case.scene_id);
        assert_eq!(sources[0].source_utf16, 0..1);
        assert_eq!(sources[1].source_utf16, 0..1);
        assert_eq!(sources[0].paragraph_utf8, 0..1);
        assert_eq!(sources[1].paragraph_utf8, 1..2);
    } else if case.scene_id.ends_with("source-split-max-content") {
        assert_eq!(sources.len(), 5, "{}", case.scene_id);
        assert_eq!(sources[0].node, text_nodes[0].id());
        assert_eq!(sources[1].node, text_nodes[0].id());
        assert!(
            sources[..2]
                .iter()
                .all(|source| source.source_utf16 == (0..1))
        );
        assert!(
            sources[2..]
                .iter()
                .all(|source| source.node == text_nodes[1].id())
        );
        assert_eq!(sources[2].source_utf16, 0..1);
    } else if case.scene_id.ends_with("combining-max-content") {
        assert_eq!(sources.len(), 5, "{}", case.scene_id);
        assert_eq!(sources[0].source_utf16, 0..1);
        assert_eq!(sources[0].paragraph_utf8, 0..1);
        assert_eq!(sources[1].source_utf16, 1..2);
        assert_eq!(sources[1].paragraph_utf8, 1..3);
    } else if case.scene_id.ends_with("collapsed-space-max-content") {
        let spaces = sources
            .iter()
            .filter(|source| source.paragraph_utf8 == (5..6))
            .collect::<Vec<_>>();
        assert_eq!(spaces.len(), 3);
        assert_eq!(
            spaces
                .iter()
                .map(|source| source.source_utf16.clone())
                .collect::<Vec<_>>(),
            [4..5, 5..6, 6..7]
        );
        let accented = sources
            .iter()
            .find(|source| source.source_utf16 == (7..8))
            .unwrap();
        assert_eq!(accented.paragraph_utf8, 6..8);
    } else {
        assert_eq!(sources.len(), case.source_text.chars().count());
    }
}

#[test]
fn anonymous_grid_text_transform_rejects_wrong_value_or_missing_direct_language() {
    for (index, (mut case, value, lang)) in [
        (CASES[2], "lowercase", Some("en")),
        (CASES[2], "uppercase", None),
        (CASES[8], "none", Some("en")),
        (CASES[9], "capitalize", None),
    ]
    .into_iter()
    .enumerate()
    {
        case.computed_value = value;
        let (snapshot, _, _) = snapshot_for(&case, 12_920 + index as u64, lang);
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
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.scene_id));
        let Err(error) = calculation.layout else {
            panic!("{} invalid transform input published layout", case.scene_id);
        };
        assert!(
            format!("{error:?}").contains("unsupported_c114_anonymous_text_style"),
            "{error:?}"
        );
    }
}

fn snapshot_for(
    case: &Case,
    owner_id: u64,
    lang: Option<&str>,
) -> (
    std::sync::Arc<spinon_core::HostDocumentSnapshot>,
    HostNodeHandle,
    HostNodeHandle,
) {
    let (snapshot, root, text_nodes) = snapshot_for_sources(
        case.scene_id,
        case.text,
        case.computed_value,
        &[case.text],
        owner_id,
        lang,
        case.is_min_content,
    );
    (snapshot, root, text_nodes[0])
}

fn snapshot_for_sources(
    scene_id: &str,
    source_text: &str,
    computed_value: &str,
    source_nodes: &[&str],
    owner_id: u64,
    lang: Option<&str>,
    is_min_content: bool,
) -> (
    std::sync::Arc<spinon_core::HostDocumentSnapshot>,
    HostNodeHandle,
    Vec<HostNodeHandle>,
) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let text_nodes = source_nodes
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(owner_id).unwrap(),
        document.document_revision(),
    );
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    set_attribute(&mut batch, root, "id", &format!("c114-{scene_id}-grid"));
    if let Some(lang) = lang {
        set_attribute(&mut batch, root, "lang", lang);
    }
    set_attribute(
        &mut batch,
        root,
        "style",
        &format!(
            "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:\"Noto Sans KR\";font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:{};letter-spacing:normal;word-spacing:0px;text-indent:0px",
            if is_min_content {
                "min-content"
            } else {
                "max-content"
            },
            computed_value
        ),
    );
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    for (text_node, text) in text_nodes.iter().zip(source_nodes) {
        batch.push(DocumentOperation::CreateText {
            node: *text_node,
            data: (*text).into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: *text_node,
            before: None,
        });
    }
    assert_eq!(source_nodes.concat(), source_text);
    document.commit(batch).unwrap();
    (std::sync::Arc::new(document.snapshot()), root, text_nodes)
}

fn set_attribute(batch: &mut DocumentChangeBatch, node: HostNodeHandle, name: &str, value: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, name).unwrap(),
        value: value.into(),
    });
}

fn close_to(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.03125,
        "{label}: expected {expected}, got {actual} CSS px"
    );
}
