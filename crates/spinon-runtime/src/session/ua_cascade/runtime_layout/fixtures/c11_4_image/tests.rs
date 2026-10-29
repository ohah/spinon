use super::super::super::RuntimeLayoutFailure;
use super::{IMAGE_CASES, ImageCase, Track, prepare, validate_grid_style, validate_item_style};
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostDocumentSnapshot,
    HostNodeHandle, HostParent, NodeId, OwnerId, StyleRevision,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridItemPlacement, ComputedCssGridLayout,
    ComputedCssGridPlacement, ComputedCssGridTrackBreadth, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedLayoutBorder, ComputedLayoutDimensions, ComputedLayoutInsets, ComputedLayoutSpacing,
    ComputedStyleProfile, ComputedStyleSnapshot, CssViewport,
};
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
fn image_matrix_has_unique_ids_and_bounded_geometry() {
    assert_eq!(IMAGE_CASES.len(), 22);
    let mixed_track_case = IMAGE_CASES
        .iter()
        .find(|case| case.scene_id == "image-auto-max-mixed-tracks")
        .expect("the image auto/max mixed-track case is registered");
    assert_eq!(mixed_track_case.root_id, "c114-image-auto-max-mixed-grid");
    assert_eq!(mixed_track_case.item_id, "c114-image-auto-max-mixed-item");
    assert_eq!(mixed_track_case.columns.len(), 2);
    assert!(matches!(
        mixed_track_case.columns[0],
        super::Track::MinMaxAutoMaxContent
    ));
    assert!(matches!(
        mixed_track_case.columns[1],
        super::Track::Length17
    ));
    assert_eq!(mixed_track_case.rows.len(), 1);
    assert_eq!(mixed_track_case.column_gap, 5.0);
    assert!(matches!(mixed_track_case.placement, super::Placement::Auto));
    for (index, case) in IMAGE_CASES.iter().enumerate() {
        assert!(
            IMAGE_CASES[..index]
                .iter()
                .all(|earlier| earlier.scene_id != case.scene_id)
        );
        assert!(case.grid_width.is_finite() && case.grid_width > 0.0);
        assert!(case.grid_height.is_finite() && case.grid_height > 0.0);
        assert!(matches!(case.columns.len(), 1 | 2));
        assert!(matches!(case.rows.len(), 1 | 2));
        assert!(case.column_gap.is_finite() && case.column_gap >= 0.0);
        assert!(case.row_gap.is_finite() && case.row_gap >= 0.0);
        assert!(case.grid_border.is_finite() && case.grid_border >= 0.0);
    }
}

#[test]
fn border_overflow_case_is_fixed_and_rejects_borderless_or_content_box_roots() {
    let case = border_overflow_case();
    assert_eq!(case.root_id, "c114-image-auto-max-border-overflow-grid");
    assert_eq!(case.item_id, "c114-image-auto-max-border-overflow-item");
    assert_eq!(case.grid_width, 12.0);
    assert_eq!(case.grid_height, 40.0);
    assert_eq!(case.grid_border, 1.0);
    assert_eq!(case.column_gap, 5.0);
    assert_eq!(case.row_gap, 0.0);
    assert!(matches!(case.placement, super::Placement::Auto));
    assert!(matches!(case.sizing, super::ImageSizing::Intrinsic));
    assert!(matches!(case.columns[0], Track::MinMaxAutoMaxContent));
    assert!(matches!(case.columns[1], Track::Length17));
    assert_eq!(case.rows.len(), 1);
    assert!(matches!(case.rows[0], Track::Auto));

    let mut style = mixed_root_style(case);
    assert!(validate_grid_style(&style, case).is_ok());

    style.layout_border = ComputedLayoutBorder::default();
    assert_failure(
        validate_grid_style(&style, case),
        "unsupported_c114_image_case",
        "container box geometry",
    );

    let mut style = mixed_root_style(case);
    style
        .properties
        .insert("box-sizing".to_owned(), "content-box".to_owned());
    assert_failure(
        validate_grid_style(&style, case),
        "unsupported_c114_image_style",
        "box-sizing expected border-box",
    );
}

#[test]
fn rejects_unsupported_auto_max_mixed_track_and_root_variants() {
    let case = mixed_track_case();

    let mut style = mixed_root_style(case);
    style.grid_layout.as_mut().unwrap().columns[0] = ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::MinContent,
        max: ComputedCssGridTrackBreadth::MaxContent,
    };
    assert_failure(
        validate_grid_style(&style, case),
        "unsupported_c114_image_case",
        "track functions",
    );

    let mut style = mixed_root_style(case);
    style.grid_layout.as_mut().unwrap().columns[1] = ComputedCssGridTrackSize::LengthPx(18.0);
    assert_failure(
        validate_grid_style(&style, case),
        "unsupported_c114_image_case",
        "track functions",
    );

    let mut style = mixed_root_style(case);
    style.grid_layout.as_mut().unwrap().rows[0] = ComputedCssGridTrackSize::LengthPx(1.0);
    assert_failure(
        validate_grid_style(&style, case),
        "unsupported_c114_image_case",
        "track functions",
    );

    for mutation in [
        RootMutation::Width,
        RootMutation::Height,
        RootMutation::Border,
        RootMutation::ColumnGap,
        RootMutation::RowGap,
    ] {
        let mut style = mixed_root_style(case);
        match mutation {
            RootMutation::Width => {
                style.layout_dimensions.width = ComputedCssDimension::LengthPx(99.0)
            }
            RootMutation::Height => {
                style.layout_dimensions.height = ComputedCssDimension::LengthPx(39.0)
            }
            RootMutation::Border => {
                style.layout_border = ComputedLayoutBorder {
                    top: 1.0,
                    right: 1.0,
                    bottom: 1.0,
                    left: 1.0,
                }
            }
            RootMutation::ColumnGap => {
                style.layout_spacing.column_gap = ComputedCssSpacingValue::LengthPx(4.0)
            }
            RootMutation::RowGap => {
                style.layout_spacing.row_gap = ComputedCssSpacingValue::LengthPx(1.0)
            }
        }
        assert_failure(
            validate_grid_style(&style, case),
            "unsupported_c114_image_case",
            "container box geometry",
        );
    }

    for property in ["align-items", "align-content", "justify-content"] {
        let mut style = mixed_root_style(case);
        style
            .properties
            .insert(property.to_owned(), "center".to_owned());
        assert_failure(
            validate_grid_style(&style, case),
            "unsupported_c114_image_style",
            &format!("{property} expected start"),
        );
    }
}

#[derive(Clone, Copy)]
enum RootMutation {
    Width,
    Height,
    Border,
    ColumnGap,
    RowGap,
}

#[test]
fn rejects_explicit_placement_and_span_for_auto_max_mixed_image() {
    let case = mixed_track_case();
    let mut style = mixed_item_style();
    style.grid_item_placement = Some(ComputedCssGridItemPlacement {
        row_start: ComputedCssGridPlacement::Auto,
        row_end: ComputedCssGridPlacement::Auto,
        column_start: ComputedCssGridPlacement::Line(1),
        column_end: ComputedCssGridPlacement::Span(2),
    });
    assert_failure(
        validate_item_style(&style, case),
        "unsupported_c114_image_case",
        "replaced item box or placement",
    );
}

fn mixed_track_case() -> &'static ImageCase {
    IMAGE_CASES
        .iter()
        .find(|case| case.scene_id == "image-auto-max-mixed-tracks")
        .expect("the auto/max mixed-track scene is registered")
}

fn border_overflow_case() -> &'static ImageCase {
    IMAGE_CASES
        .iter()
        .find(|case| case.scene_id == "image-auto-max-mixed-tracks-border-overflow")
        .expect("the auto/max border-overflow scene is registered")
}

fn mixed_root_style(case: &ImageCase) -> ComputedElementStyle {
    let properties = BTreeMap::from([
        ("display".to_owned(), "grid".to_owned()),
        ("box-sizing".to_owned(), "border-box".to_owned()),
        ("align-items".to_owned(), "start".to_owned()),
        ("align-content".to_owned(), "start".to_owned()),
        ("justify-content".to_owned(), "start".to_owned()),
    ]);
    let grid_layout = ComputedCssGridLayout {
        columns: case.columns.iter().copied().map(computed_track).collect(),
        rows: case.rows.iter().copied().map(computed_track).collect(),
        columns_auto_repeat: None,
        rows_auto_repeat: None,
        columns_line_names: Vec::new(),
        rows_line_names: Vec::new(),
        template_areas: None,
        auto_flow: ComputedCssGridAutoFlow::Row,
        auto_rows: ComputedCssGridAutoRows::Auto,
        auto_columns: ComputedCssGridAutoColumns::Auto,
    };
    style(
        properties,
        ComputedLayoutDimensions {
            width: ComputedCssDimension::LengthPx(case.grid_width),
            height: ComputedCssDimension::LengthPx(case.grid_height),
            min_width: ComputedCssDimension::Auto,
            max_width: ComputedCssMaxSize::None,
            min_height: ComputedCssDimension::Auto,
            max_height: ComputedCssMaxSize::None,
            flex_basis: ComputedCssDimension::Auto,
        },
        spacing(0.0, 0.0, case.row_gap, case.column_gap),
        ComputedLayoutBorder {
            top: case.grid_border,
            right: case.grid_border,
            bottom: case.grid_border,
            left: case.grid_border,
        },
        Some(grid_layout),
        None,
    )
}

fn mixed_item_style() -> ComputedElementStyle {
    style(
        BTreeMap::from([
            ("display".to_owned(), "block".to_owned()),
            ("box-sizing".to_owned(), "border-box".to_owned()),
        ]),
        ComputedLayoutDimensions {
            width: ComputedCssDimension::Auto,
            height: ComputedCssDimension::Auto,
            min_width: ComputedCssDimension::Auto,
            max_width: ComputedCssMaxSize::None,
            min_height: ComputedCssDimension::Auto,
            max_height: ComputedCssMaxSize::None,
            flex_basis: ComputedCssDimension::Auto,
        },
        spacing(0.0, 0.0, 0.0, 0.0),
        ComputedLayoutBorder::default(),
        None,
        None,
    )
}

fn style(
    properties: BTreeMap<String, String>,
    layout_dimensions: ComputedLayoutDimensions,
    layout_spacing: ComputedLayoutSpacing,
    layout_border: ComputedLayoutBorder,
    grid_layout: Option<ComputedCssGridLayout>,
    grid_item_placement: Option<ComputedCssGridItemPlacement>,
) -> ComputedElementStyle {
    ComputedElementStyle {
        node_id: NodeId::new(11_414).expect("non-zero fixture node id"),
        properties,
        layout_position: ComputedCssPosition::Static,
        stacking_z_index: None,
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        c11_4_zoom: None,
        stacking_opacity: None,
        stacking_visibility: None,
        layout_overflow_x: None,
        layout_overflow_y: None,
        overflow_clip_margin: None,
        input_overscroll_behavior_x: None,
        input_overscroll_behavior_y: None,
        input_touch_action: None,
        fixed_containing_block_effects: Vec::new(),
        layout_insets: ComputedLayoutInsets {
            top: ComputedCssSpacingValue::Auto,
            right: ComputedCssSpacingValue::Auto,
            bottom: ComputedCssSpacingValue::Auto,
            left: ComputedCssSpacingValue::Auto,
        },
        font_size_css_px: 16.0,
        layout_dimensions,
        layout_aspect_ratio: None,
        layout_spacing,
        grid_layout,
        grid_item_placement,
        layout_border,
        layout_border_paint: None,
        layout_math_values: BTreeMap::new(),
        background_color: None,
        background_paint: None,
        computed_color: None,
        computed_background_color: None,
    }
}

fn spacing(margin: f32, padding: f32, row_gap: f32, column_gap: f32) -> ComputedLayoutSpacing {
    let edges = |value| spinon_style::ComputedCssEdges {
        top: ComputedCssSpacingValue::LengthPx(value),
        right: ComputedCssSpacingValue::LengthPx(value),
        bottom: ComputedCssSpacingValue::LengthPx(value),
        left: ComputedCssSpacingValue::LengthPx(value),
    };
    ComputedLayoutSpacing {
        margin: edges(margin),
        padding: edges(padding),
        row_gap: ComputedCssSpacingValue::LengthPx(row_gap),
        column_gap: ComputedCssSpacingValue::LengthPx(column_gap),
    }
}

fn computed_track(track: Track) -> ComputedCssGridTrackSize {
    match track {
        Track::Auto => ComputedCssGridTrackSize::Auto,
        Track::MinMaxAutoMaxContent => ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Auto,
            max: ComputedCssGridTrackBreadth::MaxContent,
        },
        Track::Length17 => ComputedCssGridTrackSize::LengthPx(17.0),
        other => panic!("unexpected track in auto/max mixed-track fixture: {other:?}"),
    }
}

fn assert_failure(result: Result<(), RuntimeLayoutFailure>, code: &str, property_fragment: &str) {
    let error = result.expect_err("unsupported image fixture variant must fail closed");
    assert_eq!(error.code, code);
    assert!(
        error
            .property
            .as_deref()
            .is_some_and(|property| property.contains(property_fragment)),
        "unexpected error property: {:?}",
        error.property
    );
}

#[test]
fn rejects_non_html_namespace_for_allowlisted_grid_root() {
    const SVG: &str = "http://www.w3.org/2000/svg";
    let (snapshot, root) = image_tree(
        SVG,
        "c114-image-auto-max-mixed-grid",
        "c114-image-auto-max-mixed-item",
        super::IMAGE_URL,
    );
    let styles = empty_styles(&snapshot);
    let error = match prepare(&snapshot, root, &styles) {
        Err(error) => error,
        Ok(_) => panic!("non-HTML root must not enter the image fixture"),
    };
    assert_failure(
        Err(error),
        "unsupported_c114_image_case",
        "one direct HTML img required",
    );
}

#[test]
fn rejects_wrong_item_id_and_source_before_computed_style_lookup() {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let (snapshot, root) = image_tree(
        HTML,
        "c114-image-auto-max-mixed-grid",
        "wrong-item-id",
        super::IMAGE_URL,
    );
    let styles = empty_styles(&snapshot);
    let error = match prepare(&snapshot, root, &styles) {
        Err(error) => error,
        Ok(_) => panic!("wrong image item id must not enter the fixture"),
    };
    assert_failure(
        Err(error),
        "unsupported_c114_image_case",
        "image tag or id mismatch",
    );

    let (snapshot, root) = image_tree(
        HTML,
        "c114-image-auto-max-mixed-grid",
        "c114-image-auto-max-mixed-item",
        "wrong-source",
    );
    let styles = empty_styles(&snapshot);
    let error = match prepare(&snapshot, root, &styles) {
        Err(error) => error,
        Ok(_) => panic!("wrong image source must not enter the fixture"),
    };
    assert_failure(
        Err(error),
        "unsupported_c114_image_case",
        "fixed PNG source required",
    );

    let (snapshot, root) = image_tree(
        HTML,
        "wrong-root-id",
        "c114-image-auto-max-mixed-item",
        super::IMAGE_URL,
    );
    let styles = empty_styles(&snapshot);
    let error = match prepare(&snapshot, root, &styles) {
        Err(error) => error,
        Ok(_) => panic!("unknown image root id must not enter the fixture"),
    };
    assert_failure(
        Err(error),
        "unsupported_c114_image_fixture_root",
        "wrong-root-id",
    );
}

#[test]
fn prepare_rejects_invalid_auto_max_root_geometry_before_image_input() {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let (snapshot, root) = image_tree(
        HTML,
        "c114-image-auto-max-mixed-grid",
        "c114-image-auto-max-mixed-item",
        super::IMAGE_URL,
    );
    let image = snapshot
        .children(root)
        .and_then(|mut children| children.next())
        .expect("fixture root has its one direct image child");

    let case = mixed_track_case();
    let mut root_style = mixed_root_style(case);
    root_style.node_id = root.id();
    root_style.layout_dimensions.width = ComputedCssDimension::LengthPx(99.0);
    let mut item_style = mixed_item_style();
    item_style.node_id = image.id();
    let styles = styles_with_elements(&snapshot, vec![root_style, item_style]);

    let error = match prepare(&snapshot, root, &styles) {
        Err(error) => error,
        Ok(_) => panic!("invalid root geometry must not produce C11_4ImageInput"),
    };
    assert_failure(
        Err(error),
        "unsupported_c114_image_case",
        "container box geometry",
    );
}

fn image_tree(
    root_namespace: &str,
    root_id: &str,
    item_id: &str,
    source: &str,
) -> (HostDocumentSnapshot, HostNodeHandle) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_414).unwrap(), document.document_revision());
    for (node, namespace, tag, id, parent) in [
        (root, root_namespace, "div", root_id, HostParent::Root),
        (image, HTML, "img", item_id, HostParent::Node(root)),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: namespace.to_owned(),
            local_name: tag.to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    batch.push(DocumentOperation::SetAttribute {
        node: image,
        name: AttributeName::new(None, "src").unwrap(),
        value: source.to_owned().into(),
    });
    document.commit(batch).unwrap();
    (document.snapshot(), root)
}

fn empty_styles(snapshot: &HostDocumentSnapshot) -> ComputedStyleSnapshot {
    styles_with_elements(snapshot, Vec::new())
}

fn styles_with_elements(
    snapshot: &HostDocumentSnapshot,
    elements: Vec<ComputedElementStyle>,
) -> ComputedStyleSnapshot {
    ComputedStyleSnapshot {
        profile: ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        viewport: CssViewport::C04_FIXTURE,
        style_revision: StyleRevision::INITIAL,
        generation: snapshot.generation(),
        document_revision: snapshot.document_revision(),
        render_tree_revision: snapshot.render_tree_revision(),
        document_root_font_size_css_px: 16.0,
        elements: Arc::from(elements),
        writing_styles: Arc::from([]),
        diagnostics: Vec::new(),
    }
}
