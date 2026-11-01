use super::*;

const GRID_STYLE: &str = "display:grid;box-sizing:border-box;width:40px;height:20px;grid-template-columns:40px;grid-template-rows:20px;margin:0;padding:0";

fn computed_hyphens(
    fixture: &Fixture,
    stylesheets: &[StylesheetSource],
    node: spinon_core::NodeId,
) -> Option<String> {
    let snapshot = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &fixture.view,
        stylesheets,
        viewport(),
        Default::default(),
    )
    .unwrap();
    snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .and_then(|style| style.properties.get("hyphens").cloned())
}

#[test]
fn fixture_cascade_exposes_stylo_computed_alias_and_maps_initial() {
    for (declaration, expected) in [
        ("hyphens:none", Some("none")),
        ("hyphens:manual", Some("manual")),
        ("hyphens:initial", Some("manual")),
        ("hyphens:balanced", Some("balanced")),
        ("hyphens:auto", Some("auto")),
        ("hyphens:revert", None),
        ("hyphens:revert-layer", None),
        ("hyphens:unset", None),
    ] {
        let fixture = fixture(&format!("{GRID_STYLE};{declaration}"));
        assert_eq!(
            computed_hyphens(&fixture, &[], fixture.root).as_deref(),
            expected,
            "{declaration}"
        );
    }

    let fixture = fixture(GRID_STYLE);
    assert_eq!(computed_hyphens(&fixture, &[], fixture.root), None);
}

#[test]
fn fixture_cascade_preserves_inheritance_importance_specificity_and_source_order() {
    let inherited = fixture_with_child_style(
        &format!("{GRID_STYLE};hyphens:none"),
        "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0;hyphens:inherit",
    );
    assert_eq!(
        computed_hyphens(&inherited, &[], inherited.child).as_deref(),
        Some("none")
    );

    let cascade_fixture = fixture(&format!("{GRID_STYLE};hyphens:none"));
    let stylesheet = StylesheetSource {
        id: "hyphens-cascade".to_owned(),
        base_url: "https://spinon.invalid/hyphens-cascade.css".to_owned(),
        origin: CssOrigin::Author,
        css: "div { hyphens: none !important; } #root { hyphens: none !important; } #root { hyphens: manual !important; }".to_owned(),
    };
    assert_eq!(
        computed_hyphens(&cascade_fixture, &[stylesheet], cascade_fixture.root).as_deref(),
        Some("manual")
    );

    let fixture = fixture(GRID_STYLE);
    let stylesheet = StylesheetSource {
        id: "hyphens-inline-important".to_owned(),
        base_url: "https://spinon.invalid/hyphens-inline-important.css".to_owned(),
        origin: CssOrigin::Author,
        css: "#root { hyphens: manual !important; }".to_owned(),
    };
    assert_eq!(
        computed_hyphens(&fixture, &[stylesheet], fixture.root).as_deref(),
        Some("manual")
    );
}

#[test]
fn fixture_cascade_fails_closed_for_alias_collision_registration_all_and_dynamic_values() {
    let base_fixture = fixture(GRID_STYLE);
    for declaration in ["--shyph:none", "all:initial", "hyphens:var(--mode)"] {
        let fixture_with_declaration = fixture(&format!("{GRID_STYLE};{declaration}"));
        assert!(
            matches!(
                compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                    &fixture_with_declaration.view,
                    &[],
                    viewport(),
                    Default::default(),
                ),
                Err(crate::CssCascadeError::UnsupportedInlineAuthorProperty { .. })
            ),
            "{declaration}"
        );
    }

    for (id, css) in [
        ("hyphens-alias-collision", "#root { --shyph: none; }"),
        ("hyphens-all", "#root { all: initial; }"),
        (
            "hyphens-property-registration",
            "@property --shyph { syntax: '<custom-ident>'; inherits: true; }",
        ),
        ("hyphens-other-custom-property", "#root { --mode: none; }"),
    ] {
        let stylesheet = StylesheetSource {
            id: id.to_owned(),
            base_url: format!("https://spinon.invalid/{id}.css"),
            origin: CssOrigin::Author,
            css: css.to_owned(),
        };
        assert!(
            matches!(
                compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                    &base_fixture.view,
                    &[stylesheet],
                    viewport(),
                    Default::default(),
                ),
                Err(crate::CssCascadeError::UnsupportedAuthorCss { .. })
            ),
            "{css}"
        );
    }
}
