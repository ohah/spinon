use spinon_core::NodeId;

use crate::{
    GridPlacementInput, LayoutError,
    grid_placement::{explicit_line, invalid_placement, positive_span},
};

#[derive(Clone, Copy)]
pub(super) struct AxisPlacement {
    pub(super) start: Option<usize>,
    pub(super) span: usize,
}

impl AxisPlacement {
    pub(super) fn is_definite(self) -> bool {
        self.start.is_some()
    }
}

#[derive(Clone, Copy)]
enum Endpoint {
    Auto,
    Line(i64),
    Span(usize),
}

pub(super) fn resolve_axis(
    node: NodeId,
    start_property: &'static str,
    end_property: &'static str,
    start: &GridPlacementInput,
    end: &GridPlacementInput,
    track_count: usize,
    allow_positive_implicit: bool,
) -> Result<AxisPlacement, LayoutError> {
    let mut start = endpoint(
        node,
        start_property,
        start,
        track_count,
        allow_positive_implicit,
    )?;
    let mut end = endpoint(
        node,
        end_property,
        end,
        track_count,
        allow_positive_implicit,
    )?;
    match (start, end) {
        (Endpoint::Line(start_line), Endpoint::Line(end_line)) if start_line > end_line => {
            start = Endpoint::Line(end_line);
            end = Endpoint::Line(start_line);
        }
        (Endpoint::Line(start_line), Endpoint::Line(end_line)) if start_line == end_line => {
            end = Endpoint::Auto;
        }
        (Endpoint::Span(_), Endpoint::Span(_)) => end = Endpoint::Auto,
        _ => {}
    }

    let definite = match (start, end) {
        (Endpoint::Line(start_line), Endpoint::Line(end_line)) => {
            let span = usize::try_from(end_line - start_line).map_err(|_| {
                invalid_placement(node, end_property, "line 사이 track 수가 유효하지 않습니다")
            })?;
            Some((start_line, span))
        }
        (Endpoint::Line(start_line), Endpoint::Span(span)) => Some((start_line, span)),
        (Endpoint::Span(span), Endpoint::Line(end_line)) => {
            let start_line = end_line.checked_sub(span as i64).ok_or_else(|| {
                invalid_placement(
                    node,
                    start_property,
                    "span 시작 line 계산이 범위를 넘었습니다",
                )
            })?;
            Some((start_line, span))
        }
        (Endpoint::Line(start_line), Endpoint::Auto) => Some((start_line, 1)),
        (Endpoint::Auto, Endpoint::Line(end_line)) => Some((end_line - 1, 1)),
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

    let (start_line, span) = definite.expect("every remaining pair has a definite line");
    let end_line = start_line.checked_add(span as i64).ok_or_else(|| {
        invalid_placement(
            node,
            end_property,
            "line과 span 계산이 정수 범위를 넘었습니다",
        )
    })?;
    let last_line = i64::try_from(track_count)
        .unwrap_or(i64::MAX - 1)
        .saturating_add(1);
    if start_line < 1 || (!allow_positive_implicit && end_line > last_line) {
        return Err(invalid_placement(
            node,
            start_property,
            "definite placement가 explicit track 범위를 벗어나 implicit track이 필요합니다",
        ));
    }
    let start = usize::try_from(start_line - 1).map_err(|_| {
        invalid_placement(
            node,
            start_property,
            "정규화한 line이 정수 범위를 넘었습니다",
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
    placement: &GridPlacementInput,
    track_count: usize,
    allow_positive_implicit: bool,
) -> Result<Endpoint, LayoutError> {
    match placement {
        GridPlacementInput::Auto => Ok(Endpoint::Auto),
        GridPlacementInput::Line(line) if allow_positive_implicit && *line > 0 => {
            Ok(Endpoint::Line(i64::from(*line)))
        }
        GridPlacementInput::Line(line) => {
            explicit_line(node, property, *line, track_count).map(Endpoint::Line)
        }
        GridPlacementInput::Span(span) => {
            let span = positive_span(node, property, *span)?;
            Ok(Endpoint::Span(span as usize))
        }
        GridPlacementInput::NamedLine { .. } | GridPlacementInput::SpanNamed { .. } => {
            Err(invalid_placement(
                node,
                property,
                "이름 있는 배치는 auto-repeat track count가 계산된 Taffy 경로에서만 지원됩니다",
            ))
        }
    }
}
