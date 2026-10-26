use spinon_runtime::RuntimeLayoutCompleted;

pub(super) fn report_json(
    completed: &RuntimeLayoutCompleted,
) -> Result<Option<serde_json::Value>, String> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| "C11.4 layout diagnostic snapshot이 없습니다".to_owned())?;
    let Some(observation) = diagnostics.fit_content_auto_neighbor.as_ref() else {
        return Ok(None);
    };
    let mut root_styles = observation
        .style_snapshot
        .elements
        .iter()
        .filter(|style| style.node_id == observation.grid_root);
    let style = root_styles
        .next()
        .ok_or_else(|| "C11.4 computed root style이 없습니다".to_owned())?;
    if root_styles.next().is_some() {
        return Err("C11.4 computed root style이 중복되었습니다".to_owned());
    }

    let frame_by_node = completed
        .frames
        .iter()
        .filter(|frame| {
            observation
                .node_ids
                .iter()
                .any(|node| node.get() == frame.node_id)
        })
        .map(|frame| {
            (
                frame.node_id,
                serde_json::json!({
                    "x": frame.x,
                    "y": frame.y,
                    "width": frame.width,
                    "height": frame.height,
                }),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    if frame_by_node.len() != observation.node_ids.len() {
        return Err("C11.4 fixture node frame 집합이 완전하지 않습니다".to_owned());
    }
    let [root, fit_item, fit_child, auto_item, auto_child] = observation.node_ids;
    let style_property = |property: &str| style.properties.get(property).cloned();
    let report = serde_json::json!({
        "sceneId": "fit-content-auto-neighbor-wpt",
        "styleSnapshot": {
            "profile": format!("{:?}", observation.style_snapshot.profile),
            "generation": observation.style_snapshot.generation.get(),
            "documentRevision": observation.style_snapshot.document_revision.get(),
            "renderTreeRevision": observation.style_snapshot.render_tree_revision.get(),
            "styleRevision": observation.style_snapshot.style_revision.get(),
            "environmentRevision": diagnostics.revision.environment().get(),
            "rootNode": observation.grid_root.get(),
            "computedProperties": {
                "display": style_property("display"),
                "width": format!("{:?}", style.layout_dimensions.width),
                "height": format!("{:?}", style.layout_dimensions.height),
                "gridTemplateColumns": style_property("grid-template-columns"),
                "gridTemplateRows": style_property("grid-template-rows"),
                "gridAutoRows": style_property("grid-auto-rows"),
                "gridAutoFlow": style_property("grid-auto-flow"),
                "alignContent": style_property("align-content"),
                "rowGap": format!("{:?}", style.layout_spacing.row_gap),
            },
            "typedGridStyle": format!("{:?}", style.grid_layout),
        },
        "layoutInputGridContainer": {
            "rootNode": observation.grid_root.get(),
            "columns": format!("{:?}", observation.grid_container_input.columns),
            "explicitRows": format!("{:?}", observation.grid_container_input.rows),
            "autoRows": format!("{:?}", observation.grid_container_input.auto_rows),
            "autoFlow": format!("{:?}", observation.grid_container_input.auto_flow),
            "placementMode": format!("{:?}", observation.grid_container_input.placement_mode),
        },
        "nodeIds": {
            "root": root.get(),
            "fitItem": fit_item.get(),
            "fitChild": fit_child.get(),
            "autoItem": auto_item.get(),
            "autoChild": auto_child.get(),
        },
        "framesCssPx": {
            "root": frame_by_node.get(&root.get()),
            "fitItem": frame_by_node.get(&fit_item.get()),
            "fitChild": frame_by_node.get(&fit_child.get()),
            "autoItem": frame_by_node.get(&auto_item.get()),
            "autoChild": frame_by_node.get(&auto_child.get()),
        },
        "frameSource": "pre-render LayoutOutput CSS px",
    });
    Ok(Some(report))
}
