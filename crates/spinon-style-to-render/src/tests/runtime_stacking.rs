use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_block_flex_stacking_cascade_with_stylesheets,
};
use spinon_style_to_layout::compute_runtime_style_layout;

use crate::{CurrentLayoutInputs, StyleRenderError, build_runtime_render_snapshot};

const HTML: &str = "http://www.w3.org/1999/xhtml";

pub(super) fn fixture(
    styles: &[&str],
    parents: &[Option<usize>],
) -> Result<spinon_render::RuntimeRenderSnapshot, StyleRenderError> {
    assert_eq!(styles.len(), parents.len());
    assert!(!styles.is_empty() && parents[0].is_none());

    let mut document = HostDocument::new().unwrap();
    let handles = styles
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(901).unwrap(), document.document_revision());
    for (index, (handle, style)) in handles.iter().zip(styles).enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: *handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: *handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: (*style).to_owned().into(),
        });
        let parent = match parents[index] {
            None => HostParent::Root,
            Some(parent) => HostParent::Node(handles[parent]),
        };
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: *handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    let snapshot = document.snapshot();
    let viewport = CssViewport {
        width_css_px: 120.0,
        height_css_px: 100.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), handles[0])
            .unwrap();
    let styles = compute_runtime_block_flex_stacking_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        Default::default(),
    )
    .unwrap();
    let output = compute_runtime_style_layout(&snapshot, handles[0], styles, viewport).unwrap();
    let current = CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
    build_runtime_render_snapshot(&snapshot, handles[0], &output, current)
}

pub(super) fn ids(snapshot: &spinon_render::RuntimeRenderSnapshot) -> Vec<u64> {
    snapshot
        .boxes()
        .iter()
        .map(|box_| box_.node_id().get())
        .collect()
}

#[test]
fn negative_zero_and_positive_contexts_follow_css_paint_phases() {
    let scene = fixture(
        &[
            "display:block;position:relative;width:120px;height:100px;background-color:#111111",
            "position:absolute;z-index:2;left:0;top:0;width:40px;height:40px;background-color:#ff0000",
            "position:absolute;z-index:-1;left:0;top:0;width:40px;height:40px;background-color:#0000ff",
            "display:block;width:40px;height:20px;background-color:#ffff00",
            "position:absolute;z-index:0;left:0;top:0;width:40px;height:40px;background-color:#00ff00",
        ],
        &[None, Some(0), Some(0), Some(0), Some(0)],
    )
    .unwrap();

    assert_eq!(ids(&scene), [1, 3, 4, 5, 2]);
}

#[test]
fn nested_positive_context_cannot_escape_its_parent_context() {
    let scene = fixture(
        &[
            "display:block;position:relative;width:120px;height:100px;background-color:#111111",
            "position:absolute;z-index:1;left:0;top:0;width:60px;height:60px;background-color:#ff0000",
            "position:absolute;z-index:100;left:0;top:0;width:30px;height:30px;background-color:#00ff00",
            "position:absolute;z-index:2;left:0;top:0;width:60px;height:60px;background-color:#0000ff",
        ],
        &[None, Some(0), Some(1), Some(0)],
    )
    .unwrap();

    assert_eq!(ids(&scene), [1, 2, 3, 4]);
}

#[test]
fn zero_context_subtree_is_atomic_before_a_positive_sibling() {
    let scene = fixture(
        &[
            "display:block;position:relative;width:120px;height:100px;background-color:#111111",
            "position:absolute;z-index:0;left:0;top:0;width:60px;height:60px;background-color:#ff0000",
            "position:absolute;z-index:999;left:0;top:0;width:30px;height:30px;background-color:#00ff00",
            "position:absolute;z-index:1;left:0;top:0;width:60px;height:60px;background-color:#0000ff",
        ],
        &[None, Some(0), Some(1), Some(0)],
    )
    .unwrap();

    assert_eq!(ids(&scene), [1, 2, 3, 4]);
}

#[test]
fn positioned_auto_does_not_trap_descendant_integer_contexts() {
    let scene = fixture(
        &[
            "display:block;position:relative;width:120px;height:100px;background-color:#111111",
            "position:relative;z-index:auto;width:60px;height:60px;background-color:#ff0000",
            "position:absolute;z-index:2;left:0;top:0;width:30px;height:30px;background-color:#00ff00",
            "position:absolute;z-index:1;left:0;top:0;width:60px;height:60px;background-color:#0000ff",
        ],
        &[None, Some(0), Some(1), Some(0)],
    )
    .unwrap();

    assert_eq!(ids(&scene), [1, 2, 4, 3]);
}

#[test]
fn fixed_auto_is_a_real_context_and_static_flex_items_honor_z_index() {
    let fixed_scene = fixture(
        &[
            "display:block;position:relative;width:120px;height:100px;background-color:#111111",
            "position:fixed;left:0;top:0;width:60px;height:60px;background-color:#ff0000",
            "position:absolute;left:0;top:0;width:60px;height:60px;z-index:9999;background-color:#0000ff",
            "position:absolute;left:0;top:0;width:60px;height:60px;z-index:1;background-color:#00ff00",
        ],
        &[None, Some(0), Some(1), Some(0)],
    )
    .unwrap();
    assert_eq!(ids(&fixed_scene), [1, 2, 3, 4]);

    let flex_scene = fixture(
        &[
            "display:flex;position:relative;width:120px;height:100px;flex-direction:row;align-items:flex-start;justify-content:flex-start;background-color:#111111",
            "display:block;position:static;z-index:1;width:100px;height:60px;margin-right:-50px;background-color:#0000ff",
            "display:block;position:static;z-index:2;width:100px;height:60px;background-color:#00ff00",
        ],
        &[None, Some(0), Some(0)],
    )
    .unwrap();
    assert_eq!(ids(&flex_scene), [1, 2, 3]);
}

#[test]
fn flex_order_sets_stable_paint_ties_but_not_absolute_child_order() {
    let scene = fixture(
        &[
            "display:flex;position:relative;width:120px;height:100px;flex-direction:row;align-items:flex-start;justify-content:flex-start;background-color:#111111",
            "display:block;order:2;z-index:0;width:40px;height:40px;background-color:#ff0000",
            "display:block;order:-1;z-index:0;width:40px;height:40px;background-color:#00ff00",
            "position:absolute;order:-99;z-index:0;left:0;top:0;width:40px;height:40px;background-color:#0000ff",
        ],
        &[None, Some(0), Some(0), Some(0)],
    )
    .unwrap();

    assert_eq!(ids(&scene), [1, 3, 2, 4]);
}

#[test]
fn flex_reverse_paint_order_matches_the_pinned_chrome_overlap_matrix() {
    let cases = [
        ("row", [0, 0], [1, 2, 3]),
        ("row", [1, 0], [1, 3, 2]),
        ("row-reverse", [0, 0], [1, 3, 2]),
        ("row-reverse", [1, 0], [1, 2, 3]),
        ("column", [0, 0], [1, 2, 3]),
        ("column", [1, 0], [1, 3, 2]),
        ("column-reverse", [0, 0], [1, 3, 2]),
        ("column-reverse", [1, 0], [1, 2, 3]),
    ];

    for (direction, orders, expected) in cases {
        let (blue_size, red_size, overlap_margin) = if direction.starts_with("row") {
            let margin = if direction.ends_with("reverse") {
                "margin-left:-40px"
            } else {
                "margin-right:-40px"
            };
            (
                "flex:0 0 80px;width:80px;height:60px",
                "flex:0 0 80px;width:80px;height:60px",
                margin,
            )
        } else {
            let margin = if direction.ends_with("reverse") {
                "margin-top:-40px"
            } else {
                "margin-bottom:-40px"
            };
            (
                "flex:0 0 60px;width:100px;height:60px",
                "flex:0 0 60px;width:100px;height:60px",
                margin,
            )
        };
        let blue_style = format!(
            "{blue_size};{overlap_margin};order:{};background-color:#2866f6",
            orders[0]
        );
        let red_style = format!(
            "{red_size};{overlap_margin};order:{};background-color:#e5484d",
            orders[1]
        );
        let root_style = format!(
            "display:flex;position:relative;width:100px;height:100px;flex-direction:{direction};align-items:flex-start;justify-content:flex-start;background-color:#111111"
        );
        let scene = fixture(
            &[&root_style, &blue_style, &red_style],
            &[None, Some(0), Some(0)],
        )
        .unwrap();

        assert_eq!(
            ids(&scene),
            expected,
            "direction={direction}, order={orders:?}"
        );
    }
}

#[test]
fn display_none_subtrees_are_omitted_and_nonvisible_visibility_fails_closed() {
    let scene = fixture(
        &[
            "display:block;position:relative;width:120px;height:100px;background-color:#111111",
            "display:none;width:40px;height:40px;background-color:#ff0000",
            "width:20px;height:20px;background-color:#00ff00",
            "display:block;width:40px;height:20px;background-color:#ffff00",
        ],
        &[None, Some(0), Some(1), Some(0)],
    )
    .unwrap();
    assert_eq!(ids(&scene), [1, 4]);

    let error = fixture(
        &[
            "display:block;position:relative;width:120px;height:100px;background-color:#111111",
            "visibility:hidden;width:40px;height:40px;background-color:#ff0000",
        ],
        &[None, Some(0)],
    )
    .unwrap_err();
    assert!(matches!(
        error,
        StyleRenderError::UnsupportedComputedValue {
            property: "visibility",
            ..
        }
    ));

    let collapse_error = fixture(
        &[
            "display:block;position:relative;width:120px;height:100px;background-color:#111111",
            "visibility:collapse;width:40px;height:40px;background-color:#ff0000",
        ],
        &[None, Some(0)],
    )
    .unwrap_err();
    assert!(matches!(
        collapse_error,
        StyleRenderError::UnsupportedComputedValue {
            property: "visibility",
            ..
        }
    ));
}
