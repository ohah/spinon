use std::collections::BTreeMap;

use spinon_core::NodeId;
use style::{
    properties::ComputedValues,
    values::computed::{
        GridTemplateComponent, TrackBreadth, TrackSize,
        length_percentage::Unpacked as LengthPercentageValue, position::GridAutoFlow,
    },
    values::generics::grid::TrackListValue,
};

use super::super::{
    ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRows,
    ComputedCssGridItemPlacement, ComputedCssGridLayout, ComputedCssGridPlacement,
    ComputedCssGridTrackSize, CssCascadeError,
};

const MAX_COMPUTED_GRID_TRACKS: usize = 10_000;

pub(super) fn computed_grid_layout(
    computed: &ComputedValues,
    node: NodeId,
    properties: &BTreeMap<String, String>,
) -> Result<ComputedCssGridLayout, CssCascadeError> {
    let columns_value = property_value(properties, "grid-template-columns");
    let rows_value = property_value(properties, "grid-template-rows");
    let columns = computed_grid_tracks(
        computed.clone_grid_template_columns(),
        node,
        "grid-template-columns",
        columns_value,
    )?;
    let rows = computed_grid_tracks(
        computed.clone_grid_template_rows(),
        node,
        "grid-template-rows",
        rows_value,
    )?;
    let auto_rows = computed_grid_auto_rows(
        computed.clone_grid_auto_rows(),
        property_value(properties, "grid-auto-rows"),
    );
    let auto_columns = computed_grid_auto_columns(
        computed.clone_grid_auto_columns(),
        property_value(properties, "grid-auto-columns"),
    );
    let flow = computed.clone_grid_auto_flow();
    let auto_flow = match (
        flow.contains(GridAutoFlow::COLUMN),
        flow.contains(GridAutoFlow::DENSE),
    ) {
        (false, false) => ComputedCssGridAutoFlow::Row,
        (true, false) => ComputedCssGridAutoFlow::Column,
        (false, true) => ComputedCssGridAutoFlow::RowDense,
        (true, true) => ComputedCssGridAutoFlow::ColumnDense,
    };

    Ok(ComputedCssGridLayout {
        columns,
        rows,
        auto_flow,
        auto_rows,
        auto_columns,
    })
}

fn computed_grid_auto_columns(
    value: style::values::computed::ImplicitGridTracks,
    serialized: &str,
) -> ComputedCssGridAutoColumns {
    if value.is_initial() {
        return ComputedCssGridAutoColumns::Auto;
    }
    if value.0.len() != 1 {
        return ComputedCssGridAutoColumns::Unsupported(serialized.to_owned());
    }
    let TrackSize::Breadth(TrackBreadth::Breadth(value)) = &value.0[0] else {
        return ComputedCssGridAutoColumns::Unsupported(serialized.to_owned());
    };
    let LengthPercentageValue::Length(length) = value.unpack() else {
        return ComputedCssGridAutoColumns::Unsupported(serialized.to_owned());
    };
    let pixels = length.px();
    if pixels.is_finite() && pixels >= 0.0 {
        ComputedCssGridAutoColumns::LengthPx(pixels)
    } else {
        ComputedCssGridAutoColumns::Unsupported(serialized.to_owned())
    }
}

fn computed_grid_auto_rows(
    value: style::values::computed::ImplicitGridTracks,
    serialized: &str,
) -> ComputedCssGridAutoRows {
    if value.is_initial() {
        return ComputedCssGridAutoRows::Auto;
    }
    if value.0.len() != 1 {
        return ComputedCssGridAutoRows::Unsupported(serialized.to_owned());
    }
    let TrackSize::Breadth(TrackBreadth::Breadth(value)) = &value.0[0] else {
        return ComputedCssGridAutoRows::Unsupported(serialized.to_owned());
    };
    let LengthPercentageValue::Length(length) = value.unpack() else {
        return ComputedCssGridAutoRows::Unsupported(serialized.to_owned());
    };
    let pixels = length.px();
    if pixels.is_finite() && pixels >= 0.0 {
        ComputedCssGridAutoRows::LengthPx(pixels)
    } else {
        ComputedCssGridAutoRows::Unsupported(serialized.to_owned())
    }
}

pub(super) fn computed_grid_item_placement(
    computed: &ComputedValues,
    properties: &BTreeMap<String, String>,
) -> ComputedCssGridItemPlacement {
    let row_start = computed.clone_grid_row_start();
    let row_end = computed.clone_grid_row_end();
    let column_start = computed.clone_grid_column_start();
    let column_end = computed.clone_grid_column_end();
    ComputedCssGridItemPlacement {
        row_start: computed_grid_placement(row_start, property_value(properties, "grid-row-start")),
        row_end: computed_grid_placement(row_end, property_value(properties, "grid-row-end")),
        column_start: computed_grid_placement(
            column_start,
            property_value(properties, "grid-column-start"),
        ),
        column_end: computed_grid_placement(
            column_end,
            property_value(properties, "grid-column-end"),
        ),
    }
}

fn computed_grid_placement(
    value: style::values::computed::GridLine,
    serialized: &str,
) -> ComputedCssGridPlacement {
    if !value.ident.0.as_str().is_empty() {
        return ComputedCssGridPlacement::Unsupported(serialized.to_owned());
    }
    if value.is_auto() {
        return ComputedCssGridPlacement::Auto;
    }
    if value.is_span {
        ComputedCssGridPlacement::Span(value.line_num)
    } else {
        ComputedCssGridPlacement::Line(value.line_num)
    }
}

fn computed_grid_tracks(
    component: GridTemplateComponent,
    node: NodeId,
    property: &'static str,
    serialized: &str,
) -> Result<Vec<ComputedCssGridTrackSize>, CssCascadeError> {
    let GridTemplateComponent::TrackList(list) = component else {
        return Err(unsupported_grid_value(node, property, serialized));
    };
    if list.values.is_empty()
        || list.values.len() > MAX_COMPUTED_GRID_TRACKS
        || list.has_auto_repeat()
        || list.line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported_grid_value(node, property, serialized));
    }

    let mut tracks = Vec::new();
    reserve_computed_tracks(
        &mut tracks,
        list.values.len(),
        node,
        property,
        |tracks, count| tracks.try_reserve_exact(count).map_err(|_| ()),
    )?;
    for value in list.values.iter() {
        let TrackListValue::TrackSize(size) = value else {
            return Err(unsupported_grid_value(node, property, serialized));
        };
        tracks.push(
            track_size(size).ok_or_else(|| unsupported_grid_value(node, property, serialized))?,
        );
    }
    Ok(tracks)
}

fn reserve_computed_tracks(
    tracks: &mut Vec<ComputedCssGridTrackSize>,
    count: usize,
    node: NodeId,
    property: &'static str,
    reserve: impl FnOnce(&mut Vec<ComputedCssGridTrackSize>, usize) -> Result<(), ()>,
) -> Result<(), CssCascadeError> {
    reserve(tracks, count).map_err(|_| CssCascadeError::GridAllocationFailure { node, property })
}

fn track_size(size: &TrackSize) -> Option<ComputedCssGridTrackSize> {
    let track = match size {
        TrackSize::Breadth(TrackBreadth::Breadth(value)) => match value.unpack() {
            LengthPercentageValue::Length(length) => {
                ComputedCssGridTrackSize::LengthPx(length.px())
            }
            LengthPercentageValue::Percentage(percentage) => {
                ComputedCssGridTrackSize::Percentage(percentage.0)
            }
            LengthPercentageValue::Calc(_) => return None,
        },
        TrackSize::Breadth(TrackBreadth::Flex(value)) => {
            ComputedCssGridTrackSize::Fraction(value.0)
        }
        TrackSize::Breadth(
            TrackBreadth::Auto | TrackBreadth::MinContent | TrackBreadth::MaxContent,
        )
        | TrackSize::Minmax(_, _)
        | TrackSize::FitContent(_) => return None,
    };
    let valid = match track {
        ComputedCssGridTrackSize::LengthPx(value) | ComputedCssGridTrackSize::Percentage(value) => {
            value.is_finite() && value >= 0.0
        }
        ComputedCssGridTrackSize::Fraction(value) => value.is_finite() && value > 0.0,
    };
    valid.then_some(track)
}

fn property_value<'a>(properties: &'a BTreeMap<String, String>, property: &str) -> &'a str {
    properties
        .get(property)
        .map(String::as_str)
        .unwrap_or("<계산값 없음>")
}

fn unsupported_grid_value(node: NodeId, property: &'static str, value: &str) -> CssCascadeError {
    CssCascadeError::UnsupportedComputedGridValue {
        node,
        property,
        value: value.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::reserve_computed_tracks;

    #[test]
    fn track_reservation_failure_keeps_node_and_property_context() {
        let mut tracks = Vec::new();
        let error = reserve_computed_tracks(
            &mut tracks,
            4,
            spinon_core::NodeId::new(6059).unwrap(),
            "grid-template-columns",
            |_, _| Err(()),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            crate::stylo_dom::cascade::CssCascadeError::GridAllocationFailure {
                node,
                property: "grid-template-columns"
            } if node == spinon_core::NodeId::new(6059).unwrap()
        ));
        assert!(tracks.is_empty());
    }
}
