use spinon_layout::LayoutGridTrackSizingAxis;
use spinon_runtime::RuntimeLayoutCompleted;

fn stages_json(pass: &spinon_layout::LayoutGridTrackSizingPassTrace) -> serde_json::Value {
    let stages = pass
        .stages
        .iter()
        .map(|stage| {
            let tracks = stage
                .tracks
                .iter()
                .map(|track| {
                    serde_json::json!({
                        "index": track.internal_index,
                        "gutter": track.is_gutter,
                        "collapsed": track.is_collapsed,
                        "baseSizeCssPx": track.base_size,
                        "growthLimitCssPx": track.growth_limit,
                        "infinitelyGrowable": track.infinitely_growable,
                    })
                })
                .collect::<Vec<_>>();
            serde_json::json!({ "stage": format!("{:?}", stage.stage), "tracks": tracks })
        })
        .collect::<Vec<_>>();
    serde_json::Value::Array(stages)
}

fn contributions_json(pass: &spinon_layout::LayoutGridTrackSizingPassTrace) -> serde_json::Value {
    serde_json::Value::Array(
        pass.contributions
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
            .collect(),
    )
}

pub(super) fn report_json(
    completed: &RuntimeLayoutCompleted,
) -> Result<Option<serde_json::Value>, String> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| "C11.4 layout diagnostic snapshot이 없습니다".to_owned())?;
    let Some(observation) = diagnostics.fit_content_limit_columns.as_ref() else {
        return Ok(None);
    };
    let root_style = observation
        .style_snapshot
        .elements
        .iter()
        .find(|style| style.node_id == observation.grid_root)
        .ok_or_else(|| "C11.4 fit-content column limit root style이 없습니다".to_owned())?;
    let item_style = observation
        .style_snapshot
        .elements
        .iter()
        .find(|style| style.node_id == observation.node_ids[1])
        .ok_or_else(|| "C11.4 fit-content column limit item style이 없습니다".to_owned())?;
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
        return Err("C11.4 fit-content column limit frame 집합이 완전하지 않습니다".to_owned());
    }
    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == observation.grid_root)
        .ok_or_else(|| "C11.4 fit-content column limit root trace가 없습니다".to_owned())?;
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .ok_or_else(|| "C11.4 fit-content column limit column trace가 없습니다".to_owned())?;
    let rows = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .ok_or_else(|| "C11.4 fit-content column limit row trace가 없습니다".to_owned())?;
    let minimum_css_px = if observation.automatic_minimum {
        100.0
    } else {
        0.0
    };
    Ok(Some(serde_json::json!({
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
                "display": root_style.properties.get("display"),
                "width": format!("{:?}", root_style.layout_dimensions.width),
                "height": format!("{:?}", root_style.layout_dimensions.height),
                "gridTemplateColumns": root_style.properties.get("grid-template-columns"),
                "gridTemplateRows": root_style.properties.get("grid-template-rows"),
                "gridAutoColumns": root_style.properties.get("grid-auto-columns"),
                "gridAutoRows": root_style.properties.get("grid-auto-rows"),
                "gridAutoFlow": root_style.properties.get("grid-auto-flow"),
                "alignContent": root_style.properties.get("align-content"),
                "itemMinWidth": format!("{:?}", item_style.layout_dimensions.min_width),
                "itemOverflowX": item_style.layout_overflow_x.map(|value| format!("{:?}", value).to_lowercase()),
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
        "expectedLimitCssPx": observation.limit_css_px,
        "expectedMinimumCssPx": minimum_css_px,
        "expectedMaxContentCssPx": 100,
        "expectedColumnCssPx": observation.expected_column_css_px,
        "automaticMinimumControl": observation.automatic_minimum,
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
        "columnSizingStages": stages_json(columns),
        "columnContributions": contributions_json(columns),
        "rowSizingStages": stages_json(rows),
        "frameSource": "pre-render LayoutOutput CSS px",
    })))
}
