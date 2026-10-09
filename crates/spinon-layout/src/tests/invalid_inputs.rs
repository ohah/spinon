use super::*;

#[test]
fn invalid_viewport_root_and_graph_are_rejected_before_layout() {
    let valid = to_input(&fixture());

    let mut invalid = valid.clone();
    invalid.viewport.width = f32::NAN;
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::InvalidViewport)
    );

    let mut invalid = valid.clone();
    invalid.root = node_id(99);
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::MissingRoot(node_id(99)))
    );

    let mut invalid = valid.clone();
    invalid.nodes[0].children.push(node_id(99));
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::MissingChild {
            parent: node_id(1),
            child: node_id(99)
        })
    );

    let mut invalid = valid.clone();
    invalid.nodes[0].children.push(node_id(2));
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::DuplicateChild {
            parent: node_id(1),
            child: node_id(2)
        })
    );

    let mut invalid = valid.clone();
    invalid.nodes[1].children.push(node_id(1));
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::RootHasParent(node_id(1)))
    );

    let mut invalid = valid.clone();
    invalid.nodes[0].children.push(node_id(3));
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::MultipleParents(node_id(3)))
    );

    let mut invalid = valid.clone();
    invalid.nodes[4].children.clear();
    assert_eq!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::DetachedNode(node_id(6)))
    );

    let mut invalid = valid;
    let mut first = fixed_node(node_id(8), 10.0, 10.0);
    let mut second = fixed_node(node_id(9), 10.0, 10.0);
    first.children.push(node_id(9));
    second.children.push(node_id(8));
    invalid.nodes.push(first);
    invalid.nodes.push(second);
    assert!(matches!(
        TaffyLayoutEngine.compute(&invalid),
        Err(LayoutError::Cycle(_))
    ));
}
