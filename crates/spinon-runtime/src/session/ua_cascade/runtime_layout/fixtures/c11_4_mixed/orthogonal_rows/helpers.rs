use spinon_core::NodeId;

use super::super::super::super::{RuntimeLayoutCompleted, RuntimeLayoutFailure, layout_failure};

pub(super) fn assert_frame(
    completed: &RuntimeLayoutCompleted,
    node: NodeId,
    expected: (f32, f32, f32, f32),
    label: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let mut frames = completed
        .frames
        .iter()
        .filter(|frame| frame.node_id == node.get());
    let frame = frames
        .next()
        .ok_or_else(|| fail("c114_orthogonal_frame_missing", node, label))?;
    if frames.next().is_some()
        || !frame_matches((frame.x, frame.y, frame.width, frame.height), expected)
    {
        return Err(fail(
            "c114_orthogonal_frame_mismatch",
            node,
            format!("{label}: expected {expected:?}, got {frame:?}"),
        ));
    }
    Ok(())
}

pub(super) fn frame_matches(actual: (f32, f32, f32, f32), expected: (f32, f32, f32, f32)) -> bool {
    [actual.0, actual.1, actual.2, actual.3]
        .into_iter()
        .zip([expected.0, expected.1, expected.2, expected.3])
        .all(|(actual, expected)| near(actual, expected))
}

pub(super) fn option_near(actual: Option<f32>, expected: f32) -> bool {
    actual.is_some_and(|actual| near(actual, expected))
}

pub(super) fn near(actual: f32, expected: f32) -> bool {
    actual.is_finite() && (actual - expected).abs() <= 1.0 / 64.0
}

pub(super) fn fail(
    code: &'static str,
    node: NodeId,
    detail: impl ToString,
) -> RuntimeLayoutFailure {
    layout_failure(code, Some(node.get()), Some(detail.to_string()))
}

pub(super) fn unsupported(node: NodeId, detail: &str) -> RuntimeLayoutFailure {
    fail("unsupported_c114_orthogonal_case", node, detail)
}
