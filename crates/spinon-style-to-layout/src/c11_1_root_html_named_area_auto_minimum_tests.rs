use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::{LayoutFrame, TextLayoutEngine};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_root_html_text_paint_cascade_with_stylesheets,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout_with_root_html_grid_text_profiles};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy)]
enum Axis {
    Columns,
    Rows,
}

#[derive(Clone, Copy)]
enum TrackMinimum {
    AutoOne,
    ZeroOne,
    AutoZero,
}

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    items: [HostNodeHandle; 3],
}

impl Fixture {
    fn new(
        axis: Axis,
        minimum: TrackMinimum,
        item_overrides: [&str; 3],
        rename_items: bool,
    ) -> Self {
        Self::with_grid_override(axis, minimum, item_overrides, rename_items, "", false)
    }

    fn with_grid_override(
        axis: Axis,
        minimum: TrackMinimum,
        item_overrides: [&str; 3],
        rename_items: bool,
        grid_override: &str,
        extra_direct_item: bool,
    ) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = [
            document.reserve_node_handle().unwrap(),
            document.reserve_node_handle().unwrap(),
            document.reserve_node_handle().unwrap(),
        ];
        let (width, height, columns, rows, default_items) = match (axis, minimum) {
            (Axis::Columns, TrackMinimum::AutoOne) => (
                "100px",
                "40px",
                "minmax(auto,1fr) 40px",
                "20px 20px",
                [
                    "width:auto;height:20px",
                    "width:80px;height:20px",
                    "width:40px;height:20px",
                ],
            ),
            (Axis::Columns, TrackMinimum::ZeroOne) => (
                "100px",
                "40px",
                "minmax(0,1fr) 40px",
                "20px 20px",
                [
                    "width:auto;height:20px",
                    "width:80px;height:20px",
                    "width:40px;height:20px",
                ],
            ),
            (Axis::Columns, TrackMinimum::AutoZero) => (
                "100px",
                "40px",
                "minmax(auto,0fr) 40px",
                "20px 20px",
                [
                    "width:auto;height:20px",
                    "width:80px;height:20px",
                    "width:40px;height:20px",
                ],
            ),
            (Axis::Rows, TrackMinimum::AutoOne) => (
                "80px",
                "100px",
                "40px 40px",
                "minmax(auto,1fr) 40px",
                [
                    "width:80px;height:80px",
                    "width:40px;height:40px",
                    "width:40px;height:40px",
                ],
            ),
            (Axis::Rows, TrackMinimum::ZeroOne) => (
                "80px",
                "100px",
                "40px 40px",
                "minmax(0,1fr) 40px",
                [
                    "width:80px;height:80px",
                    "width:40px;height:40px",
                    "width:40px;height:40px",
                ],
            ),
            (Axis::Rows, TrackMinimum::AutoZero) => (
                "80px",
                "100px",
                "40px 40px",
                "minmax(auto,0fr) 40px",
                [
                    "width:80px;height:80px",
                    "width:40px;height:40px",
                    "width:40px;height:40px",
                ],
            ),
        };
        let grid_style = format!(
            "display:grid;box-sizing:content-box;width:{width};height:{height};grid-template-columns:{columns};grid-template-rows:{rows};grid-template-areas:'header header' 'side content';gap:0;margin:0;padding:0;border:2px solid;overflow:visible;{grid_override}"
        );
        let item_areas = ["header", "side", "content"];
        let item_defaults = std::array::from_fn::<_, 3, _>(|index| {
            format!(
                "display:block;box-sizing:content-box;{};min-width:auto;min-height:auto;max-width:none;max-height:none;grid-area:{};margin:0;padding:0;border:0;overflow:visible",
                default_items[index], item_areas[index]
            )
        });
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_282).unwrap(), document.document_revision());
        let extra_child = extra_direct_item.then(|| document.reserve_node_handle().unwrap());
        let entries = [
            (
                root,
                HostParent::Root,
                "root",
                "root",
                "display:block;box-sizing:border-box;width:400px;height:400px;margin:0;padding:0;border:0".to_owned(),
            ),
            (
                grid,
                HostParent::Node(root),
                "grid",
                "grid",
                grid_style,
            ),
            (
                items[0],
                HostParent::Node(grid),
                if rename_items { "renamed-a" } else { "header" },
                if rename_items { "renamed-x" } else { "header" },
                item_defaults[0].clone(),
            ),
            (
                items[1],
                HostParent::Node(grid),
                if rename_items { "renamed-b" } else { "side" },
                if rename_items { "renamed-y" } else { "side" },
                item_defaults[1].clone(),
            ),
            (
                items[2],
                HostParent::Node(grid),
                if rename_items { "renamed-c" } else { "content" },
                if rename_items { "renamed-z" } else { "content" },
                item_defaults[2].clone(),
            ),
        ];
        for (node, parent, id, class, style) in entries {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: id.to_owned().into(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "class").unwrap(),
                value: class.to_owned().into(),
            });
            let item_style = items
                .iter()
                .position(|item| *item == node)
                .map(|index| {
                    let override_style = item_overrides[index];
                    if override_style.is_empty() {
                        style.clone()
                    } else {
                        format!("{style};{override_style}")
                    }
                })
                .unwrap_or(style);
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: item_style.into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent,
                node,
                before: None,
            });
        }
        if let Some(node) = extra_child {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: "display:block;box-sizing:content-box;width:1px;height:1px;grid-area:header;margin:0;padding:0;border:0;overflow:visible".to_owned().into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(grid),
                node,
                before: None,
            });
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            grid,
            items,
        }
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view = StyloDocumentView::new(snapshot.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        let viewport = CssViewport {
            width_css_px: 400.0,
            height_css_px: 400.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let computed_styles = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .map_err(StyleLayoutError::Cascade)?;
        compute_runtime_style_layout_with_root_html_grid_text_profiles(
            &snapshot,
            self.root,
            computed_styles,
            viewport,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &TextLayoutEngine::default(),
        )
    }
}

fn assert_frame(actual: LayoutFrame, expected: [f32; 4], name: &str) {
    for (component, (actual, expected)) in [actual.x, actual.y, actual.width, actual.height]
        .into_iter()
        .zip(expected)
        .enumerate()
    {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.02,
            "{name} frame component {component}: actual={actual:?}, expected={expected:?} ± 0.02"
        );
    }
}

#[test]
fn root_html_named_area_auto_minimum_tracks_match_both_axis_oracles() {
    for (axis, minimum, expected_root, expected_items, rename) in [
        (
            Axis::Columns,
            TrackMinimum::AutoOne,
            [0.0, 0.0, 104.0, 44.0],
            [
                [2.0, 2.0, 120.0, 20.0],
                [2.0, 22.0, 80.0, 20.0],
                [82.0, 22.0, 40.0, 20.0],
            ],
            true,
        ),
        (
            Axis::Columns,
            TrackMinimum::ZeroOne,
            [0.0, 0.0, 104.0, 44.0],
            [
                [2.0, 2.0, 100.0, 20.0],
                [2.0, 22.0, 80.0, 20.0],
                [62.0, 22.0, 40.0, 20.0],
            ],
            false,
        ),
        (
            Axis::Columns,
            TrackMinimum::AutoZero,
            [0.0, 0.0, 104.0, 44.0],
            [
                [2.0, 2.0, 120.0, 20.0],
                [2.0, 22.0, 80.0, 20.0],
                [82.0, 22.0, 40.0, 20.0],
            ],
            false,
        ),
        (
            Axis::Rows,
            TrackMinimum::AutoOne,
            [0.0, 0.0, 84.0, 104.0],
            [
                [2.0, 2.0, 80.0, 80.0],
                [2.0, 82.0, 40.0, 40.0],
                [42.0, 82.0, 40.0, 40.0],
            ],
            true,
        ),
        (
            Axis::Rows,
            TrackMinimum::ZeroOne,
            [0.0, 0.0, 84.0, 104.0],
            [
                [2.0, 2.0, 80.0, 80.0],
                [2.0, 62.0, 40.0, 40.0],
                [42.0, 62.0, 40.0, 40.0],
            ],
            false,
        ),
        (
            Axis::Rows,
            TrackMinimum::AutoZero,
            [0.0, 0.0, 84.0, 104.0],
            [
                [2.0, 2.0, 80.0, 80.0],
                [2.0, 82.0, 40.0, 40.0],
                [42.0, 82.0, 40.0, 40.0],
            ],
            false,
        ),
    ] {
        let fixture = Fixture::new(axis, minimum, ["", "", ""], rename);
        let output = fixture.compute().unwrap_or_else(|error| {
            panic!("bounded named-area auto-minimum scene should project: {error:?}")
        });
        assert_frame(
            output.layout.frames[&fixture.grid.id()],
            expected_root,
            "root",
        );
        for (index, expected) in expected_items.into_iter().enumerate() {
            assert_frame(
                output.layout.frames[&fixture.items[index].id()],
                expected,
                ["header", "side", "content"][index],
            );
        }
    }
}

#[test]
fn named_area_auto_minimum_rejects_item_min_max_and_aspect_ratio_changes() {
    for (axis, overrides, property) in [
        (Axis::Columns, ["", "min-width:0", ""], "min-width"),
        (Axis::Rows, ["min-height:0", "", ""], "min-height"),
        (Axis::Columns, ["", "max-width:40px", ""], "max-width"),
        (Axis::Rows, ["max-height:40px", "", ""], "max-height"),
        (
            Axis::Columns,
            ["", "aspect-ratio:1 / 1", ""],
            "aspect-ratio",
        ),
    ] {
        let fixture = Fixture::new(axis, TrackMinimum::AutoOne, overrides, false);
        let error = fixture
            .compute()
            .expect_err("changed item constraints must fail closed");
        assert!(
            matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    property: actual,
                    ..
                } if actual == property
            ),
            "expected {property} rejection, got {error:?}"
        );
    }
}

#[test]
fn named_area_auto_minimum_rejects_container_and_tree_boundary_changes() {
    for (grid_override, property) in [
        ("min-width:0", "min-width"),
        ("max-width:90px", "max-width"),
        ("aspect-ratio:1 / 1", "aspect-ratio"),
        ("width:auto", "width"),
        ("box-sizing:border-box", "box-sizing"),
        ("border:0", "border-width"),
        ("overflow:hidden", "overflow-x"),
        (
            "grid-template-areas:'header header' 'side .'",
            "grid-template-areas",
        ),
    ] {
        let fixture = Fixture::with_grid_override(
            Axis::Columns,
            TrackMinimum::AutoOne,
            ["", "", ""],
            false,
            grid_override,
            false,
        );
        let error = fixture
            .compute()
            .expect_err("changed container or area geometry must fail closed");
        assert!(
            matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    property: actual,
                    ..
                } if actual == property
            ),
            "expected {property} rejection for {grid_override}, got {error:?}"
        );
    }

    for (overrides, property) in [
        (["", "", "overflow:hidden"], "overflow-x"),
        (["", "grid-area:unknown", ""], "grid-area"),
    ] {
        let fixture = Fixture::new(Axis::Columns, TrackMinimum::AutoOne, overrides, false);
        let error = fixture
            .compute()
            .expect_err("changed item overflow or area placement must fail closed");
        assert!(
            matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    property: actual,
                    ..
                } if actual == property
            ),
            "expected {property} rejection, got {error:?}"
        );
    }

    let fixture = Fixture::with_grid_override(
        Axis::Columns,
        TrackMinimum::AutoOne,
        ["", "", ""],
        false,
        "",
        true,
    );
    let error = fixture
        .compute()
        .expect_err("an extra direct Grid child must not enter the fixed named-area profile");
    assert!(matches!(
        error,
        StyleLayoutError::UnsupportedComputedValue {
            property: "grid-template-areas",
            ..
        }
    ));
}
