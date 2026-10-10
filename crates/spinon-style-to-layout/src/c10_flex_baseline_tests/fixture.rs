use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    ComputedStyleProfile, CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource,
    StyloDocumentView, compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_paint_cascade, compute_runtime_flex_layout_cascade,
    compute_runtime_flex_paint_cascade,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
};

use crate::{StyleLayoutError, compute_runtime_style_layout};

use super::reference::{CapturedCase, ReferenceFile};

pub(super) const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const MAX_CSS_PX_ERROR: f32 = 0.5;

pub(super) fn assert_reference_matches_all_runtime_profiles() {
    let reference: ReferenceFile = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/css/references/c10-3-4-flex-baseline-v1.json"
    ))
    .expect("C10.3.4 Chromium reference JSON을 읽어야 합니다");
    assert_eq!(reference.observations.len(), 2);

    for observation in &reference.observations {
        let viewport = CssViewport {
            width_css_px: 320.0,
            height_css_px: 640.0,
            device_scale_factor: observation.viewport.device_scale_factor,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        for case in &observation.cases {
            let (document, root, handles) = document_for_case(case);
            let snapshot = document.snapshot();
            let view =
                StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
                    .expect("fixture root 아래에 Stylo view를 만들어야 합니다");
            let profiles = [
                (
                    ComputedStyleProfile::RuntimeFlexLayoutV1,
                    compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL)
                        .unwrap(),
                ),
                (
                    ComputedStyleProfile::RuntimeFlexPaintV1,
                    compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL)
                        .unwrap(),
                ),
                (
                    ComputedStyleProfile::RuntimeFlexCustomPropertiesV1,
                    compute_runtime_flex_custom_properties_cascade(
                        &view,
                        viewport,
                        StyleRevision::INITIAL,
                    )
                    .unwrap(),
                ),
                (
                    ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1,
                    compute_runtime_flex_custom_properties_paint_cascade(
                        &view,
                        viewport,
                        StyleRevision::INITIAL,
                    )
                    .unwrap(),
                ),
                (
                    ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1,
                    compute_runtime_flex_registered_properties_cascade_with_stylesheets(
                        &view,
                        &[],
                        viewport,
                        StyleRevision::INITIAL,
                    )
                    .unwrap(),
                ),
                (
                    ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1,
                    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
                        &view,
                        &[],
                        viewport,
                        StyleRevision::INITIAL,
                    )
                    .unwrap(),
                ),
            ];

            for (expected_profile, computed) in profiles {
                assert_eq!(computed.profile, expected_profile, "{}", case.id);
                let output = compute_runtime_style_layout(&snapshot, root, computed, viewport)
                    .unwrap_or_else(|error| {
                        panic!(
                            "{} {expected_profile:?} DPR {}: {error}",
                            case.id, observation.viewport.device_scale_factor
                        )
                    });
                assert_case_output(
                    case,
                    &handles,
                    &output,
                    expected_profile,
                    observation.viewport.device_scale_factor,
                );
            }
        }
    }
}

fn document_for_case(
    case: &CapturedCase,
) -> (
    HostDocument,
    HostNodeHandle,
    std::collections::BTreeMap<String, HostNodeHandle>,
) {
    let mut document = HostDocument::new().unwrap();
    let handles = case
        .nodes
        .iter()
        .map(|node| (node.id.clone(), document.reserve_node_handle().unwrap()))
        .collect::<std::collections::BTreeMap<_, _>>();
    let root = case
        .nodes
        .iter()
        .find(|node| node.input.parent_id.is_none())
        .map(|node| handles[&node.id])
        .unwrap_or_else(|| panic!("{} fixture에 root가 없습니다", case.id));
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(10_134).unwrap(), document.document_revision());
    for node in &case.nodes {
        let handle = handles[&node.id];
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML_NAMESPACE.to_owned(),
            local_name: node.input.tag.clone(),
        });
        for (name, value) in [
            ("id", node.id.as_str()),
            ("style", node.input.style.as_str()),
        ] {
            batch.push(DocumentOperation::SetAttribute {
                node: handle,
                name: AttributeName::new(None, name).unwrap(),
                value: value.to_owned().into(),
            });
        }
        let parent = node
            .input
            .parent_id
            .as_ref()
            .map(|id| HostParent::Node(handles[id]))
            .unwrap_or(HostParent::Root);
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, root, handles)
}

fn assert_case_output(
    case: &CapturedCase,
    handles: &std::collections::BTreeMap<String, HostNodeHandle>,
    output: &crate::StyleLayoutOutput,
    profile: ComputedStyleProfile,
    dpr: f32,
) {
    for expected in &case.nodes {
        let node_id = handles[&expected.id].id();
        let computed = output
            .computed_styles
            .elements
            .iter()
            .find(|element| element.node_id == node_id)
            .unwrap_or_else(|| {
                panic!(
                    "{} {profile:?}: {} computed style 누락",
                    case.id, expected.id
                )
            });
        for property in ["align-items", "align-self", "align-content"] {
            if let Some(expected_value) = expected.properties.get(property) {
                assert_eq!(
                    computed.properties.get(property).map(String::as_str),
                    Some(expected_value.as_str()),
                    "{} {profile:?} DPR {dpr} {}.{property}",
                    case.id,
                    expected.id,
                );
            }
        }

        let actual = output
            .layout
            .frames
            .get(&node_id)
            .unwrap_or_else(|| panic!("{} {profile:?}: {} frame 누락", case.id, expected.id));
        for (property, actual_value, expected_value) in [
            ("x", actual.x, expected.rect.x),
            ("y", actual.y, expected.rect.y),
            ("width", actual.width, expected.rect.width),
            ("height", actual.height, expected.rect.height),
        ] {
            assert!(
                (actual_value - expected_value).abs() <= MAX_CSS_PX_ERROR,
                "{} {profile:?} DPR {dpr} {}.{property}: actual={actual_value}, expected={expected_value}",
                case.id,
                expected.id,
            );
        }
    }
}

pub(super) fn assert_legacy_profile_rejects_baseline() {
    let reference = serde_json::from_str::<ReferenceFile>(include_str!(
        "../../../../tests/fixtures/css/references/c10-3-4-flex-baseline-v1.json"
    ))
    .unwrap();
    let case = reference.observations[0]
        .cases
        .iter()
        .find(|case| case.id == "last-group")
        .expect("last baseline align-items fixture가 있어야 합니다");
    let (document, root, _) = document_for_case(case);
    let snapshot = document.snapshot();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 640.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let mut computed =
        spinon_style::compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL)
            .unwrap();
    computed.profile = ComputedStyleProfile::FlexAlignmentV1;
    assert!(matches!(
        compute_runtime_style_layout(&snapshot, root, computed, viewport),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "align-items",
            ..
        })
    ));
}

pub(super) fn assert_author_stylesheet_last_baseline_falls_back() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let first = document.reserve_node_handle().unwrap();
    let second = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(10_135).unwrap(), document.document_revision());

    for (handle, id, style, parent) in [
        (
            root,
            "c1034-sheet-root",
            "display:flex;box-sizing:border-box;width:100px;height:100px;flex-direction:row;flex-wrap:wrap;align-items:flex-start;justify-content:flex-start;min-width:0;min-height:0;margin:0;padding:0;border:0",
            None,
        ),
        (
            first,
            "c1034-sheet-first",
            "display:block;box-sizing:border-box;width:60px;height:20px;flex:0 0 auto;align-self:auto;margin:0;padding:0;border:0",
            Some(root),
        ),
        (
            second,
            "c1034-sheet-second",
            "display:block;box-sizing:border-box;width:60px;height:30px;flex:0 0 auto;align-self:auto;margin:0;padding:0;border:0",
            Some(root),
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node: handle,
            namespace: HTML_NAMESPACE.to_owned(),
            local_name: "div".to_owned(),
        });
        for (name, value) in [("id", id), ("style", style)] {
            batch.push(DocumentOperation::SetAttribute {
                node: handle,
                name: AttributeName::new(None, name).unwrap(),
                value: value.to_owned().into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent: parent.map(HostParent::Node).unwrap_or(HostParent::Root),
            node: handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), root)
        .unwrap();
    let stylesheet = StylesheetSource {
        id: "c1034-baseline-fallback".to_owned(),
        base_url: "https://spinon.invalid/c10-3-4.css".to_owned(),
        origin: CssOrigin::Author,
        css: "#c1034-sheet-root { align-content:flex-start; align-content:center; align-content:last baseline; place-content:flex-start flex-start; place-content:center flex-start; place-content:center last baseline; }".to_owned(),
    };
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 640.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let computed = spinon_style::compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let root_style = computed
        .elements
        .iter()
        .find(|style| style.node_id == root.id())
        .unwrap();
    assert_eq!(root_style.properties["align-content"], "center");
    assert_eq!(root_style.properties["justify-content"], "flex-start");

    let output = compute_runtime_style_layout(&snapshot, root, computed, viewport).unwrap();
    assert_eq!(output.layout.frames[&first.id()].y, 25.0);
    assert_eq!(output.layout.frames[&second.id()].y, 45.0);
}
