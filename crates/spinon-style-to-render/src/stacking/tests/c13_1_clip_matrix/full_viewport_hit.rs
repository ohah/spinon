use super::{
    SystemReserve, build_runtime_render_snapshot_with_reserve,
    overflow_runtime_inputs_with_viewport, target,
};
use serde_json::Value;
use spinon_core::NodeId;
use spinon_render::RuntimeRenderSnapshot;

const CHROME_FULL_VIEWPORT_HIT_IDENTITY: &str = include_str!(
    "../../../../../../spec/internal/evidence/c13-1-overflow-clip-full-viewport-hit-identity-2026-10-14/chrome155-full-viewport-hit-identity.json"
);
const EXPECTED_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36";
const VIEWPORT_WIDTH: i32 = 1280;
const VIEWPORT_HEIGHT: i32 = 1100;
const EXPECTED_CASE_COUNT: usize = 13;
const EXPECTED_POINT_COUNT: usize = VIEWPORT_WIDTH as usize * VIEWPORT_HEIGHT as usize;

struct CaseSpec {
    id: &'static str,
    section_x: i32,
    section_y: i32,
    box_position: &'static str,
    box_overflow: &'static str,
    target_style: &'static str,
    nested_inner: bool,
}

const CASES: [CaseSpec; EXPECTED_CASE_COUNT] = [
    CaseSpec {
        id: "visible",
        section_x: 20,
        section_y: 64,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow:visible;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "clip-x",
        section_x: 218,
        section_y: 64,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow-x:clip;overflow-y:visible;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "clip-y",
        section_x: 416,
        section_y: 64,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow-x:visible;overflow-y:clip;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "clip-both",
        section_x: 20,
        section_y: 232,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow:clip;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "clip-margin-padding",
        section_x: 218,
        section_y: 232,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow:clip;overflow-clip-margin:padding-box 8px;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "clip-margin-content",
        section_x: 416,
        section_y: 232,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow:clip;overflow-clip-margin:content-box 8px;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "clip-margin-border",
        section_x: 20,
        section_y: 400,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow:clip;overflow-clip-margin:border-box 8px;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "clip-margin-hidden",
        section_x: 416,
        section_y: 400,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow:hidden;overflow-clip-margin:border-box 8px;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "mixed-visible-hidden",
        section_x: 20,
        section_y: 568,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow-x:visible;overflow-y:hidden;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "mixed-clip-auto",
        section_x: 218,
        section_y: 568,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow-x:clip;overflow-y:auto;overflow-clip-margin:border-box 8px;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: false,
    },
    CaseSpec {
        id: "positioned-contained",
        section_x: 416,
        section_y: 736,
        box_position: "position:relative;left:0px;top:20px;",
        box_overflow: "overflow:clip;",
        target_style: "position:absolute;left:84px;top:42px;box-sizing:border-box;width:42px;height:30px;background-color:#e3544e",
        nested_inner: false,
    },
    CaseSpec {
        id: "positioned-escape",
        section_x: 20,
        section_y: 904,
        box_position: "position:static;",
        box_overflow: "overflow:clip;",
        target_style: "position:absolute;left:94px;top:64px;box-sizing:border-box;width:42px;height:30px;background-color:#e3544e",
        nested_inner: false,
    },
    CaseSpec {
        id: "nested-clip",
        section_x: 218,
        section_y: 904,
        box_position: "position:absolute;left:0px;top:20px;",
        box_overflow: "overflow:clip;",
        target_style: "width:140px;height:120px;background-color:#57a6d9",
        nested_inner: true,
    },
];

#[derive(Clone, Copy)]
struct CaseNodes {
    section: NodeId,
    clip_box: NodeId,
    target: NodeId,
    inner: Option<NodeId>,
}

fn assert_frame_matches(
    scene: &RuntimeRenderSnapshot,
    node: NodeId,
    expected: &Value,
    label: &str,
) {
    let frame = target(scene, node).frame_css_px();
    for (field, actual) in [
        ("x", frame.x()),
        ("y", frame.y()),
        ("width", frame.width()),
        ("height", frame.height()),
    ] {
        let expected_value = expected[field]
            .as_f64()
            .unwrap_or_else(|| panic!("Chrome {label} frame field {field}"))
            as f32;
        assert!(
            (actual - expected_value).abs() <= 0.5,
            "Chrome {label} frame {field}: runtime={actual}, oracle={expected_value}"
        );
    }
}

fn expected_node(
    identity: &str,
    nodes: &[(String, CaseNodes)],
    root_node: NodeId,
) -> Option<NodeId> {
    if identity == "root" {
        return Some(root_node);
    }
    if identity == "null" {
        return None;
    }
    let (id, kind) = identity
        .split_once(':')
        .unwrap_or_else(|| panic!("invalid Chrome full viewport identity {identity}"));
    let case = nodes
        .iter()
        .find(|(case_id, _)| case_id == id)
        .map(|(_, nodes)| nodes)
        .unwrap_or_else(|| panic!("unknown Chrome case identity {identity}"));
    match kind {
        "case" => Some(case.section),
        "box" => Some(case.clip_box),
        "target" => Some(case.target),
        "inner" => Some(case.inner.unwrap_or_else(|| {
            panic!("Chrome reported nested inner for a scene without one: {identity}")
        })),
        other => panic!("unknown Chrome full viewport identity kind {other}"),
    }
}

#[test]
fn c131_combined_page_full_viewport_hit_identity_matches_chrome() {
    let observation: Value = serde_json::from_str(CHROME_FULL_VIEWPORT_HIT_IDENTITY)
        .expect("agent-browser Chrome 155 combined full viewport hit identity observation");
    assert_eq!(
        observation["browser"]["version"].as_str(),
        Some("155.0.8059.40")
    );
    assert_eq!(
        observation["browser"]["revision"].as_str(),
        Some("cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}")
    );
    assert_eq!(
        observation["browser"]["userAgent"].as_str(),
        Some(EXPECTED_USER_AGENT)
    );
    assert_eq!(
        observation["browser"]["viewport"]["width"].as_i64(),
        Some(VIEWPORT_WIDTH as i64)
    );
    assert_eq!(
        observation["browser"]["viewport"]["height"].as_i64(),
        Some(VIEWPORT_HEIGHT as i64)
    );
    assert_eq!(
        observation["browser"]["viewport"]["dpr"].as_f64(),
        Some(1.0)
    );
    assert_eq!(
        observation["browser"]["viewport"]["scrollX"].as_i64(),
        Some(0)
    );
    assert_eq!(
        observation["browser"]["viewport"]["scrollY"].as_i64(),
        Some(0)
    );
    assert_eq!(
        observation["fixture"]["sha256"].as_str(),
        Some("b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a")
    );
    assert_eq!(
        observation["fixture"]["targetFrameHitIdentity2dSha256"].as_str(),
        Some("248c9d15a56ef332428b0c1e341bac1b1ebe5ce6ea3ac7d01c95a95bd787fe5b")
    );
    assert_eq!(
        observation["scan"]["mode"].as_str(),
        Some("full-viewport-integer-grid-2d-combined-c131-page")
    );
    assert_eq!(
        observation["scan"]["caseCount"].as_u64(),
        Some(EXPECTED_CASE_COUNT as u64)
    );
    assert_eq!(
        observation["scan"]["pointCount"].as_u64(),
        Some(EXPECTED_POINT_COUNT as u64)
    );
    assert_eq!(
        observation["scan"]["identityProjection"].as_str(),
        Some(
            "each valid case section, box, target and nested inner keeps its case id; invalid/float rows and other page elements map to runtime viewport root; viewport-exterior null controls are preserved separately"
        )
    );

    let cases = observation["cases"]
        .as_array()
        .expect("13 fixed Chrome case frames");
    assert_eq!(cases.len(), EXPECTED_CASE_COUNT);
    let mut expected_identities = vec!["root".to_owned()];
    assert_eq!(
        observation["rows"].as_array().unwrap().len(),
        VIEWPORT_HEIGHT as usize
    );
    for (case_index, spec) in CASES.iter().enumerate() {
        assert_eq!(cases[case_index]["id"].as_str(), Some(spec.id));
        assert_eq!(
            cases[case_index]["targetId"].as_str(),
            Some(format!("{}-target", spec.id).as_str())
        );
        expected_identities.extend([
            format!("{}:case", spec.id),
            format!("{}:box", spec.id),
            format!("{}:target", spec.id),
        ]);
        if spec.nested_inner {
            expected_identities.push(format!("{}:inner", spec.id));
        }
    }
    expected_identities.sort();
    let mut observed_identities: Vec<String> = observation["scan"]["uniqueIdentities"]
        .as_array()
        .expect("Chrome unique full viewport identities")
        .iter()
        .map(|identity| {
            identity
                .as_str()
                .expect("Chrome identity string")
                .to_owned()
        })
        .collect();
    observed_identities.sort();
    assert_eq!(observed_identities, expected_identities);

    let mut styles = vec![format!(
        "display:block;position:relative;width:{VIEWPORT_WIDTH}px;height:{VIEWPORT_HEIGHT}px;background-color:#ffffff"
    )];
    let mut parents = vec![None];
    let mut node_indices = Vec::with_capacity(EXPECTED_CASE_COUNT);
    for spec in CASES {
        let section_index = styles.len();
        styles.push(format!(
            "position:absolute;left:{}px;top:{}px;box-sizing:border-box;width:180px;height:150px;padding-top:20px;background-color:#ffffff",
            spec.section_x, spec.section_y
        ));
        parents.push(Some(0));

        let box_index = styles.len();
        styles.push(format!(
            "{}box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;{}background-color:#e7edf2",
            spec.box_position, spec.box_overflow
        ));
        parents.push(Some(section_index));

        let inner_index = if spec.nested_inner {
            let index = styles.len();
            styles.push("position:absolute;left:24px;top:26px;box-sizing:border-box;width:78px;height:68px;padding:4px;border:3px solid;overflow-x:clip;overflow-y:visible;background-color:#f4ecf7".to_owned());
            parents.push(Some(box_index));
            Some(index)
        } else {
            None
        };
        let target_parent = inner_index.unwrap_or(box_index);
        let target_index = styles.len();
        styles.push(spec.target_style.to_owned());
        parents.push(Some(target_parent));
        node_indices.push((
            spec.id.to_owned(),
            section_index,
            box_index,
            target_index,
            inner_index,
        ));
    }
    let style_refs: Vec<&str> = styles.iter().map(String::as_str).collect();
    let (input, raw_nodes) = overflow_runtime_inputs_with_viewport(
        &style_refs,
        &parents,
        VIEWPORT_WIDTH as f32,
        VIEWPORT_HEIGHT as f32,
    );
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .expect("combined C13.1 Chrome page runtime scene");
    let root_node = raw_nodes[0];
    let nodes: Vec<(String, CaseNodes)> = node_indices
        .iter()
        .map(|(id, section, clip_box, target, inner)| {
            (
                id.clone(),
                CaseNodes {
                    section: raw_nodes[*section],
                    clip_box: raw_nodes[*clip_box],
                    target: raw_nodes[*target],
                    inner: inner.map(|index| raw_nodes[index]),
                },
            )
        })
        .collect();

    for (case_index, (id, case_nodes)) in nodes.iter().enumerate() {
        let observed = &cases[case_index];
        assert_frame_matches(
            &scene,
            case_nodes.section,
            &observed["section"],
            &format!("{id} section"),
        );
        assert_frame_matches(
            &scene,
            case_nodes.clip_box,
            &observed["box"],
            &format!("{id} clip box"),
        );
        assert_frame_matches(
            &scene,
            case_nodes.target,
            &observed["target"],
            &format!("{id} target"),
        );
        if let Some(inner) = case_nodes.inner {
            assert_frame_matches(&scene, inner, &observed["inner"], &format!("{id} inner"));
        } else {
            assert!(
                observed["inner"].is_null(),
                "Chrome {id} unexpected nested inner"
            );
        }
    }

    let rows = observation["rows"].as_array().unwrap();
    let mut point_count = 0usize;
    for (row_index, row) in rows.iter().enumerate() {
        let y = row_index as i32;
        assert_eq!(
            row["y"].as_i64(),
            Some(y as i64),
            "Chrome full page y row gap"
        );
        let runs = row["runs"]
            .as_array()
            .unwrap_or_else(|| panic!("Chrome full page y={y} runs"));
        let mut next_x = 0i32;
        for run in runs {
            let from = run["from"].as_i64().unwrap() as i32;
            let to = run["to"].as_i64().unwrap() as i32;
            assert_eq!(from, next_x, "Chrome full page y={y} x run gap");
            assert!(
                to >= from && to < VIEWPORT_WIDTH,
                "Chrome full page y={y} invalid run"
            );
            let expected_node = expected_node(run["identity"].as_str().unwrap(), &nodes, root_node);
            for x in from..=to {
                let actual_node = scene
                    .hit_test_css_point(x as f32, y as f32)
                    .map(|hit| hit.node_id());
                assert_eq!(
                    actual_node, expected_node,
                    "Chrome full viewport hit identity at ({x}, {y})"
                );
                point_count += 1;
            }
            next_x = to + 1;
        }
        assert_eq!(
            next_x, VIEWPORT_WIDTH,
            "Chrome full page y={y} runs do not cover viewport"
        );
    }
    assert_eq!(
        point_count, EXPECTED_POINT_COUNT,
        "Chrome full viewport point count"
    );

    let off_viewport_controls = observation["offViewportNullControls"]
        .as_array()
        .expect("four Chrome viewport-exterior null controls");
    let expected_controls = [
        ("viewport-left-outside", -1, 20),
        ("viewport-top-outside", 100, -1),
        ("viewport-right-outside", VIEWPORT_WIDTH, 20),
        ("viewport-bottom-outside", 100, VIEWPORT_HEIGHT),
    ];
    assert_eq!(off_viewport_controls.len(), expected_controls.len());
    for (control, (id, x, y)) in off_viewport_controls.iter().zip(expected_controls) {
        assert_eq!(control["id"].as_str(), Some(id));
        assert_eq!(control["xCssPx"].as_i64(), Some(x as i64));
        assert_eq!(control["yCssPx"].as_i64(), Some(y as i64));
        assert_eq!(control["identity"].as_str(), Some("null"));
        assert!(control["elementId"].is_null());
        assert!(control["tagName"].is_null());
        assert_eq!(
            scene
                .hit_test_css_point(x as f32, y as f32)
                .map(|hit| hit.node_id()),
            None,
            "Chrome viewport-exterior null control {id} at ({x}, {y})"
        );
    }
}
