use std::{collections::BTreeMap, fs};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, OwnerId, StyleRevision,
};
use style::context::QuirksMode;

use super::{
    ComputedStyleProfile, CssCascadeError, CssViewport, compute_supported_elements_ua_cascade,
};
use crate::stylo_dom::StyloDocumentView;
use crate::{CssOrigin, StylesheetSource};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/chromium-macos-arm64-154.0.8037.95-ua-profile-override-v4-inventory-1293be438ca5/ua-supported-elements.json"
);
const INVENTORY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c01/inventory.v1.json"
);

struct UaFixture {
    document: HostDocument,
    root: HostNodeHandle,
    body: HostNodeHandle,
    targets: BTreeMap<&'static str, HostNodeHandle>,
    fixture_ids: BTreeMap<&'static str, &'static str>,
}

impl UaFixture {
    fn new() -> Self {
        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(405).unwrap();
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        let root = append(&mut document, &mut batch, HostParent::Root, "html");
        let body = append(&mut document, &mut batch, HostParent::Node(root), "body");
        let mut targets = BTreeMap::new();
        let mut fixture_ids = BTreeMap::new();
        for (tag, selector) in [
            ("div", "div"),
            ("span", "span"),
            ("a", "a"),
            ("img", "img"),
            ("button", "button"),
            ("input", "input"),
            ("p", "p"),
            ("ul", "ul"),
        ] {
            let node = append(&mut document, &mut batch, HostParent::Node(body), tag);
            let id = fixture_id(selector);
            set_id(&mut batch, node, id);
            targets.insert(selector, node);
            fixture_ids.insert(selector, id);
            if tag == "ul" {
                let list_item = append(&mut document, &mut batch, HostParent::Node(node), "li");
                set_id(&mut batch, list_item, "ua-list-item");
                targets.insert("li", list_item);
                fixture_ids.insert("li", "ua-list-item");
            }
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            body,
            targets,
            fixture_ids,
        }
    }

    fn view(&self) -> StyloDocumentView {
        StyloDocumentView::new_with_base_url(
            self.document.snapshot(),
            self.root,
            true,
            QuirksMode::NoQuirks,
            "https://spinon.invalid/c01/supported-html-ua.html",
        )
        .unwrap()
    }

    fn viewport(&self) -> CssViewport {
        CssViewport {
            width_css_px: 800.0,
            height_css_px: 600.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::default(),
            media_environment: super::CssMediaEnvironment::DESKTOP,
        }
    }
}

fn append(
    document: &mut HostDocument,
    batch: &mut DocumentChangeBatch,
    parent: HostParent,
    tag: &str,
) -> HostNodeHandle {
    append_in_namespace(document, batch, parent, tag, HTML)
}

fn append_in_namespace(
    document: &mut HostDocument,
    batch: &mut DocumentChangeBatch,
    parent: HostParent,
    tag: &str,
    namespace: &str,
) -> HostNodeHandle {
    let node = document.reserve_node_handle().unwrap();
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: namespace.to_owned(),
        local_name: tag.to_owned(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node,
        before: None,
    });
    node
}

fn set_id(batch: &mut DocumentChangeBatch, node: HostNodeHandle, id: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "id").unwrap(),
        value: id.into(),
    });
}

fn fixture_id(selector: &str) -> &'static str {
    match selector {
        "div" => "ua-div",
        "span" => "ua-span",
        "a" => "ua-anchor",
        "img" => "ua-image",
        "button" => "ua-button",
        "input" => "ua-input",
        "p" => "ua-paragraph",
        "ul" => "ua-list",
        _ => panic!("알 수 없는 C01 selector: {selector}"),
    }
}

fn reference() -> Value {
    let value: Value = serde_json::from_str(&fs::read_to_string(REFERENCE).unwrap()).unwrap();
    assert_eq!(value["schema"], "spinon-css-reference/v1");
    assert_eq!(
        value["referenceId"],
        "chromium-macos-arm64-154.0.8037.95-ua-profile-override-v4-inventory-1293be438ca5"
    );
    assert_eq!(value["oracle"]["product"], "Google Chrome 154.0.8037.95");
    assert_eq!(value["oracle"]["version"], "154.0.8037.95");
    assert_eq!(
        value["oracle"]["revision"],
        "@05d469856e75794131cc2e5d9b2f6b6f10a70388"
    );
    assert_eq!(value["fixture"]["id"], "C01-UAv0-supported-html-elements");
    assert_eq!(
        value["fixture"]["sha256"],
        "dba2edfd1268afdb9f32d9ad725191e903ef2af30e1a549e29279f1576ff3863"
    );
    assert_eq!(
        value["fixture"]["profileCssSha256"],
        "bd15dd612a21cc8cb48e25eb38b86803868667df75cd56f111d7c476ddba3ebd"
    );
    assert_eq!(
        value["environment"]["emulation"]["cssViewportPx"]["width"],
        800
    );
    assert_eq!(
        value["environment"]["emulation"]["cssViewportPx"]["height"],
        600
    );
    assert_eq!(value["environment"]["emulation"]["deviceScaleFactor"], 1);
    assert_eq!(value["fixture"]["inventory"]["elementCount"], 9);
    assert_eq!(value["fixture"]["inventory"]["featureCount"], 19);
    assert_eq!(
        value["fixture"]["inventory"]["sha256"],
        "1293be438ca54d184c0f614d49fb0549bded5484806e73467a73898e6dc4fc6c"
    );
    value
}

fn inventory() -> Value {
    let value: Value = serde_json::from_str(&fs::read_to_string(INVENTORY).unwrap()).unwrap();
    assert_eq!(value["schema"], "spinon-css-feature-inventory/v1");
    assert_eq!(value["inventoryId"], "C01-UAv0-supported-html-elements");
    assert_eq!(value["profileId"], "spinon-html-ua/0.1.0-draft");
    value
}

#[test]
fn supported_elements_ua_profile_matches_all_c01_computed_values() {
    let fixture = UaFixture::new();
    let input = fixture.document.snapshot();
    let style_revision = StyleRevision::default().checked_next().unwrap();
    let output = compute_supported_elements_ua_cascade(
        &fixture.view(),
        &[],
        fixture.viewport(),
        style_revision,
    )
    .unwrap();

    assert_eq!(output.profile, ComputedStyleProfile::SupportedElementsUaV1);
    assert_eq!(output.generation, input.generation());
    assert_eq!(output.document_revision, input.document_revision());
    assert_eq!(output.render_tree_revision, input.render_tree_revision());
    assert_eq!(output.style_revision, style_revision);
    assert_eq!(output.viewport, fixture.viewport());
    assert!(output.diagnostics.is_empty());
    let expected_node_ids = std::iter::once(fixture.root.id())
        .chain(std::iter::once(fixture.body.id()))
        .chain(fixture.targets.values().map(|node| node.id()))
        .collect::<std::collections::BTreeSet<_>>();
    let actual_node_ids = output
        .elements
        .iter()
        .map(|element| element.node_id)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(actual_node_ids, expected_node_ids);
    assert_eq!(output.elements.len(), 11);
    let expected_properties = [
        "display",
        "list-style-type",
        "margin-block-start",
        "margin-block-end",
        "margin-inline-start",
        "margin-inline-end",
        "padding-inline-start",
    ]
    .into_iter()
    .collect::<std::collections::BTreeSet<_>>();
    assert!(output.elements.iter().all(|element| {
        element
            .properties
            .keys()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>()
            == expected_properties
    }));

    let reference = reference();
    let inventory = inventory();
    let expected_elements = reference["observations"]["chromiumDefault"]
        .as_array()
        .unwrap();
    let inventory_elements = inventory["elements"].as_array().unwrap();
    assert_eq!(expected_elements.len(), 9);
    assert_eq!(inventory_elements.len(), 9);
    let mut expected_selectors = std::collections::BTreeSet::new();
    let mut expected_features = std::collections::BTreeSet::new();
    let mut compared_features = 0;
    for expected_element in expected_elements {
        let selector = expected_element["selector"].as_str().unwrap();
        assert!(
            expected_selectors.insert(selector),
            "중복 oracle selector: {selector}"
        );
        let expected_matches = expected_element["matches"].as_array().unwrap();
        assert_eq!(expected_matches.len(), 1, "고정 C01 요소 수: {selector}");
        let expected_match = &expected_matches[0];
        let node = fixture.targets[selector].id();
        assert_eq!(expected_match["id"], fixture.fixture_ids[selector]);
        assert_eq!(expected_match["namespace"], HTML);
        let inventory_element = inventory_elements
            .iter()
            .find(|element| element["selector"] == selector)
            .unwrap_or_else(|| panic!("inventory selector 누락: {selector}"));
        assert_eq!(
            inventory_element["nodeIds"][0],
            fixture.fixture_ids[selector]
        );
        let oracle_features = expected_element["features"].as_array().unwrap();
        let inventory_features = inventory_element["features"].as_array().unwrap();
        assert_eq!(oracle_features.len(), inventory_features.len());
        for (feature, inventory_feature) in oracle_features.iter().zip(inventory_features) {
            assert_eq!(feature["id"], inventory_feature["id"]);
            assert_eq!(feature["property"], inventory_feature["property"]);
            let feature_id = feature["id"].as_str().unwrap();
            assert!(
                expected_features.insert(feature_id),
                "중복 feature ID: {feature_id}"
            );
            let property = feature["property"].as_str().unwrap();
            let expected = expected_match["computed"][property].as_str().unwrap();
            let actual = output
                .elements
                .iter()
                .find(|element| element.node_id == node)
                .unwrap_or_else(|| panic!("계산 style이 없는 요소: {selector}"));
            assert_eq!(
                actual.properties.get(property).map(String::as_str),
                Some(expected),
                "{} ({})",
                feature["id"].as_str().unwrap(),
                selector
            );
            compared_features += 1;
        }
    }
    assert_eq!(
        expected_selectors,
        fixture.targets.keys().copied().collect()
    );
    let inventory_selectors = inventory_elements
        .iter()
        .map(|element| element["selector"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(expected_selectors, inventory_selectors);
    assert_eq!(expected_selectors.len(), 9);
    assert_eq!(expected_features.len(), 19);
    assert_eq!(compared_features, 19);
}

#[test]
fn author_rules_override_embedded_ua_rules_in_the_same_snapshot() {
    let fixture = UaFixture::new();
    let stylesheet = StylesheetSource {
        id: "c04-ua-author-override".to_owned(),
        base_url: "https://spinon.invalid/c01/override.css".to_owned(),
        origin: CssOrigin::Author,
        css: "ul { display: flex; padding-inline-start: 12px; }".to_owned(),
    };
    let output = compute_supported_elements_ua_cascade(
        &fixture.view(),
        &[stylesheet],
        fixture.viewport(),
        StyleRevision::default(),
    )
    .unwrap();
    let list = output
        .elements
        .iter()
        .find(|element| element.node_id == fixture.targets["ul"].id())
        .unwrap();
    assert_eq!(list.properties["display"], "flex");
    assert_eq!(list.properties["padding-inline-start"], "12px");
}

#[test]
fn inline_style_overrides_embedded_ua_rules_after_author_stylesheets() {
    let mut fixture = UaFixture::new();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(405).unwrap(),
        fixture.document.document_revision(),
    );
    batch.push(DocumentOperation::SetAttribute {
        node: fixture.targets["ul"],
        name: AttributeName::new(None, "style").unwrap(),
        value: "display: block; padding-inline-start: 18px".into(),
    });
    fixture.document.commit(batch).unwrap();
    let author = StylesheetSource {
        id: "c04-ua-author-before-inline".to_owned(),
        base_url: "https://spinon.invalid/c01/inline.css".to_owned(),
        origin: CssOrigin::Author,
        css: "ul { display: flex; padding-inline-start: 12px; }".to_owned(),
    };
    let output = compute_supported_elements_ua_cascade(
        &fixture.view(),
        &[author],
        fixture.viewport(),
        StyleRevision::default(),
    )
    .unwrap();
    let list = output
        .elements
        .iter()
        .find(|element| element.node_id == fixture.targets["ul"].id())
        .unwrap();
    assert!(output.diagnostics.is_empty());
    assert_eq!(list.properties["display"], "block");
    assert_eq!(list.properties["padding-inline-start"], "18px");
}

#[test]
fn unsupported_author_origin_and_invalid_viewport_fail_closed() {
    let fixture = UaFixture::new();
    let wrong_origin = StylesheetSource {
        id: "unexpected-ua-origin".to_owned(),
        base_url: "https://spinon.invalid/c01/unexpected.css".to_owned(),
        origin: CssOrigin::UserAgent,
        css: "div { display: none; }".to_owned(),
    };
    assert!(matches!(
        compute_supported_elements_ua_cascade(
            &fixture.view(),
            &[wrong_origin],
            fixture.viewport(),
            StyleRevision::default(),
        ),
        Err(CssCascadeError::InvalidStylesheetOrigin { id }) if id == "unexpected-ua-origin"
    ));

    for invalid_viewport in [
        CssViewport {
            width_css_px: 0.0,
            ..fixture.viewport()
        },
        CssViewport {
            height_css_px: f32::INFINITY,
            ..fixture.viewport()
        },
        CssViewport {
            device_scale_factor: f32::NAN,
            ..fixture.viewport()
        },
        CssViewport {
            width_css_px: f32::MAX,
            device_scale_factor: f32::MAX,
            ..fixture.viewport()
        },
    ] {
        assert!(matches!(
            compute_supported_elements_ua_cascade(
                &fixture.view(),
                &[],
                invalid_viewport,
                StyleRevision::default(),
            ),
            Err(CssCascadeError::InvalidViewport)
        ));
    }
}

#[test]
fn html_ua_selectors_do_not_match_a_foreign_namespace_element() {
    let mut document = HostDocument::new().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(406).unwrap(), document.document_revision());
    let root = append(&mut document, &mut batch, HostParent::Root, "html");
    let body = append(&mut document, &mut batch, HostParent::Node(root), "body");
    let foreign_div = append_in_namespace(
        &mut document,
        &mut batch,
        HostParent::Node(body),
        "div",
        "urn:spinon:test",
    );
    document.commit(batch).unwrap();
    let view = StyloDocumentView::new_with_base_url(
        document.snapshot(),
        root,
        true,
        QuirksMode::NoQuirks,
        "https://spinon.invalid/c01/foreign-namespace.html",
    )
    .unwrap();
    let output = compute_supported_elements_ua_cascade(
        &view,
        &[],
        CssViewport {
            width_css_px: 800.0,
            height_css_px: 600.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::default(),
            media_environment: super::CssMediaEnvironment::DESKTOP,
        },
        StyleRevision::default(),
    )
    .unwrap();
    let computed = output
        .elements
        .iter()
        .find(|element| element.node_id == foreign_div.id())
        .unwrap();
    assert_eq!(computed.properties["display"], "inline");
}

#[test]
fn stylesheet_parse_diagnostics_are_returned_without_loading_remote_imports() {
    let fixture = UaFixture::new();
    let stylesheet = StylesheetSource {
        id: "c04-ua-import-diagnostic".to_owned(),
        base_url: "https://spinon.invalid/c01/diagnostic.css".to_owned(),
        origin: CssOrigin::Author,
        css: "@import url(https://example.invalid/style.css); div { display: block; }".to_owned(),
    };
    let output = compute_supported_elements_ua_cascade(
        &fixture.view(),
        &[stylesheet],
        fixture.viewport(),
        StyleRevision::default(),
    )
    .unwrap();
    assert!(
        output
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.source_id == "c04-ua-import-diagnostic" })
    );
}
