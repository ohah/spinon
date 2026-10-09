use std::fs;

use serde_json::Value;
use spinon_core::{EnvironmentRevision, StyleRevision};
use spinon_style::{
    ComputedStyleProfile, CssCascadeError, CssOrigin, CssViewport, StylesheetSource,
};

use super::tests::DocumentFixture;
use crate::{StyleLayoutError, compute_flex_alignment_layers_style_layout};

#[test]
fn flex_alignment_cascade_layers_profile_matches_chromium_values_and_frames() {
    let input: Value = serde_json::from_str(
        &fs::read_to_string(super::tests::FLEX_ALIGNMENT_LAYERS_INPUT).unwrap(),
    )
    .unwrap();
    let reference: Value = serde_json::from_str(
        &fs::read_to_string(super::tests::FLEX_ALIGNMENT_LAYERS_REFERENCE).unwrap(),
    )
    .unwrap();
    assert_eq!(input["schema"], "spinon-css-c04-cascade-layers-input/v1");
    assert_eq!(
        reference["schema"],
        "spinon-css-c04-cascade-layers-reference/v1"
    );
    assert_eq!(reference["fixture"]["id"], input["fixtureId"]);
    assert_eq!(reference["oracle"]["name"], "Chromium");
    assert_eq!(reference["observations"].as_array().unwrap().len(), 16);

    let tolerance = input["comparison"]["perCoordinateMaximumAbsoluteError"]
        .as_f64()
        .unwrap() as f32;
    for case in input["cases"].as_array().unwrap() {
        let case_id = case["id"].as_str().unwrap();
        let expected = reference["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|observation| observation["caseId"] == case_id)
            .unwrap_or_else(|| panic!("Chromium reference에 {case_id}가 없습니다"));
        let fixture = DocumentFixture::new(false);
        let author_stylesheets = case["authorStylesheets"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(index, css)| author_stylesheet(case_id, index, css.as_str().unwrap()))
            .collect::<Vec<_>>();
        let output = fixture
            .compute_flex_alignment_layers(&author_stylesheets)
            .unwrap_or_else(|error| panic!("{case_id} 계산 실패: {error}"));

        assert_eq!(
            output.computed_styles.profile,
            ComputedStyleProfile::FlexAlignmentCascadeLayersV1
        );
        assert_eq!(
            computed_property(&fixture, &output, "flex-parent", "align-items"),
            expected["computed"]["alignItems"].as_str().unwrap(),
            "{case_id} align-items"
        );
        assert_eq!(
            computed_property(&fixture, &output, "flex-parent", "justify-content"),
            expected["computed"]["justifyContent"].as_str().unwrap(),
            "{case_id} justify-content"
        );
        for item_style in expected["itemStyles"].as_array().unwrap() {
            let node = item_style["id"].as_str().unwrap();
            for (field, property) in [
                ("flexGrow", "flex-grow"),
                ("flexShrink", "flex-shrink"),
                ("flexBasis", "flex-basis"),
            ] {
                assert_eq!(
                    computed_property(&fixture, &output, node, property),
                    item_style[field].as_str().unwrap(),
                    "{case_id} {node}.{property}"
                );
            }
        }

        let expected_frames = expected["frames"].as_array().unwrap();
        assert_eq!(expected_frames.len(), 4, "{case_id} frame 수");
        for expected_frame in expected_frames {
            let node_name = expected_frame["id"].as_str().unwrap();
            let node = fixture.nodes[node_name].id();
            let actual = output.layout.frames.get(&node).unwrap();
            for (field, actual_value) in [
                ("x", actual.x),
                ("y", actual.y),
                ("width", actual.width),
                ("height", actual.height),
            ] {
                let expected_value = expected_frame[field].as_f64().unwrap() as f32;
                assert!(
                    (actual_value - expected_value).abs() <= tolerance,
                    "{case_id} {node_name}.{field}: Spinon {actual_value}, Chromium {expected_value}, 허용치 {tolerance} CSS px"
                );
            }
        }
    }
}

#[test]
fn cascade_layers_allowlist_is_checked_recursively_and_old_profile_rejects_layers() {
    let fixture = DocumentFixture::new(false);
    for css in [
        "@layer app { #flex-parent { align-self: center; } }",
        "@layer app { #flex-parent { --alignment: center; } }",
        "@layer app { @supports (display: grid) { #flex-parent { justify-content: center; } } }",
        "@media (min-width: 1px) { @layer app { #flex-parent { justify-content: center; } } }",
        "@layer app { #flex-parent { & > div { justify-content: center; } } }",
    ] {
        let source = author_stylesheet("unsupported", 0, css);
        assert!(
            matches!(
                fixture.compute_flex_alignment_layers(&[source]),
                Err(StyleLayoutError::Cascade(
                    CssCascadeError::UnsupportedAuthorCss { .. }
                ))
            ),
            "지원하지 않는 CSS가 layer 아래에서도 거부되어야 합니다: {css}"
        );
    }

    let import = author_stylesheet(
        "import",
        0,
        "@import url(\"https://spinon.invalid/c04/import.css\") layer(app); \
         #flex-parent { justify-content: center; }",
    );
    assert!(
        matches!(
            fixture.compute_flex_alignment_layers(&[import]),
            Err(StyleLayoutError::CascadeDiagnostic(_))
                | Err(StyleLayoutError::Cascade(
                    CssCascadeError::UnsupportedAuthorCss { .. }
                ))
        ),
        "@import는 외부 로더 없이 요청하지 않고 전체 계산을 거부해야 합니다"
    );

    let diagnostic = author_stylesheet(
        "diagnostic",
        0,
        "@layer app { #flex-parent { display: grid; }",
    );
    assert!(
        matches!(
            fixture.compute_flex_alignment_layers(&[diagnostic]),
            Err(StyleLayoutError::CascadeDiagnostic(_))
        ),
        "손상된 layer 입력은 partial style/layout을 만들면 안 됩니다"
    );

    let legacy = fixture.compute_flex_alignment(Some(
        "@layer app { #flex-parent { justify-content: center; } }",
    ));
    assert!(
        matches!(
            legacy,
            Err(StyleLayoutError::Cascade(
                CssCascadeError::UnsupportedAuthorCss { .. }
            ))
        ),
        "C04.3 profile의 허용 범위는 바뀌면 안 됩니다"
    );
}

#[test]
fn cascade_layers_profile_preserves_revision_tuple_and_rejects_foreign_generation() {
    let fixture = DocumentFixture::new(false);
    let style_revision = StyleRevision::default().checked_next().unwrap();
    let environment_revision = EnvironmentRevision::default().checked_next().unwrap();
    let viewport = CssViewport {
        environment_revision,
        ..fixture.viewport()
    };
    let snapshot = fixture.document.snapshot();
    let source = fixture.stylesheet();
    let output = compute_flex_alignment_layers_style_layout(
        &snapshot,
        &fixture.view(),
        fixture.root,
        std::slice::from_ref(&source),
        viewport,
        style_revision,
    )
    .unwrap();
    assert_eq!(
        output.computed_styles.profile,
        ComputedStyleProfile::FlexAlignmentCascadeLayersV1
    );
    assert_eq!(output.computed_styles.style_revision, style_revision);
    assert_eq!(output.computed_styles.viewport, viewport);
    assert_eq!(output.layout.revision.style(), style_revision);
    assert_eq!(output.layout.revision.environment(), environment_revision);

    let foreign = DocumentFixture::new(false);
    assert!(matches!(
        compute_flex_alignment_layers_style_layout(
            &snapshot,
            &foreign.view(),
            fixture.root,
            &[source],
            fixture.viewport(),
            StyleRevision::default(),
        ),
        Err(StyleLayoutError::SnapshotMismatch {
            field: "DocumentGeneration"
        })
    ));
}

fn author_stylesheet(case_id: &str, index: usize, css: &str) -> StylesheetSource {
    let id = format!("c04-layers-{case_id}-{index}");
    StylesheetSource {
        base_url: format!("https://spinon.invalid/c04/layers/{id}.css"),
        id,
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

fn computed_property<'a>(
    fixture: &'a DocumentFixture,
    output: &'a crate::StyleLayoutOutput,
    node_name: &str,
    property: &str,
) -> &'a str {
    let node = fixture.nodes[node_name].id();
    output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap_or_else(|| panic!("{node_name} computed style가 없습니다"))
        .properties
        .get(property)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("{node_name}.{property} computed value가 없습니다"))
}
