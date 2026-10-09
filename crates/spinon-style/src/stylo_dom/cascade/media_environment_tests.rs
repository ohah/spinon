use std::{collections::BTreeMap, fs};

use serde_json::Value;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, OwnerId, StyleRevision,
};
use style::context::QuirksMode;

use super::{
    CssCascadeError, CssColorScheme, CssMediaEnvironment, CssPointerCapabilities,
    CssPrimaryPointer, CssViewport, compute_flex_margin_cascade,
    compute_flex_media_environment_cascade,
};
use crate::stylo_dom::StyloDocumentView;
use crate::{CssOrigin, StylesheetSource};

const INPUT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c04/media-environment.v1.json"
);
const CSS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c04/media-environment.css"
);
const REFERENCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c04-media-environment-v1.json"
);
const HTML: &str = "http://www.w3.org/1999/xhtml";

struct Fixture {
    view: StyloDocumentView,
    nodes: BTreeMap<String, HostNodeHandle>,
    stylesheet: StylesheetSource,
}

impl Fixture {
    fn load(probes: &[String]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(740).unwrap(), document.document_revision());
        let root = append(&mut document, &mut batch, HostParent::Root, "div", "root");
        let mut nodes = BTreeMap::new();
        for probe in probes {
            let node = append(
                &mut document,
                &mut batch,
                HostParent::Node(root),
                "div",
                probe,
            );
            assert!(nodes.insert(probe.clone(), node).is_none());
        }
        document.commit(batch).unwrap();
        let view = StyloDocumentView::new_with_base_url(
            document.snapshot(),
            root,
            true,
            QuirksMode::NoQuirks,
            "https://spinon.invalid/c04/media-environment.html",
        )
        .unwrap();
        let stylesheet = StylesheetSource {
            id: "c04-media-environment-author".to_owned(),
            base_url: "https://spinon.invalid/c04/media-environment.css".to_owned(),
            origin: CssOrigin::Author,
            css: fs::read_to_string(CSS).unwrap(),
        };
        Self {
            view,
            nodes,
            stylesheet,
        }
    }

    fn compute(
        &self,
        media_environment: CssMediaEnvironment,
    ) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
        compute_flex_media_environment_cascade(
            &self.view,
            std::slice::from_ref(&self.stylesheet),
            CssViewport {
                width_css_px: 390.0,
                height_css_px: 844.0,
                device_scale_factor: 3.0,
                environment_revision: EnvironmentRevision::INITIAL.checked_next().unwrap(),
                media_environment,
            },
            StyleRevision::INITIAL,
        )
    }

    fn display<'a>(&self, snapshot: &'a super::ComputedStyleSnapshot, probe: &str) -> &'a str {
        let node = self.nodes[probe].id();
        snapshot
            .elements
            .iter()
            .find(|style| style.node_id == node)
            .unwrap()
            .properties["display"]
            .as_str()
    }
}

fn append(
    document: &mut HostDocument,
    batch: &mut DocumentChangeBatch,
    parent: HostParent,
    tag: &str,
    id: &str,
) -> HostNodeHandle {
    let node = document.reserve_node_handle().unwrap();
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: tag.to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "id").unwrap(),
        value: id.to_owned().into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node,
        before: None,
    });
    node
}

fn input() -> Value {
    let value: Value = serde_json::from_str(&fs::read_to_string(INPUT).unwrap()).unwrap();
    assert_eq!(value["schema"], "spinon-css-c04-media-environment-input/v1");
    value
}

fn probes(input: &Value) -> Vec<String> {
    input["probes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|probe| probe.as_str().unwrap().to_owned())
        .collect()
}

fn media_environment(value: &Value) -> CssMediaEnvironment {
    let color_scheme = match value["colorScheme"].as_str().unwrap() {
        "light" => CssColorScheme::Light,
        "dark" => CssColorScheme::Dark,
        other => panic!("알 수 없는 color scheme: {other}"),
    };
    let primary_pointer = match value["primaryPointer"].as_str().unwrap() {
        "none" => CssPrimaryPointer::None,
        "coarse" => CssPrimaryPointer::Coarse,
        "fine" => CssPrimaryPointer::Fine,
        other => panic!("알 수 없는 primary pointer: {other}"),
    };
    let all = &value["allPointers"];
    CssMediaEnvironment {
        color_scheme,
        primary_pointer,
        primary_hover: value["primaryHover"].as_bool().unwrap(),
        all_pointers: CssPointerCapabilities {
            coarse: all["coarse"].as_bool().unwrap(),
            fine: all["fine"].as_bool().unwrap(),
            hover: all["hover"].as_bool().unwrap(),
        },
    }
}

#[test]
fn computed_media_probes_match_pinned_chromium_reference() {
    let input = input();
    let reference: Value = serde_json::from_str(&fs::read_to_string(REFERENCE).unwrap()).unwrap();
    assert_eq!(
        reference["schema"],
        "spinon-css-c04-media-environment-reference/v1"
    );
    assert_eq!(reference["fixture"]["id"], input["fixtureId"]);
    assert_eq!(reference["oracle"]["name"], "Chromium");
    let fixture = Fixture::load(&probes(&input));

    for case in input["cases"].as_array().unwrap() {
        let case_id = case["id"].as_str().unwrap();
        let expected = reference["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|observation| observation["caseId"] == case_id)
            .unwrap_or_else(|| panic!("Chromium reference에 {case_id}가 없습니다"));
        let environment = media_environment(case);
        let snapshot = fixture
            .compute(environment)
            .unwrap_or_else(|error| panic!("{case_id} cascade 실패: {error}"));
        assert_eq!(
            snapshot.profile,
            super::ComputedStyleProfile::FlexMediaEnvironmentV1
        );
        assert_eq!(snapshot.viewport.media_environment, environment);
        assert_eq!(snapshot.viewport.environment_revision.get(), 1);
        for probe in probes(&input) {
            assert_eq!(
                fixture.display(&snapshot, &probe),
                expected["displays"][&probe].as_str().unwrap(),
                "{case_id} {probe}"
            );
        }
    }
}

#[test]
fn mixed_and_absent_pointer_mappings_follow_stylo_022_media_features() {
    let input = input();
    let fixture = Fixture::load(&probes(&input));
    let cases = [
        (
            "no pointer",
            CssMediaEnvironment {
                color_scheme: CssColorScheme::Light,
                primary_pointer: CssPrimaryPointer::None,
                primary_hover: false,
                all_pointers: CssPointerCapabilities {
                    coarse: false,
                    fine: false,
                    hover: false,
                },
            },
            [
                ("pointer-none", "none"),
                ("pointer-coarse", "block"),
                ("pointer-fine", "block"),
                ("hover-none", "none"),
                ("hover-hover", "block"),
                ("any-pointer-none", "none"),
                ("any-pointer-coarse", "block"),
                ("any-pointer-fine", "block"),
                ("any-hover-none", "none"),
                ("any-hover-hover", "block"),
            ],
        ),
        (
            "mixed pointers",
            CssMediaEnvironment {
                color_scheme: CssColorScheme::Dark,
                primary_pointer: CssPrimaryPointer::Coarse,
                primary_hover: false,
                all_pointers: CssPointerCapabilities {
                    coarse: true,
                    fine: true,
                    hover: true,
                },
            },
            [
                ("pointer-none", "block"),
                ("pointer-coarse", "none"),
                ("pointer-fine", "block"),
                ("hover-none", "none"),
                ("hover-hover", "block"),
                ("any-pointer-none", "block"),
                ("any-pointer-coarse", "none"),
                ("any-pointer-fine", "none"),
                ("any-hover-none", "block"),
                ("any-hover-hover", "none"),
            ],
        ),
    ];

    for (label, environment, expected) in cases {
        let snapshot = fixture.compute(environment).unwrap();
        for (probe, display) in expected {
            assert_eq!(
                fixture.display(&snapshot, probe),
                display,
                "{label}: {probe}"
            );
        }
    }

    let primary_absent_but_secondary_pointer_available = CssMediaEnvironment {
        primary_pointer: CssPrimaryPointer::None,
        primary_hover: false,
        all_pointers: CssPointerCapabilities {
            coarse: false,
            fine: true,
            hover: true,
        },
        ..CssMediaEnvironment::DESKTOP
    };
    let snapshot = fixture
        .compute(primary_absent_but_secondary_pointer_available)
        .unwrap();
    assert_eq!(fixture.display(&snapshot, "pointer-none"), "none");
    assert_eq!(fixture.display(&snapshot, "any-pointer-fine"), "none");
}

#[test]
fn contradictory_pointer_environment_fails_closed_before_stylesheet_work() {
    let input = input();
    let fixture = Fixture::load(&probes(&input));
    let valid = CssMediaEnvironment::DESKTOP;
    let invalid = [
        CssMediaEnvironment {
            primary_hover: true,
            primary_pointer: CssPrimaryPointer::None,
            ..valid
        },
        CssMediaEnvironment {
            primary_pointer: CssPrimaryPointer::Coarse,
            all_pointers: CssPointerCapabilities {
                coarse: false,
                fine: true,
                hover: true,
            },
            ..valid
        },
        CssMediaEnvironment {
            primary_pointer: CssPrimaryPointer::Fine,
            all_pointers: CssPointerCapabilities {
                coarse: true,
                fine: false,
                hover: true,
            },
            ..valid
        },
        CssMediaEnvironment {
            primary_hover: true,
            all_pointers: CssPointerCapabilities {
                hover: false,
                ..valid.all_pointers
            },
            ..valid
        },
        CssMediaEnvironment {
            all_pointers: CssPointerCapabilities {
                coarse: false,
                fine: false,
                hover: true,
            },
            ..valid
        },
    ];

    for environment in invalid {
        assert!(matches!(
            fixture.compute(environment),
            Err(CssCascadeError::InvalidMediaEnvironment)
        ));
    }

    let invalid_environment = CssMediaEnvironment {
        primary_pointer: CssPrimaryPointer::Coarse,
        ..valid
    };
    let invalid_stylesheet = StylesheetSource {
        base_url: "not an absolute URL".to_owned(),
        ..fixture.stylesheet.clone()
    };
    assert!(matches!(
        compute_flex_media_environment_cascade(
            &fixture.view,
            &[invalid_stylesheet],
            CssViewport {
                media_environment: invalid_environment,
                ..CssViewport::C04_FIXTURE
            },
            StyleRevision::INITIAL,
        ),
        Err(CssCascadeError::InvalidMediaEnvironment)
    ));
}

#[test]
fn unsupported_media_features_and_at_rules_are_rejected_by_the_new_profile() {
    let input = input();
    let fixture = Fixture::load(&probes(&input));
    for css in [
        "@media (min-width: 1px) { #scheme-light { display: none; } }",
        "@media (prefers-reduced-motion: reduce) { #scheme-light { display: none; } }",
        "@media (spinon-unknown-feature: on) { #scheme-light { display: none; } }",
        "@media (prefers-color-scheme:) { #scheme-light { display: none; } }",
        "@media (prefers-color-scheme) { #scheme-light { display: none; } }",
        "@media (pointer: light) { #scheme-light { display: none; } }",
        "@media (prefers-color-scheme: fine) { #scheme-light { display: none; } }",
        "@media (hover: coarse) { #scheme-light { display: none; } }",
        "@media (prefers-color-scheme: dark light) { #scheme-light { display: none; } }",
        "@media print { #scheme-light { display: none; } }",
        "@media (pointer: fine) or (hover: hover) { #scheme-light { display: none; } }",
        "@supports (display: grid) { #scheme-light { display: none; } }",
        "@import url(\"https://spinon.invalid/remote.css\");",
        "@spinon-unknown rule;",
    ] {
        let stylesheet = StylesheetSource {
            css: css.to_owned(),
            ..fixture.stylesheet.clone()
        };
        assert!(
            matches!(
                compute_flex_media_environment_cascade(
                    &fixture.view,
                    &[stylesheet],
                    CssViewport::C04_FIXTURE,
                    StyleRevision::INITIAL,
                ),
                Err(CssCascadeError::UnsupportedAuthorCss { .. })
            ),
            "지원하지 않는 at-rule 또는 media feature가 거부되어야 합니다: {css}"
        );
    }

    assert!(matches!(
        compute_flex_margin_cascade(
            &fixture.view,
            &[StylesheetSource {
                css: "@media (pointer: fine) { #scheme-light { display: none; } }".to_owned(),
                ..fixture.stylesheet.clone()
            }],
            CssViewport::C04_FIXTURE,
            StyleRevision::INITIAL,
        ),
        Err(CssCascadeError::UnsupportedAuthorCss { .. })
    ));
}

#[test]
fn media_type_conjunction_negation_and_comma_lists_are_evaluated() {
    let fixture = Fixture::load(&["query-probe".to_owned()]);
    for (query, expected_display) in [
        ("screen", "none"),
        ("all", "none"),
        ("screen and (pointer: fine) and (hover: hover)", "none"),
        ("not (pointer: coarse)", "none"),
        ("(pointer: coarse), (prefers-color-scheme: light)", "none"),
        ("not screen", "block"),
    ] {
        let stylesheet = StylesheetSource {
            id: format!("media-query-{query}"),
            css: format!(
                "div {{ display: block; }} @media {query} {{ #query-probe {{ display: none; }} }}"
            ),
            ..fixture.stylesheet.clone()
        };
        let snapshot = compute_flex_media_environment_cascade(
            &fixture.view,
            &[stylesheet],
            CssViewport::C04_FIXTURE,
            StyleRevision::INITIAL,
        )
        .unwrap_or_else(|error| panic!("지원 query `{query}`가 거부됨: {error}"));
        assert_eq!(
            fixture.display(&snapshot, "query-probe"),
            expected_display,
            "{query}"
        );
    }
}

#[test]
fn unrelated_recoverable_property_diagnostic_is_preserved_not_misclassified_as_media() {
    let fixture = Fixture::load(&["query-probe".to_owned()]);
    let stylesheet = StylesheetSource {
        css: "#query-probe { margin-left: media; }".to_owned(),
        ..fixture.stylesheet.clone()
    };
    let snapshot = compute_flex_media_environment_cascade(
        &fixture.view,
        &[stylesheet],
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .expect("허용 범위 CSS 선언의 일반 parse diagnostic은 보존해야 합니다");
    assert!(snapshot.diagnostics.iter().any(|diagnostic| {
        diagnostic.source_id == "c04-media-environment-author"
            && diagnostic.diagnostic.message.contains("margin-left: media")
    }));
}

#[test]
fn invalid_viewport_error_precedes_invalid_media_environment() {
    let input = input();
    let fixture = Fixture::load(&probes(&input));
    let mut viewport = CssViewport::C04_FIXTURE;
    viewport.width_css_px = 0.0;
    viewport.media_environment = CssMediaEnvironment {
        primary_pointer: CssPrimaryPointer::Coarse,
        ..CssMediaEnvironment::DESKTOP
    };
    assert!(matches!(
        compute_flex_margin_cascade(
            &fixture.view,
            std::slice::from_ref(&fixture.stylesheet),
            viewport,
            StyleRevision::INITIAL,
        ),
        Err(CssCascadeError::InvalidViewport)
    ));
}
