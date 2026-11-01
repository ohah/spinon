use super::*;

use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LineBreakMode,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const XML: &str = "http://www.w3.org/XML/1998/namespace";

#[derive(Clone, Copy)]
struct Case {
    scene_id: &'static str,
    text: &'static str,
    language: &'static str,
    line_break_css: &'static str,
    line_break: LineBreakMode,
    is_min_content: bool,
    sizes: IntrinsicSizes,
}

#[derive(Clone, Copy)]
struct IntrinsicSizes {
    min_content: f32,
    max_content: f32,
    min_content_height: f32,
    max_content_height: f32,
}

const CASES: [Case; 9] = [
    case(
        "line-break-ja-normal-min-content",
        "ja",
        "normal",
        LineBreakMode::Normal,
        true,
        IntrinsicSizes {
            min_content: 10.0,
            max_content: 30.0,
            min_content_height: 30.0,
            max_content_height: 10.0,
        },
    ),
    case(
        "line-break-ja-normal-max-content",
        "ja",
        "normal",
        LineBreakMode::Normal,
        false,
        IntrinsicSizes {
            min_content: 10.0,
            max_content: 30.0,
            min_content_height: 30.0,
            max_content_height: 10.0,
        },
    ),
    case(
        "line-break-ja-strict-min-content",
        "ja",
        "strict",
        LineBreakMode::Strict,
        true,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
    case(
        "line-break-ja-strict-max-content",
        "ja",
        "strict",
        LineBreakMode::Strict,
        false,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
    case(
        "line-break-en-normal-min-content",
        "en",
        "normal",
        LineBreakMode::Normal,
        true,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
    case_with_text(
        "line-break-ja-loose-min-content",
        "文＄文",
        "ja",
        "loose",
        LineBreakMode::Loose,
        true,
        IntrinsicSizes {
            min_content: 10.0,
            max_content: 30.0,
            min_content_height: 30.0,
            max_content_height: 10.0,
        },
    ),
    case_with_text(
        "line-break-ja-loose-max-content",
        "文＄文",
        "ja",
        "loose",
        LineBreakMode::Loose,
        false,
        IntrinsicSizes {
            min_content: 10.0,
            max_content: 30.0,
            min_content_height: 30.0,
            max_content_height: 10.0,
        },
    ),
    case_with_text(
        "line-break-en-loose-min-content",
        "文＄文",
        "en",
        "loose",
        LineBreakMode::Loose,
        true,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
    case_with_text(
        "line-break-ja-normal-prefix-control-min-content",
        "文＄文",
        "ja",
        "normal",
        LineBreakMode::Normal,
        true,
        IntrinsicSizes {
            min_content: 20.0,
            max_content: 30.0,
            min_content_height: 20.0,
            max_content_height: 10.0,
        },
    ),
];

const fn case(
    scene_id: &'static str,
    language: &'static str,
    line_break_css: &'static str,
    line_break: LineBreakMode,
    is_min_content: bool,
    sizes: IntrinsicSizes,
) -> Case {
    case_with_text(
        scene_id,
        "文〜文",
        language,
        line_break_css,
        line_break,
        is_min_content,
        sizes,
    )
}

const fn case_with_text(
    scene_id: &'static str,
    text: &'static str,
    language: &'static str,
    line_break_css: &'static str,
    line_break: LineBreakMode,
    is_min_content: bool,
    sizes: IntrinsicSizes,
) -> Case {
    Case {
        scene_id,
        text,
        language,
        line_break_css,
        line_break,
        is_min_content,
        sizes,
    }
}

#[test]
fn computed_line_break_scenes_require_their_exact_intrinsic_track() {
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
        let (snapshot, root, text_node) = snapshot_for(
            case,
            12_800 + index as u64,
            Some(case.language),
            None,
            case.line_break_css,
            case.text,
            false,
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
        let computed_style = calculation.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.node_id == root.id())
            .expect("root computed style belongs to the calculation");
        assert_eq!(
            computed_style
                .properties
                .get("line-break")
                .map(String::as_str),
            Some(case.line_break_css),
            "{} computed line-break",
            case.scene_id
        );
        assert_eq!(
            calculation.roots[0].styles.document_revision,
            snapshot.document_revision()
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
            case.sizes.min_content
        } else {
            case.sizes.max_content
        };
        let expected_height = if case.is_min_content {
            case.sizes.min_content_height
        } else {
            case.sizes.max_content_height
        };
        assert_close(root_frame.width, 120.0, case, "root width");
        assert_close(root_frame.height, expected_height, case, "root height");

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
        assert_close(
            sizes.min_content(),
            case.sizes.min_content,
            case,
            "min-content",
        );
        assert_close(
            sizes.max_content(),
            case.sizes.max_content,
            case,
            "max-content",
        );
        assert_close(
            sizes.min_content_height(),
            case.sizes.min_content_height,
            case,
            "min-content height",
        );
        assert_close(
            sizes.max_content_height(),
            case.sizes.max_content_height,
            case,
            "max-content height",
        );

        let text_layout = diagnostics
            .item_text_layouts
            .iter()
            .find(|text| text.input.node == text_node.id())
            .expect("anonymous text layout");
        assert_eq!(text_layout.input.style.line_break, case.line_break);
        assert_eq!(
            text_layout.input.style.locale.as_deref(),
            Some(case.language)
        );
        assert_eq!(text_layout.input.style.font_family, "Noto Sans KR");
        assert_eq!(text_layout.layout.text, case.text);
        assert_eq!(
            text_layout.layout.lines.len(),
            (expected_height / 10.0) as usize,
            "{} line count",
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
        let expected_kind = if case.is_min_content {
            LayoutGridTrackSizingContributionKind::MinContent
        } else {
            LayoutGridTrackSizingContributionKind::MaxContent
        };
        assert!(column_pass.contributions.iter().any(|contribution| {
            contribution.kind == expected_kind
                && (contribution.value - expected_track).abs() <= 1.0 / 64.0
        }));
    }
}

#[test]
fn anonymous_grid_line_break_rejects_language_property_text_and_child_mismatches() {
    let case = CASES[0];
    for (owner_id, lang, xml_lang, line_break, text, extra_child, expected_error) in [
        (
            12_890,
            None,
            None,
            "normal",
            "文〜文",
            false,
            "Grid style owner lang",
        ),
        (
            12_891,
            Some("en"),
            None,
            "normal",
            "文〜文",
            false,
            "Grid style owner lang",
        ),
        (
            12_892,
            Some("ja"),
            Some("ja"),
            "normal",
            "文〜文",
            false,
            "xml:lang",
        ),
        (
            12_893,
            Some("ja"),
            None,
            "strict",
            "文〜文",
            false,
            "line-break",
        ),
        (
            12_894,
            Some("ja"),
            None,
            "auto",
            "文〜文",
            false,
            "line-break",
        ),
        (
            12_895,
            Some("ja"),
            None,
            "loose",
            "文〜文",
            false,
            "line-break",
        ),
        (
            12_896,
            Some("ja"),
            None,
            "normal",
            "文文〜",
            false,
            "direct text content",
        ),
        (
            12_897,
            Some("ja"),
            None,
            "normal",
            "文〜文",
            true,
            "direct child count",
        ),
    ] {
        let (snapshot, _, _) = snapshot_for(
            &case,
            owner_id,
            lang,
            xml_lang,
            line_break,
            text,
            extra_child,
        );
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
            panic!("unsupported line-break input published layout: owner {owner_id}");
        };
        let error = format!("{error:?}");
        assert!(
            error.contains("unsupported_c114_anonymous_text"),
            "{owner_id}: {error}"
        );
        assert!(error.contains(expected_error), "{owner_id}: {error}");
    }
}

#[test]
fn anonymous_grid_line_break_rejects_unlisted_scenes_and_stale_revisions() {
    let case = CASES[0];
    let unknown = Case {
        scene_id: "line-break-unlisted-min-content",
        ..case
    };
    let (snapshot, _, _) = snapshot_for(
        &unknown,
        12_898,
        Some(case.language),
        None,
        case.line_break_css,
        "文〜文",
        false,
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
    let unknown_error = match compute_request_for_block_flex_grid_track_sizing(&request) {
        Err(error) => error,
        Ok(calculation) => format!("{:?}", calculation.layout.expect_err("unknown scene")),
    };
    assert!(
        unknown_error.contains("unknown anonymous text scene"),
        "{unknown_error}"
    );

    let (snapshot, _, _) = snapshot_for(
        &case,
        12_899,
        Some(case.language),
        None,
        case.line_break_css,
        "文〜文",
        false,
    );
    let mut stale_request = WorkRequest {
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
    stale_request.key.document_revision += 1;
    let stale_error = match compute_request_for_block_flex_grid_track_sizing(&stale_request) {
        Err(error) => error,
        Ok(calculation) => format!("{:?}", calculation.layout.expect_err("stale revision")),
    };
    assert!(stale_error.contains("revision"), "{stale_error}");
}

#[test]
fn anonymous_grid_line_break_loose_rejects_locale_mode_and_text_mismatches() {
    let case = CASES[5];
    for (owner_id, language, line_break, text, expected_error) in [
        (12_910, "en", "loose", case.text, "Grid style owner lang"),
        (12_911, "ja", "normal", case.text, "line-break"),
        (12_912, "ja", "loose", "文〜文", "direct text content"),
    ] {
        let (snapshot, _, _) = snapshot_for(
            &case,
            owner_id,
            Some(language),
            None,
            line_break,
            text,
            false,
        );
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
            panic!("unsupported loose line-break input published layout: owner {owner_id}");
        };
        let error = format!("{error:?}");
        assert!(error.contains("unsupported_c114_anonymous_text"), "{error}");
        assert!(error.contains(expected_error), "{owner_id}: {error}");
    }
}

fn snapshot_for(
    case: &Case,
    owner_id: u64,
    lang: Option<&str>,
    xml_lang: Option<&str>,
    line_break: &str,
    text_value: &str,
    extra_child: bool,
) -> (Arc<HostDocumentSnapshot>, HostNodeHandle, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let extra = extra_child.then(|| document.reserve_node_handle().unwrap());
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
    set_attribute(
        &mut batch,
        root,
        "style",
        &format!(
            "box-sizing:border-box;display:grid;width:120px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{grid_track};grid-template-rows:auto;font-family:\"Noto Sans KR\";font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:{line_break};text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px"
        ),
    );
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: text_value.into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(root),
        node: text,
        before: None,
    });
    if let Some(extra) = extra {
        batch.push(DocumentOperation::CreateText {
            node: extra,
            data: "extra".into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: extra,
            before: None,
        });
    }
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
