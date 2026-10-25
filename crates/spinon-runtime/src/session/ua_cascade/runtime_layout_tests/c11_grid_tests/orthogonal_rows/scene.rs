use spinon_layout::LayoutGridTrackSizingContributionKind;

#[derive(Clone, Copy)]
pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) min_content: bool,
    pub(super) reversed: bool,
    content: &'static str,
    min_logical: (f32, f32),
    max_logical: (f32, f32),
    generalized_target: bool,
}

pub(super) const SCENES: [Scene; 14] = [
    Scene {
        id: "direct-orthogonal-row-min-content-forward",
        min_content: true,
        reversed: false,
        content: "XX XXXXX",
        min_logical: (50.0, 20.0),
        max_logical: (80.0, 10.0),
        generalized_target: false,
    },
    Scene {
        id: "direct-orthogonal-row-min-content-reversed",
        min_content: true,
        reversed: true,
        content: "XX XXXXX",
        min_logical: (50.0, 20.0),
        max_logical: (80.0, 10.0),
        generalized_target: false,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-forward",
        min_content: false,
        reversed: false,
        content: "XX XXXXX",
        min_logical: (50.0, 20.0),
        max_logical: (80.0, 10.0),
        generalized_target: false,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-reversed",
        min_content: false,
        reversed: true,
        content: "XX XXXXX",
        min_logical: (50.0, 20.0),
        max_logical: (80.0, 10.0),
        generalized_target: false,
    },
    target(
        "single-token-min-content",
        true,
        "ABC",
        (30.0, 10.0),
        (30.0, 10.0),
    ),
    target(
        "single-token-max-content",
        false,
        "ABC",
        (30.0, 10.0),
        (30.0, 10.0),
    ),
    target(
        "two-words-min-content",
        true,
        "A BC",
        (20.0, 20.0),
        (40.0, 10.0),
    ),
    target(
        "two-words-max-content",
        false,
        "A BC",
        (20.0, 20.0),
        (40.0, 10.0),
    ),
    target(
        "three-words-min-content",
        true,
        "AB CDE F",
        (30.0, 30.0),
        (80.0, 10.0),
    ),
    target(
        "three-words-max-content",
        false,
        "AB CDE F",
        (30.0, 30.0),
        (80.0, 10.0),
    ),
    target(
        "collapsed-whitespace-min-content",
        true,
        "  A   9Z  ",
        (20.0, 20.0),
        (40.0, 10.0),
    ),
    target(
        "collapsed-whitespace-max-content",
        false,
        "  A   9Z  ",
        (20.0, 20.0),
        (40.0, 10.0),
    ),
    target(
        "spaces-only-min-content",
        true,
        "   ",
        (0.0, 0.0),
        (0.0, 0.0),
    ),
    target(
        "spaces-only-max-content",
        false,
        "   ",
        (0.0, 0.0),
        (0.0, 0.0),
    ),
];

const fn target(
    id: &'static str,
    min_content: bool,
    content: &'static str,
    min_logical: (f32, f32),
    max_logical: (f32, f32),
) -> Scene {
    Scene {
        id,
        min_content,
        reversed: false,
        content,
        min_logical,
        max_logical,
        generalized_target: true,
    }
}

impl Scene {
    pub(super) fn is_generalized_target(self) -> bool {
        self.generalized_target
    }

    pub(super) fn content(self) -> &'static str {
        self.content
    }

    pub(super) fn min_logical(self) -> (f32, f32) {
        self.min_logical
    }

    pub(super) fn max_logical(self) -> (f32, f32) {
        self.max_logical
    }

    pub(super) fn inline_contribution(self, min_content: bool) -> f32 {
        if min_content {
            self.min_logical.0
        } else {
            self.max_logical.0
        }
    }

    pub(super) fn row_height(self) -> f32 {
        self.inline_contribution(self.min_content).max(16.0)
    }

    pub(super) fn stem(self) -> String {
        if self.generalized_target {
            self.id.to_owned()
        } else if self.min_content {
            if self.reversed {
                "min-content-reversed".to_owned()
            } else {
                "min-content-forward".to_owned()
            }
        } else if self.reversed {
            "max-content-reversed".to_owned()
        } else {
            "max-content-forward".to_owned()
        }
    }

    pub(super) fn row_contribution_kind(self) -> LayoutGridTrackSizingContributionKind {
        if self.min_content {
            LayoutGridTrackSizingContributionKind::MinContent
        } else {
            LayoutGridTrackSizingContributionKind::MaxContent
        }
    }

    pub(super) fn line_break_value(self) -> &'static str {
        if self.generalized_target {
            "normal"
        } else {
            "auto"
        }
    }
}
