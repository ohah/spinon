use super::*;
use style::properties::LonghandId;

fn sheet(id: &str, css: &str) -> StylesheetSource {
    sheet_with_origin(id, css, CssOrigin::Author)
}

fn sheet_with_origin(id: &str, css: &str, origin: CssOrigin) -> StylesheetSource {
    StylesheetSource {
        id: id.to_owned(),
        base_url: "https://spinon.invalid/word-break.css".to_owned(),
        origin,
        css: css.to_owned(),
    }
}

fn computed_word_break(stylesheets: &[StylesheetSource]) -> String {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:40px;height:20px;grid-template-columns:min-content;margin:0;padding:0",
    );
    computed_word_break_for_node(&fixture, stylesheets, fixture.root)
}

fn computed_child_word_break(stylesheets: &[StylesheetSource]) -> String {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:40px;height:20px;grid-template-columns:min-content;margin:0;padding:0",
    );
    computed_word_break_for_node(&fixture, stylesheets, fixture.child)
}

fn computed_word_break_for_node(
    fixture: &Fixture,
    stylesheets: &[StylesheetSource],
    node_id: spinon_core::NodeId,
) -> String {
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
        .find(|style| style.node_id == node_id)
        .unwrap()
        .properties
        .get("word-break")
        .unwrap()
        .clone()
}

fn computed_word_break_with_cascade_origins(
    fixture: &Fixture,
    author_stylesheets: &[StylesheetSource],
    user_agent_stylesheets: &[StylesheetSource],
    node_id: spinon_core::NodeId,
) -> String {
    let snapshot = super::super::compute_cascade_with_additional_user_agent_stylesheets(
        &fixture.view,
        user_agent_stylesheets,
        author_stylesheets,
        viewport(),
        Default::default(),
        &[("word-break", LonghandId::WordBreak)],
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
    )
    .unwrap();
    snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node_id)
        .unwrap()
        .properties
        .get("word-break")
        .unwrap()
        .clone()
}

#[test]
fn stylesheet_alias_uses_the_winning_rule_and_importance() {
    assert_eq!(
        computed_word_break(&[sheet("alias", "#root { word-break: break-word; }")]),
        "break-word"
    );
    assert_eq!(
        computed_word_break(&[sheet(
            "specificity",
            "#root { word-break: break-word; } div { word-break: normal; }",
        )]),
        "break-word"
    );
    assert_eq!(
        computed_word_break(&[
            sheet("alias", "#root { word-break: break-word; }"),
            sheet("normal", "#root { word-break: normal; }"),
        ]),
        "normal"
    );
    assert_eq!(
        computed_word_break(&[
            sheet(
                "important-alias",
                "#root { word-break: break-word !important; }"
            ),
            sheet("normal", "#root { word-break: normal; }"),
        ]),
        "break-word"
    );
}

#[test]
fn inherited_and_unset_child_values_preserve_the_parent_alias() {
    for (id, child_declaration) in [
        ("inherited", ""),
        ("inherit", "#child { word-break: inherit; }"),
        ("unset", "#child { word-break: unset; }"),
    ] {
        assert_eq!(
            computed_child_word_break(&[sheet(
                id,
                &format!("#root {{ word-break: break-word; }} {child_declaration}"),
            )]),
            "break-word",
            "child declaration: {child_declaration}"
        );
    }
}

#[test]
fn revert_uses_the_lower_cascade_origin_before_parent_inheritance() {
    let fixture = fixture(
        "display:grid;box-sizing:border-box;width:40px;height:20px;grid-template-columns:min-content;margin:0;padding:0",
    );
    assert_eq!(
        computed_word_break_with_cascade_origins(
            &fixture,
            &[sheet(
                "author",
                "#root { word-break: break-word; } #child { word-break: revert; }",
            )],
            &[sheet_with_origin(
                "additional-ua",
                "#child { word-break: normal; }",
                CssOrigin::UserAgent,
            )],
            fixture.child,
        ),
        "normal"
    );
}

#[test]
fn revert_layer_exposes_a_lower_layer_word_break_alias() {
    let fixture = fixture_with_child_style(
        "display:grid;box-sizing:border-box;width:40px;height:20px;grid-template-columns:min-content;margin:0;padding:0",
        "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0;word-break:revert-layer",
    );
    assert_eq!(
        computed_word_break_with_cascade_origins(
            &fixture,
            &[sheet("lower-layer", "#child { word-break: break-word; }")],
            &[],
            fixture.child,
        ),
        "break-word"
    );
}
