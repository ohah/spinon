use spinon_layout::LayoutGridTrackSizingAxis;
use spinon_runtime::RuntimeLayoutCompleted;

pub(super) fn report_json(
    completed: &RuntimeLayoutCompleted,
) -> Result<Option<serde_json::Value>, String> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| "C11.4 layout diagnostic snapshot이 없습니다".to_owned())?;
    let Some(observation) = diagnostics.fit_content_zoomed_grid.as_ref() else {
        return Ok(None);
    };
    let root_style = find_style(observation, observation.grid_root)
        .ok_or_else(|| "C11.4 zoomed grid computed root style이 없습니다".to_owned())?;
    let item_style = find_style(observation, observation.node_ids[1])
        .ok_or_else(|| "C11.4 zoomed grid computed item style이 없습니다".to_owned())?;
    let child_style = find_style(observation, observation.node_ids[2])
        .ok_or_else(|| "C11.4 zoomed grid computed child style이 없습니다".to_owned())?;
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
        return Err("C11.4 zoomed grid frame 집합이 완전하지 않습니다".to_owned());
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
        .ok_or_else(|| "C11.4 zoomed grid row track trace가 없습니다".to_owned())?;
    let stage_values = row_pass
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
    let contributions = row_pass
        .contributions
        .iter()
        .map(|contribution| {
            serde_json::json!({
                "item": contribution.item.get(),
                "kind": format!("{:?}", contribution.kind),
                "span": contribution.span,
                "crossesFlexibleTrack": contribution.crosses_flexible_track,
                "valueCssPx": contribution.value,
                "minimumSource": contribution.minimum_source.map(|source| format!("{:?}", source)),
            })
        })
        .collect::<Vec<_>>();
    let zoom = |style: &spinon_style::ComputedElementStyle| {
        style.c11_4_zoom.map(
            |zoom| serde_json::json!({ "specified": zoom.specified, "effective": zoom.effective }),
        )
    };
    Ok(Some(serde_json::json!({
        "sceneId": "fit-content-zoomed-grid-wpt",
        "styleSnapshot": {
            "profile": format!("{:?}", observation.style_snapshot.profile),
            "generation": observation.style_snapshot.generation.get(),
            "documentRevision": observation.style_snapshot.document_revision.get(),
            "renderTreeRevision": observation.style_snapshot.render_tree_revision.get(),
            "styleRevision": observation.style_snapshot.style_revision.get(),
            "environmentRevision": diagnostics.revision.environment().get(),
            "rootNode": observation.grid_root.get(),
            "zoom": {
                "root": zoom(root_style),
                "item": zoom(item_style),
                "child": zoom(child_style),
            },
            "computedPropertyGridTemplateRows": root_style.properties.get("grid-template-rows"),
            "typedDimensions": {
                "rootWidth": format!("{:?}", root_style.layout_dimensions.width),
                "rootHeight": format!("{:?}", root_style.layout_dimensions.height),
                "itemMinHeight": format!("{:?}", item_style.layout_dimensions.min_height),
                "childHeight": format!("{:?}", child_style.layout_dimensions.height),
            },
            "typedGridStyle": format!("{:?}", root_style.grid_layout),
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
        "expected": {
            "upstreamWptCssomGridTemplateRows": "40px",
            "taffyFitContentLimitCssPx": 80,
            "minimumContributionCssPx": 0,
            "maxContentContributionCssPx": 800,
            "usedRowCssPx": 80,
        },
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
        "rowSizingStages": stage_values,
        "rowContributions": contributions,
        "frameSource": "pre-render LayoutOutput CSS px",
    })))
}

fn find_style(
    observation: &spinon_runtime::RuntimeC11_4FitContentZoomedGridObservation,
    node: spinon_core::NodeId,
) -> Option<&spinon_style::ComputedElementStyle> {
    observation
        .style_snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node)
}
