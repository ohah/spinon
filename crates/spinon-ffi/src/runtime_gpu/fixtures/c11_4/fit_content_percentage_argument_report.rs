use spinon_layout::LayoutGridTrackSizingAxis;
use spinon_runtime::RuntimeLayoutCompleted;

pub(super) fn report_json(
    completed: &RuntimeLayoutCompleted,
) -> Result<Option<serde_json::Value>, String> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| "C11.4 layout diagnostic snapshot이 없습니다".to_owned())?;
    let Some(observation) = diagnostics.fit_content_percentage_argument.as_ref() else {
        return Ok(None);
    };
    let style = observation
        .style_snapshot
        .elements
        .iter()
        .find(|style| style.node_id == observation.grid_root)
        .ok_or_else(|| "C11.4 percentage computed root style이 없습니다".to_owned())?;
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
        return Err("C11.4 percentage fixture frame 집합이 완전하지 않습니다".to_owned());
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
        .ok_or_else(|| "C11.4 percentage row trace가 없습니다".to_owned())?;
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
        "layoutCssMathRegistry": observation
            .css_math
            .iter()
            .map(|value| serde_json::json!({
                "id": value.id.0,
                "node": value.node_id.get(),
                "property": format!("{:?}", value.property),
                "expression": format!("{:?}", value.expression),
            }))
            .collect::<Vec<_>>(),
        "nodeIds": {
            "root": observation.node_ids[0].get(),
            "item": observation.node_ids[1].get(),
            "child": observation.node_ids[2].get(),
        },
        "framesCssPx": {
            "root": frames.get(&observation.node_ids[0].get()),
            "item": frames.get(&observation.node_ids[1].get()),
            "child": frames.get(&observation.node_ids[2].get()),
        },
        "rowSizingStages": row_stages,
        "frameSource": "pre-render LayoutOutput CSS px",
    });
    Ok(Some(report))
}
