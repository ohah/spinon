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

struct RootHtmlFrGrid {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    items: Vec<HostNodeHandle>,
}

impl RootHtmlFrGrid {
    fn new(width: &str, columns: &str, auto_columns: &str, item_widths: &[f32]) -> Self {
        Self::with_extra(width, columns, auto_columns, "", item_widths)
    }

    fn with_extra(
        width: &str,
        columns: &str,
        auto_columns: &str,
        extra: &str,
        item_widths: &[f32],
    ) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = item_widths
            .iter()
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let extra_style = if extra.is_empty() {
            String::new()
        } else {
            format!(";{extra}")
        };
        let grid_style = format!(
            "display:grid;box-sizing:border-box;width:{width}{extra_style};height:20px;grid-template-columns:{columns};grid-template-rows:20px;grid-auto-columns:{auto_columns};grid-auto-flow:column;column-gap:5px;row-gap:0px;direction:ltr;margin:0;padding:0;border:0;overflow:visible"
        );
        let mut entries = vec![
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:400px;height:400px;margin:0;padding:0;border:0".to_owned(),
            ),
            (grid, HostParent::Node(root), grid_style),
        ];
        entries.extend(
            items
                .iter()
                .copied()
                .zip(item_widths.iter())
                .map(|(node, width)| {
                    (
                        node,
                        HostParent::Node(grid),
                        format!(
                            "display:block;box-sizing:border-box;width:{width}px;height:1px;margin:0;padding:0;border:0"
                        ),
                    )
                }),
        );
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_248).unwrap(), document.document_revision());
        for (node, parent, style) in entries {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.into(),
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

fn assert_x_positions(frames: &[LayoutFrame], expected: &[f32]) {
    assert_eq!(frames.len(), expected.len());
    for (index, (frame, expected)) in frames.iter().zip(expected).enumerate() {
        assert!(
            (frame.x - expected).abs() <= 0.02,
            "item {index}: expected x={expected}, got {}; frames={frames:?}",
            frame.x
        );
    }
}

#[test]
fn parsed_root_html_fr_auto_columns_match_fixed_precomparison_scenes() {
    let cases = [
        (
            "list cycling",
            RootHtmlFrGrid::new("240px", "30px 40px", "1fr 2fr", &[1.0; 6]),
            vec![0.0, 35.0, 80.0, 109.166_67, 162.5, 191.666_67],
        ),
        (
            "scalar repetition",
            RootHtmlFrGrid::new("240px", "30px 40px", "1fr", &[1.0; 6]),
            vec![0.0, 35.0, 80.0, 121.25, 162.5, 203.75],
        ),
        (
            "fixed and fraction mixed",
            RootHtmlFrGrid::new("240px", "30px 40px", "10px 1fr", &[1.0; 6]),
            vec![0.0, 35.0, 80.0, 95.0, 162.5, 177.5],
        ),
        (
            "fixed percentage and fraction mixed",
            RootHtmlFrGrid::new("300px", "30px 40px", "10px 10% 1fr", &[1.0; 8]),
            vec![0.0, 35.0, 80.0, 95.0, 130.0, 192.5, 207.5, 242.5],
        ),
        (
            "subunit factors leave trailing free space",
            RootHtmlFrGrid::new("120px", "30px", "0.25fr", &[1.0; 3]),
            vec![0.0, 35.0, 60.0],
        ),
        (
            "zero factor observes fixed item automatic minimum",
            RootHtmlFrGrid::new("120px", "30px", "0fr 1fr", &[1.0, 10.0, 1.0]),
            vec![0.0, 35.0, 50.0],
        ),
        (
            "large finite factors use the f64 sum path",
            RootHtmlFrGrid::new("215px", "30px 40px", "1e38fr", &[1.0; 6]),
            vec![0.0, 35.0, 80.0, 115.0, 150.0, 185.0],
        ),
    ];
    for (label, fixture, expected) in cases {
        let output = fixture
            .compute()
            .unwrap_or_else(|error| panic!("{label}: {error:?}"));
        let style = output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == fixture.grid.id())
            .and_then(|style| style.grid_layout.as_ref())
            .expect("parsed grid style should be retained");
        assert!(matches!(
            &style.auto_columns,
            spinon_style::ComputedCssGridAutoColumns::TrackList(_)
        ));
        let frames = fixture
            .items
            .iter()
            .map(|item| output.layout.frames[&item.id()])
            .collect::<Vec<_>>();
        assert_x_positions(&frames, &expected);
    }
}

#[test]
fn parsed_root_html_fr_auto_columns_require_unconstrained_definite_width() {
    for (width, extra, expected_property) in [
        ("auto", "", "width"),
        ("120px", "min-width:1px", "min-width"),
        ("120px", "max-width:200px", "max-width"),
        ("120px", "aspect-ratio:1", "aspect-ratio"),
        ("calc(60px + 60px)", "", "width"),
    ] {
        let fixture = RootHtmlFrGrid::with_extra(width, "30px", "1fr", extra, &[1.0, 1.0]);
        let result = fixture.compute();
        if let Ok(output) = &result {
            let style = output
                .computed_styles
                .elements
                .iter()
                .find(|style| style.node_id == fixture.grid.id())
                .unwrap();
            panic!(
                "relative tracks unexpectedly accepted for width={width}{extra}: property={:?}, dimension={:?}, min={:?}/{:?}, max={:?}/{:?}, aspect={:?}, math={:?}",
                style.properties.get("width"),
                style.layout_dimensions.width,
                style.properties.get("min-width"),
                style.layout_dimensions.min_width,
                style.properties.get("max-width"),
                style.layout_dimensions.max_width,
                style.layout_aspect_ratio,
                style.layout_math_values
            );
        }
        let error = result.unwrap_err();
        assert!(
            matches!(error, StyleLayoutError::UnsupportedComputedValue { property, .. } if property == expected_property),
            "expected {expected_property}, got {error:?}"
        );
    }
}

#[test]
fn parsed_root_html_css_invalid_fraction_falls_back_to_unsupported_auto_columns() {
    let fixture = RootHtmlFrGrid::new("120px", "30px", "-1fr", &[1.0, 1.0, 1.0]);
    assert!(matches!(
        fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "grid-auto-columns",
            ..
        })
    ));
}
