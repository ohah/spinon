#[derive(Clone, Copy)]
enum Axis {
    Columns,
    Rows,
}

#[derive(Clone, Copy)]
enum Neighbors {
    Both,
    Prefix,
    Suffix,
    None,
}

#[derive(Clone, Copy)]
enum Pattern {
    MinMax,
    Fixed,
}

#[derive(Clone, Copy)]
enum Placement {
    Empty,
    Single,
    SpanTwo,
    SecondRepeat,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    axis: Axis,
    min_size: u16,
    neighbors: Neighbors,
    pattern: Pattern,
    placement: Placement,
}

macro_rules! scene {
    ($id:literal, $axis:ident, $min_size:literal, $neighbors:ident, $pattern:ident, $placement:ident) => {
        Scene {
            id: $id,
            axis: Axis::$axis,
            min_size: $min_size,
            neighbors: Neighbors::$neighbors,
            pattern: Pattern::$pattern,
            placement: Placement::$placement,
        }
    };
}

const SCENES: [Scene; 20] = [
    scene!(
        "auto-fit-min-columns-both-190",
        Columns,
        190,
        Both,
        MinMax,
        Empty
    ),
    scene!(
        "auto-fit-min-columns-both-200",
        Columns,
        200,
        Both,
        MinMax,
        Empty
    ),
    scene!(
        "auto-fit-min-columns-both-342",
        Columns,
        342,
        Both,
        MinMax,
        Empty
    ),
    scene!("auto-fit-min-rows-both-190", Rows, 190, Both, MinMax, Empty),
    scene!("auto-fit-min-rows-both-200", Rows, 200, Both, MinMax, Empty),
    scene!("auto-fit-min-rows-both-342", Rows, 342, Both, MinMax, Empty),
    scene!(
        "auto-fit-min-columns-prefix-200",
        Columns,
        200,
        Prefix,
        MinMax,
        Empty
    ),
    scene!(
        "auto-fit-min-columns-suffix-200",
        Columns,
        200,
        Suffix,
        MinMax,
        Empty
    ),
    scene!(
        "auto-fit-min-columns-none-200",
        Columns,
        200,
        None,
        MinMax,
        Empty
    ),
    scene!(
        "auto-fit-min-rows-prefix-200",
        Rows,
        200,
        Prefix,
        MinMax,
        Empty
    ),
    scene!(
        "auto-fit-min-rows-suffix-200",
        Rows,
        200,
        Suffix,
        MinMax,
        Empty
    ),
    scene!("auto-fit-min-rows-none-200", Rows, 200, None, MinMax, Empty),
    scene!(
        "auto-fit-min-columns-single-200",
        Columns,
        200,
        Both,
        MinMax,
        Single
    ),
    scene!(
        "auto-fit-min-rows-single-200",
        Rows,
        200,
        Both,
        MinMax,
        Single
    ),
    scene!(
        "auto-fit-min-columns-span-two-200",
        Columns,
        200,
        Both,
        MinMax,
        SpanTwo
    ),
    scene!(
        "auto-fit-min-rows-span-two-200",
        Rows,
        200,
        Both,
        MinMax,
        SpanTwo
    ),
    scene!(
        "auto-fit-min-columns-second-repeat-200",
        Columns,
        200,
        Both,
        MinMax,
        SecondRepeat
    ),
    scene!(
        "auto-fit-min-rows-second-repeat-200",
        Rows,
        200,
        Both,
        MinMax,
        SecondRepeat
    ),
    scene!(
        "auto-fit-min-columns-fixed-pattern-200",
        Columns,
        200,
        Both,
        Fixed,
        Empty
    ),
    scene!(
        "auto-fit-min-rows-fixed-pattern-200",
        Rows,
        200,
        Both,
        Fixed,
        Empty
    ),
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id).copied()?;
    Some(source(scene))
}

fn source(scene: Scene) -> String {
    let root_id = format!("c114-{}-grid", scene.id);
    let item_id = format!("c114-{}-item", scene.id);
    let repeat = match scene.pattern {
        Pattern::MinMax => "repeat(auto-fit,minmax(40px,1fr) 20px)",
        Pattern::Fixed => "repeat(auto-fit,40px 20px)",
    };
    let template = match scene.neighbors {
        Neighbors::Both => format!("20px {repeat} 10px"),
        Neighbors::Prefix => format!("20px {repeat}"),
        Neighbors::Suffix => format!("{repeat} 10px"),
        Neighbors::None => repeat.to_owned(),
    };
    let root_style = match scene.axis {
        Axis::Columns => format!(
            "box-sizing:border-box;display:grid;width:max-content;min-width:{}px;height:60px;margin:0;padding:0;border:0;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;grid-template-columns:{template};grid-template-rows:40px;background-color:#101820",
            scene.min_size
        ),
        Axis::Rows => format!(
            "box-sizing:border-box;display:grid;width:50px;height:max-content;min-height:{}px;margin:0;padding:0;border:0;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;grid-template-columns:40px;grid-template-rows:{template};background-color:#101820",
            scene.min_size
        ),
    };
    let create_item_node = |column: &str, row: &str| {
        let placement = match scene.axis {
            Axis::Columns => format!("grid-column:{column};grid-row:1;"),
            Axis::Rows => format!("grid-column:1;grid-row:{row};"),
        };
        format!(
            "const item=document.createElement('div');item.setAttribute('id','{item_id}');item.setAttribute('data-node','');item.setAttribute('style','box-sizing:border-box;width:12px;height:12px;margin:0;padding:0;border:0;{placement}background-color:#e5484d');root.appendChild(item);"
        )
    };
    let item_node = match scene.placement {
        Placement::Empty => String::new(),
        Placement::Single => create_item_node("2", "2"),
        Placement::SpanTwo => create_item_node("2 / span 2", "2 / span 2"),
        Placement::SecondRepeat => create_item_node("4", "4"),
    };
    let node_ids = if item_node.is_empty() {
        format!("'{root_id}'")
    } else {
        format!("'{root_id}','{item_id}'")
    };
    format!(
        "(() => {{ const root=document.createElement('div');root.setAttribute('id','{root_id}');root.setAttribute('data-node','');root.setAttribute('style','{root_style}');document.appendChild(root);{item_node}return {{sceneId:'{scene_id}',rootId:'{root_id}',nodeIds:[{node_ids}]}}; }})()",
        scene_id = scene.id,
    )
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn minimum_size_scene_matrix_has_unique_compilable_inputs() {
        let mut ids = BTreeSet::new();
        for scene in SCENES {
            assert!(ids.insert(scene.id), "duplicate scene ID: {}", scene.id);
            let source = runtime_source(scene.id).unwrap();
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("repeat(auto-fit,"));
            assert!(source.contains(&format!("sceneId:'{}'", scene.id)));
            let expected_items = if matches!(scene.placement, super::Placement::Empty) {
                0
            } else {
                1
            };
            assert_eq!(source.matches("const item=").count(), expected_items);
        }
        assert_eq!(ids.len(), 20);
    }
}
