use spinon_layout::LayoutGridTrackSizingContributionKind;
use spinon_style::ComputedCssGridTrackSize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Track {
    MinContent,
    MaxContent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ItemKind {
    Text,
    Image,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Item {
    pub(super) kind: ItemKind,
    pub(super) column: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) track: Track,
    pub(super) reversed: bool,
    content: &'static str,
    min_content: (f32, f32),
    max_content: (f32, f32),
    generalized_target: bool,
}

pub(super) const SCENES: [Scene; 14] = [
    Scene {
        id: "direct-orthogonal-row-min-content-forward",
        track: Track::MinContent,
        reversed: false,
        content: "XX XXXXX",
        min_content: (50.0, 20.0),
        max_content: (80.0, 10.0),
        generalized_target: false,
    },
    Scene {
        id: "direct-orthogonal-row-min-content-reversed",
        track: Track::MinContent,
        reversed: true,
        content: "XX XXXXX",
        min_content: (50.0, 20.0),
        max_content: (80.0, 10.0),
        generalized_target: false,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-forward",
        track: Track::MaxContent,
        reversed: false,
        content: "XX XXXXX",
        min_content: (50.0, 20.0),
        max_content: (80.0, 10.0),
        generalized_target: false,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-reversed",
        track: Track::MaxContent,
        reversed: true,
        content: "XX XXXXX",
        min_content: (50.0, 20.0),
        max_content: (80.0, 10.0),
        generalized_target: false,
    },
    target(
        "single-token-min-content",
        Track::MinContent,
        "ABC",
        (30.0, 10.0),
        (30.0, 10.0),
    ),
    target(
        "single-token-max-content",
        Track::MaxContent,
        "ABC",
        (30.0, 10.0),
        (30.0, 10.0),
    ),
    target(
        "two-words-min-content",
        Track::MinContent,
        "A BC",
        (20.0, 20.0),
        (40.0, 10.0),
    ),
    target(
        "two-words-max-content",
        Track::MaxContent,
        "A BC",
        (20.0, 20.0),
        (40.0, 10.0),
    ),
    target(
        "three-words-min-content",
        Track::MinContent,
        "AB CDE F",
        (30.0, 30.0),
        (80.0, 10.0),
    ),
    target(
        "three-words-max-content",
        Track::MaxContent,
        "AB CDE F",
        (30.0, 30.0),
        (80.0, 10.0),
    ),
    target(
        "collapsed-whitespace-min-content",
        Track::MinContent,
        "  A   9Z  ",
        (20.0, 20.0),
        (40.0, 10.0),
    ),
    target(
        "collapsed-whitespace-max-content",
        Track::MaxContent,
        "  A   9Z  ",
        (20.0, 20.0),
        (40.0, 10.0),
    ),
    target(
        "spaces-only-min-content",
        Track::MinContent,
        "   ",
        (0.0, 0.0),
        (0.0, 0.0),
    ),
    target(
        "spaces-only-max-content",
        Track::MaxContent,
        "   ",
        (0.0, 0.0),
        (0.0, 0.0),
    ),
];

const fn target(
    id: &'static str,
    track: Track,
    content: &'static str,
    min_content: (f32, f32),
    max_content: (f32, f32),
) -> Scene {
    Scene {
        id,
        track,
        reversed: false,
        content,
        min_content,
        max_content,
        generalized_target: true,
    }
}

impl Scene {
    pub(super) fn content(self) -> &'static str {
        self.content
    }

    pub(super) fn min_content(self) -> (f32, f32) {
        self.min_content
    }

    pub(super) fn max_content(self) -> (f32, f32) {
        self.max_content
    }

    pub(super) fn selected_inline_contribution(self) -> f32 {
        match self.track {
            Track::MinContent => self.min_content.0,
            Track::MaxContent => self.max_content.0,
        }
    }

    pub(super) fn row_height(self) -> f32 {
        self.selected_inline_contribution().max(16.0)
    }

    pub(super) fn items(self) -> [Item; 2] {
        let text = Item {
            kind: ItemKind::Text,
            column: 1,
        };
        let image = Item {
            kind: ItemKind::Image,
            column: 2,
        };
        if self.reversed {
            [image, text]
        } else {
            [text, image]
        }
    }

    pub(super) fn root_id(self) -> String {
        if self.generalized_target {
            format!("c114-orthogonal-text-{}-grid", self.id)
        } else {
            format!(
                "c114-orthogonal-{}-content-{}-grid",
                self.track.name(),
                self.order_name()
            )
        }
    }

    pub(super) fn item_id(self, kind: ItemKind) -> String {
        let suffix = match kind {
            ItemKind::Text => "text",
            ItemKind::Image => "image",
        };
        if self.generalized_target {
            format!("c114-orthogonal-text-{}-{suffix}", self.id)
        } else {
            format!(
                "c114-orthogonal-{}-content-{}-{suffix}",
                self.track.name(),
                self.order_name()
            )
        }
    }

    pub(super) fn order_name(self) -> &'static str {
        if self.reversed { "reversed" } else { "forward" }
    }

    pub(super) fn line_break_value(self) -> &'static str {
        if self.generalized_target {
            "normal"
        } else {
            "auto"
        }
    }
}

impl Track {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::MinContent => "min",
            Self::MaxContent => "max",
        }
    }

    pub(super) fn style_value(self) -> ComputedCssGridTrackSize {
        match self {
            Self::MinContent => ComputedCssGridTrackSize::MinContent,
            Self::MaxContent => ComputedCssGridTrackSize::MaxContent,
        }
    }

    pub(super) fn contribution_kind(self) -> LayoutGridTrackSizingContributionKind {
        match self {
            Self::MinContent => LayoutGridTrackSizingContributionKind::MinContent,
            Self::MaxContent => LayoutGridTrackSizingContributionKind::MaxContent,
        }
    }
}

pub(super) fn scene_for_root_id(root_id: &str) -> Option<Scene> {
    SCENES
        .iter()
        .copied()
        .find(|scene| scene.root_id() == root_id)
}
