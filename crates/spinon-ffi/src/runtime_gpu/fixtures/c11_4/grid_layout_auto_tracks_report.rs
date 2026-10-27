use spinon_runtime::RuntimeLayoutCompleted;

pub(super) fn report_json(
    completed: &RuntimeLayoutCompleted,
) -> Result<Option<serde_json::Value>, String> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| "C11.4 layout diagnostic snapshot이 없습니다".to_owned())?;
    let Some(observation) = diagnostics.grid_layout_auto_tracks.as_ref() else {
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

    let frames = completed
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
    if frames.len() != observation.node_ids.len() {
        return Err("C11.4 fixture node frame 집합이 완전하지 않습니다".to_owned());
    }
    let [root, a, b, b_content, c] = observation.node_ids;
    let property = |name: &str| style.properties.get(name).cloned();
    Ok(Some(serde_json::json!({
        "sceneId": "grid-layout-auto-tracks-c114-wpt",
        "styleSnapshot": {
            "profile": format!("{:?}", observation.style_snapshot.profile),
            "generation": observation.style_snapshot.generation.get(),
            "documentRevision": observation.style_snapshot.document_revision.get(),
            "renderTreeRevision": observation.style_snapshot.render_tree_revision.get(),
            "styleRevision": observation.style_snapshot.style_revision.get(),
            "environmentRevision": diagnostics.revision.environment().get(),
            "rootNode": observation.grid_root.get(),
            "computedProperties": {
                "display": property("display"),
                "width": format!("{:?}", style.layout_dimensions.width),
                "height": format!("{:?}", style.layout_dimensions.height),
                "gridTemplateColumns": property("grid-template-columns"),
                "gridTemplateRows": property("grid-template-rows"),
                "gridAutoColumns": property("grid-auto-columns"),
                "gridAutoRows": property("grid-auto-rows"),
                "gridAutoFlow": property("grid-auto-flow"),
                "alignItems": property("align-items"),
                "direction": property("direction"),
                "writingMode": property("writing-mode"),
                "rowGap": format!("{:?}", style.layout_spacing.row_gap),
                "columnGap": format!("{:?}", style.layout_spacing.column_gap),
            },
            "typedGridStyle": format!("{:?}", style.grid_layout),
        },
        "layoutInputGridContainer": {
            "rootNode": observation.grid_root.get(),
            "columns": format!("{:?}", observation.grid_container_input.columns),
            "explicitRows": format!("{:?}", observation.grid_container_input.rows),
            "autoColumns": format!("{:?}", observation.grid_container_input.auto_columns),
            "autoRows": format!("{:?}", observation.grid_container_input.auto_rows),
            "autoFlow": format!("{:?}", observation.grid_container_input.auto_flow),
            "placementMode": format!("{:?}", observation.grid_container_input.placement_mode),
        },
        "nodeIds": {
            "root": root.get(), "a": a.get(), "b": b.get(),
            "bContent": b_content.get(), "c": c.get(),
        },
        "framesCssPx": {
            "root": frames.get(&root.get()), "a": frames.get(&a.get()),
            "b": frames.get(&b.get()), "bContent": frames.get(&b_content.get()),
            "c": frames.get(&c.get()),
        },
        "frameSource": "pre-render LayoutOutput CSS px",
    })))
}
