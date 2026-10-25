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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) track: Track,
    pub(super) reversed: bool,
}

pub(super) const SCENES: [Scene; 4] = [
    Scene {
        id: "direct-orthogonal-row-min-content-forward",
        track: Track::MinContent,
        reversed: false,
    },
    Scene {
        id: "direct-orthogonal-row-min-content-reversed",
        track: Track::MinContent,
        reversed: true,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-forward",
        track: Track::MaxContent,
        reversed: false,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-reversed",
        track: Track::MaxContent,
        reversed: true,
    },
];

impl Scene {
    pub(super) fn row_height(self) -> f32 {
        match self.track {
            Track::MinContent => 50.0,
            Track::MaxContent => 80.0,
        }
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
        format!(
            "c114-orthogonal-{}-content-{}-grid",
            self.track.name(),
            self.order_name()
        )
    }

    pub(super) fn item_id(self, kind: ItemKind) -> String {
        let suffix = match kind {
            ItemKind::Text => "text",
            ItemKind::Image => "image",
        };
        format!(
            "c114-orthogonal-{}-content-{}-{suffix}",
            self.track.name(),
            self.order_name()
        )
    }

    pub(super) fn order_name(self) -> &'static str {
        if self.reversed { "reversed" } else { "forward" }
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
