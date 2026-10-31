use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
};
use spinon_text_layout::WordBreakMode;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const XML: &str = "http://www.w3.org/XML/1998/namespace";

#[derive(Clone, Copy)]
struct IntrinsicSizes {
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
}

#[derive(Clone, Copy)]
struct WordBreakSpec {
    css: &'static str,
    mode: WordBreakMode,
}

const NORMAL: WordBreakSpec = WordBreakSpec {
    css: "normal",
    mode: WordBreakMode::Normal,
};
const BREAK_ALL: WordBreakSpec = WordBreakSpec {
    css: "break-all",
    mode: WordBreakMode::BreakAll,
};
const KEEP_ALL: WordBreakSpec = WordBreakSpec {
    css: "keep-all",
    mode: WordBreakMode::KeepAll,
};

#[derive(Clone, Copy)]
struct Case {
    scene_id: &'static str,
    text: &'static str,
    language: &'static str,
    font_family: &'static str,
    word_break_css: &'static str,
    word_break: WordBreakMode,
    is_min_content: bool,
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
}

const CASES: [Case; 12] = [
    case(
        "word-break-normal-unbroken-min-content",
        "XXXX",
        "en",
        "Ahem",
        NORMAL,
        true,
        intrinsic_sizes(40.0, 40.0, 10.0, 10.0),
    ),
    case(
        "word-break-normal-unbroken-max-content",
        "XXXX",
        "en",
        "Ahem",
        NORMAL,
        false,
        intrinsic_sizes(40.0, 40.0, 10.0, 10.0),
    ),
    case(
        "word-break-break-all-unbroken-min-content",
        "XXXX",
        "en",
        "Ahem",
        BREAK_ALL,
        true,
        intrinsic_sizes(10.0, 40.0, 40.0, 10.0),
    ),
    case(
        "word-break-break-all-unbroken-max-content",
        "XXXX",
        "en",
        "Ahem",
        BREAK_ALL,
        false,
        intrinsic_sizes(10.0, 40.0, 40.0, 10.0),
    ),
    case(
        "word-break-normal-two-words-min-content",
        "XX XX",
        "en",
        "Ahem",
        NORMAL,
        true,
        intrinsic_sizes(20.0, 50.0, 20.0, 10.0),
    ),
    case(
        "word-break-normal-two-words-max-content",
        "XX XX",
        "en",
        "Ahem",
        NORMAL,
        false,
        intrinsic_sizes(20.0, 50.0, 20.0, 10.0),
    ),
    case(
        "word-break-break-all-two-words-min-content",
        "XX XX",
        "en",
        "Ahem",
        BREAK_ALL,
        true,
        intrinsic_sizes(10.0, 50.0, 40.0, 10.0),
    ),
    case(
        "word-break-break-all-two-words-max-content",
        "XX XX",
        "en",
        "Ahem",
        BREAK_ALL,
        false,
        intrinsic_sizes(10.0, 50.0, 40.0, 10.0),
    ),
    case(
        "word-break-normal-hangul-min-content",
        "가나다라",
        "ko",
        "Noto Sans KR",
        NORMAL,
        true,
        intrinsic_sizes(9.2, 36.8, 40.0, 10.0),
    ),
    case(
        "word-break-normal-hangul-max-content",
        "가나다라",
        "ko",
        "Noto Sans KR",
        NORMAL,
        false,
        intrinsic_sizes(9.2, 36.8, 40.0, 10.0),
    ),
    case(
        "word-break-keep-all-hangul-min-content",
        "가나다라",
        "ko",
        "Noto Sans KR",
        KEEP_ALL,
        true,
        intrinsic_sizes(36.8, 36.8, 10.0, 10.0),
    ),
    case(
        "word-break-keep-all-hangul-max-content",
        "가나다라",
        "ko",
        "Noto Sans KR",
        KEEP_ALL,
        false,
        intrinsic_sizes(36.8, 36.8, 10.0, 10.0),
    ),
];

const fn case(
    scene_id: &'static str,
    text: &'static str,
    language: &'static str,
    font_family: &'static str,
    word_break: WordBreakSpec,
    is_min_content: bool,
    sizes: IntrinsicSizes,
) -> Case {
    Case {
        scene_id,
        text,
        language,
        font_family,
        word_break_css: word_break.css,
        word_break: word_break.mode,
        is_min_content,
        min_content: sizes.min_content,
        max_content: sizes.max_content,
        min_content_height: sizes.min_content_height,
        max_content_height: sizes.max_content_height,
    }
}

const fn intrinsic_sizes(
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
) -> IntrinsicSizes {
    IntrinsicSizes {
        min_content,
        max_content,
        min_content_height,
        max_content_height,
    }
}

#[test]
fn anonymous_grid_word_break_reaches_computed_style_paragraph_intrinsics_and_tracks() {
    assert_eq!(CASES.len(), 12);
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
            12_500 + index as u64,
            Some(case.language),
            None,
            case.word_break_css,
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
        let computed_style = calculation
            .roots
            .iter()
            .flat_map(|root| root.styles.elements.iter())
            .find(|style| style.node_id == root.id())
            .expect("root computed style belongs to the calculation");
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
            case.min_content
        } else {
            case.max_content
        };
        let expected_height = if case.is_min_content {
            case.min_content_height
        } else {
            case.max_content_height
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
        assert_eq!(
            text_layout.input.style.locale.as_deref(),
            Some(case.language)
        );
        assert_eq!(text_layout.input.style.font_family, case.font_family);
        assert_eq!(text_layout.layout.text, case.text);
        let expected_line_count = if case.is_min_content {
            (case.min_content_height / 10.0) as usize
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
                (line.advance - expected_track).abs() <= 1.0 / 64.0
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
                    && (contribution.value - expected_contribution).abs() <= 1.0 / 64.0
            }),
            "{} missing {expected_contribution_kind:?} contribution",
            case.scene_id
        );
    }
}

#[test]
fn anonymous_grid_word_break_rejects_missing_or_xml_language_and_mismatched_style() {
    let case = CASES[8];
    for (owner_id, lang, xml_lang, word_break, expected_error) in [
        (12_590, None, None, "normal", "Grid style owner lang"),
        (12_591, Some("en"), None, "normal", "Grid style owner lang"),
        (12_592, Some("ko"), Some("ko"), "normal", "xml:lang"),
        (12_593, Some("ko"), None, "break-all", "word-break"),
    ] {
        let (snapshot, _, _) = snapshot_for(&case, owner_id, lang, xml_lang, word_break);
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
            panic!("unsupported word-break input published layout: owner {owner_id}");
        };
        let error = format!("{error:?}");
        assert!(
            error.contains("unsupported_c114_anonymous_text_style"),
            "owner {owner_id}: {error}"
        );
        assert!(error.contains(expected_error), "owner {owner_id}: {error}");
    }
}

fn snapshot_for(
    case: &Case,
    owner_id: u64,
    lang: Option<&str>,
    xml_lang: Option<&str>,
    word_break: &str,
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
    if let Some(lang) = lang {
        set_attribute(&mut batch, root, "lang", lang);
    }
    if let Some(xml_lang) = xml_lang {
        batch.push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(Some(XML.to_owned()), "lang").unwrap(),
            value: xml_lang.into(),
        });
    }
    let grid_track = if case.is_min_content {
        "min-content"
    } else {
        "max-content"
    };
    let font_family = if case.font_family == "Noto Sans KR" {
        "\"Noto Sans KR\""
    } else {
        case.font_family
    };
    set_attribute(
        &mut batch,
        root,
        "style",
        &format!(
            "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{grid_track};grid-template-rows:auto;font-family:{font_family};font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:{word_break};direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px"
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

fn assert_close(actual: f32, expected: f32, case: &Case, field: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{} {field}: expected {expected}, got {actual}",
        case.scene_id
    );
}
