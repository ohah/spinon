use super::*;

use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument};
use spinon_layout::LayoutGridItemIntrinsicSource;

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy)]
struct Case {
    scene_id: &'static str,
    text: &'static str,
    columns: &'static str,
    width: f32,
    declared_hyphens: Option<&'static str>,
    inherited_hyphens: Option<&'static str>,
    computed_hyphens: &'static str,
    min_content: f32,
    max_content: f32,
    height: f32,
    lines: &'static [f32],
}

const CASES: [Case; 7] = [
    case(
        "hyphens-none-soft-min",
        "AB\u{00ad}CD",
        "min-content",
        120.0,
        Some("none"),
        None,
        "none",
        40.0,
        40.0,
        10.0,
        &[40.0],
    ),
    case(
        "hyphens-manual-soft-min",
        "AB\u{00ad}CD",
        "min-content",
        120.0,
        Some("manual"),
        None,
        "manual",
        30.0,
        40.0,
        20.0,
        &[30.0, 20.0],
    ),
    case(
        "hyphens-none-soft-fixed-30",
        "AB\u{00ad}CD",
        "30px",
        30.0,
        Some("none"),
        None,
        "none",
        40.0,
        40.0,
        10.0,
        &[40.0],
    ),
    case(
        "hyphens-manual-soft-fixed-30",
        "AB\u{00ad}CD",
        "30px",
        30.0,
        Some("manual"),
        None,
        "manual",
        30.0,
        40.0,
        20.0,
        &[30.0, 20.0],
    ),
    case(
        "hyphens-default-soft-fixed-30",
        "AB\u{00ad}CD",
        "30px",
        30.0,
        None,
        None,
        "manual",
        30.0,
        40.0,
        20.0,
        &[30.0, 20.0],
    ),
    case(
        "hyphens-inherited-none-fixed-30",
        "AB\u{00ad}CD",
        "30px",
        30.0,
        None,
        Some("none"),
        "none",
        40.0,
        40.0,
        10.0,
        &[40.0],
    ),
    case(
        "hyphens-inherited-manual-fixed-30",
        "AB\u{00ad}CD",
        "30px",
        30.0,
        None,
        Some("manual"),
        "manual",
        30.0,
        40.0,
        20.0,
        &[30.0, 20.0],
    ),
];

const fn case(
    scene_id: &'static str,
    text: &'static str,
    columns: &'static str,
    width: f32,
    declared_hyphens: Option<&'static str>,
    inherited_hyphens: Option<&'static str>,
    computed_hyphens: &'static str,
    min_content: f32,
    max_content: f32,
    height: f32,
    lines: &'static [f32],
) -> Case {
    Case {
        scene_id,
        text,
        columns,
        width,
        declared_hyphens,
        inherited_hyphens,
        computed_hyphens,
        min_content,
        max_content,
        height,
        lines,
    }
}

#[test]
fn anonymous_grid_hyphens_scenes_use_cascaded_style_and_grid_intrinsics() {
    for (index, case) in CASES.iter().enumerate() {
        let (snapshot, root, text) = snapshot_for(case, 13_000 + index as u64);
        let calculation = calculate(&snapshot, case.scene_id);
        let root_style = calculation.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.node_id == root.id())
            .expect("root computed style");
        assert_eq!(
            root_style
                .properties
                .get("hyphens")
                .map(String::as_str)
                .unwrap_or("manual"),
            case.computed_hyphens,
            "{} computed hyphens",
            case.scene_id
        );

        let layout = calculation
            .layout
            .unwrap_or_else(|error| panic!("{} layout: {error:?}", case.scene_id));
        let frame = layout
            .frames
            .iter()
            .find(|frame| frame.node_id == root.id().get())
            .expect("Grid root frame");
        assert_close(frame.width, case.width, case.scene_id, "root width");
        assert_close(frame.height, case.height, case.scene_id, "root height");

        let diagnostics = layout
            .grid_track_sizing_diagnostics
            .as_ref()
            .expect("C11.4 track sizing diagnostics");
        let item = diagnostics
            .item_inputs
            .iter()
            .find(|item| item.node == text.id())
            .expect("anonymous direct text item");
        let LayoutGridItemIntrinsicSource::Text { sizes } = &item.source else {
            panic!("{} expected a measured text item", case.scene_id);
        };
        assert_close(
            sizes.min_content(),
            case.min_content,
            case.scene_id,
            "min-content",
        );
        assert_close(
            sizes.max_content(),
            case.max_content,
            case.scene_id,
            "max-content",
        );

        let text_layout = diagnostics
            .item_text_layouts
            .iter()
            .find(|item| item.input.node == text.id())
            .expect("anonymous text layout");
        assert_eq!(
            text_layout.layout.lines.len(),
            case.lines.len(),
            "{} line count",
            case.scene_id
        );
        for (line, expected) in text_layout.layout.lines.iter().zip(case.lines) {
            assert_close(line.advance, *expected, case.scene_id, "line advance");
        }
    }
}

#[test]
fn anonymous_grid_hyphens_rejects_auto_source_soft_hyphen_and_invalid_value() {
    let auto_case = Case {
        scene_id: "hyphens-auto-expected-reject",
        text: "AB\u{00ad}CD",
        columns: "30px",
        width: 30.0,
        declared_hyphens: Some("auto"),
        inherited_hyphens: None,
        computed_hyphens: "auto",
        min_content: 0.0,
        max_content: 0.0,
        height: 0.0,
        lines: &[],
    };
    let (snapshot, _, _) = snapshot_for(&auto_case, 13_100);
    let auto_error = format!(
        "{:?}",
        calculate(&snapshot, auto_case.scene_id)
            .layout
            .expect_err("auto source SHY is outside the fixed profile")
    );
    assert!(auto_error.contains("layout_failed"), "{auto_error}");
    assert!(
        auto_error.contains(
            "hyphens 입력이 고정 Ahem 10px ASCII direct-text profile과 일치하지 않습니다"
        ),
        "{auto_error}"
    );

    let invalid_case = Case {
        scene_id: "hyphens-invalid-expected-reject",
        declared_hyphens: Some("balanced"),
        computed_hyphens: "balanced",
        ..CASES[3]
    };
    let (snapshot, _, _) = snapshot_for(&invalid_case, 13_101);
    let invalid_error = format!(
        "{:?}",
        calculate(&snapshot, invalid_case.scene_id)
            .layout
            .expect_err("unsupported computed value")
    );
    assert!(
        invalid_error.contains("unsupported_c114_hyphens_value"),
        "{invalid_error}"
    );
}

fn calculate(
    snapshot: &std::sync::Arc<spinon_core::HostDocumentSnapshot>,
    scene_id: &str,
) -> crate::session::ua_cascade::calculation::RuntimeCalculation {
    let request = WorkRequest {
        key: key_for(snapshot, EnvironmentRevision::INITIAL),
        snapshot: snapshot.clone(),
        viewport: CssViewport {
            width_css_px: 120.0,
            height_css_px: 100.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };
    compute_request_for_block_flex_grid_track_sizing(&request)
        .unwrap_or_else(|error| panic!("{scene_id} runtime calculation: {error:?}"))
}

fn snapshot_for(
    case: &Case,
    owner_id: u64,
) -> (
    std::sync::Arc<spinon_core::HostDocumentSnapshot>,
    spinon_core::HostNodeHandle,
    spinon_core::HostNodeHandle,
) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let parent = case
        .inherited_hyphens
        .map(|_| document.reserve_node_handle().unwrap());
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(owner_id).unwrap(),
        document.document_revision(),
    );
    if let Some(parent) = parent {
        batch.push(DocumentOperation::CreateElement {
            node: parent,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        set_attribute(
            &mut batch,
            parent,
            "id",
            &format!("c114-{}-parent", case.scene_id),
        );
        set_attribute(
            &mut batch,
            parent,
            "style",
            &format!(
                "box-sizing:border-box;display:block;margin:0;padding:0;border:0;hyphens:{}",
                case.inherited_hyphens.unwrap()
            ),
        );
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: parent,
            before: None,
        });
    }
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    set_attribute(
        &mut batch,
        root,
        "id",
        &format!("c114-{}-grid", case.scene_id),
    );
    set_attribute(&mut batch, root, "lang", "en");
    let mut declarations = format!(
        "box-sizing:border-box;display:grid;width:{}px;height:auto;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:auto;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:normal;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px",
        case.width, case.columns
    );
    if let Some(value) = case.declared_hyphens {
        declarations.push_str(&format!(";hyphens:{value}"));
    }
    set_attribute(&mut batch, root, "style", &declarations);
    batch.push(DocumentOperation::InsertBefore {
        parent: parent.map_or(HostParent::Root, HostParent::Node),
        node: root,
        before: None,
    });
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: case.text.to_owned().into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(root),
        node: text,
        before: None,
    });
    document.commit(batch).unwrap();
    (std::sync::Arc::new(document.snapshot()), root, text)
}

fn set_attribute(
    batch: &mut DocumentChangeBatch,
    node: spinon_core::HostNodeHandle,
    name: &str,
    value: &str,
) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, name).unwrap(),
        value: value.into(),
    });
}

fn assert_close(actual: f32, expected: f32, scene_id: &str, field: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{scene_id} {field}: expected {expected}, got {actual}"
    );
}
