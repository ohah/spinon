use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};

use crate::{
    FlexDirection, GridContainerInput, GridPlacementMode, GridTrackSize, LayoutAlignItems,
    LayoutBorder, LayoutDimension, LayoutDisplay, LayoutEdges, LayoutEngine, LayoutError,
    LayoutInput, LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutLengthPercentage,
    LayoutPosition, LayoutPositioning, LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine,
    Viewport,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const WIDTH: f32 = 120.0;
const HEIGHT: f32 = 80.0;

fn document() -> (HostDocument, HostNodeHandle, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(1414).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "main".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: image,
            namespace: HTML.to_owned(),
            local_name: "img".to_owned(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: image,
            before: None,
        });
    document.commit(batch).unwrap();
    (document, root, image)
}

fn source(document: &HostDocument) -> LayoutSourceRevision {
    let snapshot = document.snapshot();
    LayoutSourceRevision::HostDocument {
        generation: snapshot.generation(),
        document: snapshot.document_revision(),
        render_tree: snapshot.render_tree_revision(),
    }
}

fn image_snapshot(
    document: &HostDocument,
    image: HostNodeHandle,
    size: LayoutIntrinsicSize,
) -> LayoutIntrinsicSizeSnapshot {
    LayoutIntrinsicSizeSnapshot::new(
        source(document),
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        BTreeMap::from([(image.id(), size)]),
    )
    .unwrap()
}

fn layout_input(
    document: &HostDocument,
    root: HostNodeHandle,
    image: HostNodeHandle,
    root_style: LayoutStyle,
    image_style: LayoutStyle,
) -> LayoutInput {
    LayoutInput::from_host_document(
        &document.snapshot(),
        root,
        Viewport {
            width: WIDTH,
            height: HEIGHT,
        },
        &BTreeMap::from([(root.id(), root_style), (image.id(), image_style)]),
        StyleRevision::default(),
        EnvironmentRevision::default(),
    )
    .unwrap()
}

fn root_style() -> LayoutStyle {
    LayoutStyle {
        display: LayoutDisplay::Block,
        width: LayoutDimension::Fixed(WIDTH),
        height: LayoutDimension::Fixed(HEIGHT),
        ..LayoutStyle::default()
    }
}

fn image_style() -> LayoutStyle {
    LayoutStyle {
        display: LayoutDisplay::Block,
        ..LayoutStyle::default()
    }
}

#[test]
fn measured_leaf_uses_css_pixel_snapshot_and_stamps_its_revision() {
    let (document, root, image) = document();
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(
            &layout_input(&document, root, image, root_style(), image_style()),
            &snapshot,
        )
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 0.0, 24.0, 16.0)
    );
    assert_eq!(output.revision.intrinsic_size(), snapshot.revision());
}

#[test]
fn grid_image_intrinsic_measurement_keeps_style_source_and_both_revisions() {
    let (document, root, image) = document();
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let mut root_style = root_style();
    root_style.display = LayoutDisplay::Grid;
    let mut image_style = image_style();
    image_style.width = LayoutDimension::MinContent;
    let input = layout_input(&document, root, image, root_style, image_style).with_grid_layout(
        BTreeMap::from([(
            root.id(),
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(WIDTH)],
                rows: vec![GridTrackSize::LengthPx(HEIGHT)],
                placement_mode: GridPlacementMode::TemplateGrid,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::new(),
    );

    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let measure = output
        .intrinsic_measurements
        .iter()
        .find(|measure| measure.node == image.id())
        .expect("Grid image measure callback is recorded");

    assert_eq!(measure.revision, output.revision);
    assert_eq!(measure.revision.intrinsic_size(), snapshot.revision());
    assert!(matches!(
        measure.source,
        crate::LayoutIntrinsicMeasurementSource::Replaced {
            natural,
        } if natural == LayoutIntrinsicSize::new(24.0, 16.0)
    ));
    assert_eq!(measure.item_style, image_style);
    assert_eq!(
        (measure.content_size.width, measure.content_size.height),
        (24.0, 16.0)
    );
}

#[test]
fn intrinsic_measurement_reservation_failure_discards_output_and_recovers() {
    let (document, root, image) = document();
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let mut root_style = root_style();
    root_style.display = LayoutDisplay::Grid;
    let mut item_style = image_style();
    item_style.width = LayoutDimension::MinContent;
    let input = layout_input(&document, root, image, root_style, item_style).with_grid_layout(
        BTreeMap::from([(
            root.id(),
            GridContainerInput {
                columns: vec![GridTrackSize::LengthPx(WIDTH)],
                rows: vec![GridTrackSize::LengthPx(HEIGHT)],
                placement_mode: GridPlacementMode::TemplateGrid,
                ..GridContainerInput::default()
            },
        )]),
        BTreeMap::new(),
    );
    let engine = TaffyLayoutEngine;
    let guard = crate::calc_tree::fail_next_intrinsic_measurement_reservation();
    let failed = engine.compute_with_intrinsic_size_snapshot(&input, &snapshot);
    let observed_failure = crate::test_allocator::take_failure_observed();
    assert!(observed_failure);
    assert!(matches!(
        failed,
        Err(LayoutError::IntrinsicMeasurementAllocationFailure { node }) if node == image.id()
    ));
    drop(guard);

    let recovered = engine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .expect("같은 worker의 다음 layout 계산은 다시 성공해야 합니다");
    assert!(
        recovered
            .intrinsic_measurements
            .iter()
            .any(|measurement| measurement.node == image.id())
    );
    assert!(!crate::test_allocator::take_failure_observed());
}

#[test]
fn hidden_leaf_does_not_require_measurement() {
    let (document, root, image) = document();
    let root_style = LayoutStyle {
        display: LayoutDisplay::None,
        width: LayoutDimension::Fixed(WIDTH),
        height: LayoutDimension::Fixed(HEIGHT),
        ..LayoutStyle::default()
    };
    let output = TaffyLayoutEngine
        .compute(&layout_input(
            &document,
            root,
            image,
            root_style,
            image_style(),
        ))
        .unwrap();
    assert_eq!(output.frames[&image.id()].width, 0.0);
    assert_eq!(output.frames[&image.id()].height, 0.0);
}

#[test]
fn partial_auto_dimensions_transfer_ratio_and_clamp_the_auto_axis() {
    let (document, root, image) = document();
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    for (style, expected) in [
        (
            LayoutStyle {
                width: LayoutDimension::Fixed(48.0),
                ..image_style()
            },
            (48.0, 32.0),
        ),
        (
            LayoutStyle {
                height: LayoutDimension::Fixed(24.0),
                ..image_style()
            },
            (36.0, 24.0),
        ),
        (
            LayoutStyle {
                width: LayoutDimension::Fixed(48.0),
                max_height: LayoutDimension::Fixed(20.0),
                ..image_style()
            },
            (48.0, 20.0),
        ),
        (
            LayoutStyle {
                height: LayoutDimension::Fixed(24.0),
                max_width: LayoutDimension::Fixed(30.0),
                ..image_style()
            },
            (30.0, 24.0),
        ),
    ] {
        let input = layout_input(&document, root, image, root_style(), style);
        let output = TaffyLayoutEngine
            .compute_with_intrinsic_size_snapshot(&input, &snapshot)
            .unwrap();
        let frame = output.frames[&image.id()];
        assert_eq!((frame.width, frame.height), expected);
    }
}

#[test]
fn successful_empty_and_partial_natural_metadata_use_default_object_axes() {
    let (document, root, image) = document();
    for (natural, expected) in [
        (LayoutIntrinsicSize::no_natural_dimensions(), (300.0, 150.0)),
        (
            LayoutIntrinsicSize::from_natural_parts(Some(30.0), None, None),
            (30.0, 150.0),
        ),
        (
            LayoutIntrinsicSize::from_natural_parts(None, Some(30.0), None),
            (300.0, 30.0),
        ),
        (
            LayoutIntrinsicSize::from_natural_parts(Some(40.0), None, Some(2.0)),
            (40.0, 20.0),
        ),
    ] {
        let input = layout_input(&document, root, image, root_style(), image_style());
        let snapshot = image_snapshot(&document, image, natural);
        let output = TaffyLayoutEngine
            .compute_with_intrinsic_size_snapshot(&input, &snapshot)
            .unwrap();
        let frame = output.frames[&image.id()];
        assert_eq!((frame.width, frame.height), expected);
    }
}

#[test]
fn ratio_only_auto_auto_is_rejected_but_an_explicit_axis_can_use_its_ratio() {
    let (document, root, image) = document();
    let ratio_only = LayoutIntrinsicSize::from_natural_parts(None, None, Some(1.5));
    let snapshot = image_snapshot(&document, image, ratio_only);
    let auto_input = layout_input(&document, root, image, root_style(), image_style());
    assert!(matches!(
        TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&auto_input, &snapshot),
        Err(LayoutError::UnsupportedIntrinsicSizing { node, .. }) if node == image.id()
    ));

    let explicit_width = LayoutStyle {
        width: LayoutDimension::Fixed(48.0),
        ..image_style()
    };
    let explicit_input = layout_input(&document, root, image, root_style(), explicit_width);
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&explicit_input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (48.0, 32.0));
}

#[test]
fn authored_ratio_overrides_natural_ratio_and_border_box_edges_apply_once() {
    let (document, root, image) = document();
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let ratio_style = LayoutStyle {
        width: LayoutDimension::Fixed(48.0),
        aspect_ratio: Some(1.0),
        ..image_style()
    };
    let input = layout_input(&document, root, image, root_style(), ratio_style);
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (48.0, 48.0));

    let border_box_ratio_style = LayoutStyle {
        box_sizing: crate::LayoutBoxSizing::BorderBox,
        width: LayoutDimension::Fixed(48.0),
        aspect_ratio: Some(1.5),
        padding: LayoutEdges {
            top: LayoutLengthPercentage::length(2.0),
            right: LayoutLengthPercentage::length(2.0),
            bottom: LayoutLengthPercentage::length(2.0),
            left: LayoutLengthPercentage::length(2.0),
        },
        border: LayoutBorder {
            top: 1.0,
            right: 1.0,
            bottom: 1.0,
            left: 1.0,
        },
        ..image_style()
    };
    let input = layout_input(&document, root, image, root_style(), border_box_ratio_style);
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (48.0, 32.0));

    let border_box_style = LayoutStyle {
        box_sizing: crate::LayoutBoxSizing::BorderBox,
        width: LayoutDimension::Fixed(48.0),
        padding: LayoutEdges {
            top: LayoutLengthPercentage::length(2.0),
            right: LayoutLengthPercentage::length(2.0),
            bottom: LayoutLengthPercentage::length(2.0),
            left: LayoutLengthPercentage::length(2.0),
        },
        border: LayoutBorder {
            top: 1.0,
            right: 1.0,
            bottom: 1.0,
            left: 1.0,
        },
        ..image_style()
    };
    let input = layout_input(&document, root, image, root_style(), border_box_style);
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (48.0, 34.0));
}

#[test]
fn explicit_natural_ratio_must_match_both_natural_dimensions() {
    let (document, root, image) = document();
    let input = layout_input(&document, root, image, root_style(), image_style());
    let snapshot = image_snapshot(
        &document,
        image,
        LayoutIntrinsicSize::from_natural_parts(Some(24.0), Some(16.0), Some(2.0)),
    );
    assert!(matches!(
        TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &snapshot),
        Err(LayoutError::InvalidIntrinsicSize { node, axis: "aspect ratio" })
            if node == image.id()
    ));
}

#[test]
fn definite_image_percentage_uses_the_parent_content_basis() {
    let (document, root, image) = document();
    let input = layout_input(
        &document,
        root,
        image,
        root_style(),
        LayoutStyle {
            width: LayoutDimension::Percent(0.5),
            ..image_style()
        },
    );
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (60.0, 40.0));
}

#[test]
fn percentage_edges_and_authored_ratio_constraints_fail_closed() {
    let (document, root, image) = document();
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    for style in [
        LayoutStyle {
            padding: LayoutEdges {
                top: LayoutLengthPercentage::percent(0.1),
                ..LayoutEdges::default()
            },
            ..image_style()
        },
        LayoutStyle {
            aspect_ratio: Some(1.0),
            max_height: LayoutDimension::Fixed(20.0),
            ..image_style()
        },
    ] {
        let input = layout_input(&document, root, image, root_style(), style);
        assert!(matches!(
            TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &snapshot),
            Err(LayoutError::UnsupportedIntrinsicSizing { node, .. }) if node == image.id()
        ));
    }

    for invalid in [
        LayoutIntrinsicSize::from_natural_parts(None, None, Some(0.0)),
        LayoutIntrinsicSize::from_natural_parts(None, None, Some(f32::NAN)),
        LayoutIntrinsicSize::from_natural_parts(None, None, Some(f32::INFINITY)),
        LayoutIntrinsicSize::from_natural_parts(Some(f32::MAX), Some(f32::MIN_POSITIVE), None),
    ] {
        let input = layout_input(&document, root, image, root_style(), image_style());
        let snapshot = image_snapshot(&document, image, invalid);
        assert!(matches!(
            TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &snapshot),
            Err(LayoutError::InvalidIntrinsicSize { node, .. }) if node == image.id()
        ));
    }
}

#[test]
fn fully_definite_leaf_size_ignores_cached_natural_size() {
    let (document, root, image) = document();
    let image_style = LayoutStyle {
        width: LayoutDimension::Fixed(48.0),
        height: LayoutDimension::Fixed(32.0),
        ..image_style()
    };
    let input = layout_input(&document, root, image, root_style(), image_style);
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (48.0, 32.0));
}

#[test]
fn padding_and_border_are_added_once_to_intrinsic_content_size() {
    let (document, root, image) = document();
    let image_style = LayoutStyle {
        padding: LayoutEdges {
            top: LayoutLengthPercentage::length(2.0),
            right: LayoutLengthPercentage::length(2.0),
            bottom: LayoutLengthPercentage::length(2.0),
            left: LayoutLengthPercentage::length(2.0),
        },
        border: LayoutBorder {
            top: 1.0,
            right: 1.0,
            bottom: 1.0,
            left: 1.0,
        },
        ..image_style()
    };
    let input = layout_input(&document, root, image, root_style(), image_style);
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (30.0, 22.0));
}

#[test]
fn flex_auto_basis_uses_intrinsic_measurement() {
    let (document, root, image) = document();
    let root_style = LayoutStyle {
        display: LayoutDisplay::Flex,
        flex_direction: FlexDirection::Row,
        align_items: LayoutAlignItems::FlexStart,
        ..root_style()
    };
    let input = layout_input(&document, root, image, root_style, image_style());
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (24.0, 16.0));
}

#[test]
fn flex_used_size_keeps_auto_axis_min_max_result_after_parent_layout() {
    let (document, root, image) = document();
    let root_style = LayoutStyle {
        display: LayoutDisplay::Flex,
        flex_direction: FlexDirection::Row,
        align_items: LayoutAlignItems::FlexStart,
        ..root_style()
    };
    let image_style = LayoutStyle {
        width: LayoutDimension::Fixed(48.0),
        max_height: LayoutDimension::Fixed(20.0),
        flex_shrink: 0.0,
        ..image_style()
    };
    let input = layout_input(&document, root, image, root_style, image_style);
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (48.0, 20.0));
}

#[test]
fn flex_known_cross_axis_resolves_auto_main_axis_with_ratio_and_maximum() {
    let (document, root, image) = document();
    let root_style = LayoutStyle {
        display: LayoutDisplay::Flex,
        flex_direction: FlexDirection::Row,
        align_items: LayoutAlignItems::FlexStart,
        ..root_style()
    };
    let image_style = LayoutStyle {
        height: LayoutDimension::Fixed(24.0),
        max_width: LayoutDimension::Fixed(30.0),
        flex_shrink: 0.0,
        ..image_style()
    };
    let input = layout_input(&document, root, image, root_style, image_style);
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (30.0, 24.0));
}

#[test]
fn positioned_image_uses_the_same_replaced_size_in_flow_and_visual_passes() {
    let (document, root, image) = document();
    let input = layout_input(
        &document,
        root,
        image,
        root_style(),
        LayoutStyle {
            width: LayoutDimension::Fixed(48.0),
            ..image_style()
        },
    )
    .with_positioning(BTreeMap::from([(
        image.id(),
        LayoutPositioning {
            position: LayoutPosition::Absolute,
            inset: LayoutEdges::auto(),
        },
    )]));
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let visual = output.frames[&image.id()];
    let flow = output.flow_frames[&image.id()];
    assert_eq!((visual.width, visual.height), (48.0, 32.0));
    assert_eq!((flow.width, flow.height), (48.0, 32.0));
}

#[test]
fn min_max_constraints_clamp_measured_leaf_size() {
    let (document, root, image) = document();
    let image_style = LayoutStyle {
        max_width: LayoutDimension::Fixed(20.0),
        min_height: LayoutDimension::Fixed(20.0),
        ..image_style()
    };
    let input = layout_input(&document, root, image, root_style(), image_style);
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &snapshot)
        .unwrap();
    let frame = output.frames[&image.id()];
    assert_eq!((frame.width, frame.height), (20.0, 20.0));
}

#[test]
fn measurement_snapshot_must_match_source_and_reference_known_nodes() {
    let (document, root, image) = document();
    let input = layout_input(&document, root, image, root_style(), image_style());
    let valid = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    let wrong_source = LayoutIntrinsicSizeSnapshot::new(
        LayoutSourceRevision::Tree(Default::default()),
        valid.revision(),
        valid.sizes().clone(),
    )
    .unwrap();
    assert!(matches!(
        TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &wrong_source),
        Err(LayoutError::IntrinsicSizeSourceMismatch { .. })
    ));

    let unknown = LayoutIntrinsicSizeSnapshot::new(
        source(&document),
        IntrinsicSizeRevision::INITIAL,
        BTreeMap::from([(
            spinon_core::NodeId::new(999).unwrap(),
            LayoutIntrinsicSize::new(24.0, 16.0),
        )]),
    )
    .unwrap();
    assert!(matches!(
        TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &unknown),
        Err(LayoutError::UnknownIntrinsicSizeNode(id))
            if id == spinon_core::NodeId::new(999).unwrap()
    ));
}

#[test]
fn measurement_on_non_leaf_and_invalid_dimensions_are_rejected_but_zero_pending_is_valid() {
    let (document, root, image) = document();
    let input = layout_input(&document, root, image, root_style(), image_style());
    let non_leaf = LayoutIntrinsicSizeSnapshot::new(
        source(&document),
        IntrinsicSizeRevision::INITIAL,
        BTreeMap::from([(root.id(), LayoutIntrinsicSize::new(24.0, 16.0))]),
    )
    .unwrap();
    assert!(matches!(
        TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &non_leaf),
        Err(LayoutError::UnsupportedIntrinsicSizing { node, .. }) if node == root.id()
    ));

    let pending = image_snapshot(&document, image, LayoutIntrinsicSize::new(0.0, 0.0));
    let output = TaffyLayoutEngine
        .compute_with_intrinsic_size_snapshot(&input, &pending)
        .unwrap();
    assert_eq!(
        (
            output.frames[&image.id()].width,
            output.frames[&image.id()].height
        ),
        (0.0, 0.0)
    );

    for invalid in [
        LayoutIntrinsicSize::new(0.0, 16.0),
        LayoutIntrinsicSize::new(-1.0, 16.0),
        LayoutIntrinsicSize::new(f32::NAN, 16.0),
        LayoutIntrinsicSize::new(24.0, f32::INFINITY),
    ] {
        let snapshot = image_snapshot(&document, image, invalid);
        assert!(matches!(
            TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &snapshot),
            Err(LayoutError::InvalidIntrinsicSize { node, .. }) if node == image.id()
        ));
    }
}

#[test]
fn duplicate_measurement_node_is_rejected_during_snapshot_creation() {
    let (document, _, image) = document();
    let size = LayoutIntrinsicSize::new(24.0, 16.0);
    assert!(matches!(
        LayoutIntrinsicSizeSnapshot::new(
            source(&document),
            IntrinsicSizeRevision::INITIAL,
            [(image.id(), size), (image.id(), size)],
        ),
        Err(LayoutError::DuplicateIntrinsicSizeNode(id)) if id == image.id()
    ));
}
