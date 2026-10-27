use std::collections::{BTreeMap, BTreeSet};

use spinon_core::StyleRevision;
use spinon_layout::{ParagraphInput, TextLayoutEngine, TextSource};
use spinon_style::{
    CssCascadeError, compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};

use super::super::AHEM;
use super::super::generic_direct_text;
use super::cases::*;
use super::test_fixture::*;

#[test]
fn collector_rejects_ambiguous_sources_and_three_or_more_candidates() {
    let malformed = [
        (
            "multiple direct Text nodes",
            vec![
                item("a", "item", vec![NodeSpec::text("A"), NodeSpec::text("B")]),
                item("b", "item", vec![NodeSpec::text("C")]),
            ],
            "unsupported_c114_text_source",
        ),
        (
            "descendant Text source",
            vec![
                item(
                    "a",
                    "item",
                    vec![element(
                        HTML,
                        "span",
                        Some("nested"),
                        None,
                        None,
                        vec![NodeSpec::text("A")],
                    )],
                ),
                item("b", "item", vec![NodeSpec::text("B")]),
            ],
            "unsupported_c114_text_source",
        ),
        (
            "replaced image mixed with direct text",
            vec![
                item(
                    "a",
                    "item",
                    vec![
                        NodeSpec::text("A"),
                        element(HTML, "img", Some("image"), None, None, vec![]),
                    ],
                ),
                item("b", "item", vec![NodeSpec::text("B")]),
            ],
            "unsupported_c114_text_source",
        ),
        (
            "root image mixed with two direct text items",
            vec![
                item("a", "item", vec![NodeSpec::text("A")]),
                item("b", "item", vec![NodeSpec::text("B")]),
                element(HTML, "img", Some("image"), None, None, vec![]),
            ],
            "unsupported_c114_text_source",
        ),
        (
            "foreign namespace item",
            vec![
                element(
                    SVG,
                    "g",
                    Some("a"),
                    Some("item"),
                    None,
                    vec![NodeSpec::text("A")],
                ),
                item("b", "item", vec![NodeSpec::text("B")]),
            ],
            "unsupported_c114_text_source",
        ),
        (
            "three direct text item candidates",
            vec![
                item("a", "item", vec![NodeSpec::text("A")]),
                item("b", "item", vec![NodeSpec::text("B")]),
                item("c", "item", vec![NodeSpec::text("C")]),
            ],
            "unsupported_c114_text_source",
        ),
    ];

    for (label, items, expected_code) in malformed {
        let fixture = custom_fixture("generic-malformed-root", "scene", items, None);
        let computed = cascade(&fixture.view, POSITIVE_STYLESHEET);
        let error = take_error(
            generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed),
            label,
        );
        assert_eq!(error.code, expected_code, "{label}: {error:?}");
        assert!(
            error.node_id.is_some(),
            "{label}: source owner must be identified"
        );
    }

    let anonymous = custom_fixture(
        "generic-anonymous-root",
        "scene",
        vec![
            NodeSpec::text("A"),
            item("a", "item", vec![NodeSpec::text("B")]),
            item("b", "item", vec![NodeSpec::text("C")]),
        ],
        None,
    );
    let computed = cascade(&anonymous.view, POSITIVE_STYLESHEET);
    let error = take_error(
        generic_direct_text::prepare(&anonymous.snapshot, anonymous.root, &computed),
        "anonymous Grid text must fail closed",
    );
    assert_eq!(error.code, "unsupported_c114_text_source");
}

#[test]
fn collector_ignores_non_html_img_and_html_image_decoys() {
    for (label, namespace, tag) in [
        ("SVG img local name", SVG, "img"),
        ("HTML image local name", HTML, "image"),
    ] {
        for position in 0..=2 {
            let mut children = vec![
                item("a", "item", vec![NodeSpec::text("A")]),
                item("b", "item", vec![NodeSpec::text("B")]),
            ];
            children.insert(
                position,
                element(namespace, tag, Some("decoy"), None, None, vec![]),
            );
            let fixture = custom_fixture("generic-decoy-root", "scene", children, None);
            let computed = cascade(&fixture.view, POSITIVE_STYLESHEET);
            let input = generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed)
                .unwrap_or_else(|error| panic!("{label} at child {position}: {error:?}"));
            let input = input.unwrap_or_else(|| {
                panic!("{label} at child {position}: valid text candidates were skipped")
            });

            let decoy_node_id = fixture.items[position].id();
            let expected_owners = fixture
                .items
                .iter()
                .map(|handle| handle.id())
                .filter(|node_id| *node_id != decoy_node_id)
                .collect::<BTreeSet<_>>();
            let actual_owners = input
                .paragraph_styles
                .keys()
                .copied()
                .collect::<BTreeSet<_>>();
            assert_eq!(
                actual_owners, expected_owners,
                "{label} at child {position}"
            );
        }
    }
}

#[test]
fn collector_reports_the_exact_html_image_node() {
    let fixture = custom_fixture(
        "generic-image-owner-root",
        "scene",
        vec![
            item("a", "item", vec![NodeSpec::text("A")]),
            item("b", "item", vec![NodeSpec::text("B")]),
            element(HTML, "img", Some("offending-image"), None, None, vec![]),
        ],
        None,
    );
    let expected_image_node_id = fixture.items[2].id().get();
    let computed = cascade(&fixture.view, POSITIVE_STYLESHEET);
    let error = take_error(
        generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed),
        "root HTML image must fail closed",
    );

    assert_eq!(error.code, "unsupported_c114_text_source");
    assert_eq!(error.node_id, Some(expected_image_node_id));
}

#[test]
fn collector_leaves_non_grid_shapes_for_existing_fallback() {
    let non_grid = custom_fixture(
        "generic-non-grid",
        "not-a-grid",
        vec![item("legacy-item", "item", vec![NodeSpec::text("A BC")])],
        None,
    );
    let computed = cascade(&non_grid.view, POSITIVE_STYLESHEET);
    assert!(
        generic_direct_text::prepare(&non_grid.snapshot, non_grid.root, &computed)
            .unwrap()
            .is_none()
    );
}

#[test]
fn collector_rejects_computed_item_styles_outside_the_fixed_profile() {
    for (declaration, label) in [
        ("display:flex", "display"),
        ("position:absolute", "position"),
        ("order:1", "order"),
        ("font-style:italic", "font-style"),
        ("white-space-collapse:preserve", "white-space-collapse"),
        ("word-break:keep-all", "word-break"),
        ("min-width:1px", "min-width"),
        ("max-height:1px", "max-height"),
        ("aspect-ratio:2", "aspect-ratio"),
        ("align-self:start", "align-self"),
        ("justify-self:start", "justify-self"),
        ("grid-column:2", "grid placement"),
        ("margin-left:1px", "margin"),
        ("padding-left:1px", "padding"),
        ("border-left:1px solid", "border"),
    ] {
        let fixture = positive_fixture(SCENES[0]);
        let stylesheet = format!("{POSITIVE_STYLESHEET}\n.scene .item {{ {declaration}; }}");
        let computed = cascade(&fixture.view, &stylesheet);
        let error = take_error(
            generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed),
            label,
        );
        assert_eq!(
            error.code, "unsupported_c114_text_style",
            "{label}: {error:?}"
        );
        assert!(
            error.node_id.is_some(),
            "{label}: item owner must be identified"
        );
    }

    let fixture = positive_fixture(SCENES[0]);
    let stylesheet = POSITIVE_STYLESHEET.replace(
        "align-items: start;",
        "align-items: start; grid-template-columns: 30px 50px;",
    );
    let computed = cascade(&fixture.view, &stylesheet);
    let error = take_error(
        generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed),
        "different explicit tracks must fail closed",
    );
    assert_eq!(error.code, "unsupported_c114_text_grid_style");
    assert_eq!(error.node_id, Some(fixture.root.id().get()));

    for (declaration, label) in [
        ("row-gap:1px", "row gap"),
        ("justify-content:end", "justify-content"),
        ("align-content:end", "align-content"),
        ("justify-items:start", "justify-items"),
        ("direction:rtl", "direction"),
        ("writing-mode:vertical-rl", "writing-mode"),
    ] {
        let fixture = positive_fixture(SCENES[0]);
        let stylesheet = format!("{POSITIVE_STYLESHEET}\n.scene {{ {declaration}; }}");
        let computed = cascade(&fixture.view, &stylesheet);
        let error = take_error(
            generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed),
            label,
        );
        assert_eq!(
            error.code, "unsupported_c114_text_grid_style",
            "{label}: {error:?}"
        );
        assert_eq!(error.node_id, Some(fixture.root.id().get()), "{label}");
    }
}

#[test]
fn inline_unsupported_properties_fail_before_snapshot_or_layout_creation() {
    let mut failures = Vec::new();
    for (declaration, expected_property) in [
        ("visibility:collapse", "visibility"),
        ("font-variant-ligatures:none", "font-variant-ligatures"),
    ] {
        let fixture = positive_fixture_with_inline_style(SCENES[0], declaration);
        let result = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
            &fixture.view,
            &[stylesheet(POSITIVE_STYLESHEET)],
            VIEWPORT,
            StyleRevision::INITIAL,
        );
        match result {
            Err(CssCascadeError::UnsupportedInlineAuthorProperty { node, property })
                if node == fixture.items[0].id() && property == expected_property => {}
            Err(error) => failures.push(format!("{expected_property}: wrong error {error}")),
            Ok(snapshot) => failures.push(format!(
                "{expected_property}: snapshot returned; diagnostics={:?}",
                snapshot.diagnostics
            )),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn hyphens_inline_declaration_is_reported_as_a_stylo_unknown_property() {
    let fixture = positive_fixture_with_inline_style(SCENES[0], "hyphens:none");
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        &[stylesheet(POSITIVE_STYLESHEET)],
        VIEWPORT,
        StyleRevision::INITIAL,
    )
    .expect("Stylo 0.22 ignores this unknown declaration and returns a snapshot");
    let diagnostic = snapshot
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.node_id == Some(fixture.items[0].id()))
        .expect("the inline property parser observation must identify its element");
    assert!(diagnostic.diagnostic.message.contains("'hyphens'"));
    assert!(diagnostic.diagnostic.message.contains("UnknownProperty"));
}

#[test]
fn stale_computed_snapshot_cannot_be_used_for_another_host_document() {
    let old_fixture = positive_fixture(SCENES[0]);
    let old_styles = cascade(&old_fixture.view, POSITIVE_STYLESHEET);
    let new_fixture = positive_fixture(SCENES[0]);
    assert_ne!(
        old_fixture.snapshot.generation(),
        new_fixture.snapshot.generation()
    );
    let error = take_error(
        generic_direct_text::prepare(&new_fixture.snapshot, new_fixture.root, &old_styles),
        "stale style snapshot must not produce a collector input",
    );
    assert_eq!(error.code, "unsupported_c114_text_style");
    assert_eq!(error.node_id, Some(new_fixture.root.id().get()));
}

#[test]
fn two_candidates_allow_a_non_text_block_sibling_without_adding_text_input() {
    let mut children = SCENES[0].items.iter().map(item_spec).collect::<Vec<_>>();
    children.push(element(
        HTML,
        "div",
        Some("empty-block"),
        Some("item"),
        None,
        vec![],
    ));
    let fixture = custom_fixture("generic-empty-sibling", "scene", children, None);
    let computed = cascade(&fixture.view, POSITIVE_STYLESHEET);
    let input = generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed)
        .unwrap()
        .expect("two direct text candidates should be collected");
    assert_eq!(input.paragraph_styles.len(), 2);
}

#[test]
fn independent_font_registry_token_is_not_equated_by_numeric_revision() {
    let fixture = positive_fixture(SCENES[0]);
    let computed = cascade(&fixture.view, POSITIVE_STYLESHEET);
    let mut input = generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed)
        .unwrap()
        .unwrap();
    let item = fixture.items[0];
    let text_node = direct_text(&fixture.snapshot, item);
    let paragraph = ParagraphInput {
        node: item.id(),
        sources: vec![TextSource {
            node: text_node.id(),
            inline_ancestors: Vec::new(),
            text: "A BC".to_owned(),
        }],
        style: input.paragraph_styles[&item.id()].clone(),
        inline_styles: BTreeMap::new(),
        inline_run_styles: BTreeMap::new(),
    };
    let original = input.engine.intrinsic_inline_sizes(&paragraph).unwrap();
    let mut independent_engine = TextLayoutEngine::new_without_system_fonts();
    independent_engine.register_font("Ahem", AHEM).unwrap();
    let foreign = independent_engine
        .intrinsic_inline_sizes(&paragraph)
        .unwrap();
    assert_eq!(
        original.font_registry_revision(),
        foreign.font_registry_revision(),
        "both engines register one identical font asset at the same numeric revision"
    );
    assert!(!original.uses_same_font_registry_revision(&foreign));
}
