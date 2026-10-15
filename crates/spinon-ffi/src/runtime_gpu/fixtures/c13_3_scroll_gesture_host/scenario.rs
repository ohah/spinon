use super::super::c13_3_scroll_gesture::RUNTIME_CSS_C13_3_STYLE_INVALIDATION_SOURCE;
use spinon_render::{RuntimeOverscrollBehavior, RuntimeTouchAction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum C13_3GestureFixtureScenario {
    AutoZero,
    AutoBoundary,
    ContainXAutoYBoundary,
    TouchActionNone,
    TouchActionPanX,
    TouchActionPanY,
    TouchActionTargetPanX,
    TouchActionIntersectionNone,
    StylePolicyInvalidation,
    GeometryInvalidation,
    PaintInvalidation,
}

impl TryFrom<i32> for C13_3GestureFixtureScenario {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AutoZero),
            1 => Ok(Self::AutoBoundary),
            2 => Ok(Self::ContainXAutoYBoundary),
            3 => Ok(Self::TouchActionNone),
            4 => Ok(Self::TouchActionPanX),
            5 => Ok(Self::TouchActionPanY),
            6 => Ok(Self::TouchActionTargetPanX),
            7 => Ok(Self::TouchActionIntersectionNone),
            8 => Ok(Self::StylePolicyInvalidation),
            9 => Ok(Self::GeometryInvalidation),
            10 => Ok(Self::PaintInvalidation),
            _ => Err(format!("알 수 없는 C13.3 gesture scenario id={value}")),
        }
    }
}

impl C13_3GestureFixtureScenario {
    pub(super) const fn script_prelude(self) -> &'static str {
        match self {
            Self::AutoZero => "globalThis.__spinonC133GesturePolicyScenario = 0;\n",
            Self::AutoBoundary => "globalThis.__spinonC133GesturePolicyScenario = 1;\n",
            Self::ContainXAutoYBoundary => "globalThis.__spinonC133GesturePolicyScenario = 2;\n",
            Self::TouchActionNone => "globalThis.__spinonC133GesturePolicyScenario = 3;\n",
            Self::TouchActionPanX => "globalThis.__spinonC133GesturePolicyScenario = 4;\n",
            Self::TouchActionPanY => "globalThis.__spinonC133GesturePolicyScenario = 5;\n",
            Self::TouchActionTargetPanX => "globalThis.__spinonC133GesturePolicyScenario = 6;\n",
            Self::TouchActionIntersectionNone => {
                "globalThis.__spinonC133GesturePolicyScenario = 7;\n"
            }
            Self::StylePolicyInvalidation => {
                "globalThis.__spinonC133GesturePolicyScenario = 4; globalThis.__spinonC133StyleMode = 8;\n"
            }
            Self::GeometryInvalidation => {
                "globalThis.__spinonC133GesturePolicyScenario = 0; globalThis.__spinonC133StyleMode = 9;\n"
            }
            Self::PaintInvalidation => {
                "globalThis.__spinonC133GesturePolicyScenario = 0; globalThis.__spinonC133StyleMode = 10;\n"
            }
        }
    }

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::AutoZero => "auto-zero",
            Self::AutoBoundary => "auto-boundary",
            Self::ContainXAutoYBoundary => "contain-x-auto-y-boundary",
            Self::TouchActionNone => "touch-action-none",
            Self::TouchActionPanX => "touch-action-pan-x",
            Self::TouchActionPanY => "touch-action-pan-y",
            Self::TouchActionTargetPanX => "touch-action-target-pan-x",
            Self::TouchActionIntersectionNone => "touch-action-intersection-none",
            Self::StylePolicyInvalidation => "style-policy-invalidation",
            Self::GeometryInvalidation => "geometry-invalidation",
            Self::PaintInvalidation => "paint-invalidation",
        }
    }

    pub(super) const fn policy_label(self) -> &'static str {
        match self {
            Self::AutoZero
            | Self::AutoBoundary
            | Self::TouchActionNone
            | Self::TouchActionPanX
            | Self::TouchActionPanY
            | Self::TouchActionTargetPanX
            | Self::TouchActionIntersectionNone
            | Self::StylePolicyInvalidation
            | Self::GeometryInvalidation
            | Self::PaintInvalidation => "auto/auto",
            Self::ContainXAutoYBoundary => "contain/auto",
        }
    }

    pub(super) const fn seed_inner_boundary(self) -> bool {
        matches!(
            self,
            Self::AutoBoundary
                | Self::ContainXAutoYBoundary
                | Self::StylePolicyInvalidation
                | Self::GeometryInvalidation
        )
    }

    pub(super) const fn inner_overscroll(
        self,
    ) -> (RuntimeOverscrollBehavior, RuntimeOverscrollBehavior) {
        match self {
            Self::AutoZero | Self::AutoBoundary => (
                RuntimeOverscrollBehavior::Auto,
                RuntimeOverscrollBehavior::Auto,
            ),
            Self::ContainXAutoYBoundary => (
                RuntimeOverscrollBehavior::Contain,
                RuntimeOverscrollBehavior::Auto,
            ),
            Self::TouchActionNone
            | Self::TouchActionPanX
            | Self::TouchActionPanY
            | Self::TouchActionTargetPanX
            | Self::TouchActionIntersectionNone
            | Self::StylePolicyInvalidation
            | Self::GeometryInvalidation
            | Self::PaintInvalidation => (
                RuntimeOverscrollBehavior::Auto,
                RuntimeOverscrollBehavior::Auto,
            ),
        }
    }

    pub(super) const fn inner_touch_action(self) -> &'static str {
        match self {
            Self::TouchActionNone => "none",
            Self::TouchActionPanX => "pan-x",
            Self::TouchActionIntersectionNone => "pan-y",
            Self::StylePolicyInvalidation => "pan-x",
            Self::TouchActionPanY => "pan-y",
            Self::AutoZero
            | Self::AutoBoundary
            | Self::ContainXAutoYBoundary
            | Self::TouchActionTargetPanX
            | Self::GeometryInvalidation
            | Self::PaintInvalidation => "auto",
        }
    }

    pub(super) const fn inner_content_touch_action(self) -> &'static str {
        match self {
            Self::TouchActionTargetPanX | Self::TouchActionIntersectionNone => "pan-x",
            Self::AutoZero
            | Self::AutoBoundary
            | Self::ContainXAutoYBoundary
            | Self::TouchActionNone
            | Self::TouchActionPanX
            | Self::TouchActionPanY
            | Self::StylePolicyInvalidation
            | Self::GeometryInvalidation
            | Self::PaintInvalidation => "auto",
        }
    }

    pub(super) const fn expected_touch_action(self) -> RuntimeTouchAction {
        match self {
            Self::TouchActionNone | Self::TouchActionIntersectionNone => RuntimeTouchAction::None,
            Self::TouchActionPanX => RuntimeTouchAction::PanX,
            Self::TouchActionPanY => RuntimeTouchAction::PanY,
            Self::TouchActionTargetPanX => RuntimeTouchAction::PanX,
            Self::StylePolicyInvalidation => RuntimeTouchAction::PanX,
            Self::AutoZero | Self::AutoBoundary | Self::ContainXAutoYBoundary => {
                RuntimeTouchAction::Auto
            }
            Self::GeometryInvalidation | Self::PaintInvalidation => RuntimeTouchAction::Auto,
        }
    }

    pub(super) const fn expected_touch_action_label(self) -> &'static str {
        match self.expected_touch_action() {
            RuntimeTouchAction::Auto => "auto",
            RuntimeTouchAction::None => "none",
            RuntimeTouchAction::PanX => "pan-x",
            RuntimeTouchAction::PanY => "pan-y",
            RuntimeTouchAction::Unsupported => "unsupported",
        }
    }

    pub(super) const fn style_invalidation_source(self) -> Option<&'static str> {
        match self {
            Self::StylePolicyInvalidation
            | Self::GeometryInvalidation
            | Self::PaintInvalidation => Some(RUNTIME_CSS_C13_3_STYLE_INVALIDATION_SOURCE),
            _ => None,
        }
    }

    pub(super) const fn style_invalidation_mode(self) -> Option<u8> {
        match self {
            Self::StylePolicyInvalidation => Some(8),
            Self::GeometryInvalidation => Some(9),
            Self::PaintInvalidation => Some(10),
            _ => None,
        }
    }
}
