use spinon_style::{CssMediaEnvironment, CssViewport};

pub(super) const HTML: &str = "http://www.w3.org/1999/xhtml";
pub(super) const SVG: &str = "http://www.w3.org/2000/svg";
pub(super) const VIEWPORT: CssViewport = CssViewport {
    width_css_px: 1280.0,
    height_css_px: 633.0,
    device_scale_factor: 1.0,
    environment_revision: spinon_core::EnvironmentRevision::INITIAL,
    media_environment: CssMediaEnvironment::DESKTOP,
};

pub(super) const POSITIVE_STYLESHEET: &str = r#"
.scene {
  display: grid;
  width: 100px;
  grid-template-columns: min-content max-content;
  grid-template-rows: auto;
  grid-auto-flow: row;
  column-gap: 10px;
  align-items: start;
  font-family: Ahem;
  font-size: 10px;
  font-weight: 400;
  line-height: 10px;
}
.item {
  display: block;
  position: static;
  font-family: inherit;
  font-size: inherit;
  font-weight: inherit;
  font-style: normal;
  line-height: inherit;
  white-space-collapse: collapse;
  text-wrap-mode: wrap;
  word-break: normal;
  overflow-wrap: normal;
}
.nowrap .item { text-wrap-mode: nowrap; }
.break-all .item,
.item.break-all { word-break: break-all; }
"#;

#[derive(Clone, Copy)]
pub(super) struct ItemExpectation {
    pub(super) id: &'static str,
    pub(super) text: &'static str,
    pub(super) class: &'static str,
    pub(super) x: f32,
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) min_width: f32,
    pub(super) max_width: f32,
    pub(super) min_height: f32,
    pub(super) max_height: f32,
    pub(super) line_advances: &'static [f32],
    pub(super) word_break: &'static str,
}

#[derive(Clone, Copy)]
pub(super) struct SceneExpectation {
    pub(super) label: &'static str,
    pub(super) root_id: &'static str,
    pub(super) root_class: &'static str,
    pub(super) root_height: f32,
    pub(super) items: [ItemExpectation; 2],
}

pub(super) const NORMAL_LINES_FIRST: &[f32] = &[10.0, 20.0];
pub(super) const ONE_LINE_40: &[f32] = &[40.0];
pub(super) const FOUR_LINES_10: &[f32] = &[10.0, 10.0, 10.0, 10.0];
pub(super) const A_BCD_LINES: &[f32] = &[10.0, 30.0];
pub(super) const ONE_LINE_50: &[f32] = &[50.0];

pub(super) const SCENES: [SceneExpectation; 6] = [
    SceneExpectation {
        label: "normal",
        root_id: "generic-root-normal",
        root_class: "scene",
        root_height: 20.0,
        items: [
            ItemExpectation {
                id: "generic-normal-a",
                text: "A BC",
                class: "item",
                x: 0.0,
                width: 20.0,
                height: 20.0,
                min_width: 20.0,
                max_width: 40.0,
                min_height: 20.0,
                max_height: 10.0,
                line_advances: NORMAL_LINES_FIRST,
                word_break: "normal",
            },
            ItemExpectation {
                id: "generic-normal-b",
                text: "A BC",
                class: "item",
                x: 30.0,
                width: 40.0,
                height: 10.0,
                min_width: 20.0,
                max_width: 40.0,
                min_height: 20.0,
                max_height: 10.0,
                line_advances: ONE_LINE_40,
                word_break: "normal",
            },
        ],
    },
    SceneExpectation {
        label: "nowrap",
        root_id: "generic-root-nowrap",
        root_class: "scene nowrap",
        root_height: 10.0,
        items: [
            ItemExpectation {
                id: "generic-nowrap-a",
                text: "A BC",
                class: "item",
                x: 0.0,
                width: 40.0,
                height: 10.0,
                min_width: 40.0,
                max_width: 40.0,
                min_height: 10.0,
                max_height: 10.0,
                line_advances: ONE_LINE_40,
                word_break: "normal",
            },
            ItemExpectation {
                id: "generic-nowrap-b",
                text: "A BC",
                class: "item",
                x: 50.0,
                width: 40.0,
                height: 10.0,
                min_width: 40.0,
                max_width: 40.0,
                min_height: 10.0,
                max_height: 10.0,
                line_advances: ONE_LINE_40,
                word_break: "normal",
            },
        ],
    },
    SceneExpectation {
        label: "break-all",
        root_id: "generic-root-break-all",
        root_class: "scene break-all",
        root_height: 40.0,
        items: [
            ItemExpectation {
                id: "generic-break-a",
                text: "ABCD",
                class: "item",
                x: 0.0,
                width: 10.0,
                height: 40.0,
                min_width: 10.0,
                max_width: 40.0,
                min_height: 40.0,
                max_height: 10.0,
                line_advances: FOUR_LINES_10,
                word_break: "break-all",
            },
            ItemExpectation {
                id: "generic-break-b",
                text: "ABCD",
                class: "item",
                x: 20.0,
                width: 40.0,
                height: 10.0,
                min_width: 10.0,
                max_width: 40.0,
                min_height: 40.0,
                max_height: 10.0,
                line_advances: ONE_LINE_40,
                word_break: "break-all",
            },
        ],
    },
    SceneExpectation {
        label: "grid-9x",
        root_id: "grid-9x",
        root_class: "scene",
        root_height: 20.0,
        items: [
            ItemExpectation {
                id: "item-zeta",
                text: "A BCD",
                class: "item",
                x: 0.0,
                width: 30.0,
                height: 20.0,
                min_width: 30.0,
                max_width: 50.0,
                min_height: 20.0,
                max_height: 10.0,
                line_advances: A_BCD_LINES,
                word_break: "normal",
            },
            ItemExpectation {
                id: "item-3",
                text: "A BCD",
                class: "item",
                x: 40.0,
                width: 50.0,
                height: 10.0,
                min_width: 30.0,
                max_width: 50.0,
                min_height: 20.0,
                max_height: 10.0,
                line_advances: ONE_LINE_50,
                word_break: "normal",
            },
        ],
    },
    SceneExpectation {
        label: "grid-ws",
        root_id: "generic-root-whitespace",
        root_class: "scene",
        root_height: 20.0,
        items: [
            ItemExpectation {
                id: "generic-ws-a",
                text: " A  BC ",
                class: "item",
                x: 0.0,
                width: 20.0,
                height: 20.0,
                min_width: 20.0,
                max_width: 40.0,
                min_height: 20.0,
                max_height: 10.0,
                line_advances: NORMAL_LINES_FIRST,
                word_break: "normal",
            },
            ItemExpectation {
                id: "generic-ws-b",
                text: " A  BC ",
                class: "item",
                x: 30.0,
                width: 40.0,
                height: 10.0,
                min_width: 20.0,
                max_width: 40.0,
                min_height: 20.0,
                max_height: 10.0,
                line_advances: ONE_LINE_40,
                word_break: "normal",
            },
        ],
    },
    SceneExpectation {
        label: "grid-reorder",
        root_id: "grid-reorder",
        root_class: "scene",
        root_height: 40.0,
        items: [
            ItemExpectation {
                id: "item-a",
                text: "ABCD",
                class: "item break-all",
                x: 0.0,
                width: 10.0,
                height: 40.0,
                min_width: 10.0,
                max_width: 40.0,
                min_height: 40.0,
                max_height: 10.0,
                line_advances: FOUR_LINES_10,
                word_break: "break-all",
            },
            ItemExpectation {
                id: "item-z",
                text: "A BC",
                class: "item",
                x: 20.0,
                width: 40.0,
                height: 10.0,
                min_width: 20.0,
                max_width: 40.0,
                min_height: 20.0,
                max_height: 10.0,
                line_advances: ONE_LINE_40,
                word_break: "normal",
            },
        ],
    },
];
