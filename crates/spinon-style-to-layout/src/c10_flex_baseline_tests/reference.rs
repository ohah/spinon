use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct ReferenceFile {
    pub(super) observations: Vec<Observation>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Observation {
    pub(super) viewport: Viewport,
    pub(super) cases: Vec<CapturedCase>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Viewport {
    pub(super) device_scale_factor: f32,
}

#[derive(Debug, Deserialize)]
pub(super) struct CapturedCase {
    pub(super) id: String,
    pub(super) nodes: Vec<CapturedNode>,
}

#[derive(Debug, Deserialize)]
pub(super) struct CapturedNode {
    pub(super) id: String,
    pub(super) input: InputNode,
    pub(super) properties: BTreeMap<String, String>,
    pub(super) rect: CapturedFrame,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct InputNode {
    pub(super) tag: String,
    pub(super) style: String,
    pub(super) parent_id: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub(super) struct CapturedFrame {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) width: f32,
    pub(super) height: f32,
}
