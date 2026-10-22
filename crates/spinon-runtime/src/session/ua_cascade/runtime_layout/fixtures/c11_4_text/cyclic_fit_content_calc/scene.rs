#[derive(Clone, Copy)]
pub(super) enum Dimension {
    Auto,
    MaxContent,
    Px(f32),
}

#[derive(Clone, Copy)]
pub(super) enum BoxSizing {
    ContentBox,
    BorderBox,
}

#[derive(Clone, Copy)]
pub(super) enum Track {
    Auto,
    Px(f32),
    FitContentCalc { percentage: f32, length_px: f32 },
    MinMaxCalc { percentage: f32, length_px: f32 },
    MinMaxCalcMinimum { percentage: f32, length_px: f32 },
}

#[derive(Clone, Copy)]
pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) width: Dimension,
    pub(super) height: Dimension,
    pub(super) box_sizing: BoxSizing,
    pub(super) border_width: f32,
    pub(super) columns: Track,
    pub(super) rows: Track,
    pub(super) text: &'static str,
}

const COLUMN_LIMIT: Track = Track::FitContentCalc {
    percentage: 0.5,
    length_px: 4.0,
};

pub(super) const SCENES: [Scene; 16] = [
    Scene {
        id: "cyclic-fit-content-calc-column",
        width: Dimension::MaxContent,
        height: Dimension::Auto,
        box_sizing: BoxSizing::BorderBox,
        border_width: 0.0,
        columns: COLUMN_LIMIT,
        rows: Track::Auto,
        text: "aa aa",
    },
    Scene {
        id: "cyclic-fit-content-calc-row",
        width: Dimension::Px(20.0),
        height: Dimension::MaxContent,
        box_sizing: BoxSizing::BorderBox,
        border_width: 0.0,
        columns: Track::Px(20.0),
        rows: Track::FitContentCalc {
            percentage: 0.5,
            length_px: 2.0,
        },
        text: "a a a a",
    },
    Scene {
        id: "definite-fit-content-calc-column",
        width: Dimension::Px(40.0),
        height: Dimension::Auto,
        box_sizing: BoxSizing::BorderBox,
        border_width: 0.0,
        columns: COLUMN_LIMIT,
        rows: Track::Auto,
        text: "aa aa",
    },
    Scene {
        id: "definite-fit-content-calc-small-limit",
        width: Dimension::Px(10.0),
        height: Dimension::Auto,
        box_sizing: BoxSizing::BorderBox,
        border_width: 0.0,
        columns: COLUMN_LIMIT,
        rows: Track::Auto,
        text: "aa aa",
    },
    Scene {
        id: "cyclic-minmax-calc-column",
        width: Dimension::MaxContent,
        height: Dimension::Auto,
        box_sizing: BoxSizing::BorderBox,
        border_width: 0.0,
        columns: Track::MinMaxCalc {
            percentage: 0.5,
            length_px: 4.0,
        },
        rows: Track::Auto,
        text: "aa aa",
    },
    Scene {
        id: "cyclic-minmax-calc-row",
        width: Dimension::Px(20.0),
        height: Dimension::MaxContent,
        box_sizing: BoxSizing::BorderBox,
        border_width: 0.0,
        columns: Track::Px(20.0),
        rows: Track::MinMaxCalc {
            percentage: 0.5,
            length_px: 2.0,
        },
        text: "a a a a",
    },
    Scene {
        id: "definite-minmax-calc-column",
        width: Dimension::Px(40.0),
        height: Dimension::Auto,
        box_sizing: BoxSizing::BorderBox,
        border_width: 0.0,
        columns: Track::MinMaxCalc {
            percentage: 0.5,
            length_px: 4.0,
        },
        rows: Track::Auto,
        text: "aa aa",
    },
    Scene {
        id: "definite-minmax-calc-small-limit",
        width: Dimension::Px(10.0),
        height: Dimension::Auto,
        box_sizing: BoxSizing::BorderBox,
        border_width: 0.0,
        columns: Track::MinMaxCalc {
            percentage: 0.5,
            length_px: 4.0,
        },
        rows: Track::Auto,
        text: "aa aa",
    },
    Scene {
        id: "cyclic-minmax-calc-min-column",
        width: Dimension::MaxContent,
        height: Dimension::Px(10.0),
        box_sizing: BoxSizing::ContentBox,
        border_width: 2.0,
        columns: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 4.0,
        },
        rows: Track::Px(10.0),
        text: "a",
    },
    Scene {
        id: "cyclic-minmax-calc-min-row",
        width: Dimension::Px(10.0),
        height: Dimension::MaxContent,
        box_sizing: BoxSizing::ContentBox,
        border_width: 2.0,
        columns: Track::Px(10.0),
        rows: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 2.0,
        },
        text: "a",
    },
    Scene {
        id: "definite-minmax-calc-min-column",
        width: Dimension::Px(10.0),
        height: Dimension::Px(10.0),
        box_sizing: BoxSizing::ContentBox,
        border_width: 2.0,
        columns: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 4.0,
        },
        rows: Track::Px(10.0),
        text: "a",
    },
    Scene {
        id: "definite-minmax-calc-min-row",
        width: Dimension::Px(10.0),
        height: Dimension::Px(10.0),
        box_sizing: BoxSizing::ContentBox,
        border_width: 2.0,
        columns: Track::Px(10.0),
        rows: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 2.0,
        },
        text: "a",
    },
    Scene {
        id: "cyclic-minmax-calc-min-both-axes",
        width: Dimension::MaxContent,
        height: Dimension::MaxContent,
        box_sizing: BoxSizing::ContentBox,
        border_width: 2.0,
        columns: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 4.0,
        },
        rows: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 2.0,
        },
        text: "aa aa",
    },
    Scene {
        id: "cyclic-minmax-calc-min-column-definite-row",
        width: Dimension::MaxContent,
        height: Dimension::Px(10.0),
        box_sizing: BoxSizing::ContentBox,
        border_width: 2.0,
        columns: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 4.0,
        },
        rows: Track::Px(10.0),
        text: "aa aa",
    },
    Scene {
        id: "definite-column-cyclic-minmax-calc-min-row",
        width: Dimension::Px(10.0),
        height: Dimension::MaxContent,
        box_sizing: BoxSizing::ContentBox,
        border_width: 2.0,
        columns: Track::Px(10.0),
        rows: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 2.0,
        },
        text: "aa aa",
    },
    Scene {
        id: "definite-minmax-calc-min-both-axes",
        width: Dimension::Px(10.0),
        height: Dimension::Px(10.0),
        box_sizing: BoxSizing::ContentBox,
        border_width: 2.0,
        columns: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 4.0,
        },
        rows: Track::MinMaxCalcMinimum {
            percentage: 2.0,
            length_px: 2.0,
        },
        text: "aa aa",
    },
];
