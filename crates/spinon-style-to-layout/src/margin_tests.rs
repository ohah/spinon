use std::fs;

use serde_json::Value;
use spinon_core::{EnvironmentRevision, StyleRevision};
use spinon_style::{
    ComputedStyleProfile, CssCascadeError, CssMediaEnvironment, CssOrigin, CssViewport,
    StylesheetSource,
};

use crate::{
    StyleLayoutError, compute_flex_alignment_layers_style_layout,
    compute_flex_alignment_style_layout, compute_flex_margin_style_layout,
    compute_s04_style_layout, compute_style_layout,
};

const INPUT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c04/margin-layout.v1.json"
);
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c04-margin-layout-v1.json"
);
const BASE_CSS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c04/margin-layout.css"
);
const LOGICAL_INPUT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c04/margin-logical-shorthand.v1.json"
);
const LOGICAL_REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c04-margin-logical-shorthand-v1.json"
);

#[test]
fn flex_margin_profile_matches_chromium_values_and_frames() {
    let input: Value = serde_json::from_str(&fs::read_to_string(INPUT).unwrap()).unwrap();
    let reference: Value = serde_json::from_str(&fs::read_to_string(REFERENCE).unwrap()).unwrap();
    assert_eq!(input["schema"], "spinon-css-c04-margin-layout-input/v1");
    assert_eq!(
        reference["schema"],
        "spinon-css-c04-margin-layout-reference/v1"
    );
    assert_eq!(reference["fixture"]["id"], input["fixtureId"]);
    assert_eq!(reference["oracle"]["name"], "Chromium");
    assert_eq!(reference["observations"].as_array().unwrap().len(), 10);

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
        let fixture = super::tests::DocumentFixture::new(false);
        let stylesheets = stylesheets(case_id, case["overrideCss"].as_str().unwrap());
        let viewport = CssViewport {
            width_css_px: input["viewport"]["widthCssPx"].as_f64().unwrap() as f32,
            height_css_px: input["viewport"]["heightCssPx"].as_f64().unwrap() as f32,
            device_scale_factor: input["viewport"]["deviceScaleFactor"].as_f64().unwrap() as f32,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::DESKTOP,
        };
        let output = compute_flex_margin_style_layout(
            &fixture.document.snapshot(),
            &fixture.view(),
            fixture.root,
            &stylesheets,
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap_or_else(|error| panic!("{case_id} 계산 실패: {error}"));
        assert_eq!(
            output.computed_styles.profile,
            ComputedStyleProfile::FlexMarginV1
        );

        for expected_style in expected["itemStyles"].as_array().unwrap() {
            let node_name = expected_style["id"].as_str().unwrap();
            for property in ["margin-top", "margin-right", "margin-bottom", "margin-left"] {
                let node_id = fixture.nodes[node_name].id();
                let style = output
                    .computed_styles
                    .elements
                    .iter()
                    .find(|style| style.node_id == node_id)
                    .unwrap_or_else(|| panic!("{case_id} {node_name} computed style 없음"));
                assert_eq!(
                    style.properties[property],
                    expected_style["margins"][property].as_str().unwrap(),
                    "{case_id} {node_name}.{property}"
                );
            }
        }

        let expected_frames = expected["frames"].as_array().unwrap();
        assert_eq!(expected_frames.len(), 4, "{case_id} frame 수");
        for expected_frame in expected_frames {
            let node_name = expected_frame["id"].as_str().unwrap();
            let is_hidden = expected["itemStyles"]
                .as_array()
                .unwrap()
                .iter()
                .any(|style| style["id"] == node_name && style["display"] == "none");
            if is_hidden {
                // display:none 요소에는 CSS layout box가 없고 getBoundingClientRect()는
                // viewport 원점을 반환하므로 그 요소 자체의 좌표는 비교하지 않습니다.
                continue;
            }
            let node_id = fixture.nodes[node_name].id();
            let actual = output
                .layout
                .frames
                .get(&node_id)
                .unwrap_or_else(|| panic!("{case_id} {node_name} layout frame 없음"));
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
fn logical_margin_shorthands_match_chromium_values_and_frames() {
    let input: Value = serde_json::from_str(&fs::read_to_string(LOGICAL_INPUT).unwrap()).unwrap();
    let reference: Value =
        serde_json::from_str(&fs::read_to_string(LOGICAL_REFERENCE).unwrap()).unwrap();
    assert_eq!(
        input["schema"],
        "spinon-css-c04-margin-logical-shorthand-input/v1"
    );
    assert_eq!(
        reference["schema"],
        "spinon-css-c04-margin-logical-shorthand-reference/v1"
    );
    assert_eq!(reference["fixture"]["id"], input["fixtureId"]);
    assert_eq!(reference["oracle"]["name"], "Chromium");
    assert_eq!(reference["observations"].as_array().unwrap().len(), 3);

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
        let fixture = super::tests::DocumentFixture::new(false);
        let stylesheets = stylesheets(case_id, case["overrideCss"].as_str().unwrap());
        let output = compute_flex_margin_style_layout(
            &fixture.document.snapshot(),
            &fixture.view(),
            fixture.root,
            &stylesheets,
            CssViewport {
                width_css_px: input["viewport"]["widthCssPx"].as_f64().unwrap() as f32,
                height_css_px: input["viewport"]["heightCssPx"].as_f64().unwrap() as f32,
                device_scale_factor: input["viewport"]["deviceScaleFactor"].as_f64().unwrap()
                    as f32,
                environment_revision: EnvironmentRevision::INITIAL,
                media_environment: CssMediaEnvironment::DESKTOP,
            },
            StyleRevision::INITIAL,
        )
        .unwrap_or_else(|error| panic!("{case_id} 계산 실패: {error}"));
        assert_eq!(
            output.computed_styles.profile,
            ComputedStyleProfile::FlexMarginV1
        );

        for expected_style in expected["itemStyles"].as_array().unwrap() {
            let node = fixture.nodes[expected_style["id"].as_str().unwrap()].id();
            let actual_style = output
                .computed_styles
                .elements
                .iter()
                .find(|style| style.node_id == node)
                .unwrap();
            for property in ["margin-top", "margin-right", "margin-bottom", "margin-left"] {
                assert_eq!(
                    actual_style.properties[property],
                    expected_style["margins"][property].as_str().unwrap(),
                    "{case_id} {property}"
                );
            }
        }

        for expected_frame in expected["frames"].as_array().unwrap() {
            let node_name = expected_frame["id"].as_str().unwrap();
            let node = fixture.nodes[node_name].id();
            let actual = &output.layout.frames[&node];
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
fn unsupported_margin_values_and_other_properties_fail_closed() {
    let fixture = super::tests::DocumentFixture::new(false);
    let css = "#flex-a { writing-mode: vertical-rl; }";
    let result = compute(&fixture, css);
    assert!(
        matches!(
            &result,
            Err(StyleLayoutError::Cascade(
                CssCascadeError::UnsupportedAuthorCss { .. }
            )) | Err(StyleLayoutError::CascadeDiagnostic(_))
        ),
        "수직 writing mode가 허용되면 안 됩니다: {css}; 결과 {result:?}"
    );

    for css in [
        "#flex-a { margin-left: 10%; }",
        "#flex-a { margin-left: auto; }",
        "#flex-a { margin-left: calc(10% + 2px); }",
    ] {
        assert!(
            matches!(
                compute(&fixture, css),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    property: "margin-left",
                    ..
                })
            ),
            "px 이외 computed margin을 조용히 변환하면 안 됩니다: {css}"
        );
    }

    assert!(matches!(
        compute(&fixture, "#flex-a { padding: 1px; }"),
        Err(StyleLayoutError::Cascade(
            CssCascadeError::UnsupportedAuthorCss { .. }
        ))
    ));
    assert!(matches!(
        compute(&fixture, "#flex-parent { margin: 1px; }"),
        Err(StyleLayoutError::UnsupportedRootMargin(_))
    ));
    assert!(matches!(
        compute(&fixture, "#flex-a { margin-left: ???; }"),
        Err(StyleLayoutError::CascadeDiagnostic(_))
    ));
}

#[test]
fn new_profile_preserves_revisions_inline_style_rejection_and_old_profile_boundary() {
    let fixture = super::tests::DocumentFixture::new(false);
    let style_revision = StyleRevision::INITIAL.checked_next().unwrap();
    let environment_revision = EnvironmentRevision::INITIAL.checked_next().unwrap();
    let viewport = CssViewport {
        width_css_px: 301.0,
        height_css_px: 80.0,
        device_scale_factor: 1.0,
        environment_revision,
        media_environment: CssMediaEnvironment::DESKTOP,
    };
    let output = compute_flex_margin_style_layout(
        &fixture.document.snapshot(),
        &fixture.view(),
        fixture.root,
        &stylesheets("revision", "#flex-a { margin-left: -7.5px; }")
            .into_iter()
            .collect::<Vec<_>>(),
        viewport,
        style_revision,
    )
    .unwrap();
    assert_eq!(output.computed_styles.style_revision, style_revision);
    assert_eq!(output.computed_styles.viewport, viewport);
    assert_eq!(output.layout.revision.style(), style_revision);
    assert_eq!(output.layout.revision.environment(), environment_revision);
    assert_eq!(output.layout.frames[&fixture.nodes["flex-a"].id()].x, -7.5);

    let document = fixture.document.snapshot();
    let view = fixture.view();
    let sheets = stylesheets("old", "#flex-a { margin-left: 2px; }");
    let old_profiles = [
        compute_style_layout(
            &document,
            &view,
            fixture.root,
            &sheets,
            viewport,
            style_revision,
        ),
        compute_flex_alignment_style_layout(
            &document,
            &view,
            fixture.root,
            &sheets,
            viewport,
            style_revision,
        ),
        compute_flex_alignment_layers_style_layout(
            &document,
            &view,
            fixture.root,
            &sheets,
            viewport,
            style_revision,
        ),
        compute_s04_style_layout(
            &document,
            &view,
            fixture.root,
            &sheets,
            viewport,
            style_revision,
        ),
    ];
    for old_profile in old_profiles {
        assert!(matches!(
            old_profile,
            Err(StyleLayoutError::Cascade(
                CssCascadeError::UnsupportedAuthorCss { .. }
            ))
        ));
    }

    let inline = super::tests::DocumentFixture::with_inline_style("margin-left: 2px");
    assert!(matches!(
        compute(&inline, "#flex-a { margin-left: 2px; }"),
        Err(StyleLayoutError::UnsupportedInlineStyle(_))
    ));
}

fn compute(
    fixture: &super::tests::DocumentFixture,
    extra_css: &str,
) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
    compute_flex_margin_style_layout(
        &fixture.document.snapshot(),
        &fixture.view(),
        fixture.root,
        &stylesheets("case", extra_css),
        CssViewport {
            width_css_px: 301.0,
            height_css_px: 80.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::DESKTOP,
        },
        StyleRevision::INITIAL,
    )
}

fn stylesheets(case_id: &str, extra_css: &str) -> [StylesheetSource; 2] {
    let base_id = format!("c04-margin-base-{case_id}");
    let case_id = format!("c04-margin-case-{case_id}");
    [
        StylesheetSource {
            id: base_id.clone(),
            base_url: format!("https://spinon.invalid/c04/margin/{base_id}.css"),
            origin: CssOrigin::Author,
            css: fs::read_to_string(BASE_CSS).unwrap(),
        },
        StylesheetSource {
            id: case_id.clone(),
            base_url: format!("https://spinon.invalid/c04/margin/{case_id}.css"),
            origin: CssOrigin::Author,
            css: extra_css.to_owned(),
        },
    ]
}
