use serde_json::Value;

use super::fixture::{PROFILES, compute_case_with_device_scale_factor};

const CHROMIUM: &str = include_str!(
    "../../../../spec/internal/evidence/c10-3-3-android-2026-10-11/c1033-chrome-runtime.json"
);

#[test]
fn runtime_v8_alignment_fixture_matches_pinned_chromium() {
    let reference: Value = serde_json::from_str(CHROMIUM).unwrap();
    assert_eq!(
        reference["schema"],
        "spinon-c10-3-3-runtime-chromium-observation/v1"
    );
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");
    let nodes = reference["nodes"].as_array().unwrap();
    let case = serde_json::json!({"id": "runtime-v8", "nodes": nodes});
    for profile in PROFILES {
        for device_scale_factor in [1.0, 2.625] {
            let (actual, handles) =
                compute_case_with_device_scale_factor(&case, profile, device_scale_factor);
            for expected in nodes {
                let id = expected["id"].as_str().unwrap();
                let node = handles[id];
                let computed = actual
                    .computed_styles
                    .elements
                    .iter()
                    .find(|element| element.node_id == node.id())
                    .unwrap_or_else(|| panic!("runtime-v8/{id} computed style 누락"));
                if device_scale_factor > 1.0 && id == "c1033-runtime-flex" {
                    eprintln!(
                        "runtime parent border={:?} padding={:?} width={:?} height={:?} computed={:?}",
                        computed.layout_border,
                        computed.layout_spacing.padding,
                        computed.layout_dimensions.width,
                        computed.layout_dimensions.height,
                        computed.properties
                    );
                }
                for property in [
                    "align-items",
                    "align-self",
                    "align-content",
                    "justify-content",
                ] {
                    let expected_value = expected["properties"][match property {
                        "align-items" => "alignItems",
                        "align-self" => "alignSelf",
                        "align-content" => "alignContent",
                        "justify-content" => "justifyContent",
                        _ => unreachable!(),
                    }]
                    .as_str()
                    .unwrap();
                    assert_eq!(
                        computed.properties.get(property).map(String::as_str),
                        Some(expected_value),
                        "runtime-v8/{id}.{property} {profile:?} dpr={device_scale_factor}"
                    );
                }

                let frame = actual.layout.frames.get(&node.id()).unwrap();
                for (field, value) in [
                    ("x", frame.x),
                    ("y", frame.y),
                    ("width", frame.width),
                    ("height", frame.height),
                ] {
                    let expected_value = expected["rect"][field].as_f64().unwrap();
                    assert!(
                        (f64::from(value) - expected_value).abs() <= 0.5,
                        "runtime-v8/{id}.{field} {profile:?} dpr={device_scale_factor}: Taffy {value}, Chrome {expected_value}, 허용치 0.5 CSS px"
                    );
                }
            }
        }
    }
}
