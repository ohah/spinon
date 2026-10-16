use std::sync::Arc;

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    StyleRevision,
};

use crate::{ComputedCssColorSpace, ComputedStyleProfile, CssOrigin, StylesheetSource};

use super::super::{
    CssViewport, StyloDocumentView, compute_c19_computed_color_cascade_with_stylesheets,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const ORACLE: &str = include_str!(
    "../../../../../../spec/internal/evidence/c19-1-computed-color-precomparison-2026-10-16/chromium-color-observations.json"
);

struct Fixture {
    view: StyloDocumentView,
    cases: Vec<(String, spinon_core::NodeId)>,
}

fn fixture(rows: &[Value]) -> Fixture {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let children = rows
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(19_011).unwrap(), document.document_revision());
    append_element(
        &mut batch,
        root,
        "display:block;width:1px;height:1px;color:color(display-p3 0.9 0.1 0.4)",
    );
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });

    let mut cases = Vec::with_capacity(rows.len());
    for (row, child) in rows.iter().zip(children) {
        let id = row["id"].as_str().unwrap();
        let value = row["input"].as_str().unwrap();
        let style = if id == "current-color-inherited" {
            "display:block;width:1px;height:1px;color:inherit;background-color:currentColor".into()
        } else {
            format!(
                "display:block;width:1px;height:1px;color:rgb(1 2 3);background-color:transparent;color:{value};background-color:{value}"
            )
        };
        append_element(&mut batch, child, &style);
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: child,
            before: None,
        });
        cases.push((id.to_owned(), child.id()));
    }

    document.commit(batch).unwrap();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    Fixture { view, cases }
}

fn append_element(batch: &mut DocumentChangeBatch, node: spinon_core::HostNodeHandle, style: &str) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.to_owned().into(),
    });
}

fn expected_space(input: &str, group: &str) -> ComputedCssColorSpace {
    let input = input.to_ascii_lowercase();
    if group == "currentColor" {
        ComputedCssColorSpace::DisplayP3
    } else if group == "invalid" {
        ComputedCssColorSpace::Srgb
    } else if input.starts_with("hsl(") {
        ComputedCssColorSpace::Hsl
    } else if input.starts_with("hwb(") {
        ComputedCssColorSpace::Hwb
    } else if input.starts_with("lab(") {
        ComputedCssColorSpace::Lab
    } else if input.starts_with("lch(") {
        ComputedCssColorSpace::Lch
    } else if input.starts_with("oklab(") {
        ComputedCssColorSpace::Oklab
    } else if input.starts_with("oklch(") {
        ComputedCssColorSpace::Oklch
    } else if input.starts_with("color(srgb-linear ") {
        ComputedCssColorSpace::SrgbLinear
    } else if input.starts_with("color(display-p3-linear ") {
        ComputedCssColorSpace::DisplayP3Linear
    } else if input.starts_with("color(display-p3 ") {
        ComputedCssColorSpace::DisplayP3
    } else if input.starts_with("color(a98-rgb ") {
        ComputedCssColorSpace::A98Rgb
    } else if input.starts_with("color(prophoto-rgb ") {
        ComputedCssColorSpace::ProphotoRgb
    } else if input.starts_with("color(rec2020 ") {
        ComputedCssColorSpace::Rec2020
    } else if input.starts_with("color(xyz-d50 ") {
        ComputedCssColorSpace::XyzD50
    } else if input.starts_with("color(xyz-d65 ") || input.starts_with("color(xyz ") {
        ComputedCssColorSpace::XyzD65
    } else {
        ComputedCssColorSpace::Srgb
    }
}

#[test]
fn c19_computed_color_compares_pinned_chromium_srgb_output() {
    let reference: Value = serde_json::from_str(ORACLE).unwrap();
    let rows = reference["cases"].as_array().unwrap();
    let fixture = fixture(rows);
    let snapshot = compute_c19_computed_color_cascade_with_stylesheets(
        &fixture.view,
        &[],
        CssViewport::C04_FIXTURE,
        StyleRevision::default(),
    )
    .unwrap();

    assert_eq!(snapshot.profile, ComputedStyleProfile::C19ComputedColorV1);
    assert_eq!(fixture.cases.len(), 44);
    assert_eq!(rows.len(), fixture.cases.len());
    let mut mismatches = Vec::new();

    for (row, (case_id, node_id)) in rows.iter().zip(fixture.cases) {
        assert_eq!(row["id"].as_str(), Some(case_id.as_str()));
        let style = snapshot
            .elements
            .iter()
            .find(|style| style.node_id == node_id)
            .unwrap();
        let color = style.computed_color.expect("C19 profile exports color");
        let background = style
            .computed_background_color
            .expect("C19 profile exports background-color");
        assert!(
            style.background_color.is_none(),
            "{case_id} legacy color field"
        );
        assert!(style.background_paint.is_none(), "{case_id} paint boundary");
        let input = row["input"].as_str().unwrap();
        let group = row["group"].as_str().unwrap();
        assert_eq!(
            color.color_space,
            expected_space(input, group),
            "{case_id} color"
        );
        assert_eq!(
            background.color_space,
            expected_space(input, group),
            "{case_id} background-color"
        );
        let alpha = row["canvasSrgbRgba8"][3].as_u64().unwrap() as u8;
        assert!(row["opaqueColorExtractionAccepted"].as_bool().unwrap());
        let mut expected_rgba = row["opaqueSrgbRgba8"]
            .as_array()
            .unwrap()
            .iter()
            .take(3)
            .map(|channel| channel.as_u64().unwrap() as u8)
            .collect::<Vec<_>>();
        expected_rgba.push(alpha);
        let actual_rgba = background.converted_srgb_clipped_rgba8().unwrap();
        let actual_color_rgba = color.converted_srgb_clipped_rgba8().unwrap();
        let expected_color_rgba = if group == "system" {
            match case_id.as_str() {
                "system-canvas" => [255, 255, 255, 255],
                "system-canvas-text" => [0, 0, 0, 255],
                "system-button-face" => [220, 220, 220, 255],
                "system-highlight" => [0, 65, 198, 255],
                _ => unreachable!("system palette fixture case is enumerated"),
            }
        } else if group == "invalid" {
            [1, 2, 3, 255]
        } else {
            expected_rgba
                .clone()
                .try_into()
                .expect("RGBA fixture has four channels")
        };
        assert_eq!(actual_color_rgba, expected_color_rgba, "{case_id} color");
        if group == "system" {
            let stylo_servo_rgba = match case_id.as_str() {
                "system-canvas" => [255, 255, 255, 255],
                "system-canvas-text" => [0, 0, 0, 255],
                "system-button-face" => [220, 220, 220, 255],
                "system-highlight" => [0, 65, 198, 255],
                _ => unreachable!("system palette fixture case is enumerated"),
            };
            assert_eq!(
                actual_rgba, stylo_servo_rgba,
                "{case_id} Stylo Servo palette"
            );
        } else if actual_rgba.as_slice() != expected_rgba {
            mismatches.push(format!(
                "{case_id}: {actual_rgba:?} != {expected_rgba:?}; {background:?}"
            ));
        }

        if row["group"] == "invalid" {
            assert_eq!(style.properties["color"], "rgb(1, 2, 3)", "{case_id}");
            assert_eq!(actual_rgba, [0, 0, 0, 0], "{case_id} transparent fallback");
        }
    }
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

#[test]
fn c19_runtime_mount_inherits_color_from_synthetic_body() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(OwnerId::new(19_012).unwrap(), Default::default());
    append_element(
        &mut batch,
        root,
        "display:block;width:1px;height:1px;color:inherit;background-color:currentColor",
    );
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    document.commit(batch).unwrap();

    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    let stylesheet = StylesheetSource {
        id: "c19-runtime-mount-body-color".to_owned(),
        base_url: "https://spinon.invalid/document.html".to_owned(),
        origin: CssOrigin::Author,
        css: "body { color: color(display-p3 0.9 0.1 0.4); }".to_owned(),
    };
    let snapshot = compute_c19_computed_color_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        CssViewport::C04_FIXTURE,
        StyleRevision::default(),
    )
    .unwrap();
    let color = snapshot.elements[0].computed_color.unwrap();
    let background = snapshot.elements[0].computed_background_color.unwrap();

    assert_eq!(color.color_space, ComputedCssColorSpace::DisplayP3);
    assert_eq!(color, background);
    assert_eq!(color.converted_srgb_clipped_rgba8(), Ok([251, 0, 101, 255]));
}
