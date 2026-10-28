use spinon_layout::LayoutGridTrackSizingAxis;
use spinon_runtime::RuntimeLayoutCompleted;

pub(super) fn report_json(
    completed: &RuntimeLayoutCompleted,
) -> Result<Option<serde_json::Value>, String> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| "C11.4 layout diagnostic snapshot이 없습니다".to_owned())?;
    let Some(observation) = diagnostics.fit_content_auto_min_floor.as_ref() else {
        return Ok(None);
    };
    let style = observation
        .style_snapshot
        .elements
        .iter()
        .find(|style| style.node_id == observation.grid_root)
        .ok_or_else(|| "C11.4 automatic-minimum computed root style이 없습니다".to_owned())?;
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
        return Err("C11.4 automatic-minimum fixture frame 집합이 완전하지 않습니다".to_owned());
    }
    let row_pass = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == observation.grid_root)
        .and_then(|trace| {
            trace
                .passes
                .iter()
                .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        })
        .ok_or_else(|| "C11.4 automatic-minimum row trace가 없습니다".to_owned())?;
    let row_stages = row_pass
        .stages
        .iter()
        .map(|stage| {
            let base_sizes = stage
                .tracks
                .iter()
                .filter(|track| !track.is_gutter && !track.is_collapsed)
                .map(|track| track.base_size)
                .collect::<Vec<_>>();
            serde_json::json!({
                "stage": format!("{:?}", stage.stage),
                "baseSizesCssPx": base_sizes,
            })
        })
        .collect::<Vec<_>>();
    let expected_rows = if observation.scene_id == "fit-content-auto-min-floor-wpt" {
        [10.0, 100.0]
    } else {
        [10.0, 40.0]
    };
    let item_min_heights = [observation.node_ids[1], observation.node_ids[3]]
        .into_iter()
        .map(|node| {
            observation
                .style_snapshot
                .elements
                .iter()
                .find(|item_style| item_style.node_id == node)
                .map(|item_style| format!("{:?}", item_style.layout_dimensions.min_height))
                .ok_or_else(|| "C11.4 automatic-minimum computed item style이 없습니다".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report = serde_json::json!({
        "sceneId": observation.scene_id,
        "styleSnapshot": {
            "profile": format!("{:?}", observation.style_snapshot.profile),
            "generation": observation.style_snapshot.generation.get(),
            "documentRevision": observation.style_snapshot.document_revision.get(),
            "renderTreeRevision": observation.style_snapshot.render_tree_revision.get(),
            "styleRevision": observation.style_snapshot.style_revision.get(),
            "environmentRevision": diagnostics.revision.environment().get(),
            "rootNode": observation.grid_root.get(),
            "computedProperties": {
                "display": style.properties.get("display"),
                "width": format!("{:?}", style.layout_dimensions.width),
                "height": format!("{:?}", style.layout_dimensions.height),
            "minHeight": format!("{:?}", style.layout_dimensions.min_height),
                "gridTemplateColumns": style.properties.get("grid-template-columns"),
                "gridTemplateRows": style.properties.get("grid-template-rows"),
                "gridAutoRows": style.properties.get("grid-auto-rows"),
                "alignContent": style.properties.get("align-content"),
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
        "expectedRowBaseSizesCssPx": expected_rows,
        "itemMinHeights": item_min_heights,
        "nodeIds": {
            "root": observation.node_ids[0].get(),
            "firstItem": observation.node_ids[1].get(),
            "firstChild": observation.node_ids[2].get(),
            "secondItem": observation.node_ids[3].get(),
            "secondChild": observation.node_ids[4].get(),
        },
        "framesCssPx": {
            "root": frames.get(&observation.node_ids[0].get()),
            "firstItem": frames.get(&observation.node_ids[1].get()),
            "firstChild": frames.get(&observation.node_ids[2].get()),
            "secondItem": frames.get(&observation.node_ids[3].get()),
            "secondChild": frames.get(&observation.node_ids[4].get()),
        },
        "rowSizingStages": row_stages,
        "frameSource": "pre-render LayoutOutput CSS px",
    });
    Ok(Some(report))
}
