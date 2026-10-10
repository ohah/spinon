use std::{collections::BTreeMap, fs};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, OwnerId, StyleRevision,
};
use spinon_layout::LayoutFrame;
use spinon_style::{
    ComputedCssDimension, ComputedCssSpacingValue, CssMediaEnvironment, CssViewport,
    StyloDocumentView, compute_runtime_flex_layout_cascade,
};
use style::context::QuirksMode;

use crate::compute_runtime_style_layout;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c06-absolute-lengths-v1.json"
);
const UNITS: [(&str, &str, &str, &str, &str, &str, &str); 7] = [
    ("px", "96px", "6px", "48px", "6px", "3px", "3px"),
    (
        "in",
        "1in",
        "0.0625in",
        "0.5in",
        "0.0625in",
        "0.03125in",
        "0.03125in",
    ),
    (
        "cm",
        "2.54cm",
        "0.15875cm",
        "1.27cm",
        "0.15875cm",
        "0.079375cm",
        "0.079375cm",
    ),
    (
        "mm",
        "25.4mm",
        "1.5875mm",
        "12.7mm",
        "1.5875mm",
        "0.79375mm",
        "0.79375mm",
    ),
    ("q", "101.6Q", "6.35Q", "50.8Q", "6.35Q", "3.175Q", "3.175Q"),
    ("pt", "72pt", "4.5pt", "36pt", "4.5pt", "2.25pt", "2.25pt"),
    (
        "pc", "6pc", "0.375pc", "3pc", "0.375pc", "0.1875pc", "0.1875pc",
    ),
];

#[test]
fn absolute_length_values_and_geometry_match_chromium() {
    let fixture = RuntimeAbsoluteLengthFixture::new();
    let output = fixture.compute(fixture.viewport(1.0)).unwrap();
    let reference = load_reference();
    let mut maximum_frame_error = 0.0_f32;

    for expected in reference["observation"]["nodes"].as_array().unwrap() {
        let id = expected["id"].as_str().unwrap();
        let style = fixture.computed_style(&output, id);
        for (property, value) in expected["properties"].as_object().unwrap() {
            assert_eq!(
                style.properties.get(property).map(String::as_str),
                value.as_str(),
                "{id}.{property} computed CSS value"
            );
        }
        maximum_frame_error = maximum_frame_error.max(assert_frame_matches(
            output.layout.frames.get(&style.node_id).unwrap(),
            &expected["rect"],
            id,
        ));
    }

    for (unit, ..) in UNITS {
        let row = fixture.computed_style(&output, &format!("probe-{unit}"));
        assert_eq!(
            row.layout_spacing.margin.left,
            ComputedCssSpacingValue::LengthPx(6.0),
            "{unit} margin-left"
        );
        assert_eq!(
            row.layout_spacing.padding.left,
            ComputedCssSpacingValue::LengthPx(3.0),
            "{unit} padding-left"
        );
        assert_eq!(
            row.layout_spacing.column_gap,
            ComputedCssSpacingValue::LengthPx(3.0),
            "{unit} column-gap"
        );
        let width = fixture.computed_style(&output, &format!("{unit}-a"));
        assert_eq!(
            width.layout_dimensions.width,
            ComputedCssDimension::LengthPx(96.0)
        );
        assert_eq!(
            width.layout_dimensions.height,
            ComputedCssDimension::LengthPx(6.0)
        );
        let basis = fixture.computed_style(&output, &format!("{unit}-b"));
        assert_eq!(
            basis.layout_dimensions.flex_basis,
            ComputedCssDimension::LengthPx(48.0)
        );
    }

    let negative_margin = fixture.computed_style(&output, "probe-negative");
    assert_eq!(
        negative_margin.layout_spacing.margin.left,
        ComputedCssSpacingValue::LengthPx(-6.0)
    );
    assert_eq!(
        fixture
            .computed_style(&output, "variable-a")
            .layout_dimensions
            .width,
        ComputedCssDimension::LengthPx(96.0)
    );
    eprintln!(
        "C06.3 Chromium frame 최대 절대 오차: {maximum_frame_error} CSS px / 허용치 0.5 CSS px"
    );
}

#[test]
fn absolute_css_lengths_do_not_scale_with_device_pixel_ratio() {
    let fixture = RuntimeAbsoluteLengthFixture::new();
    let css_pixels = fixture.compute(fixture.viewport(1.0)).unwrap();
    let high_density = fixture.compute(fixture.viewport(3.0)).unwrap();

    for (node, first) in &css_pixels.layout.frames {
        let second = high_density.layout.frames.get(node).unwrap();
        assert_frames_equal(first, second, &format!("node {node:?}"));
    }
}

#[test]
fn stylo_resolved_absolute_css_math_reaches_the_same_px_value() {
    let mut fixture = RuntimeAbsoluteLengthFixture::new();
    fixture.set_style(
        "px-a",
        "display:block;box-sizing:border-box;flex:0 0 auto;width:calc(1in);height:6px;background-color:#3366ff",
    );
    let output = fixture.compute(fixture.viewport(1.0)).unwrap();
    let style = fixture.computed_style(&output, "px-a");
    assert_eq!(
        style.layout_dimensions.width,
        ComputedCssDimension::LengthPx(96.0)
    );
    assert_eq!(output.layout.frames[&style.node_id].width, 96.0);
}

#[test]
fn mixed_unit_css_math_reaches_the_layout_projection() {
    let mut fixture = RuntimeAbsoluteLengthFixture::new();
    fixture.set_style(
        "px-a",
        "display:block;box-sizing:border-box;flex:0 0 auto;width:calc(1in + 10%);height:6px;background-color:#3366ff",
    );
    let output = fixture.compute(fixture.viewport(1.0)).unwrap();
    let style = fixture.computed_style(&output, "px-a");
    assert_eq!(output.layout.frames[&style.node_id].width, 123.7);
}

#[test]
fn invalid_unknown_unit_does_not_replace_a_valid_css_length() {
    let mut fixture = RuntimeAbsoluteLengthFixture::new();
    fixture.set_style(
        "px-a",
        "display:block;box-sizing:border-box;flex:0 0 auto;width:10px;width:1spinon;height:6px;background-color:#3366ff",
    );
    let output = fixture.compute(fixture.viewport(1.0)).unwrap();
    let style = fixture.computed_style(&output, "px-a");
    assert_eq!(
        style.layout_dimensions.width,
        ComputedCssDimension::LengthPx(10.0)
    );
}

fn load_reference() -> Value {
    serde_json::from_slice(&fs::read(REFERENCE).unwrap()).unwrap()
}

fn assert_frame_matches(actual: &LayoutFrame, expected: &Value, id: &str) -> f32 {
    let mut maximum_error = 0.0_f32;
    for (field, value) in [
        ("x", actual.x),
        ("y", actual.y),
        ("width", actual.width),
        ("height", actual.height),
    ] {
        let reference = expected[field].as_f64().unwrap() as f32;
        let error = (value - reference).abs();
        maximum_error = maximum_error.max(error);
        assert!(
            error <= 0.5,
            "{id}.{field}: Spinon {value}, Chromium {reference}, 허용치 0.5 CSS px"
        );
    }
    maximum_error
}

fn assert_frames_equal(left: &LayoutFrame, right: &LayoutFrame, context: &str) {
    for (field, left, right) in [
        ("x", left.x, right.x),
        ("y", left.y, right.y),
        ("width", left.width, right.width),
        ("height", left.height, right.height),
    ] {
        assert!(
            (left - right).abs() <= 0.001,
            "{context}.{field}: DPR 1 {left}, DPR 3 {right}"
        );
    }
}

fn append_unit_specs(
    specifications: &mut Vec<(String, Option<String>, String)>,
    (unit, width, height, basis, margin, padding, gap): (&str, &str, &str, &str, &str, &str, &str),
) {
    let row = format!("probe-{unit}");
    specifications.push((
        row.clone(),
        Some("root".to_owned()),
        format!("display:flex;box-sizing:border-box;flex:0 0 8px;flex-direction:row;width:280px;height:8px;margin-left:{margin};padding-left:{padding};column-gap:{gap};background-color:#26354f"),
    ));
    specifications.push((
        format!("{unit}-a"),
        Some(row.clone()),
        format!("display:block;box-sizing:border-box;flex:0 0 auto;width:{width};height:{height};background-color:#3366ff"),
    ));
    specifications.push((
        format!("{unit}-b"),
        Some(row),
        format!("display:block;box-sizing:border-box;flex:0 0 {basis};height:{height};background-color:#22aa66"),
    ));
}

struct RuntimeAbsoluteLengthFixture {
    document: HostDocument,
    root: HostNodeHandle,
    nodes: BTreeMap<String, HostNodeHandle>,
}

impl RuntimeAbsoluteLengthFixture {
    fn new() -> Self {
        let mut specifications = vec![(
            "root".to_owned(),
            None,
            "display:flex;box-sizing:border-box;width:300px;height:100px;flex-direction:column;row-gap:2px;background-color:#101827".to_owned(),
        )];
        append_unit_specs(&mut specifications, UNITS[0]);
        specifications.extend([
            (
                "probe-negative".to_owned(),
                Some("root".to_owned()),
                "display:flex;box-sizing:border-box;flex:0 0 8px;flex-direction:row;width:280px;height:8px;margin-left:-4.5pt;padding-left:8px;column-gap:0;background-color:#26354f".to_owned(),
            ),
            (
                "negative-a".to_owned(),
                Some("probe-negative".to_owned()),
                "display:block;box-sizing:border-box;flex:0 0 auto;width:1in;height:6px;background-color:#ff9933".to_owned(),
            ),
            (
                "probe-variable".to_owned(),
                Some("root".to_owned()),
                "display:flex;box-sizing:border-box;flex:0 0 8px;flex-direction:row;width:280px;height:8px;margin-left:6px;padding-left:3px;column-gap:3px;background-color:#26354f".to_owned(),
            ),
            (
                "variable-a".to_owned(),
                Some("probe-variable".to_owned()),
                "display:block;box-sizing:border-box;flex:0 0 auto;--physical-length:2.54cm;width:var(--physical-length);height:6px;background-color:#cc55aa".to_owned(),
            ),
        ]);

        for unit in UNITS.iter().skip(1) {
            append_unit_specs(&mut specifications, *unit);
        }

        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(7603).unwrap();
        let mut nodes = BTreeMap::new();
        for (id, _, _) in &specifications {
            nodes.insert(id.clone(), document.reserve_node_handle().unwrap());
        }
        let root = nodes["root"];
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        for (id, parent, style) in specifications {
            let node = nodes[&id];
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: id.into(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent: parent.map_or(HostParent::Root, |parent| HostParent::Node(nodes[&parent])),
                node,
                before: None,
            });
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            nodes,
        }
    }

    fn viewport(&self, device_scale_factor: f32) -> CssViewport {
        CssViewport {
            width_css_px: 320.0,
            height_css_px: 800.0,
            device_scale_factor,
            environment_revision: EnvironmentRevision::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        }
    }

    fn view(&self) -> StyloDocumentView {
        StyloDocumentView::new_with_base_url(
            self.document.snapshot(),
            self.root,
            true,
            QuirksMode::NoQuirks,
            "https://spinon.invalid/c06/absolute-lengths.html",
        )
        .unwrap()
    }

    fn computed_style<'a>(
        &self,
        output: &'a crate::StyleLayoutOutput,
        id: &str,
    ) -> &'a spinon_style::ComputedElementStyle {
        output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == self.nodes[id].id())
            .unwrap_or_else(|| panic!("계산 스타일 {id}가 없습니다"))
    }

    fn set_style(&mut self, id: &str, style: &str) {
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(7603).unwrap(),
            self.document.document_revision(),
        );
        batch.push(DocumentOperation::SetAttribute {
            node: self.nodes[id],
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        self.document.commit(batch).unwrap();
    }

    fn compute(
        &self,
        viewport: CssViewport,
    ) -> Result<crate::StyleLayoutOutput, crate::StyleLayoutError> {
        let computed =
            compute_runtime_flex_layout_cascade(&self.view(), viewport, StyleRevision::default())?;
        compute_runtime_style_layout(&self.document.snapshot(), self.root, computed, viewport)
    }
}
