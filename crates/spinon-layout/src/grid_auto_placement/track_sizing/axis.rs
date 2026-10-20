use spinon_core::NodeId;

use crate::{
    GridPlacementInput, LayoutError,
    grid_placement::{invalid_placement, positive_span},
};

#[derive(Clone, Copy)]
pub(super) struct AxisPlacement {
    pub(super) start: Option<i64>,
    pub(super) span: usize,
}

#[derive(Clone, Copy)]
enum Endpoint {
    Auto,
    Boundary(i64),
    Span(usize),
}

pub(super) fn resolve_axis(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: GridPlacementInput,
    end: GridPlacementInput,
    explicit_track_count: usize,
) -> Result<AxisPlacement, LayoutError> {
    let mut start = endpoint(node, start_property, start, explicit_track_count)?;
    let mut end = endpoint(node, end_property, end, explicit_track_count)?;

    match (start, end) {
        (Endpoint::Boundary(start_boundary), Endpoint::Boundary(end_boundary))
            if start_boundary > end_boundary =>
        {
            start = Endpoint::Boundary(end_boundary);
            end = Endpoint::Boundary(start_boundary);
        }
        (Endpoint::Boundary(start_boundary), Endpoint::Boundary(end_boundary))
            if start_boundary == end_boundary =>
        {
            end = Endpoint::Auto;
        }
        (Endpoint::Span(_), Endpoint::Span(_)) => end = Endpoint::Auto,
        _ => {}
    }

    let definite = match (start, end) {
        (Endpoint::Boundary(start), Endpoint::Boundary(end)) => {
            let span = usize::try_from(end.checked_sub(start).ok_or_else(|| {
                invalid_placement(
                    node,
                    end_property,
                    "line 차이 계산이 정수 범위를 넘었습니다",
                )
            })?)
            .map_err(|_| {
                invalid_placement(node, end_property, "line 사이 track 수가 유효하지 않습니다")
            })?;
            Some((start, span))
        }
        (Endpoint::Boundary(start), Endpoint::Span(span)) => Some((start, span)),
        (Endpoint::Span(span), Endpoint::Boundary(end)) => {
            let span_i64 = i64::try_from(span).map_err(|_| {
                invalid_placement(node, start_property, "span이 정수 범위를 넘었습니다")
            })?;
            let start = end.checked_sub(span_i64).ok_or_else(|| {
                invalid_placement(
                    node,
                    start_property,
                    "span 시작 line 계산이 범위를 넘었습니다",
                )
            })?;
            Some((start, span))
        }
        (Endpoint::Boundary(start), Endpoint::Auto) => Some((start, 1)),
        (Endpoint::Auto, Endpoint::Boundary(end)) => {
            let start = end.checked_sub(1).ok_or_else(|| {
                invalid_placement(
                    node,
                    start_property,
                    "끝 line 앞의 track을 계산할 수 없습니다",
                )
            })?;
            Some((start, 1))
        }
        (Endpoint::Span(span), Endpoint::Auto) | (Endpoint::Auto, Endpoint::Span(span)) => {
            return Ok(AxisPlacement { start: None, span });
        }
        (Endpoint::Auto, Endpoint::Auto) => {
            return Ok(AxisPlacement {
                start: None,
                span: 1,
            });
        }
        (Endpoint::Span(_), Endpoint::Span(_)) => unreachable!("end span was removed above"),
    };

    let (start, span) = definite.expect("every remaining pair has a definite boundary");
    let span_i64 = i64::try_from(span)
        .map_err(|_| invalid_placement(node, end_property, "span이 정수 범위를 넘었습니다"))?;
    start.checked_add(span_i64).ok_or_else(|| {
        invalid_placement(
            node,
            end_property,
            "line과 span 계산이 정수 범위를 넘었습니다",
        )
    })?;
    Ok(AxisPlacement {
        start: Some(start),
        span,
    })
}

fn endpoint(
    node: NodeId,
    property: &'static str,
    placement: GridPlacementInput,
    explicit_track_count: usize,
) -> Result<Endpoint, LayoutError> {
    match placement {
        GridPlacementInput::Auto => Ok(Endpoint::Auto),
        GridPlacementInput::Line(line) => {
            if line == 0 {
                return Err(invalid_placement(
                    node,
                    property,
                    "line 0은 CSS Grid에서 유효하지 않습니다",
                ));
            }
            let boundary = if line > 0 {
                i64::from(line) - 1
            } else {
                i64::try_from(explicit_track_count)
                    .ok()
                    .and_then(|count| count.checked_add(1))
                    .and_then(|last_boundary| last_boundary.checked_add(i64::from(line)))
                    .ok_or_else(|| {
                        invalid_placement(node, property, "음수 line 계산이 정수 범위를 넘었습니다")
                    })?
            };
            Ok(Endpoint::Boundary(boundary))
        }
        GridPlacementInput::Span(span) => {
            let span = positive_span(node, property, span)?;
            let span = usize::try_from(span)
                .map_err(|_| invalid_placement(node, property, "span이 정수 범위를 넘었습니다"))?;
            Ok(Endpoint::Span(span))
        }
    }
}
