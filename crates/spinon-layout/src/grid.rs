use std::collections::BTreeMap;

use spinon_core::NodeId;

use crate::grid_placement::normalize_grid_layout;
use crate::{
    GridPlacementInput, LayoutDisplay, LayoutError, LayoutInput, LayoutPosition,
    percentage_basis::DefiniteAxes,
};

/// Taffy Grid 코드가 축마다 내부적으로 허용하는 최대 track 수입니다.
/// Taffy가 넘친 template를 잘라 처리하므로 그 전에 명시적으로 거부합니다.
pub const GRID_MAX_TRACKS: usize = 10_000;
const MAX_GRID_LINE_METADATA_BYTES: usize = 1024 * 1024;
const MAX_EXPANDED_GRID_LINE_METADATA_BYTES: usize = 16 * 1024 * 1024;

/// Grid track의 length-percentage 값입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridTrackLengthPercentage {
    LengthPx(f32),
    Percentage(f32),
    Calc(crate::LayoutCalcId),
}

/// `minmax()`의 최소 함수입니다. Flexible sizing function은 CSS 문법상 포함하지 않습니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridTrackMinSizingFunction {
    LengthPx(f32),
    Percentage(f32),
    Calc(crate::LayoutCalcId),
    Auto,
    MinContent,
    MaxContent,
}

/// `minmax()`의 최대 함수입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridTrackMaxSizingFunction {
    LengthPx(f32),
    Percentage(f32),
    Calc(crate::LayoutCalcId),
    Fraction(f32),
    Auto,
    MinContent,
    MaxContent,
}

/// 제한 Grid profile에서 사용하는 명시 track 크기입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridTrackSize {
    LengthPx(f32),
    Percentage(f32),
    Fraction(f32),
    Calc(crate::LayoutCalcId),
    Auto,
    MinContent,
    MaxContent,
    MinMax {
        min: GridTrackMinSizingFunction,
        max: GridTrackMaxSizingFunction,
    },
    FitContent(GridTrackLengthPercentage),
}

/// CSS Grid auto-repeat의 `auto-fill`/`auto-fit` 방식입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GridAutoRepeatMode {
    AutoFill,
    AutoFit,
}

/// Explicit track 목록에서 auto-repeat fragment의 위치와 repetition 방식을 보존합니다.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridAutoRepeat {
    pub mode: GridAutoRepeatMode,
    /// `columns` 또는 `rows`에서 반복 fragment가 시작하는 inclusive track index입니다.
    pub start: usize,
    /// `columns` 또는 `rows`에서 반복 fragment가 끝나는 exclusive track index입니다.
    pub end: usize,
    /// 반복 fragment의 track 경계 이름 그룹입니다. 비어 있지 않으면 반복 track 수보다 하나 큽니다.
    pub line_names: Vec<Vec<String>>,
}

/// Auto-repeat과 함께 Taffy로 전달하는 `grid-template-areas` 입력입니다.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GridTemplateAreasInput {
    /// 모든 셀을 포함하는 authored template 행 수입니다.
    pub rows: u16,
    /// 모든 셀을 포함하는 authored template 열 수입니다.
    pub columns: u16,
    /// 이름이 있는 area의 0-based half-open rectangle 목록입니다.
    pub areas: Vec<GridTemplateAreaInput>,
}

/// Taffy Grid named resolver에 전달하는 하나의 area rectangle입니다.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridTemplateAreaInput {
    pub name: String,
    pub row_start: u16,
    pub row_end: u16,
    pub column_start: u16,
    pub column_end: u16,
}

/// C11.4 Taffy 후보 trace에 기록되는 축입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutGridTrackSizingAxis {
    Columns,
    Rows,
}

/// Grid item 하나에 대해 질의한 intrinsic 크기의 종류입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutGridTrackSizingContributionKind {
    Minimum,
    MinContent,
    MaxContent,
}

/// Grid item의 minimum contribution을 계산할 때 선택한 규칙입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutGridTrackSizingMinimumContributionSource {
    PreferredSize {
        clamped_by_min_or_max: bool,
        fixed_track_sum: Option<f64>,
        fixed_gutter_sum: Option<f64>,
        fixed_track_limit: Option<f64>,
    },
    SizingKeywordUsesMinContent,
    SpecifiedMinimum,
    OverflowAutomaticMinimum,
    AutomaticMinimum {
        spans_auto_min_track: bool,
        spans_multiple_tracks: bool,
        spans_flexible_max_track: bool,
        content_based: bool,
        is_compressible_replaced: bool,
        fixed_track_sum: Option<f64>,
        fixed_gutter_sum: Option<f64>,
        fixed_track_limit: Option<f64>,
    },
}

/// 한 축의 sizing pass에서 관찰한 item intrinsic-size contribution입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGridTrackSizingContributionTrace {
    pub item: NodeId,
    pub kind: LayoutGridTrackSizingContributionKind,
    pub span: u16,
    pub crosses_flexible_track: bool,
    pub value: f32,
    pub minimum_source: Option<LayoutGridTrackSizingMinimumContributionSource>,
}

/// 후보 track-sizing algorithm 실행 중 기록한 단계입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutGridTrackSizingStageKind {
    Initialize,
    IntrinsicItemBatch {
        span: u16,
        crosses_flexible_track: bool,
    },
    IntrinsicSizingComplete,
    MaximizeTracks,
    ExpandFlexibleTracks,
    StretchAutoTracks {
        applied: bool,
    },
    FixedTrackEarlyReturn,
}

/// 기록된 단계에서 internal track 또는 gutter 하나의 base size와 growth limit입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGridTrackSizingTrackState {
    pub internal_index: u16,
    pub is_gutter: bool,
    pub is_collapsed: bool,
    pub base_size: f32,
    /// `None`은 후보 algorithm이 growth limit을 무한대로 보고했다는 뜻입니다.
    pub growth_limit: Option<f32>,
    pub infinitely_growable: bool,
}

/// Grid 한 축에서 한 단계의 snapshot입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGridTrackSizingStageTrace {
    pub stage: LayoutGridTrackSizingStageKind,
    pub tracks: Vec<LayoutGridTrackSizingTrackState>,
}

/// 한 축에서 track-sizing algorithm을 실행한 기록입니다. sequence로 재실행을 구분합니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGridTrackSizingPassTrace {
    pub sequence: u16,
    pub axis: LayoutGridTrackSizingAxis,
    pub stages: Vec<LayoutGridTrackSizingStageTrace>,
    pub contributions: Vec<LayoutGridTrackSizingContributionTrace>,
}

/// opt-in `RuntimeBlockFlexGridTrackSizingV1` 후보가 기록한 진단 단계입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutGridTrackSizingTrace {
    pub node: NodeId,
    pub passes: Vec<LayoutGridTrackSizingPassTrace>,
}

/// CSS Grid auto-placement 방향입니다. C11.1은 기본 row만 계산합니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GridAutoFlow {
    #[default]
    Row,
    Column,
    RowDense,
    ColumnDense,
}

/// Grid container가 사용하는 item placement 계약입니다.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GridPlacementMode {
    /// C11.1의 all-auto 또는 기존 explicit-only 분류를 사용합니다.
    #[default]
    Existing,
    /// C11.2.1의 모든 visible item이 definite placement여야 합니다.
    ExplicitOnly,
    /// C11.2.2의 row sparse auto-placement와 양의 implicit row를 계산합니다.
    SparseRow,
    /// C11.2.3의 row/column sparse·dense 자동 배치와 양의 implicit track을 계산합니다.
    DenseAutoPlacement,
    /// C11.3 template profile은 dense 배치를 유지하고 definite item의 auto 크기를 허용합니다.
    TemplateGrid,
    /// C11.3.3 opt-in 경로의 explicit placement와 trailing fixed implicit tracks입니다.
    TemplateGridImplicitTracks,
    /// C11.4 intrinsic track sizing 후보 profile입니다.
    TrackSizing,
}

/// 제한 profile에서 허용하는 implicit track sizing 입력입니다.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum GridAutoTrackSize {
    /// 기본 `auto` track입니다. intrinsic sizing은 TrackSizing profile에서 지원합니다.
    #[default]
    Auto,
    /// 모든 implicit track에 반복할 하나의 고정 CSS px track입니다.
    LengthPx(f32),
    /// C11.4 TrackSizing profile에서 leading tracks는 마지막 값부터, trailing tracks는 첫 값부터 순환합니다.
    TrackList(Vec<GridTrackSize>),
}

/// 제한 profile에서 허용하는 implicit row sizing 입력입니다.
pub type GridAutoRows = GridAutoTrackSize;

/// 제한 profile에서 허용하는 implicit column sizing 입력입니다.
pub type GridAutoColumns = GridAutoTrackSize;

/// 동일 revision의 계산된 Grid container 입력입니다.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GridContainerInput {
    /// Auto-repeat이 있으면 반복 fragment를 authored 순서로 한 번 포함하는 explicit track 목록입니다.
    pub columns: Vec<GridTrackSize>,
    /// Auto-repeat이 있으면 반복 fragment를 authored 순서로 한 번 포함하는 explicit track 목록입니다.
    pub rows: Vec<GridTrackSize>,
    /// C11.4 제한 profile에서 반복할 columns fragment와 방식입니다.
    pub columns_auto_repeat: Option<GridAutoRepeat>,
    /// C11.4 제한 profile에서 반복할 rows fragment와 방식입니다.
    pub rows_auto_repeat: Option<GridAutoRepeat>,
    /// 일반 축에서는 track 경계 이름이고, auto-repeat 축에서는 template component 경계 이름입니다.
    pub columns_line_names: Vec<Vec<String>>,
    /// 일반 축에서는 track 경계 이름이고, auto-repeat 축에서는 template component 경계 이름입니다.
    pub rows_line_names: Vec<Vec<String>>,
    /// auto-repeat과 `grid-template-areas`를 함께 쓸 때만 전달합니다.
    pub template_areas: Option<GridTemplateAreasInput>,
    pub auto_flow: GridAutoFlow,
    pub auto_rows: GridAutoRows,
    pub auto_columns: GridAutoColumns,
    pub placement_mode: GridPlacementMode,
}

pub(super) fn validate_grid_layout(
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
    definite_axes: &BTreeMap<NodeId, DefiniteAxes>,
    intrinsic_size_nodes: &std::collections::BTreeSet<NodeId>,
) -> Result<(), LayoutError> {
    for &id in input.grid_containers.keys() {
        if !index.contains_key(&id) {
            return Err(LayoutError::UnknownGridContainer(id));
        }
        if input.nodes[index[&id]].style.display != LayoutDisplay::Grid {
            return Err(LayoutError::UnexpectedGridContainer(id));
        }
    }
    for &id in input.grid_items.keys() {
        if !index.contains_key(&id) {
            return Err(LayoutError::UnknownGridItem(id));
        }
    }

    for (&id, container) in &input.grid_containers {
        validate_template_areas(id, container)?;
        match (container.placement_mode, &container.auto_rows) {
            (GridPlacementMode::TrackSizing, _) => {}
            (GridPlacementMode::TemplateGridImplicitTracks, GridAutoRows::Auto) => {
                return Err(LayoutError::UnsupportedGridAutoRows {
                    node: id,
                    reason: "C11.3.3 implicit named-line mode requires fixed px grid-auto-rows",
                });
            }
            (
                GridPlacementMode::SparseRow
                | GridPlacementMode::DenseAutoPlacement
                | GridPlacementMode::TemplateGrid
                | GridPlacementMode::TemplateGridImplicitTracks,
                GridAutoRows::Auto | GridAutoRows::LengthPx(_),
            )
            | (_, GridAutoRows::Auto) => {}
            (_, GridAutoRows::LengthPx(_)) | (_, GridAutoRows::TrackList(_)) => {
                return Err(LayoutError::UnsupportedGridAutoRows {
                    node: id,
                    reason: "grid-auto-rows는 자동 배치 profile에서만 사용할 수 있습니다",
                });
            }
        }
        match (container.placement_mode, &container.auto_columns) {
            (GridPlacementMode::TrackSizing, _) => {}
            (GridPlacementMode::TemplateGridImplicitTracks, GridAutoColumns::Auto) => {
                return Err(LayoutError::UnsupportedGridAutoColumns {
                    node: id,
                    reason: "C11.3.3 implicit named-line mode requires fixed px grid-auto-columns",
                });
            }
            (
                GridPlacementMode::DenseAutoPlacement
                | GridPlacementMode::TemplateGrid
                | GridPlacementMode::TemplateGridImplicitTracks,
                GridAutoColumns::Auto | GridAutoColumns::LengthPx(_),
            )
            | (_, GridAutoColumns::Auto) => {}
            (_, GridAutoColumns::LengthPx(_)) | (_, GridAutoColumns::TrackList(_)) => {
                return Err(LayoutError::UnsupportedGridAutoColumns {
                    node: id,
                    reason: "grid-auto-columns는 C11.2.3 profile에서만 사용할 수 있습니다",
                });
            }
        }
        if let GridAutoRows::LengthPx(value) = &container.auto_rows
            && (!value.is_finite() || *value < 0.0)
        {
            return Err(LayoutError::InvalidGridTrack {
                node: id,
                property: "grid-auto-rows",
            });
        }
        if let GridAutoColumns::LengthPx(value) = &container.auto_columns
            && (!value.is_finite() || *value < 0.0)
        {
            return Err(LayoutError::InvalidGridTrack {
                node: id,
                property: "grid-auto-columns",
            });
        }
        if let GridAutoRows::TrackList(tracks) = &container.auto_rows {
            if tracks.is_empty() {
                return Err(LayoutError::InvalidGridTrackList {
                    node: id,
                    property: "grid-auto-rows",
                });
            }
            validate_tracks(
                id,
                "grid-auto-rows",
                tracks,
                container.placement_mode == GridPlacementMode::TrackSizing,
            )?;
        }
        if let GridAutoColumns::TrackList(tracks) = &container.auto_columns {
            if tracks.is_empty() {
                return Err(LayoutError::InvalidGridTrackList {
                    node: id,
                    property: "grid-auto-columns",
                });
            }
            validate_tracks(
                id,
                "grid-auto-columns",
                tracks,
                container.placement_mode == GridPlacementMode::TrackSizing,
            )?;
        }
        validate_grid_line_metadata(id, container, input, index)?;
    }

    let normalized_grid = normalize_grid_layout(input, index)?;

    let mut parent_by_child = BTreeMap::new();
    for node in &input.nodes {
        for &child in &node.children {
            parent_by_child.insert(child, node.id);
        }
    }
    for &id in input.grid_items.keys() {
        let parent = parent_by_child.get(&id).copied();
        if !parent
            .is_some_and(|parent| input.nodes[index[&parent]].style.display == LayoutDisplay::Grid)
        {
            return Err(LayoutError::UnexpectedGridItem(id));
        }
    }

    let mut hidden = BTreeMap::new();
    let mut pending = vec![(input.root, false)];
    while let Some((id, parent_hidden)) = pending.pop() {
        let node = &input.nodes[index[&id]];
        let subtree_hidden = parent_hidden || node.style.display == LayoutDisplay::None;
        hidden.insert(id, subtree_hidden);
        pending.extend(
            node.children
                .iter()
                .rev()
                .copied()
                .map(|child| (child, subtree_hidden)),
        );
    }

    for node in &input.nodes {
        if node.style.display != LayoutDisplay::Grid || hidden[&node.id] {
            continue;
        }
        let Some(container) = input.grid_containers.get(&node.id) else {
            return Err(LayoutError::MissingGridContainer(node.id));
        };
        let allow_intrinsic_tracks = container.placement_mode == GridPlacementMode::TrackSizing;
        validate_tracks(
            node.id,
            "grid-template-columns",
            &container.columns,
            allow_intrinsic_tracks,
        )?;
        validate_tracks(
            node.id,
            "grid-template-rows",
            &container.rows,
            allow_intrinsic_tracks,
        )?;
        if container.columns_auto_repeat.is_some() || container.rows_auto_repeat.is_some() {
            if container.placement_mode != GridPlacementMode::TrackSizing {
                return Err(LayoutError::InvalidGridTrackList {
                    node: node.id,
                    property: if container.columns_auto_repeat.is_some() {
                        "grid-template-columns"
                    } else {
                        "grid-template-rows"
                    },
                });
            }
            validate_auto_repeat_axis(
                node.id,
                "grid-template-columns",
                &container.columns,
                &container.columns_line_names,
                container.columns_auto_repeat.as_ref(),
            )?;
            validate_auto_repeat_axis(
                node.id,
                "grid-template-rows",
                &container.rows,
                &container.rows_line_names,
                container.rows_auto_repeat.as_ref(),
            )?;
        }
        if container.placement_mode == GridPlacementMode::SparseRow
            && container.auto_flow != GridAutoFlow::Row
        {
            return Err(LayoutError::UnsupportedGridAutoFlow(node.id));
        }
        if !matches!(
            container.placement_mode,
            GridPlacementMode::SparseRow
                | GridPlacementMode::DenseAutoPlacement
                | GridPlacementMode::TemplateGrid
                | GridPlacementMode::TrackSizing
        ) && container.auto_flow != GridAutoFlow::Row
        {
            return Err(LayoutError::UnsupportedGridAutoFlow(node.id));
        }

        let definite = definite_axes.get(&node.id).copied().unwrap_or_default();
        let can_size_indefinite_track_sizing_height = !definite.height
            && container.placement_mode == GridPlacementMode::TrackSizing
            && indefinite_track_sizing_height_is_bounded(node, container);
        if !definite.width || (!definite.height && !can_size_indefinite_track_sizing_height) {
            return Err(LayoutError::IndefiniteGridBasis {
                node: node.id,
                axis: if !definite.width { "width" } else { "height" },
            });
        }

        let cell_count = container
            .columns
            .len()
            .checked_mul(container.rows.len())
            .ok_or(LayoutError::GridTrackExpansionOverflow(node.id))?;
        let c14_3_fixed_track_context = matches!(
            container.placement_mode,
            GridPlacementMode::DenseAutoPlacement
                | GridPlacementMode::TemplateGrid
                | GridPlacementMode::TrackSizing
        ) && !container.columns.is_empty()
            && !container.rows.is_empty()
            && container
                .columns
                .iter()
                .all(|track| matches!(track, GridTrackSize::LengthPx(_)))
            && container
                .rows
                .iter()
                .all(|track| matches!(track, GridTrackSize::LengthPx(_)));
        let has_explicit_placement = node
            .children
            .iter()
            .any(|child| normalized_grid.item_placements.contains_key(child));
        let mut item_count = 0usize;
        for &child_id in &node.children {
            let child = &input.nodes[index[&child_id]];
            if hidden[&child_id] {
                continue;
            }
            if child.style.order != 0 {
                return Err(LayoutError::UnsupportedGridOrder(child_id));
            }
            if input
                .positioning
                .get(&child_id)
                .is_some_and(|value| value.position != LayoutPosition::Static)
            {
                return Err(LayoutError::UnsupportedGridPositionedItem(child_id));
            }
            let c14_3_replaced_item = c14_3_fixed_track_context
                && intrinsic_size_nodes.contains(&child_id)
                && [child.style.width, child.style.height]
                    .into_iter()
                    .any(|size| {
                        matches!(
                            size,
                            crate::LayoutDimension::Percent(_)
                                | crate::LayoutDimension::CyclicMinContentPercent(_)
                                | crate::LayoutDimension::MinContent
                                | crate::LayoutDimension::MaxContent
                                | crate::LayoutDimension::FitContent
                        )
                    });
            let valid_c14_3_dimension = |size| {
                matches!(
                    size,
                    crate::LayoutDimension::Auto
                        | crate::LayoutDimension::Fixed(_)
                        | crate::LayoutDimension::Percent(_)
                        | crate::LayoutDimension::CyclicMinContentPercent(_)
                        | crate::LayoutDimension::MinContent
                        | crate::LayoutDimension::MaxContent
                        | crate::LayoutDimension::FitContent
                )
            };
            let valid_track_sizing_dimension = |size| {
                matches!(
                    size,
                    crate::LayoutDimension::Auto
                        | crate::LayoutDimension::Fixed(_)
                        | crate::LayoutDimension::Percent(_)
                        | crate::LayoutDimension::CyclicMinContentPercent(_)
                        | crate::LayoutDimension::MinContent
                        | crate::LayoutDimension::MaxContent
                        | crate::LayoutDimension::FitContent
                )
            };
            let valid_item_size = |size| match container.placement_mode {
                GridPlacementMode::SparseRow
                | GridPlacementMode::DenseAutoPlacement
                | GridPlacementMode::TemplateGrid
                | GridPlacementMode::TemplateGridImplicitTracks
                    if normalized_grid.auto_placed_items.contains(&child_id) =>
                {
                    matches!(size, crate::LayoutDimension::Fixed(_))
                        || (c14_3_replaced_item && valid_c14_3_dimension(size))
                }
                GridPlacementMode::TrackSizing
                    if normalized_grid.auto_placed_items.contains(&child_id) =>
                {
                    valid_track_sizing_dimension(size)
                }
                GridPlacementMode::Existing | GridPlacementMode::ExplicitOnly
                    if has_explicit_placement =>
                {
                    matches!(
                        size,
                        crate::LayoutDimension::Auto | crate::LayoutDimension::Fixed(_)
                    )
                }
                GridPlacementMode::TrackSizing
                    if !normalized_grid.auto_placed_items.contains(&child_id) =>
                {
                    valid_track_sizing_dimension(size)
                        || (c14_3_replaced_item && valid_c14_3_dimension(size))
                }
                GridPlacementMode::TemplateGrid
                    if !normalized_grid.auto_placed_items.contains(&child_id) =>
                {
                    matches!(
                        size,
                        crate::LayoutDimension::Auto | crate::LayoutDimension::Fixed(_)
                    ) || (c14_3_replaced_item && valid_c14_3_dimension(size))
                }
                GridPlacementMode::TemplateGridImplicitTracks
                    if !normalized_grid.auto_placed_items.contains(&child_id) =>
                {
                    matches!(
                        size,
                        crate::LayoutDimension::Auto | crate::LayoutDimension::Fixed(_)
                    )
                }
                _ => matches!(size, crate::LayoutDimension::Fixed(_)),
            };
            if !valid_item_size(child.style.width) || !valid_item_size(child.style.height) {
                return Err(LayoutError::UnsupportedGridItemSize(child_id));
            }
            for (property, value) in [
                ("min-width", child.style.min_width),
                ("max-width", child.style.max_width),
                ("min-height", child.style.min_height),
                ("max-height", child.style.max_height),
            ] {
                let zero_minimum_on_definite_area = matches!(
                    container.placement_mode,
                    GridPlacementMode::TemplateGrid | GridPlacementMode::TemplateGridImplicitTracks
                ) && !normalized_grid
                    .auto_placed_items
                    .contains(&child_id)
                    && matches!(property, "min-width" | "min-height")
                    && value == crate::LayoutDimension::Fixed(0.0);
                let track_sizing_constraint = container.placement_mode
                    == GridPlacementMode::TrackSizing
                    && matches!(
                        value,
                        crate::LayoutDimension::Auto
                            | crate::LayoutDimension::Fixed(_)
                            | crate::LayoutDimension::Percent(_)
                    );
                if value != crate::LayoutDimension::Auto
                    && !zero_minimum_on_definite_area
                    && !track_sizing_constraint
                {
                    return Err(LayoutError::UnsupportedGridItemConstraint {
                        node: child_id,
                        property,
                    });
                }
            }
            item_count = item_count
                .checked_add(1)
                .ok_or(LayoutError::GridTrackExpansionOverflow(node.id))?;
        }
        if !matches!(
            container.placement_mode,
            GridPlacementMode::SparseRow
                | GridPlacementMode::DenseAutoPlacement
                | GridPlacementMode::TemplateGrid
                | GridPlacementMode::TemplateGridImplicitTracks
                | GridPlacementMode::TrackSizing
        ) && !has_explicit_placement
            && item_count > cell_count
        {
            return Err(LayoutError::GridImplicitTrackRequired(node.id));
        }
    }
    Ok(())
}

fn indefinite_track_sizing_height_is_bounded(
    node: &crate::LayoutNode,
    container: &GridContainerInput,
) -> bool {
    node.style.height == crate::LayoutDimension::Auto
        && node.style.min_height == crate::LayoutDimension::Auto
        && node.style.max_height == crate::LayoutDimension::Auto
        && container.rows_auto_repeat.is_none()
        && container.rows.iter().all(track_is_indefinite_height_safe)
        && match &container.auto_rows {
            GridAutoRows::Auto | GridAutoRows::LengthPx(_) => true,
            GridAutoRows::TrackList(tracks) => tracks.iter().all(track_is_indefinite_height_safe),
        }
}

fn track_is_indefinite_height_safe(track: &GridTrackSize) -> bool {
    match track {
        GridTrackSize::LengthPx(_)
        | GridTrackSize::Auto
        | GridTrackSize::MinContent
        | GridTrackSize::MaxContent => true,
        GridTrackSize::MinMax { min, max } => {
            matches!(
                min,
                GridTrackMinSizingFunction::LengthPx(_)
                    | GridTrackMinSizingFunction::Auto
                    | GridTrackMinSizingFunction::MinContent
                    | GridTrackMinSizingFunction::MaxContent
            ) && matches!(
                max,
                GridTrackMaxSizingFunction::LengthPx(_)
                    | GridTrackMaxSizingFunction::Auto
                    | GridTrackMaxSizingFunction::MinContent
                    | GridTrackMaxSizingFunction::MaxContent
            )
        }
        GridTrackSize::Percentage(_)
        | GridTrackSize::Fraction(_)
        | GridTrackSize::Calc(_)
        | GridTrackSize::FitContent(_) => false,
    }
}

fn validate_template_areas(
    node: NodeId,
    container: &GridContainerInput,
) -> Result<(), LayoutError> {
    let Some(areas) = &container.template_areas else {
        return Ok(());
    };
    let property = "grid-template-areas";
    let has_auto_repeat_axis =
        container.columns_auto_repeat.is_some() || container.rows_auto_repeat.is_some();
    if container.placement_mode != GridPlacementMode::TrackSizing
        || !has_auto_repeat_axis
        || areas.rows == 0
        || areas.columns == 0
        || usize::from(areas.rows) > GRID_MAX_TRACKS
        || usize::from(areas.columns) > GRID_MAX_TRACKS
    {
        return Err(LayoutError::InvalidGridTrackList { node, property });
    }
    if (container.rows_auto_repeat.is_none() && usize::from(areas.rows) != container.rows.len())
        || (container.columns_auto_repeat.is_none()
            && usize::from(areas.columns) != container.columns.len())
    {
        return Err(LayoutError::InvalidGridTrackList { node, property });
    }

    let mut metadata_bytes = areas
        .areas
        .len()
        .checked_mul(std::mem::size_of::<GridTemplateAreaInput>())
        .filter(|bytes| *bytes <= MAX_GRID_LINE_METADATA_BYTES)
        .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
    let mut names = std::collections::HashSet::new();
    names
        .try_reserve(areas.areas.len())
        .map_err(|_| LayoutError::GridAllocationFailure { node, property })?;
    for area in &areas.areas {
        if area.name.is_empty()
            || area.name.as_bytes().contains(&0)
            || area.row_start >= area.row_end
            || area.row_end > areas.rows
            || area.column_start >= area.column_end
            || area.column_end > areas.columns
            || !names.insert(area.name.as_str())
        {
            return Err(LayoutError::InvalidGridTrackList { node, property });
        }
        metadata_bytes = metadata_bytes
            .checked_add(area.name.len())
            .filter(|bytes| *bytes <= MAX_GRID_LINE_METADATA_BYTES)
            .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
    }
    Ok(())
}

fn validate_auto_repeat_axis(
    node: NodeId,
    property: &'static str,
    tracks: &[GridTrackSize],
    outer_line_names: &[Vec<String>],
    auto_repeat: Option<&GridAutoRepeat>,
) -> Result<(), LayoutError> {
    let Some(auto_repeat) = auto_repeat else {
        return Ok(());
    };
    if auto_repeat.start >= auto_repeat.end || auto_repeat.end > tracks.len() {
        return Err(LayoutError::InvalidGridTrackList { node, property });
    }
    let repeated_track_count = auto_repeat.end - auto_repeat.start;
    let outer_line_name_count = auto_repeat_axis_outer_line_count(tracks, auto_repeat);
    let has_authored_names = outer_line_names
        .iter()
        .chain(&auto_repeat.line_names)
        .any(|group| !group.is_empty());
    let has_repeated_names = auto_repeat.line_names.iter().any(|group| !group.is_empty());
    if outer_line_name_count.is_none_or(|count| {
        (has_authored_names && outer_line_names.len() != count)
            || (!outer_line_names.is_empty() && outer_line_names.len() != count)
            || (has_repeated_names && auto_repeat.line_names.len() != repeated_track_count + 1)
            || (!auto_repeat.line_names.is_empty()
                && auto_repeat.line_names.len() != repeated_track_count + 1)
    }) {
        return Err(LayoutError::InvalidGridTrackList { node, property });
    }
    let prefix = &tracks[..auto_repeat.start];
    let repeat_tracks = &tracks[auto_repeat.start..auto_repeat.end];
    let suffix = &tracks[auto_repeat.end..];
    if !prefix
        .iter()
        .chain(suffix.iter())
        .all(auto_repeat_neighbor_track_is_fixed_px)
        || !repeat_tracks
            .iter()
            .all(auto_repeat_track_has_fixed_minimum)
    {
        return Err(LayoutError::InvalidGridTrackList { node, property });
    }
    Ok(())
}

fn validate_grid_line_metadata(
    node: NodeId,
    container: &GridContainerInput,
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> Result<(), LayoutError> {
    let valid_column_line_names = match container.columns_auto_repeat.as_ref() {
        Some(auto_repeat) => {
            auto_repeat_axis_outer_line_count(&container.columns, auto_repeat).is_some_and(
                |count| {
                    container.columns_line_names.is_empty()
                        || container.columns_line_names.len() == count
                },
            ) && (auto_repeat.line_names.is_empty()
                || auto_repeat.line_names.len()
                    == auto_repeat
                        .end
                        .saturating_sub(auto_repeat.start)
                        .saturating_add(1))
        }
        None => {
            container.columns_line_names.is_empty()
                || container.columns.len().checked_add(1)
                    == Some(container.columns_line_names.len())
        }
    };
    let valid_row_line_names = match container.rows_auto_repeat.as_ref() {
        Some(auto_repeat) => {
            auto_repeat_axis_outer_line_count(&container.rows, auto_repeat).is_some_and(|count| {
                container.rows_line_names.is_empty() || container.rows_line_names.len() == count
            }) && (auto_repeat.line_names.is_empty()
                || auto_repeat.line_names.len()
                    == auto_repeat
                        .end
                        .saturating_sub(auto_repeat.start)
                        .saturating_add(1))
        }
        None => {
            container.rows_line_names.is_empty()
                || container.rows.len().checked_add(1) == Some(container.rows_line_names.len())
        }
    };
    let property = if !valid_column_line_names {
        "grid-template-columns"
    } else {
        "grid-template-rows"
    };
    if !valid_column_line_names || !valid_row_line_names {
        return Err(LayoutError::InvalidGridTrackList { node, property });
    }

    if container.columns_auto_repeat.is_none() && container.rows_auto_repeat.is_none() {
        return Ok(());
    };
    validate_auto_repeat_line_metadata_budget(node, container, input, index)
}

fn validate_auto_repeat_line_metadata_budget(
    node: NodeId,
    container: &GridContainerInput,
    input: &LayoutInput,
    index: &BTreeMap<NodeId, usize>,
) -> Result<(), LayoutError> {
    let mut source_bytes = 0_usize;
    let mut expanded_bytes_total = 0_usize;
    for (property, line_names, auto_repeat) in [
        (
            "grid-template-columns",
            &container.columns_line_names,
            container.columns_auto_repeat.as_ref(),
        ),
        (
            "grid-template-rows",
            &container.rows_line_names,
            container.rows_auto_repeat.as_ref(),
        ),
    ] {
        let mut bytes = grid_line_name_groups_metadata_bytes(line_names.iter())
            .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
        if let Some(auto_repeat) = auto_repeat {
            let repeated_bytes =
                grid_line_name_groups_metadata_bytes(auto_repeat.line_names.iter())
                    .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
            bytes = bytes
                .checked_add(repeated_bytes)
                .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
            if !auto_repeat.line_names.is_empty() {
                let repeated_track_count = auto_repeat
                    .end
                    .checked_sub(auto_repeat.start)
                    .filter(|count| *count > 0)
                    .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
                let maximum_repeat_count = GRID_MAX_TRACKS / repeated_track_count;
                let expanded_bytes = repeated_bytes
                    .checked_mul(maximum_repeat_count)
                    .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
                expanded_bytes_total = expanded_bytes_total
                    .checked_add(expanded_bytes)
                    .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
            }
        }
        source_bytes = source_bytes
            .checked_add(bytes)
            .ok_or(LayoutError::InvalidGridTrackList { node, property })?;
    }

    let grid_node = &input.nodes[index[&node]];
    for child in &grid_node.children {
        let Some(item) = input.grid_items.get(child) else {
            continue;
        };
        for placement in [
            &item.row_start,
            &item.row_end,
            &item.column_start,
            &item.column_end,
        ] {
            let name = match placement {
                GridPlacementInput::NamedLine { name, .. }
                | GridPlacementInput::SpanNamed { name, .. } => Some(name),
                GridPlacementInput::Auto
                | GridPlacementInput::Line(_)
                | GridPlacementInput::Span(_) => None,
            };
            if let Some(name) = name {
                if name.is_empty() {
                    return Err(LayoutError::InvalidGridPlacement {
                        node: *child,
                        property: "grid-placement",
                        reason: "named line 이름은 비어 있을 수 없습니다",
                    });
                }
                source_bytes = source_bytes
                    .checked_add(std::mem::size_of::<String>())
                    .and_then(|bytes| bytes.checked_add(name.len()))
                    .ok_or(LayoutError::InvalidGridTrackList {
                        node,
                        property: "grid-template-columns",
                    })?;
            }
        }
    }
    if source_bytes > MAX_GRID_LINE_METADATA_BYTES
        || expanded_bytes_total > MAX_EXPANDED_GRID_LINE_METADATA_BYTES
    {
        return Err(LayoutError::InvalidGridTrackList {
            node,
            property: if container.columns_auto_repeat.is_some() {
                "grid-template-columns"
            } else {
                "grid-template-rows"
            },
        });
    }
    Ok(())
}

fn grid_line_name_groups_metadata_bytes<'a>(
    groups: impl Iterator<Item = &'a Vec<String>>,
) -> Option<usize> {
    let mut bytes = 0_usize;
    for group in groups {
        bytes = bytes.checked_add(std::mem::size_of::<Vec<String>>())?;
        for name in group {
            if name.is_empty() {
                return None;
            }
            bytes = bytes
                .checked_add(std::mem::size_of::<String>())?
                .checked_add(name.len())?;
        }
    }
    Some(bytes)
}

fn auto_repeat_axis_outer_line_count(
    tracks: &[GridTrackSize],
    auto_repeat: &GridAutoRepeat,
) -> Option<usize> {
    let repeated_track_count = auto_repeat.end.checked_sub(auto_repeat.start)?;
    tracks
        .len()
        .checked_sub(repeated_track_count)?
        .checked_add(2)
}

fn validate_tracks(
    node: NodeId,
    property: &'static str,
    tracks: &[GridTrackSize],
    allow_intrinsic: bool,
) -> Result<(), LayoutError> {
    if tracks.is_empty() {
        return if allow_intrinsic {
            Ok(())
        } else {
            Err(LayoutError::InvalidGridTrackList { node, property })
        };
    }
    if tracks.len() > GRID_MAX_TRACKS {
        return Err(LayoutError::GridTrackLimitExceeded {
            node,
            property,
            count: tracks.len(),
            maximum: GRID_MAX_TRACKS,
        });
    }
    for track in tracks {
        let valid = match track {
            GridTrackSize::LengthPx(value) | GridTrackSize::Percentage(value) => {
                value.is_finite() && *value >= 0.0
            }
            GridTrackSize::Fraction(value) => value.is_finite() && *value >= 0.0,
            GridTrackSize::Calc(_) => allow_intrinsic,
            GridTrackSize::Auto | GridTrackSize::MinContent | GridTrackSize::MaxContent => {
                allow_intrinsic
            }
            GridTrackSize::MinMax { min, max } => {
                allow_intrinsic
                    && valid_min_track_sizing_function(*min)
                    && valid_max_track_sizing_function(*max)
            }
            GridTrackSize::FitContent(value) => {
                allow_intrinsic && valid_grid_track_length_percentage(*value)
            }
        };
        if !valid {
            return Err(LayoutError::InvalidGridTrack { node, property });
        }
    }
    Ok(())
}

fn valid_grid_track_length_percentage(value: GridTrackLengthPercentage) -> bool {
    match value {
        GridTrackLengthPercentage::LengthPx(value)
        | GridTrackLengthPercentage::Percentage(value) => value.is_finite() && value >= 0.0,
        GridTrackLengthPercentage::Calc(_) => true,
    }
}

fn auto_repeat_track_has_fixed_minimum(track: &GridTrackSize) -> bool {
    match track {
        GridTrackSize::LengthPx(value) => value.is_finite() && *value >= 0.0,
        GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::LengthPx(value),
            max: GridTrackMaxSizingFunction::LengthPx(_) | GridTrackMaxSizingFunction::Fraction(_),
        } => value.is_finite() && *value >= 0.0,
        _ => false,
    }
}

fn auto_repeat_neighbor_track_is_fixed_px(track: &GridTrackSize) -> bool {
    matches!(track, GridTrackSize::LengthPx(value) if value.is_finite() && *value >= 0.0)
}

fn valid_min_track_sizing_function(value: GridTrackMinSizingFunction) -> bool {
    match value {
        GridTrackMinSizingFunction::LengthPx(value)
        | GridTrackMinSizingFunction::Percentage(value) => value.is_finite() && value >= 0.0,
        GridTrackMinSizingFunction::Calc(_) => true,
        GridTrackMinSizingFunction::Auto
        | GridTrackMinSizingFunction::MinContent
        | GridTrackMinSizingFunction::MaxContent => true,
    }
}

fn valid_max_track_sizing_function(value: GridTrackMaxSizingFunction) -> bool {
    match value {
        GridTrackMaxSizingFunction::LengthPx(value)
        | GridTrackMaxSizingFunction::Percentage(value) => value.is_finite() && value >= 0.0,
        GridTrackMaxSizingFunction::Calc(_) => true,
        GridTrackMaxSizingFunction::Fraction(value) => value.is_finite() && value >= 0.0,
        GridTrackMaxSizingFunction::Auto
        | GridTrackMaxSizingFunction::MinContent
        | GridTrackMaxSizingFunction::MaxContent => true,
    }
}

#[cfg(test)]
#[path = "grid_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "grid_template_areas_tests.rs"]
mod template_areas_tests;

#[cfg(test)]
#[path = "grid_track_sizing_tests.rs"]
mod track_sizing_tests;

#[cfg(test)]
#[path = "grid_auto_track_sizing_tests.rs"]
mod auto_track_sizing_tests;

#[cfg(test)]
#[path = "grid_auto_placement_tests.rs"]
mod auto_placement_tests;

#[cfg(test)]
#[path = "grid_auto_placement/dense_column_tests.rs"]
mod dense_column_tests;

#[cfg(test)]
#[path = "grid_auto_placement/track_sizing_tests.rs"]
mod negative_track_sizing_tests;
