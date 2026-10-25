use spinon_layout::LayoutGridTrackSizingContributionKind;

#[derive(Clone, Copy)]
pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) min_content: bool,
    pub(super) reversed: bool,
}

pub(super) const SCENES: [Scene; 4] = [
    Scene {
        id: "direct-orthogonal-row-min-content-forward",
        min_content: true,
        reversed: false,
    },
    Scene {
        id: "direct-orthogonal-row-min-content-reversed",
        min_content: true,
        reversed: true,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-forward",
        min_content: false,
        reversed: false,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-reversed",
        min_content: false,
        reversed: true,
    },
];

impl Scene {
    pub(super) fn stem(self) -> &'static str {
        if self.min_content {
            if self.reversed {
                "min-content-reversed"
            } else {
                "min-content-forward"
            }
        } else if self.reversed {
            "max-content-reversed"
        } else {
            "max-content-forward"
        }
    }

    pub(super) fn row_height(self) -> f32 {
        if self.min_content { 50.0 } else { 80.0 }
    }

    pub(super) fn row_contribution_kind(self) -> LayoutGridTrackSizingContributionKind {
        if self.min_content {
            LayoutGridTrackSizingContributionKind::MinContent
        } else {
            LayoutGridTrackSizingContributionKind::MaxContent
        }
    }
}
