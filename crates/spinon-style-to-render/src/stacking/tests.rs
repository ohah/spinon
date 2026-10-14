use std::{collections::BTreeSet, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostDocumentSnapshot,
    HostNodeHandle, HostParent, NodeId, OwnerId, StyleRevision,
};
use spinon_layout::LayoutFrame;
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets,
    compute_runtime_block_flex_stacking_cascade_with_stylesheets,
};
use spinon_style_to_layout::{StyleLayoutOutput, compute_runtime_style_layout};

use super::{
    build_runtime_render_snapshot_with_reserve,
    ordering::flatten_contexts,
    reserve::{FaultInjectingReserve, SystemReserve},
    types::{OrderedContext, OrderedPaintItem, StackingContext},
};
use crate::CurrentLayoutInputs;

#[path = "tests/c13_1_clip_matrix.rs"]
mod c13_1_clip_matrix;
#[path = "tests/c13_1_paint_hit_border.rs"]
mod c13_1_paint_hit_border;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const C13_OVERFLOW_CLIP_FIXTURE_JS: &str =
    include_str!("../../../../tests/fixtures/css/c13/runtime-overflow-clip.js");
const C13_CHROME_OVERFLOW_CLIP_OBSERVATION: &str = include_str!(
    "../../../../spec/internal/evidence/c13-1-overflow-clip-android-avd-2026-10-14/chrome155-runtime-observation.json"
);
const C13_CONTENT_BOX_POSITIONED_HIT_OBSERVATION: &str = include_str!(
    "../../../../spec/internal/evidence/c13-1-overflow-clip-runtime-matrix-2026-10-14/chrome155-content-box-positioned-hit-observation.json"
);

struct RuntimeInputs {
    document: HostDocumentSnapshot,
    root: HostNodeHandle,
    output: StyleLayoutOutput,
    current: CurrentLayoutInputs,
}

fn runtime_inputs() -> RuntimeInputs {
    let styles = [
        "display:block;position:relative;width:120px;height:100px;background-color:#111111",
        "position:absolute;z-index:-1;left:0;top:0;width:60px;height:60px;background-color:#0000ff",
        "position:absolute;z-index:0;left:0;top:0;width:60px;height:60px;background-color:#00ff00",
        "display:flex;position:absolute;z-index:1;left:0;top:0;width:80px;height:80px;flex-direction:row;align-items:flex-start;justify-content:flex-start;background-color:#ff0000",
        "display:block;position:static;z-index:auto;width:70px;height:50px;margin-right:-30px;background-color:#aa00aa",
        "display:block;position:static;z-index:2;width:70px;height:50px;margin-right:-30px;background-color:#aa00aa",
        "position:absolute;z-index:3;left:0;top:0;width:30px;height:30px;background-color:#ffffff",
        "position:absolute;z-index:-2;left:0;top:0;width:20px;height:20px;background-color:#ffff00",
        "position:relative;z-index:auto;width:16px;height:16px;background-color:#eeee00",
        "display:block;width:20px;height:20px;background-color:#ffff00",
    ];
    let parents = [
        None,
        Some(0),
        Some(0),
        Some(0),
        Some(3),
        Some(3),
        Some(3),
        Some(0),
        Some(7),
        Some(0),
    ];
    let mut document = HostDocument::new().expect("HostDocument");
    let handles = styles
        .iter()
        .map(|_| document.reserve_node_handle().expect("node handle"))
        .collect::<Vec<_>>();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(8124).expect("owner id"),
        document.document_revision(),
    );
    for (index, (handle, style)) in handles.iter().zip(styles).enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: *handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: *handle,
            name: AttributeName::new(None, "style").expect("style attribute"),
            value: style.to_owned().into(),
        });
        let parent = parents[index]
            .map(|parent| HostParent::Node(handles[parent]))
            .unwrap_or(HostParent::Root);
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: *handle,
            before: None,
        });
    }
    document.commit(batch).expect("fixture commit");

    let document = document.snapshot();
    let root = handles[0];
    let viewport = CssViewport {
        width_css_px: 120.0,
        height_css_px: 100.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.clone()), root)
        .expect("Stylo document view");
    let styles = compute_runtime_block_flex_stacking_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::default(),
    )
    .expect("runtime cascade");
    let output =
        compute_runtime_style_layout(&document, root, styles, viewport).expect("runtime layout");
    let current =
        CurrentLayoutInputs::for_host_document(&document, StyleRevision::default(), viewport);
    RuntimeInputs {
        document,
        root,
        output,
        current,
    }
}

fn overflow_runtime_inputs(
    styles: &[&str],
    parents: &[Option<usize>],
) -> (RuntimeInputs, Vec<NodeId>) {
    overflow_runtime_inputs_with_viewport(styles, parents, 200.0, 160.0)
}

fn overflow_runtime_inputs_with_viewport(
    styles: &[&str],
    parents: &[Option<usize>],
    width_css_px: f32,
    height_css_px: f32,
) -> (RuntimeInputs, Vec<NodeId>) {
    assert_eq!(styles.len(), parents.len());
    let mut document = HostDocument::new().unwrap();
    let handles = styles
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(8131).unwrap(), document.document_revision());
    for (index, (handle, style)) in handles.iter().zip(styles).enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: *handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: *handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: (*style).to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: parents[index]
                .map(|parent| HostParent::Node(handles[parent]))
                .unwrap_or(HostParent::Root),
            node: *handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let document = document.snapshot();
    let root = handles[0];
    let viewport = CssViewport {
        width_css_px,
        height_css_px,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let view = StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.clone()), root)
        .unwrap();
    let styles = compute_runtime_block_flex_overflow_clip_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::default(),
    )
    .unwrap();
    let output = compute_runtime_style_layout(&document, root, styles, viewport).unwrap();
    let current =
        CurrentLayoutInputs::for_host_document(&document, StyleRevision::default(), viewport);
    (
        RuntimeInputs {
            document,
            root,
            output,
            current,
        },
        handles.iter().map(|handle| handle.id()).collect(),
    )
}

#[test]
fn each_fallible_reservation_failure_returns_its_exact_stage() {
    let input = runtime_inputs();
    let mut recording = FaultInjectingReserve::recording();
    let snapshot = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut recording,
    )
    .expect("normal reservation path");
    assert!(!snapshot.boxes().is_empty());
    let stages = recording.stages().to_vec();
    let observed_stages = stages.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(
        observed_stages,
        BTreeSet::from([
            "root context",
            "document traversal",
            "document nodes",
            "document children",
            "stacking contexts",
            "negative contexts",
            "zero contexts",
            "positive contexts",
            "positioned auto boxes",
            "flex item boxes",
            "flex order",
            "block flow boxes",
            "paint traversal",
            "paint nodes",
            "render boxes",
        ]),
        "각 Vec 예약 오류 경로가 fixture에 포함되어야 합니다: {stages:?}"
    );

    for (index, expected_stage) in stages.iter().copied().enumerate() {
        let mut injected = FaultInjectingReserve::failing_at(index + 1);
        let error = build_runtime_render_snapshot_with_reserve(
            &input.document,
            input.root,
            &input.output,
            input.current,
            &mut injected,
        )
        .expect_err("주입한 예약 실패는 장면 생성을 중단해야 합니다");
        assert!(
            matches!(error, crate::StyleRenderError::AllocationFailed { stage } if stage == expected_stage),
            "예약 {}단계는 {expected_stage:?} 오류여야 하지만 {error}를 반환했습니다",
            index + 1
        );
        assert_eq!(injected.attempts(), index + 1);
    }
}

#[test]
fn c131_hit_test_order_reservation_failures_return_their_exact_stage() {
    let (input, _) = overflow_runtime_inputs(
        &[
            "position:relative;display:flow-root;width:100px;height:80px;background-color:#111111",
            "position:static;width:60px;height:40px;overflow:clip;background-color:#222222",
            "width:80px;height:50px;background-color:#333333",
        ],
        &[None, Some(0), Some(1)],
    );

    for stage in [
        "hit-test render nodes",
        "hit-test children",
        "hit-test traversal",
    ] {
        let mut injected = FaultInjectingReserve::failing_at_stage_occurrence(stage, 1);
        let error = build_runtime_render_snapshot_with_reserve(
            &input.document,
            input.root,
            &input.output,
            input.current,
            &mut injected,
        )
        .expect_err("hit-test order reservation failure must reject the snapshot");
        assert!(
            matches!(error, crate::StyleRenderError::AllocationFailed { stage: actual } if actual == stage),
            "{stage} reservation failure returned {error}"
        );
    }
}

#[test]
fn reservation_injection_skips_requests_that_cannot_grow_the_vector() {
    let mut values = Vec::with_capacity(2);
    values.push(1_u8);
    let mut injected = FaultInjectingReserve::failing_at(1);

    super::reserve::FallibleReserve::reserve(&mut injected, &mut values, 0, "no-op")
        .expect("0개 예약은 할당 실패가 아닙니다");
    super::reserve::FallibleReserve::reserve(&mut injected, &mut values, 1, "spare capacity")
        .expect("남은 용량 안의 예약은 할당하지 않습니다");
    assert_eq!(injected.attempts(), 0);

    let capacity = values.capacity();
    values.resize(capacity, 0);
    let error =
        super::reserve::FallibleReserve::reserve(&mut injected, &mut values, 1, "growth required")
            .expect_err("용량 확장이 필요한 예약은 주입 실패해야 합니다");
    assert!(matches!(
        error,
        crate::StyleRenderError::AllocationFailed {
            stage: "growth required"
        }
    ));
    assert_eq!(injected.attempts(), 1);
}

#[test]
fn overflow_clip_ancestry_reservation_failure_returns_error_and_recovers_on_same_reserve() {
    let (input, _) = overflow_runtime_inputs(
        &[
            "display:block;position:relative;width:200px;height:160px;background-color:#111111",
            "position:relative;left:20px;top:10px;width:80px;height:60px;padding:8px;border:4px solid;overflow:clip;overflow-clip-margin:padding-box 8px;background-color:#333333",
            "position:absolute;left:50px;top:40px;width:60px;height:40px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1)],
    );
    let mut baseline_reserve = SystemReserve;
    let baseline = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut baseline_reserve,
    )
    .unwrap();

    let mut recording = FaultInjectingReserve::recording();
    let recorded = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut recording,
    )
    .unwrap();
    assert_eq!(recorded, baseline);
    let ancestry_stage_count = recording
        .stages()
        .iter()
        .filter(|stage| **stage == "overflow clip ancestry")
        .count();
    assert!(
        ancestry_stage_count >= 3,
        "target lineage must reserve for target, clip parent, and root"
    );

    let mut injected =
        FaultInjectingReserve::failing_at_stage_occurrence("overflow clip ancestry", 3);
    let error = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut injected,
    )
    .expect_err("clip ancestry allocation failure must not publish a render snapshot");
    assert!(matches!(
        error,
        crate::StyleRenderError::AllocationFailed {
            stage: "overflow clip ancestry"
        }
    ));

    let recovered = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut injected,
    )
    .expect("the same reserve must build the next request after one injected failure");
    assert_eq!(recovered, baseline);
}

#[test]
fn overflow_clip_margin_uses_padding_edge_for_paint_and_hit_test() {
    let (input, nodes) = overflow_runtime_inputs(
        &[
            "display:block;position:relative;width:200px;height:160px;background-color:#111111",
            "position:relative;left:20px;top:10px;width:80px;height:60px;padding:8px;border:4px solid;overflow:clip;overflow-clip-margin:padding-box 8px;background-color:#333333",
            "position:absolute;left:50px;top:40px;width:60px;height:40px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1)],
    );
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let target_id = nodes[2];
    let target = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == target_id)
        .unwrap();
    let x = target.clip_css_px().x().unwrap();
    let y = target.clip_css_px().y().unwrap();
    assert_eq!((x.min(), x.max()), (16.0, 128.0));
    assert_eq!((y.min(), y.max()), (6.0, 98.0));
    let hit_x = target.hit_test_clip_css_px().x().unwrap();
    let hit_y = target.hit_test_clip_css_px().y().unwrap();
    assert_eq!((hit_x.min(), hit_x.max()), (16.0, 128.0));
    assert_eq!((hit_y.min(), hit_y.max()), (6.0, 98.0));
    assert_eq!(
        scene.hit_test_css_point(80.0, 60.0).unwrap().node_id(),
        target_id
    );
    assert_ne!(
        scene.hit_test_css_point(130.0, 60.0).unwrap().node_id(),
        target_id
    );
}

#[test]
fn content_box_clip_margin_limits_positioned_descendant_hits_to_chrome_visual_edge() {
    let observation: serde_json::Value =
        serde_json::from_str(C13_CONTENT_BOX_POSITIONED_HIT_OBSERVATION)
            .expect("pinned Chrome content-box positioned hit observation");
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
        Some(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36"
        )
    );
    assert_eq!(
        observation["browser"]["viewportCssPx"]["width"].as_u64(),
        Some(200)
    );
    assert_eq!(
        observation["browser"]["viewportCssPx"]["height"].as_u64(),
        Some(160)
    );
    assert_eq!(
        observation["browser"]["viewportCssPx"]["dpr"].as_f64(),
        Some(1.0)
    );
    assert_eq!(
        observation["fixture"]["sha256"].as_str(),
        Some("cf635425b6717a565da37f29649bc04723ab3358fa8f2bf8783bd4721697217f")
    );
    assert_eq!(
        observation["case"]["id"].as_str(),
        Some("content-box-positioned-hit")
    );
    assert_eq!(observation["case"]["overflowX"].as_str(), Some("clip"));
    assert_eq!(observation["case"]["overflowY"].as_str(), Some("clip"));
    assert_eq!(
        observation["case"]["overflowClipMargin"].as_str(),
        Some("content-box 8px")
    );
    assert_eq!(
        observation["case"]["target"],
        serde_json::json!({
            "x": 74, "y": 54, "width": 60, "height": 40,
            "right": 134, "bottom": 94
        })
    );
    assert_eq!(
        observation["case"]["hitRanges"]["x"]["ranges"],
        serde_json::json!([[74, 119]])
    );
    assert_eq!(
        observation["case"]["hitRanges"]["y"]["ranges"],
        serde_json::json!([[54, 89]])
    );
    assert_eq!(
        observation["case"]["fallbackHits"],
        serde_json::json!([
            {"x":119,"y":55,"elementId":"target"},
            {"x":120,"y":55,"elementId":"clip"},
            {"x":122,"y":55,"elementId":"clip"},
            {"x":123,"y":55,"elementId":"clip"},
            {"x":124,"y":55,"elementId":"root"}
        ])
    );

    let (input, nodes) = overflow_runtime_inputs(
        &[
            "display:block;position:relative;width:200px;height:160px;background-color:#111111",
            "position:relative;left:20px;top:10px;width:80px;height:60px;padding:8px;border:4px solid;overflow:clip;overflow-clip-margin:content-box 8px;background-color:#333333",
            "position:absolute;left:50px;top:40px;width:60px;height:40px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1)],
    );
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let target_id = nodes[2];
    let target = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == target_id)
        .unwrap();
    let paint_x = target.clip_css_px().x().unwrap();
    let paint_y = target.clip_css_px().y().unwrap();
    assert_eq!((paint_x.min(), paint_x.max()), (24.0, 120.0));
    assert_eq!((paint_y.min(), paint_y.max()), (14.0, 90.0));
    let hit_x = target.hit_test_clip_css_px().x().unwrap();
    let hit_y = target.hit_test_clip_css_px().y().unwrap();
    assert_eq!((hit_x.min(), hit_x.max()), (24.0, 120.0));
    assert_eq!((hit_y.min(), hit_y.max()), (14.0, 90.0));
    assert_eq!(
        scene.hit_test_css_point(119.0, 55.0).unwrap().node_id(),
        target_id
    );
    for x in [120.0, 122.0, 123.0] {
        assert_eq!(
            scene.hit_test_css_point(x, 55.0).unwrap().node_id(),
            nodes[1],
            "Chrome's clip parent remains the fallback target at x={x}"
        );
    }
    assert_eq!(
        scene.hit_test_css_point(124.0, 55.0).unwrap().node_id(),
        nodes[0],
        "the clip parent's border box ends at its right edge"
    );
}

#[test]
fn absolute_descendant_escapes_static_clip_when_its_containing_block_is_outside() {
    let (input, nodes) = overflow_runtime_inputs(
        &[
            "display:block;position:relative;width:200px;height:160px;background-color:#111111",
            "position:relative;width:140px;height:120px;background-color:#222222",
            "position:static;width:80px;height:60px;overflow:clip;background-color:#333333",
            "position:absolute;left:100px;top:50px;width:40px;height:30px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1), Some(2)],
    );
    assert_eq!(
        input.output.layout.positioned_owners[&nodes[3]],
        spinon_layout::PositionedContainingBlockOwner::Node(nodes[1])
    );
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let target = scene
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == nodes[3])
        .unwrap();
    assert_eq!(
        target.clip_css_px(),
        spinon_render::CssClip::unconstrained()
    );
    assert_eq!(
        target.hit_test_clip_css_px(),
        spinon_render::CssClip::unconstrained()
    );
    assert_eq!(
        scene.hit_test_css_point(110.0, 55.0).unwrap().node_id(),
        nodes[3]
    );
}

#[test]
fn runtime_hit_test_matches_agent_browser_chrome_155_element_from_point_samples() {
    let ids = [
        "c131-root",
        "c131-visible",
        "c131-visible-target",
        "c131-clip",
        "c131-clip-target",
        "c131-margin",
        "c131-margin-target",
        "c131-nested-outer",
        "c131-nested-inner",
        "c131-nested-target",
        "c131-escape-outer",
        "c131-escape-static-clip",
        "c131-escape-target",
    ];
    let fixture_variables = [
        "root",
        "visible",
        "visibleTarget",
        "clipped",
        "clipTarget",
        "margin",
        "marginTarget",
        "nested",
        "inner",
        "nestedTarget",
        "escape",
        "staticClip",
        "escapeTarget",
    ];
    let styles = [
        "display:flow-root;position:relative;box-sizing:border-box;width:300px;height:100px;background-color:#101827",
        "display:block;margin-left:10px;margin-top:6px;box-sizing:border-box;width:90px;height:44px;padding:6px;border:3px solid #f0c674;background-color:#344256",
        "width:130px;height:70px;background-color:#57a6d9",
        "display:block;margin-left:105px;margin-top:-44px;box-sizing:border-box;width:90px;height:44px;padding:6px;border:3px solid #f0c674;overflow:clip;background-color:#344256",
        "width:130px;height:70px;background-color:#57a6d9",
        "display:block;margin-left:200px;margin-top:-44px;box-sizing:border-box;width:90px;height:44px;padding:6px;border:3px solid #f0c674;overflow:clip;overflow-clip-margin:padding-box 8px;background-color:#344256",
        "width:130px;height:70px;background-color:#57a6d9",
        "display:block;margin-left:10px;margin-top:12px;box-sizing:border-box;width:120px;height:34px;padding:4px;border:2px solid #f0c674;overflow:clip;background-color:#344256",
        "display:block;margin-left:21px;box-sizing:border-box;width:50px;height:20px;padding:2px;border:1px solid #8a45ad;overflow-x:clip;overflow-y:visible;background-color:#f4ecf7",
        "width:90px;height:28px;background-color:#e3544e",
        "display:block;margin-left:150px;margin-top:-34px;box-sizing:border-box;width:140px;height:34px;padding:4px;border:2px solid #f0c674;overflow:clip;background-color:#344256",
        "position:static;box-sizing:border-box;width:45px;height:24px;padding:2px;border:1px solid #8a45ad;overflow-x:clip;overflow-y:visible;background-color:#f4ecf7",
        "position:absolute;left:222px;top:68px;width:50px;height:22px;background-color:#e3544e",
    ];
    let parents = [
        None,
        Some(0),
        Some(1),
        Some(0),
        Some(3),
        Some(0),
        Some(5),
        Some(0),
        Some(7),
        Some(8),
        Some(0),
        Some(10),
        Some(11),
    ];
    let fixture_parents = [
        "document.body || document",
        "root",
        "visible",
        "root",
        "clipped",
        "root",
        "margin",
        "root",
        "nested",
        "inner",
        "root",
        "escape",
        "staticClip",
    ];
    for (index, ((id, style), fixture_parent)) in
        ids.iter().zip(styles).zip(fixture_parents).enumerate()
    {
        let runtime_parent = parents[index]
            .map(|parent| fixture_variables[parent])
            .unwrap_or("document.body || document");
        assert_eq!(
            runtime_parent, fixture_parent,
            "runtime parent for {id} must match the shared fixture"
        );
        let fixture_call = format!("c131Node(\n  {fixture_parent},\n  '{id}',\n  '{style}',");
        assert!(
            C13_OVERFLOW_CLIP_FIXTURE_JS.contains(&fixture_call),
            "runtime CSS and parent for {id} must match the shared JS fixture"
        );
    }
    let (input, nodes) = overflow_runtime_inputs_with_viewport(&styles, &parents, 320.0, 120.0);
    let scene = build_runtime_render_snapshot_with_reserve(
        &input.document,
        input.root,
        &input.output,
        input.current,
        &mut SystemReserve,
    )
    .unwrap();
    let observation: serde_json::Value = serde_json::from_str(C13_CHROME_OVERFLOW_CLIP_OBSERVATION)
        .expect("pinned Chrome hit-test observation JSON");
    assert_eq!(
        observation["browser"]["version"].as_str(),
        Some("155.0.8059.40")
    );
    let frames = observation["frames"]
        .as_array()
        .expect("pinned Chrome frame inventory");
    let samples = observation["hitTestSamples"]
        .as_array()
        .expect("pinned Chrome hit-test samples");
    assert_eq!(samples.len(), 15);

    for sample in samples {
        let sample_id = sample["id"].as_str().expect("hit-test sample ID");
        let x = sample["xCssPx"].as_f64().expect("sample CSS x") as f32;
        let y = sample["yCssPx"].as_f64().expect("sample CSS y") as f32;
        let actual_hit = scene.hit_test_css_point(x, y);
        if sample["elementId"].is_null() {
            assert!(
                actual_hit.is_none(),
                "Chrome 155 elementFromPoint sample {sample_id} at ({x}, {y}) is null"
            );
            continue;
        }
        let expected_element_id = sample["elementId"]
            .as_str()
            .expect("Chrome elementFromPoint result must be string or null");
        let expected_node_index = frames
            .iter()
            .position(|frame| frame["id"].as_str() == Some(expected_element_id))
            .expect("Chrome hit target exists in pinned frame inventory");
        assert_eq!(ids[expected_node_index], expected_element_id);
        assert_eq!(
            actual_hit
                .expect("Chrome target must have a runtime hit")
                .node_id(),
            nodes[expected_node_index],
            "Chrome 155 elementFromPoint sample {sample_id} at ({x}, {y})"
        );
    }
}

#[test]
fn deeply_nested_context_flattening_uses_bounded_call_stack() {
    const DEPTH: usize = 50_000;
    let mut input = runtime_inputs();
    let mut contexts: Vec<StackingContext> = std::iter::repeat_with(StackingContext::default)
        .take(DEPTH)
        .collect();
    for (index, context) in contexts.iter_mut().enumerate() {
        let node = NodeId::new(100_000 + index as u64).expect("synthetic NodeId");
        input.output.layout.frames.insert(
            node,
            LayoutFrame {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
        );
        context.root = Some(node);
        if index + 1 < DEPTH {
            context.zero.push(OrderedPaintItem::Context(OrderedContext {
                context: index + 1,
                stack_level: 0,
                tree_order: index as u64,
            }));
        }
    }

    let mut reserve = SystemReserve;
    let paint_nodes = flatten_contexts(DEPTH, DEPTH, &mut contexts, &input.output, &mut reserve)
        .expect("깊은 stacking context는 반복 순회로 처리되어야 합니다");
    assert_eq!(paint_nodes.len(), DEPTH);
    assert!(
        paint_nodes
            .iter()
            .enumerate()
            .all(|(index, node)| node.get() == 100_000 + index as u64)
    );
}

#[test]
fn wide_context_fanout_fits_the_document_sized_action_reservation() {
    const CHILDREN: usize = 50_000;
    let mut input = runtime_inputs();
    let mut contexts: Vec<StackingContext> = std::iter::repeat_with(StackingContext::default)
        .take(CHILDREN + 1)
        .collect();
    let root = NodeId::new(200_000).expect("synthetic root NodeId");
    input.output.layout.frames.insert(
        root,
        LayoutFrame {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        },
    );
    contexts[0].root = Some(root);
    contexts[0].zero.reserve(CHILDREN);
    for index in 1..=CHILDREN {
        let node = NodeId::new(200_000 + index as u64).expect("synthetic child NodeId");
        input.output.layout.frames.insert(
            node,
            LayoutFrame {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
        );
        contexts[index].root = Some(node);
        contexts[0]
            .zero
            .push(OrderedPaintItem::Context(OrderedContext {
                context: index,
                stack_level: 0,
                tree_order: index as u64,
            }));
    }

    let mut reserve = SystemReserve;
    let paint_nodes = flatten_contexts(
        CHILDREN + 1,
        CHILDREN + 1,
        &mut contexts,
        &input.output,
        &mut reserve,
    )
    .expect("넓은 context 형제는 문서 크기 예약 안에서 처리되어야 합니다");
    assert_eq!(paint_nodes.len(), CHILDREN + 1);
    assert_eq!(paint_nodes[0], root);
    assert!(
        paint_nodes
            .iter()
            .enumerate()
            .all(|(index, node)| node.get() == 200_000 + index as u64)
    );
}
