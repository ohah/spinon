use super::*;
use spinon_core::{TextOrientation, WritingMode};

#[test]
fn track_sizing_snapshot_preserves_computed_root_text_and_inherited_image_modes() {
    let (view, root, text, image) = root_text_image_fixture();
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap();

    assert_eq!(
        snapshot.writing_style_for(root),
        Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
    );
    assert_eq!(
        snapshot.writing_style_for(text),
        Some((WritingMode::VerticalRl, TextOrientation::Mixed))
    );
    assert_eq!(
        snapshot.writing_style_for(image),
        Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
    );
    assert_stylo_writing_preferences_restored();
}

#[test]
fn writing_mode_author_admission_is_limited_to_the_track_sizing_profile() {
    let track_sizing =
        super::super::runtime_grid::runtime_block_flex_grid_track_sizing_author_properties();
    let grid = super::super::runtime_grid::runtime_block_flex_grid_author_properties();

    assert!(track_sizing.contains(&"writing-mode"));
    assert!(!track_sizing.contains(&"text-orientation"));
    assert!(!grid.contains(&"writing-mode"));
    assert!(!grid.contains(&"text-orientation"));
}

#[test]
fn track_sizing_incremental_request_refuses_previous_snapshot_reuse() {
    use super::super::{incremental::CascadeComputeRequest, runtime_grid};

    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:100px;grid-template-rows:40px;margin:0;padding:0",
    );
    let previous = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        StyleRevision::INITIAL,
    )
    .unwrap();
    let properties = runtime_grid::runtime_block_flex_grid_track_sizing_properties();

    let result = super::super::incremental::compute_cascade_with_reuse(CascadeComputeRequest {
        view: &fixture.view,
        additional_user_agent_stylesheets: &[],
        author_stylesheets: &[],
        viewport: viewport(),
        style_revision: StyleRevision::INITIAL,
        properties: &properties,
        profile: ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        reuse: Some((&previous, &[])),
    })
    .unwrap();

    assert!(result.is_none());
    assert_stylo_writing_preferences_restored();
}

#[test]
fn track_sizing_preflight_rejects_pref_disabled_text_orientation_and_restores_preferences() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:100px;grid-template-rows:40px;margin:0;padding:0;text-orientation:upright",
    );

    let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::stylo_dom::cascade::CssCascadeError::UnsupportedInlineAuthorProperty {
            node,
            property,
        } if node == fixture.root && property == "text-orientation"
    ));
    assert_stylo_writing_preferences_restored();
}

#[test]
fn track_sizing_preflight_fails_closed_when_inline_css_cannot_be_fully_scanned() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:100px;grid-template-rows:40px;margin:0;padding:0;malformed",
    );

    let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::stylo_dom::cascade::CssCascadeError::InlineAuthorPropertyPreflightFailed { node }
            if node == fixture.root
    ));
    assert_stylo_writing_preferences_restored();
}

#[test]
fn track_sizing_preflight_rejects_pref_disabled_text_orientation_in_author_stylesheets() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:100px;grid-template-rows:40px;margin:0;padding:0",
    );
    let stylesheet = StylesheetSource {
        id: "c11-4-disabled-text-orientation".to_owned(),
        base_url: "https://spinon.invalid/c11-4-disabled-text-orientation.css".to_owned(),
        origin: CssOrigin::Author,
        css: ".item { text-orientation: upright; }".to_owned(),
    };

    let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[stylesheet],
        viewport(),
        Default::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::stylo_dom::cascade::CssCascadeError::UnsupportedAuthorCss {
            stylesheet_id,
            feature,
        } if stylesheet_id == "c11-4-disabled-text-orientation" && feature == "text-orientation"
    ));
    assert_stylo_writing_preferences_restored();
}

#[test]
fn track_sizing_rejects_computed_writing_modes_outside_the_typed_profile() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:300px;height:200px;grid-template-columns:100px;grid-template-rows:40px;margin:0;padding:0;writing-mode:vertical-lr",
    );

    let error = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[],
        viewport(),
        Default::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::stylo_dom::cascade::CssCascadeError::UnsupportedComputedWritingMode {
            node,
            value,
        } if node == fixture.root && value == "vertical-lr"
    ));
    assert_stylo_writing_preferences_restored();
}

#[test]
fn raw_inline_preflight_recognizes_case_escaped_names_and_parser_recovery() {
    use super::super::runtime_grid::{TextOrientationScan, scan_text_orientation_declaration_list};

    assert_eq!(
        scan_text_orientation_declaration_list("color:red; TEXT-ORIENTATION:upright"),
        TextOrientationScan::Found
    );
    assert_eq!(
        scan_text_orientation_declaration_list(r"color:red; text\2d orientation:upright"),
        TextOrientationScan::Found
    );
    assert_eq!(
        scan_text_orientation_declaration_list(
            "/* before */ color:red; /* between */ TEXT-ORIENTATION : mixed"
        ),
        TextOrientationScan::Found
    );
    assert_eq!(
        scan_text_orientation_declaration_list("color:rgb(1 2 3); text-orientation:mixed"),
        TextOrientationScan::Found
    );
    assert_eq!(
        scan_text_orientation_declaration_list("content:'text-orientation'; color:red"),
        TextOrientationScan::Clean
    );
    assert_eq!(
        scan_text_orientation_declaration_list(
            "/* text-orientation:upright */ content:'text-orientation'; color:red"
        ),
        TextOrientationScan::Clean
    );
    assert_eq!(
        scan_text_orientation_declaration_list("color:red; malformed; width:10px"),
        TextOrientationScan::Incomplete
    );
}

#[test]
fn raw_author_stylesheet_preflight_scans_nested_rules_and_fails_closed_on_depth_limit() {
    use super::super::runtime_grid::{TextOrientationScan, scan_text_orientation_stylesheet};

    assert_eq!(
        scan_text_orientation_stylesheet(
            "@media (min-width: 0px) { .item { TEXT-ORIENTATION:upright; } }"
        ),
        TextOrientationScan::Found
    );
    assert_eq!(
        scan_text_orientation_stylesheet(".item { content: 'text-orientation'; color: red; }"),
        TextOrientationScan::Clean
    );

    let nested = format!(
        "{} .item {{ color:red; }} {}",
        "@media all {".repeat(65),
        "}".repeat(65)
    );
    assert_eq!(
        scan_text_orientation_stylesheet(&nested),
        TextOrientationScan::Incomplete
    );
}

fn assert_stylo_writing_preferences_restored() {
    let _preferences = super::super::device::acquire_standard_stylo_preferences();
    assert!(!stylo_static_prefs::pref!("layout.writing-mode.enabled"));
    assert!(!stylo_static_prefs::pref!("layout.unimplemented"));
}

fn root_text_image_fixture() -> (
    StyloDocumentView,
    spinon_core::NodeId,
    spinon_core::NodeId,
    spinon_core::NodeId,
) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_404).unwrap(), document.document_revision());
    for (node, parent, name, tag, node_style) in [
        (
            root,
            HostParent::Root,
            "root",
            "div",
            Some("display:block;writing-mode:horizontal-tb"),
        ),
        (
            text,
            HostParent::Node(root),
            "text",
            "div",
            Some("display:block;writing-mode:vertical-rl"),
        ),
        (image, HostParent::Node(root), "image", "img", None),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: tag.to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: name.to_owned().into(),
        });
        if let Some(node_style) = node_style {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: node_style.to_owned().into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    let root_id = root.id();
    let text_id = text.id();
    let image_id = image.id();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    (view, root_id, text_id, image_id)
}
