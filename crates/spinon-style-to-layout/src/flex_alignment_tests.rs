use std::fs;

use serde_json::Value;
use spinon_core::{EnvironmentRevision, StyleRevision};
use spinon_style::{ComputedStyleProfile, CssViewport};

use super::tests::DocumentFixture;
use crate::{StyleLayoutError, compute_flex_alignment_style_layout};

#[test]
fn flex_alignment_profile_matches_pinned_chromium_values_and_frames() {
    let input: Value =
        serde_json::from_str(&fs::read_to_string(super::tests::FLEX_ALIGNMENT_INPUT).unwrap())
            .unwrap();
    let reference: Value =
        serde_json::from_str(&fs::read_to_string(super::tests::FLEX_ALIGNMENT_REFERENCE).unwrap())
            .unwrap();
    assert_eq!(input["schema"], "spinon-css-c04-flex-alignment-input/v1");
    assert_eq!(
        reference["schema"],
        "spinon-css-c04-flex-alignment-reference/v1"
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
        let snapshot = fixture.document.snapshot();
        let output = fixture
            .compute_flex_alignment(Some(case["overrideCss"].as_str().unwrap()))
            .unwrap_or_else(|error| panic!("{case_id} 계산 실패: {error}"));

        assert_eq!(
            output.computed_styles.profile,
            ComputedStyleProfile::FlexAlignmentV1
        );
        assert_eq!(
            output.computed_styles.generation,
            snapshot.generation(),
            "{case_id} generation"
        );
        assert_eq!(
            output.layout.revision.source(),
            spinon_layout::LayoutSourceRevision::HostDocument {
                generation: snapshot.generation(),
                document: snapshot.document_revision(),
                render_tree: snapshot.render_tree_revision(),
            },
            "{case_id} source revision"
        );

        assert_computed_property(
            &fixture,
            &output,
            "flex-parent",
            "align-items",
            expected["computed"]["alignItems"].as_str().unwrap(),
        );
        assert_computed_property(
            &fixture,
            &output,
            "flex-parent",
            "justify-content",
            expected["computed"]["justifyContent"].as_str().unwrap(),
        );
        assert_computed_property(
            &fixture,
            &output,
            "flex-parent",
            "flex-direction",
            expected["computed"]["flexDirection"].as_str().unwrap(),
        );
        assert_computed_property(
            &fixture,
            &output,
            "flex-parent",
            "direction",
            expected["computed"]["direction"].as_str().unwrap(),
        );

        for item_style in expected["itemStyles"].as_array().unwrap() {
            let node = item_style["id"].as_str().unwrap();
            for (field, property) in [
                ("flexGrow", "flex-grow"),
                ("flexShrink", "flex-shrink"),
                ("flexBasis", "flex-basis"),
            ] {
                assert_computed_property(
                    &fixture,
                    &output,
                    node,
                    property,
                    item_style[field].as_str().unwrap(),
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
fn flex_alignment_rejects_unsupported_valid_values_and_properties() {
    let fixture = DocumentFixture::new(false);
    for (property, value) in [
        ("align-items", "baseline"),
        ("align-items", "safe center"),
        ("align-items", "start"),
        ("justify-content", "start"),
        ("justify-content", "safe center"),
    ] {
        let css = format!("#flex-parent {{ {property}: {value}; }}");
        let result = fixture.compute_flex_alignment(Some(&css));
        assert!(
            matches!(
                &result,
                Err(StyleLayoutError::UnsupportedComputedValue {
                    property: actual_property,
                    value: actual_value,
                    ..
                }) if *actual_property == property && actual_value == value
            ),
            "{property}: {value}는 부분 적용하지 않아야 합니다: {result:?}"
        );
    }

    for (property, expected_feature) in [
        ("align-self", "align-self"),
        ("align-content", "align-content"),
        ("place-items", "justify-items"),
    ] {
        let css = format!("#flex-parent {{ {property}: center; }}");
        let result = fixture.compute_flex_alignment(Some(&css));
        assert!(
            matches!(
                &result,
                Err(StyleLayoutError::Cascade(
                    spinon_style::CssCascadeError::UnsupportedAuthorCss { feature, .. }
                )) if feature.contains(expected_feature)
            ),
            "{property}는 cascade allowlist에서 거부해야 합니다: {result:?}"
        );
    }

    let diagnosed_result = fixture.compute_flex_alignment(Some("#flex-parent { display: grid; }"));
    assert!(
        matches!(
            diagnosed_result,
            Err(StyleLayoutError::CascadeDiagnostic(_))
        ),
        "Stylo cascade 진단이 있는 stylesheet는 전체 실패해야 합니다"
    );
}

#[test]
fn flex_alignment_profile_keeps_c042_and_inline_style_boundaries() {
    let fixture = DocumentFixture::new(false);
    let c042_result = fixture.compute(Some("#flex-parent { align-items: center; }"));
    assert!(
        matches!(
            c042_result,
            Err(StyleLayoutError::Cascade(
                spinon_style::CssCascadeError::UnsupportedAuthorCss { .. }
            ))
        ),
        "기존 C04.2 profile의 허용 범위를 바꾸면 안 됩니다"
    );

    let inline_fixture = DocumentFixture::with_inline_style("align-items: center");
    assert!(matches!(
        inline_fixture.compute_flex_alignment(None),
        Err(StyleLayoutError::UnsupportedInlineStyle(node))
            if node == inline_fixture.root.id()
    ));
}

#[test]
fn flex_alignment_profile_preserves_current_revisions_and_rejects_foreign_generation() {
    let fixture = DocumentFixture::new(false);
    let style_revision = StyleRevision::default().checked_next().unwrap();
    let environment_revision = EnvironmentRevision::default().checked_next().unwrap();
    let viewport = CssViewport {
        environment_revision,
        ..fixture.viewport()
    };
    let output = compute_flex_alignment_style_layout(
        &fixture.document.snapshot(),
        &fixture.view(),
        fixture.root,
        &[fixture.stylesheet()],
        viewport,
        style_revision,
    )
    .unwrap();
    assert_eq!(output.computed_styles.style_revision, style_revision);
    assert_eq!(output.computed_styles.viewport, viewport);
    assert_eq!(output.layout.revision.style(), style_revision);
    assert_eq!(output.layout.revision.environment(), environment_revision);

    let other = DocumentFixture::new(false);
    let foreign_view = other.view();
    assert!(matches!(
        compute_flex_alignment_style_layout(
            &fixture.document.snapshot(),
            &foreign_view,
            fixture.root,
            &[fixture.stylesheet()],
            fixture.viewport(),
            StyleRevision::default(),
        ),
        Err(StyleLayoutError::SnapshotMismatch {
            field: "DocumentGeneration"
        })
    ));
}

fn assert_computed_property(
    fixture: &DocumentFixture,
    output: &crate::StyleLayoutOutput,
    node_name: &str,
    property: &str,
    expected: &str,
) {
    let node = fixture.nodes[node_name].id();
    let computed = output
        .computed_styles
        .elements
        .iter()
        .find(|style| style.node_id == node)
        .unwrap_or_else(|| panic!("{node_name} computed style가 없습니다"));
    assert_eq!(
        computed.properties.get(property).map(String::as_str),
        Some(expected),
        "{node_name}.{property}"
    );
}
