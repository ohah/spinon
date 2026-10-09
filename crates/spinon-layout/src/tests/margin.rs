use super::{fixture, node_id, to_input};
use crate::{LayoutEngine, LayoutError, TaffyLayoutEngine};

#[test]
fn negative_margin_reaches_taffy_and_non_finite_margin_is_rejected() {
    let fixture = fixture();
    let mut input = to_input(&fixture);
    let baseline = TaffyLayoutEngine.compute(&input).unwrap();
    let first_child = node_id(fixture.nodes[1].id);
    input
        .nodes
        .iter_mut()
        .find(|node| node.id == first_child)
        .unwrap()
        .style
        .margin
        .left = -7.0;
    let with_negative_margin = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(
        with_negative_margin.frames[&first_child].x,
        baseline.frames[&first_child].x - 7.0
    );

    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        input
            .nodes
            .iter_mut()
            .find(|node| node.id == first_child)
            .unwrap()
            .style
            .margin
            .top = invalid;
        assert!(matches!(
            TaffyLayoutEngine.compute(&input),
            Err(LayoutError::InvalidStyle {
                node: invalid_node,
                field: "margin.top",
            }) if invalid_node == first_child
        ));
    }
}
