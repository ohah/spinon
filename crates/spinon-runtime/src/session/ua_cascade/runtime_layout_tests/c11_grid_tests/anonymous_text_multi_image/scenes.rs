#[derive(Clone, Copy)]
pub(super) enum Child {
    Text,
    Image {
        id: &'static str,
        height: u8,
        column: u8,
        span: u8,
        row: u8,
    },
}

pub(super) struct Scene {
    pub(super) id: &'static str,
    pub(super) children: &'static [Child],
    pub(super) columns: &'static [f32],
    pub(super) input_nodes: &'static [u64],
    pub(super) item_frames: &'static [(u64, f32, f32, f32, f32)],
}

const FORWARD_TWO: [Child; 3] = [
    Child::Text,
    Child::Image {
        id: "c114-image-a-forward",
        height: 16,
        column: 1,
        span: 2,
        row: 1,
    },
    Child::Image {
        id: "c114-image-b-forward",
        height: 32,
        column: 2,
        span: 2,
        row: 2,
    },
];
const REVERSED_TWO: [Child; 3] = [
    Child::Image {
        id: "c114-image-b-reversed",
        height: 32,
        column: 2,
        span: 2,
        row: 2,
    },
    Child::Text,
    Child::Image {
        id: "c114-image-a-reversed",
        height: 16,
        column: 1,
        span: 2,
        row: 1,
    },
];
const FORWARD_THREE: [Child; 4] = [
    Child::Text,
    Child::Image {
        id: "c114-image-a-three-forward",
        height: 16,
        column: 1,
        span: 2,
        row: 1,
    },
    Child::Image {
        id: "c114-image-b-three-forward",
        height: 32,
        column: 2,
        span: 2,
        row: 2,
    },
    Child::Image {
        id: "c114-image-c-three-forward",
        height: 48,
        column: 1,
        span: 3,
        row: 3,
    },
];
const REVERSED_THREE: [Child; 4] = [
    Child::Image {
        id: "c114-image-c-three-reversed",
        height: 48,
        column: 1,
        span: 3,
        row: 3,
    },
    Child::Image {
        id: "c114-image-b-three-reversed",
        height: 32,
        column: 2,
        span: 2,
        row: 2,
    },
    Child::Text,
    Child::Image {
        id: "c114-image-a-three-reversed",
        height: 16,
        column: 1,
        span: 2,
        row: 1,
    },
];

pub(super) const SCENES: [Scene; 4] = [
    Scene {
        id: "anonymous-text-two-images-forward",
        children: &FORWARD_TWO,
        columns: &[12.0, 28.0, 20.0, 0.0],
        input_nodes: &[2, 3, 4],
        item_frames: &[
            (2, 40.0, 0.0, 20.0, 10.0),
            (3, 0.0, 0.0, 24.0, 16.0),
            (4, 12.0, 60.0, 48.0, 32.0),
        ],
    },
    Scene {
        id: "two-images-anonymous-text-reversed",
        children: &REVERSED_TWO,
        columns: &[12.0, 28.0, 20.0, 0.0],
        input_nodes: &[2, 3, 4],
        item_frames: &[
            (2, 12.0, 60.0, 48.0, 32.0),
            (3, 40.0, 0.0, 20.0, 10.0),
            (4, 0.0, 0.0, 24.0, 16.0),
        ],
    },
    Scene {
        id: "anonymous-text-three-images-forward",
        children: &FORWARD_THREE,
        columns: &[18.0, 34.0, 20.0, 0.0],
        input_nodes: &[2, 3, 4, 5],
        item_frames: &[
            (2, 52.0, 0.0, 20.0, 10.0),
            (3, 0.0, 0.0, 24.0, 16.0),
            (4, 18.0, 60.0, 48.0, 32.0),
            (5, 0.0, 120.0, 72.0, 48.0),
        ],
    },
    Scene {
        id: "three-images-anonymous-text-reversed",
        children: &REVERSED_THREE,
        columns: &[18.0, 34.0, 20.0, 0.0],
        input_nodes: &[2, 3, 4, 5],
        item_frames: &[
            (2, 0.0, 120.0, 72.0, 48.0),
            (3, 18.0, 60.0, 48.0, 32.0),
            (4, 52.0, 0.0, 20.0, 10.0),
            (5, 0.0, 0.0, 24.0, 16.0),
        ],
    },
];
