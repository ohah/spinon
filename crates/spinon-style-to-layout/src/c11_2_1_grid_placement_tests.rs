use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, NodeId, OwnerId, StyleRevision,
};
use spinon_style::{
    ComputedCssGridPlacement, ComputedStyleProfile, CssMediaEnvironment, CssViewport,
    StyloDocumentView,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_block_flex_grid_placement_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const CHROME_REFERENCE: &str =
    include_str!("../../../tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json");

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    grid: HostNodeHandle,
    items: Vec<HostNodeHandle>,
}

impl Fixture {
    fn new(grid_style: &str, item_styles: &[&str]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = (0..item_styles.len())
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_221).unwrap(), document.document_revision());
        let mut nodes = vec![(
            root,
            HostParent::Root,
            "root".to_owned(),
            "display:block;box-sizing:border-box;width:360px;height:800px;margin:0;padding:0"
                .to_owned(),
        )];
        nodes.push((
            grid,
            HostParent::Node(root),
            "grid".to_owned(),
            grid_style.to_owned(),
        ));
        nodes.extend(
            items
                .iter()
                .zip(item_styles)
                .enumerate()
                .map(|(index, (handle, style))| {
                    (
                        *handle,
                        HostParent::Node(grid),
                        format!("item-{index}"),
                        (*style).to_owned(),
                    )
                }),
        );
        for (node, parent, name, style) in nodes {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: name.into(),
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
        let document = self.document.snapshot();
        let view = StyloDocumentView::new(document.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        compute_runtime_block_flex_grid_placement_style_layout(
            &document,
            &view,
            self.root,
            &[],
            viewport(),
            StyleRevision::INITIAL,
        )
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

fn grid_style() -> &'static str {
    "display:grid;box-sizing:border-box;width:250px;height:85px;grid-template-columns:80px 80px 80px;grid-template-rows:40px 40px;column-gap:5px;row-gap:5px;margin:0;padding:0"
}

fn auto_sized_item(extra: &str) -> String {
    format!(
        "display:block;box-sizing:content-box;width:auto;height:auto;margin:0;padding:0;{extra}"
    )
}

fn compare_case_to_chrome(
    output: &crate::StyleLayoutOutput,
    case_id: &str,
    nodes: &[(&str, NodeId)],
) {
    let reference: serde_json::Value = serde_json::from_str(CHROME_REFERENCE).unwrap();
    assert_eq!(
        reference["oracle"]["product"], "Chrome/154.0.8037.98",
        "fixture 기준 브라우저가 바뀌지 않았는지 확인합니다"
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
        .find(|candidate| candidate["id"] == case_id)
        .unwrap();
    assert_eq!(
        case["scope"], "implementation-candidate",
        "oracle-only case를 제품 profile 검증에 쓰지 않습니다"
    );
    for (reference_id, node) in nodes {
        let expected = case["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|candidate| candidate["id"] == *reference_id)
            .unwrap()["rect"]
            .as_object()
            .unwrap();
        let expected = ["x", "y", "width", "height"].map(|field| {
            expected[field]
                .as_f64()
                .expect("Chrome CSS px frame field가 숫자여야 합니다") as f32
        });
        let actual = &output.layout.frames[node];
        let actual = [actual.x, actual.y, actual.width, actual.height];
        for ((field, actual), expected) in ["x", "y", "width", "height"]
            .into_iter()
            .zip(actual)
            .zip(expected)
        {
            assert!(
                (actual - expected).abs() <= 0.5,
                "Chrome 기준과 다릅니다: {case_id}/{reference_id}/{field}, 실제 {actual}, 기준 {expected}"
            );
        }
    }
}

mod hidden;
mod supported;
mod unsupported;
