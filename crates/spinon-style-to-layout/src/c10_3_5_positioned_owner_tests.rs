use super::*;

#[test]
fn flex_source_parent_and_absolute_containing_block_can_differ() {
    let (document, nodes) = fixture(&[
        (
            "positioned-root",
            None,
            "display:block;position:relative;box-sizing:border-box;width:220px;height:120px;margin:0;padding:0",
        ),
        (
            "static-flex",
            Some(0),
            "display:flex;position:static;box-sizing:border-box;width:140px;height:70px;flex-direction:row;justify-content:flex-end;align-items:center;margin:12px 0 0 18px;padding:0",
        ),
        (
            "absolute",
            Some(1),
            "display:block;position:absolute;box-sizing:border-box;width:24px;height:18px;margin:0;padding:0",
        ),
    ]);
    let output = layout(&document, nodes[0]);
    let flex = output.layout.frames[&nodes[1].id()];
    let absolute = output.layout.frames[&nodes[2].id()];

    assert_eq!(absolute.x, flex.x + 116.0);
    assert_eq!(absolute.y, flex.y + 26.0);
    assert_eq!(
        output.layout.positioned_owners[&nodes[2].id()],
        PositionedContainingBlockOwner::Node(nodes[0].id())
    );
}

#[test]
fn block_wrapper_keeps_block_static_position_and_mixed_insets_keep_c12_owner_math() {
    let (document, nodes) = fixture(&[
        (
            "root",
            None,
            "display:flex;position:relative;box-sizing:border-box;width:180px;height:100px;flex-direction:row;align-items:flex-start;padding:0",
        ),
        (
            "wrapper",
            Some(0),
            "display:block;position:static;box-sizing:border-box;width:100px;height:50px;margin:9px 0 0 12px;padding:0",
        ),
        (
            "before",
            Some(1),
            "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0",
        ),
        (
            "absolute",
            Some(1),
            "display:block;position:absolute;box-sizing:border-box;width:24px;height:16px;margin:0;padding:0",
        ),
        (
            "after",
            Some(1),
            "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0",
        ),
    ]);
    let output = layout(&document, nodes[0]);
    assert_eq!(output.layout.frames[&nodes[3].id()].x, 12.0);
    assert_eq!(output.layout.frames[&nodes[3].id()].y, 19.0);
    assert_eq!(output.layout.frames[&nodes[4].id()].y, 19.0);

    let (mixed_document, mixed_nodes) = fixture(&[
        (
            "root",
            None,
            "display:flex;position:relative;box-sizing:border-box;width:140px;height:90px;flex-direction:row;justify-content:center;align-items:flex-end;padding:5px",
        ),
        (
            "absolute",
            Some(0),
            "display:block;position:absolute;left:7px;bottom:9px;box-sizing:border-box;width:24px;height:18px;margin:0;padding:0",
        ),
    ]);
    let mixed = layout(&mixed_document, mixed_nodes[0]);
    assert_eq!(mixed.layout.frames[&mixed_nodes[1].id()].x, 7.0);
    assert_eq!(mixed.layout.frames[&mixed_nodes[1].id()].y, 63.0);
}
