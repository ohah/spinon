use serde_json::Value;

use super::fixture::{PROFILES, cases, compute_case, observations, reference};

#[test]
fn all_chromium_flex_alignment_cases_match_all_six_runtime_profiles() {
    let reference = reference();
    let observations = observations(&reference);
    assert_eq!(
        observations.len(),
        2,
        "DPR 1·2 Chromium observation이 있어야 합니다"
    );
    assert_eq!(
        observations[0]["viewport"]["deviceScaleFactor"], 1,
        "Rust layout은 DPR과 무관한 CSS px geometry를 비교합니다"
    );
    assert_eq!(observations[0]["cases"], observations[1]["cases"]);
    let fixture_cases = cases(&observations[0]);
    assert_eq!(fixture_cases.len(), 50);
    assert_eq!(
        fixture_cases
            .iter()
            .map(|case| case["nodes"].as_array().unwrap().len())
            .sum::<usize>(),
        134
    );

    for profile in PROFILES {
        for case in fixture_cases {
            let (actual, nodes) = compute_case(case, profile);
            let expected_nodes = case["nodes"].as_array().unwrap();
            assert_eq!(
                actual.layout.frames.len(),
                expected_nodes.len(),
                "{}. {profile:?}",
                case["id"]
            );
            for expected_node in expected_nodes {
                let id = expected_node["id"].as_str().unwrap();
                let handle = nodes[id];
                let computed = actual
                    .computed_styles
                    .elements
                    .iter()
                    .find(|element| element.node_id == handle.id())
                    .unwrap_or_else(|| panic!("{}.{id} computed style 누락", case["id"]));
                assert_properties(case, expected_node, computed, profile);

                let frame = actual
                    .layout
                    .frames
                    .get(&handle.id())
                    .unwrap_or_else(|| panic!("{}.{id} layout frame 누락", case["id"]));
                let expected_frame = &expected_node["rect"];
                for (field, actual) in [
                    ("x", frame.x),
                    ("y", frame.y),
                    ("width", frame.width),
                    ("height", frame.height),
                ] {
                    let expected = expected_frame[field].as_f64().unwrap();
                    assert!(
                        (f64::from(actual) - expected).abs() <= 0.5,
                        "{}.{id}.{field} {profile:?}: Taffy {actual}, Chrome {expected}, 허용치 0.5 CSS px",
                        case["id"]
                    );
                }
            }
        }
    }
}

fn assert_properties(
    case: &Value,
    expected_node: &Value,
    actual: &spinon_style::ComputedElementStyle,
    profile: spinon_style::ComputedStyleProfile,
) {
    let id = expected_node["id"].as_str().unwrap();
    let properties = expected_node["properties"].as_object().unwrap();
    for (property, expected) in properties {
        let expected = expected.as_str().unwrap();
        assert_eq!(
            actual.properties.get(property).map(String::as_str),
            Some(expected),
            "{}.{id}.{property} {profile:?}",
            case["id"]
        );
    }
}
