use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridItemPlacement, ComputedCssGridLayout,
    ComputedCssGridPlacement, ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedElementStyle,
    ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};

const SCENE_IDS: [&str; 5] = [
    "anonymous-min-content-wrap",
    "anonymous-comment-continuation",
    "anonymous-runs-around-block",
    "anonymous-hidden-child-continuation",
    "whitespace-does-not-create-item",
];

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    let (tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = root_id
        .strip_prefix("c114-")
        .and_then(|value| value.strip_suffix("-grid"))
    else {
        return Ok(None);
    };
    if !SCENE_IDS.contains(&scene_id) {
        return if scene_id.starts_with("anonymous-")
            || scene_id == "whitespace-does-not-create-item"
        {
            Err(unsupported(root.id(), "unknown anonymous text scene"))
        } else {
            Ok(None)
        };
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 || tag != "div" {
        return Err(unsupported(root.id(), "runtime profile or root tag"));
    }

    let grid_style = style_for(styles, root.id())?;
    validate_grid_style(grid_style, scene_id)?;
    validate_direct_children(snapshot, root, scene_id, styles)?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", super::c11_4_text::AHEM)
        .map_err(|error| {
            layout_failure(
                "c114_anonymous_text_font_registration",
                Some(root.id().get()),
                Some(error.to_string()),
            )
        })?;
    Ok(Some(super::C11_4TextInput {
        paragraph_styles: BTreeMap::from([(root.id(), anonymous_paragraph_style())]),
        engine,
        two_axis_nested_grid: false,
    }))
}

fn validate_direct_children(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    scene_id: &str,
    styles: &ComputedStyleSnapshot,
) -> Result<(), RuntimeLayoutFailure> {
    let children = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?
        .collect::<Vec<_>>();
    let expected: &[ExpectedChild<'_>] = match scene_id {
        "anonymous-min-content-wrap" => &[ExpectedChild::Text("XX XX")],
        "anonymous-comment-continuation" => &[ExpectedChild::Text("XX"), ExpectedChild::Text("XX")],
        "anonymous-runs-around-block" => &[
            ExpectedChild::Text("XX"),
            ExpectedChild::Block("c114-anonymous-runs-around-block-item"),
            ExpectedChild::Text("XXXX"),
        ],
        "anonymous-hidden-child-continuation" => &[
            ExpectedChild::Text("XX"),
            ExpectedChild::Hidden {
                id: "c114-anonymous-hidden-child-continuation-hidden",
                text: "ignored",
            },
            ExpectedChild::Text("XX"),
        ],
        "whitespace-does-not-create-item" => &[
            ExpectedChild::Block("c114-whitespace-does-not-create-item-first"),
            ExpectedChild::Text(" \t\n"),
            ExpectedChild::Block("c114-whitespace-does-not-create-item-second"),
        ],
        _ => return Err(unsupported(root.id(), "unrecognized anonymous text scene")),
    };
    if children.len() != expected.len() {
        return Err(unsupported(root.id(), "direct child count"));
    }
    for (&handle, expected_child) in children.iter().zip(expected) {
        if snapshot.parent(handle) != Some(HostParent::Node(root)) {
            return Err(unsupported(handle.id(), "direct child ownership"));
        }
        match expected_child {
            ExpectedChild::Text(expected_text) => {
                let actual_text = snapshot
                    .node(handle)
                    .and_then(|node| match node.kind() {
                        HostNodeKind::Text(value) => Some(value.to_string_lossy()),
                        HostNodeKind::Element(_) => None,
                    })
                    .ok_or_else(|| unsupported(handle.id(), "text node expected"))?;
                if actual_text != *expected_text {
                    return Err(unsupported(handle.id(), "direct text content"));
                }
            }
            ExpectedChild::Block(expected_id) => {
                let (tag, actual_id) = element_identity(snapshot, handle)?;
                if tag != "div" || actual_id != *expected_id {
                    return Err(unsupported(handle.id(), "block item identity"));
                }
                let style = style_for(styles, handle.id())?;
                let (width, height) = if scene_id == "anonymous-runs-around-block" {
                    (30.0, 10.0)
                } else {
                    (10.0, 10.0)
                };
                if style.properties.get("display").map(String::as_str) != Some("block")
                    || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
                    || style.properties.get("position").map(String::as_str) != Some("static")
                    || style.properties.get("order").map(String::as_str) != Some("0")
                    || style.layout_position != spinon_style::ComputedCssPosition::Static
                    || style.layout_dimensions.width != ComputedCssDimension::LengthPx(width)
                    || style.layout_dimensions.height != ComputedCssDimension::LengthPx(height)
                    || style.layout_dimensions.min_width != ComputedCssDimension::LengthPx(0.0)
                    || style.layout_dimensions.min_height != ComputedCssDimension::LengthPx(0.0)
                    || style.layout_dimensions.max_width != ComputedCssMaxSize::None
                    || style.layout_dimensions.max_height != ComputedCssMaxSize::None
                    || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
                    || style.layout_aspect_ratio.is_some()
                    || !grid_item_placement_is_auto(style.grid_item_placement.as_ref())
                    || !zero_edges(style)
                {
                    return Err(unsupported(handle.id(), "block item geometry"));
                }
                if snapshot
                    .children(handle)
                    .is_some_and(|mut descendants| descendants.next().is_some())
                {
                    return Err(unsupported(handle.id(), "block item must be empty"));
                }
            }
            ExpectedChild::Hidden { id, text } => {
                let (tag, actual_id) = element_identity(snapshot, handle)?;
                if tag != "span" || actual_id != *id {
                    return Err(unsupported(handle.id(), "hidden separator identity"));
                }
                let style = style_for(styles, handle.id())?;
                if style.properties.get("display").map(String::as_str) != Some("none")
                    || style.layout_position != spinon_style::ComputedCssPosition::Static
                    || style.properties.get("order").map(String::as_str) != Some("0")
                {
                    return Err(unsupported(handle.id(), "hidden separator style"));
                }
                let descendants = snapshot
                    .children(handle)
                    .ok_or_else(|| unsupported(handle.id(), "hidden separator children"))?
                    .collect::<Vec<_>>();
                if descendants.len() != 1
                    || !snapshot.node(descendants[0]).is_some_and(|node| {
                        matches!(node.kind(), HostNodeKind::Text(value) if value.to_string_lossy() == *text)
                    })
                {
                    return Err(unsupported(handle.id(), "hidden separator text content"));
                }
            }
        }
    }
    Ok(())
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    validate_text_profile_properties(style.node_id, &style.properties)?;
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.properties.get("align-items").map(String::as_str) != Some("start")
        || style.properties.get("align-content").map(String::as_str) != Some("start")
        || style.properties.get("justify-content").map(String::as_str) != Some("start")
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || style.properties.get("font-family").map(String::as_str) != Some("Ahem")
        || style.properties.get("font-size").map(String::as_str) != Some("10px")
        || style.properties.get("line-height").map(String::as_str) != Some("10px")
        || style
            .properties
            .get("white-space-collapse")
            .map(String::as_str)
            != Some("collapse")
        || style.properties.get("text-wrap-mode").map(String::as_str) != Some("wrap")
        || style.properties.get("order").map(String::as_str) != Some("0")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(120.0)
        || style.layout_dimensions.height
            != ComputedCssDimension::LengthPx(if scene_id == "anonymous-min-content-wrap" {
                20.0
            } else {
                10.0
            })
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style)
        || !zero_spacing(style.layout_spacing.row_gap)
        || !zero_spacing(style.layout_spacing.column_gap)
    {
        return Err(unsupported(
            style.node_id,
            "Grid style or inherited text profile",
        ));
    }
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| unsupported(style.node_id, "missing Grid style"))?;
    if !grid_tracks_match(grid, scene_id) {
        return Err(unsupported(
            style.node_id,
            "Grid track functions or auto-placement defaults",
        ));
    }
    Ok(())
}

fn grid_item_placement_is_auto(placement: Option<&ComputedCssGridItemPlacement>) -> bool {
    placement.is_some_and(|placement| {
        placement.row_start == ComputedCssGridPlacement::Auto
            && placement.row_end == ComputedCssGridPlacement::Auto
            && placement.column_start == ComputedCssGridPlacement::Auto
            && placement.column_end == ComputedCssGridPlacement::Auto
    })
}

fn grid_tracks_match(grid: &ComputedCssGridLayout, scene_id: &str) -> bool {
    let expected_columns = match scene_id {
        "anonymous-min-content-wrap" => vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::Fraction(1.0),
        ],
        "anonymous-comment-continuation" => vec![
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Fraction(1.0),
        ],
        "anonymous-hidden-child-continuation" => vec![
            ComputedCssGridTrackSize::MaxContent,
            ComputedCssGridTrackSize::Fraction(1.0),
        ],
        "anonymous-runs-around-block" => vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MinContent,
        ],
        "whitespace-does-not-create-item" => vec![
            ComputedCssGridTrackSize::LengthPx(20.0),
            ComputedCssGridTrackSize::LengthPx(30.0),
            ComputedCssGridTrackSize::LengthPx(40.0),
        ],
        _ => return false,
    };
    let expected_rows = if scene_id == "anonymous-min-content-wrap" {
        vec![ComputedCssGridTrackSize::Auto]
    } else {
        vec![ComputedCssGridTrackSize::LengthPx(10.0)]
    };
    grid.columns == expected_columns
        && grid.rows == expected_rows
        && grid.columns_auto_repeat.is_none()
        && grid.rows_auto_repeat.is_none()
        && grid.template_areas.is_none()
        && grid.auto_flow == ComputedCssGridAutoFlow::Row
        && grid.auto_rows == ComputedCssGridAutoRows::Auto
        && grid.auto_columns == ComputedCssGridAutoColumns::Auto
        && grid.columns_line_names.len() == grid.columns.len() + 1
        && grid.rows_line_names.len() == grid.rows.len() + 1
        && grid.columns_line_names.iter().all(Vec::is_empty)
        && grid.rows_line_names.iter().all(Vec::is_empty)
}

fn validate_text_profile_properties(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, expected) in [
        ("font-family", "Ahem"),
        ("font-size", "10px"),
        ("font-weight", "400"),
        ("font-style", "normal"),
        ("line-height", "10px"),
        ("white-space-collapse", "collapse"),
        ("text-wrap-mode", "wrap"),
        ("word-break", "normal"),
        ("direction", "ltr"),
        ("line-break", "auto"),
        ("text-align", "start"),
        ("overflow-wrap", "normal"),
        ("text-transform", "none"),
        ("writing-mode", "horizontal-tb"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ] {
        let value = properties.get(name).map(String::as_str);
        if value != Some(expected) {
            return Err(layout_failure(
                "unsupported_c114_anonymous_text_style",
                Some(node_id.get()),
                Some(format!("{name}: expected {expected}, got {value:?}")),
            ));
        }
    }
    Ok(())
}

fn style_for(
    styles: &ComputedStyleSnapshot,
    node_id: NodeId,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == node_id)
        .ok_or_else(|| layout_failure("missing_computed_style", Some(node_id.get()), None))
}

fn element_identity(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
) -> Result<(String, String), RuntimeLayoutFailure> {
    let node = snapshot
        .node(handle)
        .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(unsupported(handle.id(), "element expected"));
    };
    let id = element
        .attributes()
        .iter()
        .find_map(|(name, value)| {
            (name.namespace().is_none() && name.local_name() == "id")
                .then(|| value.to_string_lossy())
        })
        .unwrap_or_default();
    Ok((element.local_name().to_owned(), id))
}

fn anonymous_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
    }
}

fn zero_edges(style: &ComputedElementStyle) -> bool {
    zero_spacing(style.layout_spacing.margin.top)
        && zero_spacing(style.layout_spacing.margin.right)
        && zero_spacing(style.layout_spacing.margin.bottom)
        && zero_spacing(style.layout_spacing.margin.left)
        && zero_spacing(style.layout_spacing.padding.top)
        && zero_spacing(style.layout_spacing.padding.right)
        && zero_spacing(style.layout_spacing.padding.bottom)
        && zero_spacing(style.layout_spacing.padding.left)
        && style.layout_border == Default::default()
}

fn zero_spacing(value: spinon_style::ComputedCssSpacingValue) -> bool {
    matches!(value, spinon_style::ComputedCssSpacingValue::LengthPx(value) if value.abs() <= f32::EPSILON)
}

fn unsupported(node: NodeId, reason: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_anonymous_text_fixture",
        Some(node.get()),
        Some(reason.to_owned()),
    )
}

enum ExpectedChild<'a> {
    Text(&'a str),
    Block(&'a str),
    Hidden { id: &'a str, text: &'a str },
}

#[cfg(test)]
mod tests {
    use super::{
        SCENE_IDS, grid_item_placement_is_auto, grid_tracks_match, validate_text_profile_properties,
    };
    use spinon_core::NodeId;
    use spinon_style::{
        ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
        ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridItemPlacement,
        ComputedCssGridLayout, ComputedCssGridPlacement, ComputedCssGridTemplateAreas,
        ComputedCssGridTrackSize,
    };
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn anonymous_text_fixture_cases_are_unique() {
        let scene_ids = SCENE_IDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(SCENE_IDS.len(), 5);
        assert_eq!(scene_ids.len(), SCENE_IDS.len());
    }

    #[test]
    fn anonymous_text_block_separator_requires_auto_grid_placement() {
        let mut placement = ComputedCssGridItemPlacement {
            row_start: ComputedCssGridPlacement::Auto,
            row_end: ComputedCssGridPlacement::Auto,
            column_start: ComputedCssGridPlacement::Auto,
            column_end: ComputedCssGridPlacement::Auto,
        };
        assert!(grid_item_placement_is_auto(Some(&placement)));

        placement.column_start = ComputedCssGridPlacement::Line(1);
        assert!(!grid_item_placement_is_auto(Some(&placement)));
        assert!(!grid_item_placement_is_auto(None));
    }

    fn grid(
        columns: Vec<ComputedCssGridTrackSize>,
        rows: Vec<ComputedCssGridTrackSize>,
    ) -> ComputedCssGridLayout {
        ComputedCssGridLayout {
            columns_line_names: vec![Vec::new(); columns.len() + 1],
            rows_line_names: vec![Vec::new(); rows.len() + 1],
            columns,
            rows,
            columns_auto_repeat: None,
            rows_auto_repeat: None,
            template_areas: None,
            auto_flow: ComputedCssGridAutoFlow::Row,
            auto_rows: ComputedCssGridAutoRows::Auto,
            auto_columns: ComputedCssGridAutoColumns::Auto,
        }
    }

    #[test]
    fn anonymous_grid_fixture_rejects_typography_and_track_profile_mutations() {
        let properties = BTreeMap::from([
            ("font-family".to_owned(), "Ahem".to_owned()),
            ("font-size".to_owned(), "10px".to_owned()),
            ("font-weight".to_owned(), "400".to_owned()),
            ("font-style".to_owned(), "normal".to_owned()),
            ("line-height".to_owned(), "10px".to_owned()),
            ("white-space-collapse".to_owned(), "collapse".to_owned()),
            ("text-wrap-mode".to_owned(), "wrap".to_owned()),
            ("word-break".to_owned(), "normal".to_owned()),
            ("direction".to_owned(), "ltr".to_owned()),
            ("line-break".to_owned(), "auto".to_owned()),
            ("text-align".to_owned(), "start".to_owned()),
            ("overflow-wrap".to_owned(), "normal".to_owned()),
            ("text-transform".to_owned(), "none".to_owned()),
            ("writing-mode".to_owned(), "horizontal-tb".to_owned()),
            ("letter-spacing".to_owned(), "normal".to_owned()),
            ("word-spacing".to_owned(), "0px".to_owned()),
            ("text-indent".to_owned(), "0px".to_owned()),
        ]);
        assert!(validate_text_profile_properties(NodeId::new(1).unwrap(), &properties).is_ok());
        for (name, changed) in [
            ("font-family", "sans-serif"),
            ("font-size", "11px"),
            ("font-weight", "700"),
            ("font-style", "italic"),
            ("line-height", "normal"),
            ("white-space-collapse", "preserve"),
            ("text-wrap-mode", "nowrap"),
            ("word-break", "break-all"),
            ("direction", "rtl"),
            ("line-break", "strict"),
            ("text-align", "center"),
            ("overflow-wrap", "anywhere"),
            ("text-transform", "uppercase"),
            ("writing-mode", "vertical-rl"),
            ("letter-spacing", "1px"),
            ("word-spacing", "1px"),
            ("text-indent", "1px"),
        ] {
            let mut mutated = properties.clone();
            mutated.insert(name.to_owned(), changed.to_owned());
            assert!(
                validate_text_profile_properties(NodeId::new(1).unwrap(), &mutated).is_err(),
                "the fixture must reject a changed {name}"
            );
        }

        let valid = grid(
            vec![
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::Fraction(1.0),
            ],
            vec![ComputedCssGridTrackSize::Auto],
        );
        assert!(grid_tracks_match(&valid, "anonymous-min-content-wrap"));

        macro_rules! rejects_grid_mutation {
            ($name:literal, $mutation:expr) => {{
                let mut mutated = valid.clone();
                $mutation(&mut mutated);
                assert!(
                    !grid_tracks_match(&mutated, "anonymous-min-content-wrap"),
                    "the fixture must reject {name}",
                    name = $name
                );
            }};
        }

        rejects_grid_mutation!("different columns", |grid: &mut ComputedCssGridLayout| {
            grid.columns[0] = ComputedCssGridTrackSize::LengthPx(20.0);
        });
        rejects_grid_mutation!("different rows", |grid: &mut ComputedCssGridLayout| {
            grid.rows = vec![ComputedCssGridTrackSize::LengthPx(10.0)];
        });
        rejects_grid_mutation!("column auto-repeat", |grid: &mut ComputedCssGridLayout| {
            grid.columns_auto_repeat = Some(ComputedCssGridAutoRepeat {
                mode: ComputedCssGridAutoRepeatMode::AutoFill,
                start: 0,
                end: 1,
                line_names: Vec::new(),
                fixed_repeat_minmax_track_indices: Vec::new(),
            });
        });
        rejects_grid_mutation!("row auto-repeat", |grid: &mut ComputedCssGridLayout| {
            grid.rows_auto_repeat = Some(ComputedCssGridAutoRepeat {
                mode: ComputedCssGridAutoRepeatMode::AutoFit,
                start: 0,
                end: 1,
                line_names: Vec::new(),
                fixed_repeat_minmax_track_indices: Vec::new(),
            });
        });
        rejects_grid_mutation!("template areas", |grid: &mut ComputedCssGridLayout| {
            grid.template_areas = Some(ComputedCssGridTemplateAreas {
                rows: 1,
                columns: 1,
                areas: Vec::new(),
            });
        });
        rejects_grid_mutation!("column auto-flow", |grid: &mut ComputedCssGridLayout| {
            grid.auto_flow = ComputedCssGridAutoFlow::Column;
        });
        rejects_grid_mutation!("dense auto-flow", |grid: &mut ComputedCssGridLayout| {
            grid.auto_flow = ComputedCssGridAutoFlow::RowDense;
        });
        rejects_grid_mutation!("auto rows", |grid: &mut ComputedCssGridLayout| {
            grid.auto_rows = ComputedCssGridAutoRows::LengthPx(10.0);
        });
        rejects_grid_mutation!("auto columns", |grid: &mut ComputedCssGridLayout| {
            grid.auto_columns = ComputedCssGridAutoColumns::LengthPx(10.0);
        });
        rejects_grid_mutation!("column line names", |grid: &mut ComputedCssGridLayout| {
            grid.columns_line_names[0].push("named".to_owned());
        });
        rejects_grid_mutation!("row line names", |grid: &mut ComputedCssGridLayout| {
            grid.rows_line_names[0].push("named".to_owned());
        });
    }
}
