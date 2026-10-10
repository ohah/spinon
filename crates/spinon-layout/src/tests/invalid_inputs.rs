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

#[test]
fn negative_and_non_finite_min_max_sizes_are_rejected_before_taffy() {
    let valid = to_input(&fixture());
    let node = valid.nodes[1].id;

    for (field, set_value) in [
        ("min_width", LayoutDimension::Fixed(-1.0)),
        ("max_width", LayoutDimension::Percent(f32::NAN)),
        ("min_height", LayoutDimension::Fixed(f32::INFINITY)),
        ("max_height", LayoutDimension::Percent(-0.01)),
    ] {
        let mut invalid = valid.clone();
        match field {
            "min_width" => invalid.nodes[1].style.min_width = set_value,
            "max_width" => invalid.nodes[1].style.max_width = set_value,
            "min_height" => invalid.nodes[1].style.min_height = set_value,
            "max_height" => invalid.nodes[1].style.max_height = set_value,
            _ => unreachable!("known field"),
        }
        assert_eq!(
            TaffyLayoutEngine.compute(&invalid),
            Err(LayoutError::InvalidStyle { node, field })
        );
    }
}
