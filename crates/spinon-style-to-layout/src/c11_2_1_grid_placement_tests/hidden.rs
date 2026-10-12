use super::*;

#[test]
fn hidden_grid_item_with_unsupported_named_line_is_ignored_end_to_end() {
    let visible = auto_sized_item("grid-column:1 / 2;grid-row:1 / 2");
    let fixture = Fixture::new(
        grid_style(),
        &[
            visible.as_str(),
            "display:none;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0;grid-column:custom-start / 2;grid-row:1 / 2",
        ],
    );

    let output = fixture
        .compute()
        .expect("display:none Grid item의 named line은 보이는 Grid 계산을 거부하면 안 됩니다");

    let hidden = &output.layout.frames[&fixture.items[1].id()];
    assert_eq!([hidden.x, hidden.y, hidden.width, hidden.height], [0.0; 4]);
    assert!(
        output
            .computed_styles
            .elements
            .iter()
            .find(|element| element.node_id == fixture.items[1].id())
            .unwrap()
            .grid_item_placement
            .is_none()
    );
}

#[test]
fn hidden_grid_item_ignores_unsupported_constraints_and_alignment() {
    let fixture = Fixture::new(
        grid_style(),
        &[
            "display:none;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0;grid-column:custom-start / 2;grid-row:1 / 2;min-width:120px;max-height:120px;align-self:start;justify-self:end",
        ],
    );

    let output = fixture
        .compute()
        .expect("숨긴 Grid 항목의 범위 밖 값은 보이는 Grid 계산을 막으면 안 됩니다");
    let frame = &output.layout.frames[&fixture.items[0].id()];
    assert_eq!([frame.x, frame.y, frame.width, frame.height], [0.0; 4]);
    assert!(
        output
            .computed_styles
            .elements
            .iter()
            .find(|element| element.node_id == fixture.items[0].id())
            .unwrap()
            .grid_item_placement
            .is_none()
    );
}

#[test]
fn grid_item_min_max_constraints_remain_outside_the_explicit_placement_profile() {
    for (property, value) in [
        ("min-width", "120px"),
        ("max-width", "120px"),
        ("min-height", "120px"),
        ("max-height", "120px"),
    ] {
        let item = auto_sized_item(&format!(
            "grid-column:1 / 2;grid-row:1 / 2;{property}:{value}"
        ));
        let fixture = Fixture::new(grid_style(), &[&item]);
        assert!(matches!(
            fixture.compute(),
            Err(StyleLayoutError::Layout(
                spinon_layout::LayoutError::UnsupportedGridItemConstraint {
                    property: actual,
                    ..
                }
            )) if actual == property
        ));
    }
}

#[test]
fn grid_item_alignment_remains_outside_the_explicit_placement_profile() {
    for (alignment, expected_property) in [
        ("align-self:start", "align-self"),
        ("justify-self:start", "justify-self"),
        ("place-self:center", "align-self"),
    ] {
        let item = auto_sized_item(&format!("grid-column:1 / 2;grid-row:1 / 2;{alignment}"));
        let fixture = Fixture::new(grid_style(), &[&item]);
        assert!(matches!(
            fixture.compute(),
            Err(StyleLayoutError::UnsupportedComputedValue {
                property,
                ..
            }) if property == expected_property
        ));
    }

    let flex_item =
        auto_sized_item("grid-column:1 / 2;grid-row:1 / 2;display:flex;align-items:center");
    Fixture::new(grid_style(), &[&flex_item])
        .compute()
        .expect("Grid item 내부 Flex 정렬은 Grid item 정렬과 다른 계약입니다");
}

#[test]
fn grid_container_alignment_and_place_shorthands_remain_outside_the_profile() {
    for (alignment, expected_property) in [
        ("align-items:start", "align-items"),
        ("align-content:center", "align-content"),
        ("justify-content:center", "justify-content"),
        ("justify-items:start", "justify-items"),
        ("place-content:center", "align-content"),
        ("place-items:center", "align-items"),
    ] {
        let grid_style = format!("{};{alignment}", grid_style());
        let fixture = Fixture::new(
            &grid_style,
            &[&auto_sized_item("grid-column:1 / 2;grid-row:1 / 2")],
        );
        assert!(matches!(
            fixture.compute(),
            Err(StyleLayoutError::UnsupportedComputedValue {
                property,
                ..
            }) if property == expected_property
        ));
    }
}

#[test]
fn hidden_nested_grid_ignores_unsupported_tracks_and_item_placement_end_to_end() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let hidden = document.reserve_node_handle().unwrap();
    let grid = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_222).unwrap(), document.document_revision());
    for (node, parent, name, style) in [
        (
            root,
            HostParent::Root,
            "root",
            "display:block;box-sizing:border-box;width:360px;height:800px;margin:0;padding:0",
        ),
        (
            hidden,
            HostParent::Node(root),
            "hidden",
            "display:none;box-sizing:border-box;width:200px;height:100px;margin:0;padding:0",
        ),
        (
            grid,
            HostParent::Node(hidden),
            "grid",
            "display:grid;box-sizing:border-box;width:200px;height:100px;grid-template-columns:repeat(auto-fit,80px);grid-template-rows:40px;align-items:center;justify-content:center;place-items:center;place-content:center;margin:0;padding:0",
        ),
        (
            item,
            HostParent::Node(grid),
            "item",
            "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0;grid-column:custom-start / 2;grid-row:1 / 2",
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: name.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new(snapshot.clone(), root, true, QuirksMode::NoQuirks).unwrap();

    let output = crate::compute_runtime_block_flex_grid_placement_style_layout(
        &snapshot,
        &view,
        root,
        &[],
        viewport(),
        StyleRevision::INITIAL,
    )
    .expect("숨겨진 하위 Grid의 track·placement는 보이는 layout을 거부하면 안 됩니다");

    for node in [hidden, grid, item] {
        let frame = &output.layout.frames[&node.id()];
        assert_eq!([frame.x, frame.y, frame.width, frame.height], [0.0; 4]);
    }
    let hidden_grid = output
        .computed_styles
        .elements
        .iter()
        .find(|element| element.node_id == grid.id())
        .unwrap();
    assert!(hidden_grid.grid_layout.is_none());
    assert_eq!(hidden_grid.properties["display"], "grid");
    let hidden_item = output
        .computed_styles
        .elements
        .iter()
        .find(|element| element.node_id == item.id())
        .unwrap();
    assert!(hidden_item.grid_item_placement.is_none());
}
