use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};

use crate::{
    FlexDirection, LayoutAlignItems, LayoutBorder, LayoutDimension, LayoutDisplay, LayoutEdges,
    LayoutEngine, LayoutError, LayoutInput, LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot,
    LayoutLengthPercentage, LayoutSourceRevision, LayoutStyle, TaffyLayoutEngine, Viewport,
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
fn partial_auto_leaf_is_rejected_when_measurement_is_supplied() {
    let (document, root, image) = document();
    let image_style = LayoutStyle {
        width: LayoutDimension::Fixed(48.0),
        ..image_style()
    };
    let input = layout_input(&document, root, image, root_style(), image_style);
    let snapshot = image_snapshot(&document, image, LayoutIntrinsicSize::new(24.0, 16.0));
    assert!(matches!(
        TaffyLayoutEngine.compute_with_intrinsic_size_snapshot(&input, &snapshot),
        Err(LayoutError::UnsupportedIntrinsicSizing { node, .. }) if node == image.id()
    ));
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
fn measurement_on_non_leaf_and_invalid_dimensions_are_rejected() {
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
