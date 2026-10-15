use std::{error::Error, fmt};

use super::super::RuntimeRenderError;

/// Session input을 거부하거나 underlying delta router가 실패한 이유입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeScrollGestureError {
    /// input sequence가 이전 입력보다 크지 않습니다.
    NonMonotonicInputSequence,
    /// gesture ID가 0이거나 앞선 ID보다 크지 않습니다.
    InvalidGestureId,
    /// 같은 pointer가 이미 active 상태입니다.
    GestureAlreadyActive,
    /// 입력을 처리할 active gesture가 없습니다.
    NoActiveGesture,
    /// gesture, pointer 또는 surface generation이 현재 상태와 다릅니다.
    GestureIdentityMismatch,
    /// 억제 중인 pointer ID가 다시 등록됐습니다.
    DuplicatePointer,
    /// active/suppressed 집합에 등록되지 않은 pointer입니다.
    UnknownPointer,
    /// touch-action이 지원 profile 밖입니다.
    UnsupportedTouchAction,
    /// 입력이 지정한 snapshot key가 현재 snapshot과 다릅니다.
    StaleExpectedSnapshot,
    /// 시작 좌표가 finite CSS px가 아닙니다.
    InvalidPoint,
    /// coordinate scale이 유한한 양수가 아닙니다.
    InvalidCoordinateScale,
    /// cumulative pan translation이 유한한 CSS px가 아닙니다.
    InvalidPanTranslation,
    /// 시작점에서 clip-aware hit target을 찾지 못했습니다.
    HitTestMiss,
    /// frozen target의 scroll owner geometry가 일관되지 않습니다.
    InvalidGeometry,
    /// active gesture에서 pan 시작이 이미 처리됐습니다.
    PanAlreadyStarted,
    /// snapshot delta router가 반환한 오류입니다.
    Render(RuntimeRenderError),
}

impl fmt::Display for RuntimeScrollGestureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonMonotonicInputSequence => {
                formatter.write_str("scroll gesture input sequence가 증가하지 않았습니다")
            }
            Self::InvalidGestureId => {
                formatter.write_str("scroll gesture ID는 0보다 크고 이전보다 커야 합니다")
            }
            Self::GestureAlreadyActive => {
                formatter.write_str("같은 pointer의 scroll gesture가 이미 active입니다")
            }
            Self::NoActiveGesture => formatter.write_str("active scroll gesture가 없습니다"),
            Self::GestureIdentityMismatch => {
                formatter.write_str("scroll gesture, pointer 또는 surface identity가 다릅니다")
            }
            Self::DuplicatePointer => formatter.write_str("이미 추적 중인 pointer ID입니다"),
            Self::UnknownPointer => formatter.write_str("추적 중인 pointer가 없습니다"),
            Self::UnsupportedTouchAction => {
                formatter.write_str("touch-action이 제한 gesture profile 밖입니다")
            }
            Self::StaleExpectedSnapshot => {
                formatter.write_str("gesture input이 지정한 snapshot revision이 현재와 다릅니다")
            }
            Self::InvalidPoint => {
                formatter.write_str("gesture 시작점은 유한한 viewport CSS px여야 합니다")
            }
            Self::InvalidCoordinateScale => {
                formatter.write_str("coordinate scale은 유한한 양수여야 합니다")
            }
            Self::InvalidPanTranslation => {
                formatter.write_str("pan translation은 유한한 CSS px여야 합니다")
            }
            Self::HitTestMiss => {
                formatter.write_str("gesture 시작점에서 hit target을 찾지 못했습니다")
            }
            Self::InvalidGeometry => {
                formatter.write_str("gesture hit target의 scroll geometry가 일관되지 않습니다")
            }
            Self::PanAlreadyStarted => {
                formatter.write_str("active gesture에서 pan 시작이 이미 처리되었습니다")
            }
            Self::Render(error) => write!(formatter, "scroll gesture delta 계산 실패: {error}"),
        }
    }
}

impl Error for RuntimeScrollGestureError {}
