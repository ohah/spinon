use spinon_core::NodeId;
use taffy::geometry::{Line, Point, Rect, Size};
use taffy::style::{
    AlignContent, AlignItems, AlignSelf, BlockContainerStyle, BlockItemStyle, BoxGenerationMode,
    BoxSizing, Contain, Direction, Display, FlexDirection, FlexWrap, FlexboxContainerStyle,
    FlexboxItemStyle, GenericGridTemplateComponent, GridAutoFlow, GridContainerStyle,
    GridItemStyle, GridPlacement, GridTemplateArea, GridTemplateAreas, GridTemplateRepetition,
    JustifyContent, LengthPercentage, LengthPercentageAuto, OofItemStyle, Overflow, Position,
    TextAlign,
};
use taffy::{CoreStyle, Dimension, GridTemplateComponent, TrackSizingFunction};

type GridTemplateReference<'a> =
    GenericGridTemplateComponent<String, &'a GridTemplateRepetition<String>>;
type GridTemplateIterator<'a> = std::iter::Map<
    std::slice::Iter<'a, GridTemplateComponent<String>>,
    fn(&'a GridTemplateComponent<String>) -> GridTemplateReference<'a>,
>;
type TemplateLineNameGroups<'a> = std::iter::Map<
    std::slice::Iter<'a, Vec<String>>,
    fn(&Vec<String>) -> std::slice::Iter<'_, String>,
>;

fn grid_template_reference<'a>(
    component: &'a GridTemplateComponent<String>,
) -> GridTemplateReference<'a> {
    component.as_component_ref()
}

// Taffy 0.15's TemplateLineNames implementation requires this exact `fn(&Vec<_>)` map shape.
#[allow(clippy::ptr_arg)]
fn grid_line_name_group<'a>(group: &'a Vec<String>) -> std::slice::Iter<'a, String> {
    group.iter()
}

/// Taffy `Style`은 모든 노드에 Grid track·line·area 필드를 포함합니다.
/// 이 투영은 현재 Spinon 레이아웃 프로필이 사용하는 값만 보존합니다.
#[derive(Clone)]
pub(super) struct CompactTaffyStyle {
    pub(super) display: Display,
    item_is_replaced: bool,
    box_sizing: BoxSizing,
    direction: Direction,
    overflow: Point<Overflow>,
    scrollbar_width: f32,
    position: Position,
    inset: Rect<LengthPercentageAuto>,
    size: Size<Dimension>,
    min_size: Size<LengthPercentageAuto>,
    max_size: Size<LengthPercentageAuto>,
    pub(super) aspect_ratio: Option<f32>,
    margin: Rect<LengthPercentageAuto>,
    padding: Rect<LengthPercentage>,
    border: Rect<LengthPercentage>,
    contain: Contain,
    align_content: AlignContent,
    align_items: AlignItems,
    align_self: Option<AlignSelf>,
    justify_content: JustifyContent,
    justify_items: AlignItems,
    justify_self: Option<AlignSelf>,
    gap: Size<LengthPercentage>,
    text_align: TextAlign,
    flex_direction: FlexDirection,
    flex_wrap: FlexWrap,
    flex_basis: Dimension,
    flex_grow: f32,
    flex_shrink: f32,
    grid_template_rows: Vec<GridTemplateComponent<String>>,
    grid_template_columns: Vec<GridTemplateComponent<String>>,
    grid_template_row_names: Vec<Vec<String>>,
    grid_template_column_names: Vec<Vec<String>>,
    grid_template_areas: Vec<GridTemplateAreas<String>>,
    grid_auto_rows: Vec<TrackSizingFunction>,
    grid_auto_columns: Vec<TrackSizingFunction>,
    grid_auto_flow: GridAutoFlow,
    track_sizing_trace_enabled: bool,
    grid_row: Line<GridPlacement<String>>,
    grid_column: Line<GridPlacement<String>>,
}

impl CompactTaffyStyle {
    pub(super) fn display(&self) -> Display {
        self.display
    }

    pub(super) fn size(&self) -> Size<Dimension> {
        self.size
    }

    pub(super) fn disable_aspect_ratio(&mut self) {
        self.aspect_ratio = None;
    }

    pub(super) fn set_track_sizing_trace_enabled(&mut self, enabled: bool) {
        self.track_sizing_trace_enabled = enabled;
    }

    pub(super) fn set_grid_template_areas(
        &mut self,
        node: NodeId,
        areas: Option<GridTemplateAreas<String>>,
    ) -> Result<(), crate::LayoutError> {
        if let Some(areas) = areas {
            self.grid_template_areas.try_reserve_exact(1).map_err(|_| {
                crate::LayoutError::GridAllocationFailure {
                    node,
                    property: "grid-template-areas",
                }
            })?;
            self.grid_template_areas.push(areas);
        }
        Ok(())
    }

    pub(super) fn set_used_replaced_border_box_size(&mut self, width: f32, height: f32) {
        self.box_sizing = BoxSizing::BorderBox;
        self.size = Size {
            width: Dimension::length(width),
            height: Dimension::length(height),
        };
        self.min_size = Size {
            width: LengthPercentageAuto::auto(),
            height: LengthPercentageAuto::auto(),
        };
        self.max_size = self.min_size;
        self.aspect_ratio = None;
    }

    pub(super) fn set_grid_item_placement(
        &mut self,
        row_start: GridPlacement<String>,
        row_end: GridPlacement<String>,
        column_start: GridPlacement<String>,
        column_end: GridPlacement<String>,
    ) {
        self.grid_row = Line {
            start: row_start,
            end: row_end,
        };
        self.grid_column = Line {
            start: column_start,
            end: column_end,
        };
    }
}

impl From<taffy::Style> for CompactTaffyStyle {
    fn from(style: taffy::Style) -> Self {
        Self {
            display: style.display,
            item_is_replaced: style.item_is_replaced,
            box_sizing: style.box_sizing,
            direction: style.direction,
            overflow: style.overflow,
            scrollbar_width: style.scrollbar_width,
            position: style.position,
            inset: style.inset,
            size: style.size,
            min_size: style.min_size,
            max_size: style.max_size,
            aspect_ratio: style.aspect_ratio,
            margin: style.margin,
            padding: style.padding,
            border: style.border,
            contain: style.contain,
            align_content: style.align_content,
            align_items: style.align_items,
            align_self: style.align_self,
            justify_content: style.justify_content,
            justify_items: style.justify_items,
            justify_self: style.justify_self,
            gap: style.gap,
            text_align: style.text_align,
            flex_direction: style.flex_direction,
            flex_wrap: style.flex_wrap,
            flex_basis: style.flex_basis,
            flex_grow: style.flex_grow,
            flex_shrink: style.flex_shrink,
            grid_template_rows: style.grid_template_rows,
            grid_template_columns: style.grid_template_columns,
            grid_template_row_names: style.grid_template_row_names,
            grid_template_column_names: style.grid_template_column_names,
            grid_template_areas: Vec::new(),
            grid_auto_rows: style.grid_auto_rows,
            grid_auto_columns: style.grid_auto_columns,
            grid_auto_flow: style.grid_auto_flow,
            track_sizing_trace_enabled: false,
            grid_row: Line {
                start: GridPlacement::Auto,
                end: GridPlacement::Auto,
            },
            grid_column: Line {
                start: GridPlacement::Auto,
                end: GridPlacement::Auto,
            },
        }
    }
}

impl CoreStyle for CompactTaffyStyle {
    type CustomIdent = String;

    fn box_generation_mode(&self) -> BoxGenerationMode {
        if self.display == Display::None {
            BoxGenerationMode::None
        } else {
            BoxGenerationMode::Normal
        }
    }

    fn is_block(&self) -> bool {
        self.display == Display::Block
    }

    fn is_grid(&self) -> bool {
        self.display == Display::Grid
    }

    fn is_compressible_replaced(&self) -> bool {
        self.item_is_replaced
    }

    fn is_replaced(&self) -> bool {
        self.item_is_replaced
    }

    fn box_sizing(&self) -> BoxSizing {
        self.box_sizing
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn overflow(&self) -> Point<Overflow> {
        self.overflow
    }

    fn scrollbar_width(&self) -> f32 {
        self.scrollbar_width
    }

    fn position(&self) -> Position {
        self.position
    }

    fn inset(&self) -> Rect<LengthPercentageAuto> {
        self.inset
    }

    fn size(&self) -> Size<Dimension> {
        self.size
    }

    fn min_size(&self) -> Size<LengthPercentageAuto> {
        self.min_size
    }

    fn max_size(&self) -> Size<LengthPercentageAuto> {
        self.max_size
    }

    fn aspect_ratio(&self) -> Option<f32> {
        self.aspect_ratio
    }

    fn margin(&self) -> Rect<LengthPercentageAuto> {
        self.margin
    }

    fn padding(&self) -> Rect<LengthPercentage> {
        self.padding
    }

    fn border(&self) -> Rect<LengthPercentage> {
        self.border
    }

    fn contain(&self) -> Contain {
        self.contain
    }
}

impl BlockContainerStyle for CompactTaffyStyle {
    fn text_align(&self) -> TextAlign {
        self.text_align
    }

    fn align_content(&self) -> AlignContent {
        self.align_content
    }
}

impl BlockItemStyle for CompactTaffyStyle {}

impl FlexboxContainerStyle for CompactTaffyStyle {
    fn flex_direction(&self) -> FlexDirection {
        self.flex_direction
    }

    fn flex_wrap(&self) -> FlexWrap {
        self.flex_wrap
    }

    fn gap(&self) -> Size<LengthPercentage> {
        self.gap
    }

    fn align_content(&self) -> AlignContent {
        self.align_content
    }

    fn align_items(&self) -> AlignItems {
        self.align_items
    }

    fn justify_content(&self) -> JustifyContent {
        self.justify_content
    }
}

impl FlexboxItemStyle for CompactTaffyStyle {
    fn flex_basis(&self) -> Dimension {
        self.flex_basis
    }

    fn flex_grow(&self) -> f32 {
        self.flex_grow
    }

    fn flex_shrink(&self) -> f32 {
        self.flex_shrink
    }

    fn align_self(&self) -> Option<AlignSelf> {
        self.align_self
    }
}

impl GridContainerStyle for CompactTaffyStyle {
    type Repetition<'a> = &'a GridTemplateRepetition<String>;
    type TemplateTrackList<'a> = GridTemplateIterator<'a>;
    type AutoTrackList<'a> = std::iter::Copied<std::slice::Iter<'a, TrackSizingFunction>>;
    type TemplateLineNames<'a> = TemplateLineNameGroups<'a>;
    type GridTemplateAreas<'a> = std::iter::Cloned<std::slice::Iter<'a, GridTemplateArea<String>>>;

    fn grid_template_rows(&self) -> Option<Self::TemplateTrackList<'_>> {
        Some(
            self.grid_template_rows
                .iter()
                .map(grid_template_reference as fn(_) -> _),
        )
    }

    fn grid_template_columns(&self) -> Option<Self::TemplateTrackList<'_>> {
        Some(
            self.grid_template_columns
                .iter()
                .map(grid_template_reference as fn(_) -> _),
        )
    }

    fn grid_auto_rows(&self) -> Self::AutoTrackList<'_> {
        self.grid_auto_rows.iter().copied()
    }

    fn grid_auto_columns(&self) -> Self::AutoTrackList<'_> {
        self.grid_auto_columns.iter().copied()
    }

    fn grid_template_areas(&self) -> Option<Self::GridTemplateAreas<'_>> {
        self.grid_template_areas
            .first()
            .map(|template| template.areas.iter().cloned())
    }

    fn grid_template_area_row_count(&self) -> u16 {
        self.grid_template_areas
            .first()
            .map(|template| template.row_count)
            .unwrap_or(0)
    }

    fn grid_template_area_column_count(&self) -> u16 {
        self.grid_template_areas
            .first()
            .map(|template| template.column_count)
            .unwrap_or(0)
    }

    fn grid_template_column_names(&self) -> Option<Self::TemplateLineNames<'_>> {
        Some(
            self.grid_template_column_names
                .iter()
                .map(grid_line_name_group as fn(&Vec<String>) -> std::slice::Iter<'_, String>),
        )
    }

    fn grid_template_row_names(&self) -> Option<Self::TemplateLineNames<'_>> {
        Some(
            self.grid_template_row_names
                .iter()
                .map(grid_line_name_group as fn(&Vec<String>) -> std::slice::Iter<'_, String>),
        )
    }

    fn grid_auto_flow(&self) -> GridAutoFlow {
        self.grid_auto_flow
    }

    fn gap(&self) -> Size<LengthPercentage> {
        self.gap
    }

    fn align_content(&self) -> AlignContent {
        self.align_content
    }

    fn justify_content(&self) -> JustifyContent {
        self.justify_content
    }

    fn align_items(&self) -> AlignItems {
        self.align_items
    }

    fn justify_items(&self) -> AlignItems {
        self.justify_items
    }

    fn track_sizing_trace_enabled(&self) -> bool {
        self.track_sizing_trace_enabled
    }
}

impl GridItemStyle for CompactTaffyStyle {
    fn grid_row(&self) -> Line<GridPlacement<String>> {
        self.grid_row.clone()
    }

    fn grid_column(&self) -> Line<GridPlacement<String>> {
        self.grid_column.clone()
    }

    fn align_self(&self) -> Option<AlignSelf> {
        self.align_self
    }

    fn justify_self(&self) -> Option<AlignSelf> {
        self.justify_self
    }
}

impl OofItemStyle for CompactTaffyStyle {
    fn align_self(&self) -> Option<AlignSelf> {
        self.align_self
    }

    fn justify_self(&self) -> Option<AlignSelf> {
        self.justify_self
    }

    fn grid_row(&self) -> Line<GridPlacement<String>> {
        self.grid_row.clone()
    }

    fn grid_column(&self) -> Line<GridPlacement<String>> {
        self.grid_column.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::CompactTaffyStyle;

    #[test]
    fn grid_style_projection_is_smaller_than_taffy_style() {
        let compact_size = std::mem::size_of::<CompactTaffyStyle>();
        let taffy_size = std::mem::size_of::<taffy::Style>();
        assert!(
            compact_size < taffy_size,
            "compact layout style is {compact_size} bytes; Taffy style is {taffy_size} bytes",
        );
    }
}
