use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::TextLayoutEngine;
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_root_html_text_paint_cascade_with_stylesheets,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout_with_root_html_grid_text_profiles};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

struct GridFixture {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    items: Vec<HostNodeHandle>,
    viewport: CssViewport,
}

impl GridFixture {
    fn new(width: f32, height: f32, content_alignment: &str, item_count: usize) -> Self {
        Self::with_tracks(
            width,
            height,
            content_alignment,
            item_count,
            "40px 60px",
            "30px 50px",
        )
    }

    fn with_tracks(
        width: f32,
        height: f32,
        content_alignment: &str,
        item_count: usize,
        columns: &str,
        rows: &str,
    ) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = (0..item_count)
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let grid_style = format!(
            "display:grid;box-sizing:border-box;width:{width}px;height:{height}px;grid-template-columns:{columns};grid-template-rows:{rows};column-gap:10px;row-gap:10px;align-content:{};justify-content:{};overflow:visible;margin:0;padding:0",
            content_alignment.split('|').next().unwrap_or("normal"),
            content_alignment.split('|').nth(1).unwrap_or("normal")
        );
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_301).unwrap(), document.document_revision());
        for (node, parent, style) in [
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:300px;height:200px;margin:0;padding:0",
            ),
            (grid, HostParent::Node(root), grid_style.as_str()),
        ]
        .into_iter()
        .chain(items.iter().copied().map(|node| {
            (
                node,
                HostParent::Node(grid),
                "display:block;box-sizing:border-box;width:10px;height:10px;margin:0;padding:0",
            )
        })) {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML_NAMESPACE.to_owned(),
                local_name: "div".to_owned(),
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
            viewport: CssViewport {
                width_css_px: 300.0,
                height_css_px: 200.0,
                device_scale_factor: 1.0,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
        }
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view = StyloDocumentView::new(snapshot.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        let computed_styles = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            self.viewport,
            StyleRevision::INITIAL,
        )
        .map_err(StyleLayoutError::Cascade)?;
        compute_runtime_style_layout_with_root_html_grid_text_profiles(
            &snapshot,
            self.root,
            computed_styles,
            self.viewport,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &TextLayoutEngine::default(),
        )
    }

    fn item_frame(&self, output: &crate::StyleLayoutOutput, index: usize) -> [f32; 4] {
        let frame = output.layout.frames[&self.items[index].id()];
        [frame.x, frame.y, frame.width, frame.height]
    }
}

fn assert_css_px(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "expected {expected}px ± 1/64px, got {actual}px"
    );
}

#[test]
fn root_html_fixed_grid_projects_distribution_values_to_taffy() {
    let fixture = GridFixture::new(240.0, 180.0, "space-around|space-evenly", 4);
    let output = fixture.compute().unwrap();
    for (index, expected) in [
        [43.328125, 22.5],
        [136.65625, 22.5],
        [43.328125, 107.5],
        [136.65625, 107.5],
    ]
    .into_iter()
    .enumerate()
    {
        let actual = fixture.item_frame(&output, index);
        assert_css_px(actual[0], expected[0]);
        assert_css_px(actual[1], expected[1]);
        assert_eq!([actual[2], actual[3]], [10.0, 10.0]);
    }
    assert_eq!(output.layout.frames[&fixture.grid.id()].width, 240.0);
    assert_eq!(output.layout.frames[&fixture.grid.id()].height, 180.0);
}

#[test]
fn root_html_fixed_grid_safe_alignment_falls_back_and_unsafe_alignment_overflows() {
    let fixture = GridFixture::new(80.0, 70.0, "safe end|unsafe center", 4);
    let output = fixture.compute().unwrap();
    assert_css_px(fixture.item_frame(&output, 0)[0], -15.0);
    assert_css_px(fixture.item_frame(&output, 0)[1], 0.0);
    assert_css_px(fixture.item_frame(&output, 1)[0], 35.0);
    assert_css_px(fixture.item_frame(&output, 2)[1], 40.0);
}

#[test]
fn root_html_baseline_and_nonfixed_track_content_alignment_are_rejected() {
    let baseline = GridFixture::new(240.0, 180.0, "first baseline|normal", 4);
    let baseline_error = baseline.compute().expect_err("baseline must be rejected");
    assert!(
        matches!(
            baseline_error,
            StyleLayoutError::UnsupportedComputedValue {
                property: "align-content",
                ..
            }
        ),
        "unexpected baseline rejection: {baseline_error:?}"
    );

    let nonfixed =
        GridFixture::with_tracks(240.0, 180.0, "normal|center", 4, "40px 1fr", "30px 50px");
    assert!(matches!(
        nonfixed.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "justify-content",
            ..
        })
    ));

    let unsupported_modifier = GridFixture::new(240.0, 180.0, "normal|safe start", 4);
    let modifier_error = unsupported_modifier
        .compute()
        .expect_err("unreviewed safe modifier must be rejected");
    assert!(
        matches!(
            modifier_error,
            StyleLayoutError::UnsupportedComputedValue {
                property: "justify-content",
                ..
            }
        ),
        "unexpected modifier rejection: {modifier_error:?}"
    );
}

#[test]
fn root_html_content_alignment_is_rejected_outside_eligible_grid() {
    for property in ["align-content", "justify-content"] {
        let mut fixture = GridFixture::new(240.0, 180.0, "normal|normal", 4);
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_301).unwrap(),
            fixture.document.document_revision(),
        );
        batch.push(DocumentOperation::SetAttribute {
            node: fixture.root,
            name: AttributeName::new(None, "style").unwrap(),
            value: format!(
                "display:block;box-sizing:border-box;width:300px;height:200px;margin:0;padding:0;{property}:center"
            )
            .into(),
        });
        fixture.document.commit(batch).unwrap();

        assert!(matches!(
            fixture.compute(),
            Err(StyleLayoutError::UnsupportedComputedValue {
                property: actual,
                ..
            }) if actual == property
        ));
    }
}
