pub(super) const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub(super) const A_PNG: &[u8] =
    include_bytes!("../../../../../../tests/fixtures/resources/c18-4-async-a-slow.png");
pub(super) const B_PNG: &[u8] =
    include_bytes!("../../../../../../tests/fixtures/resources/c18-4-async-b-fast.png");
pub(super) const C_PNG: &[u8] =
    include_bytes!("../../../../../../tests/fixtures/resources/c18-4-async-c-slow.png");
pub(super) const D_PNG: &[u8] =
    include_bytes!("../../../../../../tests/fixtures/resources/c18-4-async-d-fast.png");
pub(super) const BROKEN_PNG: &[u8] = b"not an image";
pub(super) const A_SRC: &str = "spinon-fixture:c18-4/a-slow";
pub(super) const B_SRC: &str = "spinon-fixture:c18-4/b-fast";
pub(super) const C_SRC: &str = "spinon-fixture:c18-4/c-slow";
pub(super) const D_SRC: &str = "spinon-fixture:c18-4/d-fast";
pub(super) const BROKEN_SRC: &str = "spinon-fixture:c18-4/broken";

pub(super) const EXPECTED_IMAGES: [ExpectedImage; 5] = [
    ExpectedImage::new("current-b", 8.0, 8.0, B_SRC, Some(B_PNG)),
    ExpectedImage::new("pending-c", 64.0, 8.0, C_SRC, None),
    ExpectedImage::new("c-success", 120.0, 8.0, C_SRC, Some(C_PNG)),
    ExpectedImage::new("decode-failure", 176.0, 8.0, BROKEN_SRC, Some(BROKEN_PNG)),
    ExpectedImage::new("recovery-d", 8.0, 48.0, D_SRC, Some(D_PNG)),
];

pub(super) const EXPECTED_SAMPLES: [ExpectedSample; 6] = [
    ExpectedSample::new("root-background", 2, 2, [17, 34, 51, 255]),
    ExpectedSample::new("current-b", 9, 9, [24, 64, 240, 255]),
    ExpectedSample::new("pending-c-background", 65, 9, [210, 10, 180, 255]),
    ExpectedSample::new("c-success", 121, 9, [32, 220, 80, 255]),
    ExpectedSample::new("decode-failure-background", 177, 9, [210, 10, 180, 255]),
    ExpectedSample::new("recovery-d", 9, 49, [200, 40, 220, 255]),
];

#[derive(Clone, Copy)]
pub(super) struct ExpectedImage {
    pub(super) id: &'static str,
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) src: &'static str,
    pub(super) bytes: Option<&'static [u8]>,
}

impl ExpectedImage {
    pub(super) const fn new(
        id: &'static str,
        x: f32,
        y: f32,
        src: &'static str,
        bytes: Option<&'static [u8]>,
    ) -> Self {
        Self {
            id,
            x,
            y,
            src,
            bytes,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct ExpectedSample {
    pub(super) id: &'static str,
    pub(super) x: u32,
    pub(super) y: u32,
    pub(super) rgba: [u8; 4],
}

impl ExpectedSample {
    pub(super) const fn new(id: &'static str, x: u32, y: u32, rgba: [u8; 4]) -> Self {
        Self { id, x, y, rgba }
    }
}
