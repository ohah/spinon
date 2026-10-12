use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{CssMediaEnvironment, CssViewport, StyloDocumentView};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_block_flex_grid_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct GridFixture {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    items: BTreeMap<&'static str, HostNodeHandle>,
}

impl GridFixture {
    fn new(grid_style: &str) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let first = document.reserve_node_handle().unwrap();
        let second = document.reserve_node_handle().unwrap();
        let third = document.reserve_node_handle().unwrap();
        let fourth = document.reserve_node_handle().unwrap();
        let items = BTreeMap::from([
            ("first", first),
            ("second", second),
            ("third", third),
            ("fourth", fourth),
        ]);
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_101).unwrap(), document.document_revision());
        for (name, node, parent, style) in [
            (
                "root",
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:360px;height:800px;margin:0;padding:0",
            ),
            ("grid", grid, HostParent::Node(root), grid_style),
            (
                "first",
                first,
                HostParent::Node(grid),
                "display:block;box-sizing:border-box;width:40px;height:20px;margin:0;padding:0",
            ),
            (
                "second",
                second,
                HostParent::Node(grid),
                "display:block;box-sizing:border-box;width:50px;height:24px;margin:0;padding:0",
            ),
            (
                "third",
                third,
                HostParent::Node(grid),
                "display:block;box-sizing:border-box;width:60px;height:26px;margin:0;padding:0",
            ),
            (
                "fourth",
                fourth,
                HostParent::Node(grid),
                "display:block;box-sizing:border-box;width:30px;height:18px;margin:0;padding:0",
            ),
        ] {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: name.to_owned().into(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.to_owned().into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent,
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
        self.compute_with(viewport(), StyleRevision::INITIAL)
    }

    fn compute_with(
        &self,
        viewport: CssViewport,
        style_revision: StyleRevision,
    ) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let document = self.document.snapshot();
        let view = StyloDocumentView::new(document.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        compute_runtime_block_flex_grid_style_layout(
            &document,
            &view,
            self.root,
            &[],
            viewport,
            style_revision,
        )
    }

    fn item(&self, name: &'static str) -> spinon_core::NodeId {
        self.items[name].id()
    }
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 360.0,
        height_css_px: 800.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    }
}

fn grid_style(extra: &str) -> String {
    format!(
        "display:grid;box-sizing:border-box;width:200px;height:100px;grid-template-columns:50px 1fr;grid-template-rows:40px 1fr;column-gap:10px;row-gap:5px;margin:0;padding:0;{extra}"
    )
}

#[test]
fn style_layout_rejects_a_root_different_from_the_stylo_mount_root() {
    let fixture = GridFixture::new(&grid_style(""));
    let snapshot = fixture.document.snapshot();
    let view =
        StyloDocumentView::new(snapshot.clone(), fixture.root, true, QuirksMode::NoQuirks).unwrap();

    let error = compute_runtime_block_flex_grid_style_layout(
        &snapshot,
        &view,
        fixture.grid,
        &[],
        viewport(),
        StyleRevision::INITIAL,
    )
    .unwrap_err();

    assert!(
        matches!(error, StyleLayoutError::SnapshotMismatch { field: "root" }),
        "layout root must be identical to the Stylo view mount root: {error:?}"
    );
}

#[test]
fn stylo_tracks_reach_grid_layout_and_preserve_source_child_order() {
    let fixture = GridFixture::new(&grid_style(""));
    let output = fixture.compute().unwrap();
    let first = fixture.item("first");
    let second = fixture.item("second");
    let third = fixture.item("third");

    assert_eq!(output.layout.frames[&fixture.grid.id()].width, 200.0);
    assert_eq!(output.layout.frames[&first].x, 0.0);
    assert_eq!(output.layout.frames[&first].y, 0.0);
    assert_eq!(output.layout.frames[&second].x, 60.0);
    assert_eq!(output.layout.frames[&second].y, 0.0);
    assert_eq!(output.layout.frames[&third].x, 0.0);
    assert_eq!(output.layout.frames[&third].y, 45.0);
}

#[test]
fn fixed_tracks_gap_scene_matches_pinned_chromium_154_geometry() {
    // Chrome 154 고정 기준: c11-fixed-grid scene, 2026-10-11 capture.
    // 기준 JSON에서 node별 geometry를 읽어 CSS px 오차 0.5 이하인지 비교합니다.
    let reference: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/css/references/c11-grid-precomparison-v2.json"
    ))
    .unwrap();
    assert_eq!(
        reference["referenceId"],
        "chromium-darwin-arm64-Chrome-154.0.8037.98-d9996da8123a-83fc71ce41b4-ca4062baab2b"
    );
    let case = reference["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|observation| observation["deviceScaleFactor"] == 1)
        .unwrap()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "fixed-tracks-gap")
        .unwrap();
    let fixture = GridFixture::new(
        "display:grid;box-sizing:border-box;width:320px;height:130px;grid-template-columns:80px 100px 120px;grid-template-rows:50px 70px;column-gap:8px;row-gap:6px;margin:0;padding:0",
    );
    let output = fixture.compute().unwrap();
    let nodes = [
        ("c11-fixed-grid", fixture.grid.id()),
        ("c11-fixed-a", fixture.item("first")),
        ("c11-fixed-b", fixture.item("second")),
        ("c11-fixed-c", fixture.item("third")),
        ("c11-fixed-d", fixture.item("fourth")),
    ];
    for (reference_id, node) in nodes {
        let expected = case["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["id"] == reference_id)
            .unwrap()["rect"]
            .as_object()
            .unwrap();
        let expected = ["x", "y", "width", "height"].map(|field| {
            expected[field]
                .as_f64()
                .expect("Chromium frame field must be numeric") as f32
        });
        let frame = &output.layout.frames[&node];
        let actual = [frame.x, frame.y, frame.width, frame.height];
        for ((field, actual), expected) in ["x", "y", "width", "height"]
            .into_iter()
            .zip(actual)
            .zip(expected)
        {
            assert!(
                (actual - expected).abs() <= 0.5,
                "Chrome 고정 기준과 다릅니다: {reference_id}/{field}, 실제 {actual}, 기준 {expected}"
            );
        }
    }
}

#[test]
fn grid_layout_keeps_document_style_and_environment_revisions_together() {
    let fixture = GridFixture::new(&grid_style(""));
    let snapshot = fixture.document.snapshot();
    let style_revision = StyleRevision::INITIAL.checked_next().unwrap();
    let mut viewport = viewport();
    viewport.environment_revision = viewport.environment_revision.checked_next().unwrap();
    let output = fixture.compute_with(viewport, style_revision).unwrap();

    assert_eq!(output.computed_styles.style_revision, style_revision);
    assert_eq!(output.computed_styles.viewport, viewport);
    assert_eq!(output.layout.revision.style(), style_revision);
    assert_eq!(
        output.layout.revision.environment(),
        viewport.environment_revision
    );
    assert_eq!(
        output.layout.revision.source(),
        spinon_layout::LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        }
    );
}

#[test]
fn hidden_grid_subtree_does_not_require_explicit_track_values() {
    let fixture = GridFixture::new("display:none;margin:0;padding:0");
    let output = fixture.compute().unwrap();

    let computed_grid = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == fixture.grid.id())
        .unwrap();
    assert_eq!(computed_grid.properties["display"], "none");
    assert!(computed_grid.grid_layout.is_none());
    let grid_frame = &output.layout.frames[&fixture.grid.id()];
    assert_eq!(
        [grid_frame.width, grid_frame.height],
        [0.0, 0.0],
        "display:none Grid는 0 크기 출력으로 숨겨져야 합니다"
    );
    for item in fixture.items.values() {
        let frame = &output.layout.frames[&item.id()];
        assert_eq!([frame.width, frame.height], [0.0, 0.0]);
    }
}

#[test]
fn unsupported_auto_flow_is_not_silently_projected_as_row_flow() {
    let fixture = GridFixture::new(&grid_style("grid-auto-flow:column"));
    let error = fixture.compute().unwrap_err();
    assert!(
        matches!(
            error,
            StyleLayoutError::Layout(spinon_layout::LayoutError::UnsupportedGridAutoFlow(node))
                if node == fixture.grid.id()
        ),
        "{error:?}"
    );
}

#[test]
fn unsupported_inline_grid_placement_fails_before_auto_placement() {
    let fixture = GridFixture::new(&grid_style("grid-column:2"));
    let error = fixture.compute().unwrap_err();

    assert!(
        matches!(
            &error,
            StyleLayoutError::UnsupportedInlineProperty { node, property }
                if *node == fixture.grid.id() && property == "grid-column-start"
        ),
        "explicit inline placement must not be mistaken for auto-placement: {error:?}"
    );
}

#[test]
fn insufficient_explicit_cells_fail_instead_of_creating_an_implicit_track() {
    let fixture = GridFixture::new(
        "display:grid;box-sizing:border-box;width:200px;height:100px;grid-template-columns:50px;grid-template-rows:40px;column-gap:0;row-gap:0;margin:0;padding:0",
    );
    let error = fixture.compute().unwrap_err();
    assert!(
        matches!(
            error,
            StyleLayoutError::Layout(spinon_layout::LayoutError::GridImplicitTrackRequired(node))
                if node == fixture.grid.id()
        ),
        "{error:?}"
    );
}

#[test]
fn indefinite_grid_axis_fails_with_specific_basis() {
    let fixture = GridFixture::new(
        "display:grid;box-sizing:border-box;width:200px;height:auto;grid-template-columns:50px;grid-template-rows:40px 1fr;column-gap:0;row-gap:0;margin:0;padding:0",
    );
    let error = fixture.compute().unwrap_err();
    assert!(
        matches!(
            error,
            StyleLayoutError::Layout(spinon_layout::LayoutError::IndefiniteGridBasis {
                node,
                axis: "height"
            }) if node == fixture.grid.id()
        ),
        "{error:?}"
    );
}
