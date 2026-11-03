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

struct RootHtmlCalcGrid {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    items: Vec<HostNodeHandle>,
}

impl RootHtmlCalcGrid {
    fn new(width: &str, auto_columns: &str, item_count: usize) -> Self {
        Self::with_extra(width, auto_columns, "", item_count)
    }

    fn with_extra(width: &str, auto_columns: &str, extra: &str, item_count: usize) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = (0..item_count)
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let extra_style = if extra.is_empty() {
            String::new()
        } else {
            format!(";{extra}")
        };
        let grid_style = format!(
            "display:grid;box-sizing:border-box;width:{width}{extra_style};height:63px;grid-template-columns:30px 40px;grid-template-rows:20px 40px;grid-auto-columns:{auto_columns};grid-auto-flow:column;column-gap:5px;row-gap:3px;direction:ltr;margin:0;padding:0;border:0;overflow:visible"
        );
        let mut entries = vec![
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:400px;height:400px;margin:0;padding:0;border:0".to_owned(),
            ),
            (grid, HostParent::Node(root), grid_style),
        ];
        entries.extend(items.iter().copied().map(|node| {
            (
                node,
                HostParent::Node(grid),
                "display:block;box-sizing:border-box;width:10px;height:10px;margin:0;padding:0;border:0".to_owned(),
            )
        }));
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_300).unwrap(), document.document_revision());
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

fn assert_positions(fixture: &RootHtmlCalcGrid, expected_x: &[f32], expected_y: &[f32]) {
    let output = fixture
        .compute()
        .unwrap_or_else(|error| panic!("parsed Root HTML calc grid: {error:?}"));
    let grid = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == fixture.grid.id())
        .and_then(|style| style.grid_layout.as_ref())
        .expect("computed Grid style should be retained");
    assert!(matches!(
        &grid.auto_columns,
        spinon_style::ComputedCssGridAutoColumns::TrackList(_)
    ));
    let frames = fixture
        .items
        .iter()
        .map(|item| output.layout.frames[&item.id()])
        .collect::<Vec<_>>();
    assert_x_positions(&frames, expected_x);
    for (index, (frame, expected_y)) in frames.iter().zip(expected_y).enumerate() {
        assert!(
            (frame.y - expected_y).abs() <= 0.02,
            "item {index}: expected y={expected_y}, got {}; frames={frames:?}",
            frame.y
        );
    }
}

#[test]
fn parsed_root_html_calc_auto_columns_match_independent_precomparison() {
    let fixture = RootHtmlCalcGrid::new("240px", "calc(10% + 5px) calc(2% + 2px)", 12);
    assert_positions(
        &fixture,
        &[
            0.0, 0.0, 35.0, 35.0, 80.0, 80.0, 114.0, 114.0, 125.8, 125.8, 159.8, 159.8,
        ],
        &[
            0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0,
        ],
    );
}

#[test]
fn parsed_root_html_calc_auto_columns_preserve_mixed_list_order_and_fr_distribution() {
    let mixed = RootHtmlCalcGrid::new("240px", "10px calc(10% + 5px) 15%", 12);
    assert_positions(
        &mixed,
        &[
            0.0, 0.0, 35.0, 35.0, 80.0, 80.0, 95.0, 95.0, 129.0, 129.0, 170.0, 170.0,
        ],
        &[
            0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0,
        ],
    );

    let mixed_fr = RootHtmlCalcGrid::new("240px", "calc(10% + 5px) 1fr", 12);
    assert_positions(
        &mixed_fr,
        &[
            0.0, 0.0, 35.0, 35.0, 80.0, 80.0, 114.0, 114.0, 162.5, 162.5, 196.5, 196.5,
        ],
        &[
            0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0,
        ],
    );
}

#[test]
fn parsed_root_html_calc_auto_columns_clamp_negative_and_preserve_large_tracks() {
    let negative = RootHtmlCalcGrid::new("240px", "calc(1px - 10%)", 12);
    assert_positions(
        &negative,
        &[
            0.0, 0.0, 35.0, 35.0, 80.0, 80.0, 85.0, 85.0, 90.0, 90.0, 95.0, 95.0,
        ],
        &[
            0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0,
        ],
    );

    let large = RootHtmlCalcGrid::new("240px", "calc(120% + 1px)", 12);
    assert_positions(
        &large,
        &[
            0.0, 0.0, 35.0, 35.0, 80.0, 80.0, 374.0, 374.0, 668.0, 668.0, 962.0, 962.0,
        ],
        &[
            0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0,
        ],
    );
}

#[test]
fn parsed_root_html_calc_auto_columns_accept_definite_zero_width_basis() {
    let fixture = RootHtmlCalcGrid::new("0px", "calc(10% + 5px)", 12);
    assert_positions(
        &fixture,
        &[
            0.0, 0.0, 35.0, 35.0, 80.0, 80.0, 90.0, 90.0, 100.0, 100.0, 110.0, 110.0,
        ],
        &[
            0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0, 0.0, 23.0,
        ],
    );
}

#[test]
fn parsed_root_html_constant_calc_is_already_a_direct_computed_length() {
    let fixture = RootHtmlCalcGrid::new("240px", "calc(2px * 3)", 8);
    let output = fixture
        .compute()
        .expect("constant math should follow the existing direct computed-px path");
    let grid = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == fixture.grid.id())
        .and_then(|style| style.grid_layout.as_ref())
        .expect("computed Grid style should be retained");
    assert_eq!(
        grid.auto_columns,
        spinon_style::ComputedCssGridAutoColumns::LengthPx(6.0)
    );
}

#[test]
fn parsed_root_html_calc_auto_columns_reject_unsupported_math_and_width_bases() {
    for (auto_columns, extra, width, expected_property) in [
        ("min(10%, 20px)", "", "240px", Some("grid-auto-columns")),
        (
            "calc(10% + 5px)",
            "min-width:1px",
            "240px",
            Some("min-width"),
        ),
        (
            "calc(10% + 5px)",
            "max-width:200px",
            "240px",
            Some("max-width"),
        ),
        (
            "calc(10% + 5px)",
            "aspect-ratio:1",
            "240px",
            Some("aspect-ratio"),
        ),
        ("calc(10% + 5px)", "", "calc(50% + 40px)", Some("width")),
        ("calc(10% + 5px)", "", "auto", Some("width")),
        ("calc(10% + )", "", "240px", None),
    ] {
        let fixture = RootHtmlCalcGrid::with_extra(width, auto_columns, extra, 8);
        let result = fixture.compute();
        let observed = match &result {
            Ok(_) => "layout succeeded".to_owned(),
            Err(error) => format!("{error:?}"),
        };
        let rejected_as_expected = match expected_property {
            Some(property) => matches!(
                result,
                Err(StyleLayoutError::UnsupportedComputedValue {
                    property: found,
                    ..
                }) if found == property
            ),
            None => matches!(result, Err(StyleLayoutError::CascadeDiagnostic(_))),
        };
        assert!(
            rejected_as_expected,
            "expected {expected_property:?} rejection for `{auto_columns}` plus `{extra}`, got {observed}"
        );
    }
}
