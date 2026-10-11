use std::{collections::BTreeSet, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostDocumentSnapshot,
    HostNodeHandle, HostParent, NodeId, OwnerId, StyleRevision,
};
use spinon_layout::LayoutFrame;
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
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

const HTML: &str = "http://www.w3.org/1999/xhtml";

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
