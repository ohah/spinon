use std::borrow::Cow;

#[cfg(feature = "c04-runtime-gpu-fixture-test")]
#[path = "c11_4/trace_report.rs"]
mod trace_report;

#[cfg(feature = "c04-runtime-gpu-fixture-test")]
pub(crate) use trace_report::grid_track_sizing_trace_report;

const C11_4_GRID_TRACK_SIZING_SCENES: [(&str, &str); 17] = [
    (
        "auto-fixed-width-item",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-auto-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:0;gap:0;grid-template-columns:auto;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'auto-fixed-width-item', rootId:'c114-auto-grid', nodeIds:['c114-auto-grid', 'c114-auto-item'] };
})()"#,
    ),
    (
        "minmax-auto-fixed-width-item",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-minmax-auto-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:0;gap:0;grid-template-columns:minmax(0,auto);grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-minmax-auto-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'minmax-auto-fixed-width-item', rootId:'c114-minmax-auto-grid', nodeIds:['c114-minmax-auto-grid', 'c114-minmax-auto-item'] };
})()"#,
    ),
    (
        "fit-content-below-item-size",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:0;gap:0;grid-template-columns:fit-content(12px);grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-fit-content-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:24px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'fit-content-below-item-size', rootId:'c114-fit-content-grid', nodeIds:['c114-fit-content-grid', 'c114-fit-content-item'] };
})()"#,
    ),
    (
        "spanning-auto-tracks",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-spanning-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:30px;margin:0;padding:0;border:0;column-gap:4px;row-gap:0;grid-template-columns:repeat(2,minmax(0,auto));grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-spanning-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:70px;height:10px;margin:0;padding:0;border:0;grid-column:1 / span 2;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'spanning-auto-tracks', rootId:'c114-spanning-grid', nodeIds:['c114-spanning-grid', 'c114-spanning-item'] };
})()"#,
    ),
    (
        "auto-fill-repeat",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-auto-fill-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:2px solid #d9e2f2;gap:0;grid-template-columns:repeat(auto-fill,minmax(24px,1fr));grid-template-rows:36px;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-fill-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:100%;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'auto-fill-repeat', rootId:'c114-auto-fill-grid', nodeIds:['c114-auto-fill-grid', 'c114-auto-fill-item'] };
})()"#,
    ),
    (
        "auto-fit-collapse",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-auto-fit-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:2px solid #d9e2f2;gap:0;grid-template-columns:repeat(auto-fit,minmax(24px,1fr));grid-template-rows:36px;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-fit-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:100%;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'auto-fit-collapse', rootId:'c114-auto-fit-grid', nodeIds:['c114-auto-fit-grid', 'c114-auto-fit-item'] };
})()"#,
    ),
    (
        "auto-fill-track-overflow",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-auto-fill-overflow-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:40px;margin:0;padding:0;border:2px solid #d9e2f2;gap:0;grid-template-columns:repeat(auto-fill,minmax(24px,1fr));grid-template-rows:36px;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-fill-overflow-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;width:120px;height:16px;margin:0;padding:0;border:0;background-color:#e5484d');
root.appendChild(item);
return { sceneId:'auto-fill-track-overflow', rootId:'c114-auto-fill-overflow-grid', nodeIds:['c114-auto-fill-overflow-grid', 'c114-auto-fill-overflow-item'] };
})()"#,
    ),
    (
        "min-content-text-normal",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-text-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:40px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:min-content;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-track-text-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XX XXXXXXX'));
root.appendChild(item);
return { sceneId:'min-content-text-normal', rootId:'c114-text-grid', nodeIds:['c114-text-grid', 'c114-track-text-item'] };
})()"#,
    ),
    (
        "auto-text-max-content",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-auto-text-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:40px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:auto;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-auto-text-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XX XXXXXXX'));
root.appendChild(item);
return { sceneId:'auto-text-max-content', rootId:'c114-auto-text-grid', nodeIds:['c114-auto-text-grid', 'c114-auto-text-item'] };
})()"#,
    ),
    (
        "max-content-text-normal",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-max-content-text-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:40px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:max-content;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-max-content-text-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XX XXXXXXX'));
root.appendChild(item);
return { sceneId:'max-content-text-normal', rootId:'c114-max-content-text-grid', nodeIds:['c114-max-content-text-grid', 'c114-max-content-text-item'] };
})()"#,
    ),
    (
        "fit-content-text-minimum",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-text-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:40px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:fit-content(40px);grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-fit-content-text-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XX XXXXXXX'));
root.appendChild(item);
return { sceneId:'fit-content-text-minimum', rootId:'c114-fit-content-text-grid', nodeIds:['c114-fit-content-text-grid', 'c114-fit-content-text-item'] };
})()"#,
    ),
    (
        "fit-content-calc-limit",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-fit-content-calc-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:100px;height:120px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:fit-content(calc(10% + 5px));grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-fit-content-calc-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:break-all;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XXXXXXXX'));
root.appendChild(item);
return { sceneId:'fit-content-calc-limit', rootId:'c114-fit-content-calc-grid', nodeIds:['c114-fit-content-calc-grid', 'c114-fit-content-calc-item'] };
})()"#,
    ),
    (
        "min-content-text-edge",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-text-edge-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:50px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:min-content;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-text-edge-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:content-box;margin:0;padding:0 4px;border-left:1px solid #e5484d;border-right:1px solid #e5484d;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XX XXXXXXX'));
root.appendChild(item);
return { sceneId:'min-content-text-edge', rootId:'c114-text-edge-grid', nodeIds:['c114-text-edge-grid', 'c114-text-edge-item'] };
})()"#,
    ),
    (
        "min-content-text-min-width",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-text-min-width-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:50px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:min-content;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-text-min-width-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;min-width:90px;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XX XXXXXXX'));
root.appendChild(item);
return { sceneId:'min-content-text-min-width', rootId:'c114-text-min-width-grid', nodeIds:['c114-text-min-width-grid', 'c114-text-min-width-item'] };
})()"#,
    ),
    (
        "min-content-text-max-width",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-text-max-width-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:50px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:min-content;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-text-max-width-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;max-width:40px;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XX XXXXXXX'));
root.appendChild(item);
return { sceneId:'min-content-text-max-width', rootId:'c114-text-max-width-grid', nodeIds:['c114-text-max-width-grid', 'c114-text-max-width-item'] };
})()"#,
    ),
    (
        "min-content-text-break-all",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-break-all-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:40px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:min-content;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-break-all-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:break-all;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XXXXXXXX'));
root.appendChild(item);
return { sceneId:'min-content-text-break-all', rootId:'c114-break-all-grid', nodeIds:['c114-break-all-grid', 'c114-break-all-item'] };
})()"#,
    ),
    (
        "min-content-text-nowrap",
        r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c114-nowrap-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:120px;height:40px;margin:0;padding:0;border:0;gap:0;align-items:start;grid-template-columns:min-content;grid-template-rows:auto;background-color:#101820');
document.appendChild(root);
const item = document.createElement('div');
item.setAttribute('id', 'c114-nowrap-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:nowrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d');
item.appendChild(document.createTextNode('XX XXXXXXX'));
root.appendChild(item);
return { sceneId:'min-content-text-nowrap', rootId:'c114-nowrap-grid', nodeIds:['c114-nowrap-grid', 'c114-nowrap-item'] };
})()"#,
    ),
];

#[derive(Clone, Copy)]
struct ImageScene {
    scene_id: &'static str,
    root_id: &'static str,
    item_id: &'static str,
    width: u32,
    height: u32,
    columns: &'static str,
    rows: &'static str,
    column_gap: u32,
    item_placement: &'static str,
    item_style: &'static str,
}

const C11_4_IMAGE_TRACK_SCENES: [ImageScene; 16] = [
    ImageScene {
        scene_id: "auto-image-definite",
        root_id: "c114-auto-image-grid",
        item_id: "c114-auto-image-item",
        width: 100,
        height: 40,
        columns: "auto",
        rows: "auto",
        column_gap: 0,
        item_placement: "",
        item_style: "",
    },
    ImageScene {
        scene_id: "minmax-auto-image",
        root_id: "c114-minmax-auto-image-grid",
        item_id: "c114-minmax-auto-image-item",
        width: 100,
        height: 40,
        columns: "minmax(0, auto)",
        rows: "auto",
        column_gap: 0,
        item_placement: "",
        item_style: "",
    },
    ImageScene {
        scene_id: "minmax-min-max-image",
        root_id: "c114-minmax-content-image-grid",
        item_id: "c114-minmax-content-image-item",
        width: 100,
        height: 40,
        columns: "minmax(min-content, max-content)",
        rows: "auto",
        column_gap: 0,
        item_placement: "",
        item_style: "",
    },
    ImageScene {
        scene_id: "fit-content-below-minimum",
        root_id: "c114-fit-content-12-image-grid",
        item_id: "c114-fit-content-12-image-item",
        width: 100,
        height: 40,
        columns: "fit-content(12px)",
        rows: "auto",
        column_gap: 0,
        item_placement: "",
        item_style: "",
    },
    ImageScene {
        scene_id: "fit-content-above-intrinsic",
        root_id: "c114-fit-content-40-image-grid",
        item_id: "c114-fit-content-40-image-item",
        width: 100,
        height: 40,
        columns: "fit-content(40px)",
        rows: "auto",
        column_gap: 0,
        item_placement: "",
        item_style: "",
    },
    ImageScene {
        scene_id: "fit-content-percent",
        root_id: "c114-fit-content-percent-image-grid",
        item_id: "c114-fit-content-percent-image-item",
        width: 100,
        height: 40,
        columns: "fit-content(50%)",
        rows: "auto",
        column_gap: 0,
        item_placement: "",
        item_style: "",
    },
    ImageScene {
        scene_id: "fixed-small-track-overflow",
        root_id: "c114-fixed-small-image-grid",
        item_id: "c114-fixed-small-image-item",
        width: 12,
        height: 40,
        columns: "minmax(0, 12px)",
        rows: "auto",
        column_gap: 0,
        item_placement: "",
        item_style: "",
    },
    ImageScene {
        scene_id: "spanning-min-content-items",
        root_id: "c114-spanning-image-grid",
        item_id: "c114-spanning-image-item",
        width: 100,
        height: 30,
        columns: "repeat(2, min-content)",
        rows: "auto",
        column_gap: 4,
        item_placement: ";grid-column:1 / span 2",
        item_style: "",
    },
    ImageScene {
        scene_id: "min-content-row-image",
        root_id: "c114-min-content-row-image-grid",
        item_id: "c114-min-content-row-image-item",
        width: 100,
        height: 80,
        columns: "100px",
        rows: "min-content",
        column_gap: 0,
        item_placement: "",
        item_style: "",
    },
    ImageScene {
        scene_id: "column-auto-natural-ratio",
        root_id: "c114-column-auto-natural-ratio-grid",
        item_id: "c114-column-auto-natural-ratio-item",
        width: 8,
        height: 20,
        columns: "minmax(auto, 1fr)",
        rows: "20px",
        column_gap: 0,
        item_placement: "",
        item_style: ";height:8px",
    },
    ImageScene {
        scene_id: "column-target-max-width-clamp",
        root_id: "c114-column-target-max-width-grid",
        item_id: "c114-column-target-max-width-item",
        width: 8,
        height: 20,
        columns: "minmax(auto, 1fr)",
        rows: "20px",
        column_gap: 0,
        item_placement: "",
        item_style: ";height:8px;aspect-ratio:3 / 2;max-width:9px",
    },
    ImageScene {
        scene_id: "row-target-max-height-clamp",
        root_id: "c114-row-target-max-height-grid",
        item_id: "c114-row-target-max-height-item",
        width: 20,
        height: 4,
        columns: "20px",
        rows: "minmax(auto, 1fr)",
        column_gap: 0,
        item_placement: "",
        item_style: ";width:12px;aspect-ratio:3 / 2;max-height:6px",
    },
    ImageScene {
        scene_id: "column-opposite-min-over-max",
        root_id: "c114-column-min-over-max-grid",
        item_id: "c114-column-min-over-max-item",
        width: 8,
        height: 20,
        columns: "minmax(auto, 1fr)",
        rows: "20px",
        column_gap: 0,
        item_placement: "",
        item_style: ";height:8px;aspect-ratio:3 / 2;min-height:12px;max-height:4px",
    },
    ImageScene {
        scene_id: "column-border-box-transfer",
        root_id: "c114-column-border-box-grid",
        item_id: "c114-column-border-box-item",
        width: 8,
        height: 20,
        columns: "minmax(auto, 1fr)",
        rows: "20px",
        column_gap: 0,
        item_placement: "",
        item_style: ";height:8px;aspect-ratio:3 / 2;padding:1px;border:1px solid",
    },
    ImageScene {
        scene_id: "column-content-box-transfer",
        root_id: "c114-column-content-box-grid",
        item_id: "c114-column-content-box-item",
        width: 8,
        height: 20,
        columns: "minmax(auto, 1fr)",
        rows: "20px",
        column_gap: 0,
        item_placement: "",
        item_style: ";box-sizing:content-box;height:8px;aspect-ratio:3 / 2;padding:1px;border:1px solid",
    },
    ImageScene {
        scene_id: "column-target-min-width-clamp",
        root_id: "c114-column-min-width-grid",
        item_id: "c114-column-min-width-item",
        width: 8,
        height: 20,
        columns: "minmax(auto, 1fr)",
        rows: "20px",
        column_gap: 0,
        item_placement: "",
        item_style: ";height:8px;aspect-ratio:3 / 2;min-width:18px",
    },
];

pub(crate) fn c11_4_grid_track_sizing_runtime_source(
    scene_id: &str,
) -> Result<Cow<'static, str>, String> {
    if let Some((_, source)) = C11_4_GRID_TRACK_SIZING_SCENES
        .iter()
        .find(|(known_id, _)| *known_id == scene_id)
    {
        return Ok(Cow::Borrowed(source));
    }
    if let Some(scene) = C11_4_IMAGE_TRACK_SCENES
        .iter()
        .find(|scene| scene.scene_id == scene_id)
    {
        return Ok(Cow::Owned(image_scene_source(*scene)));
    }
    Err(format!("알 수 없는 C11.4 scene ID: {scene_id}"))
}

fn image_scene_source(scene: ImageScene) -> String {
    format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:{}px;height:{}px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:{};grid-template-rows:{};column-gap:{}px;row-gap:0;background-color:#101820');
document.appendChild(root);
const item = document.createElement('img');
item.setAttribute('id', {:?});
item.setAttribute('data-node', '');
item.setAttribute('src', 'https://spinon.invalid/c14-1-png-basic.png');
item.setAttribute('alt', '');
item.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;background-color:#e5484d{}{}');
root.appendChild(item);
return {{ sceneId:{:?}, rootId:{:?}, nodeIds:[{:?}, {:?}] }};
}})()"#,
        scene.root_id,
        scene.width,
        scene.height,
        scene.columns,
        scene.rows,
        scene.column_gap,
        scene.item_id,
        scene.item_placement,
        scene.item_style,
        scene.scene_id,
        scene.root_id,
        scene.root_id,
        scene.item_id,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        C11_4_GRID_TRACK_SIZING_SCENES, C11_4_IMAGE_TRACK_SCENES,
        c11_4_grid_track_sizing_runtime_source,
    };

    #[test]
    fn exposes_only_bounded_intrinsic_track_sizing_scenes() {
        assert_eq!(C11_4_GRID_TRACK_SIZING_SCENES.len(), 17);
        assert_eq!(C11_4_IMAGE_TRACK_SCENES.len(), 16);
        assert_eq!(
            C11_4_GRID_TRACK_SIZING_SCENES
                .iter()
                .filter(|(_, source)| source.contains("document.createTextNode"))
                .count(),
            10
        );
        for (scene_id, source) in C11_4_GRID_TRACK_SIZING_SCENES {
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("document.createElement"));
            assert!(source.contains(&format!("sceneId:'{scene_id}'")));
            assert!(c11_4_grid_track_sizing_runtime_source(scene_id).is_ok());
        }
        for scene in C11_4_IMAGE_TRACK_SCENES {
            let source = c11_4_grid_track_sizing_runtime_source(scene.scene_id).unwrap();
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("document.createElement('img')"));
            assert!(source.contains("https://spinon.invalid/c14-1-png-basic.png"));
            assert!(source.contains(&format!("sceneId:{:?}", scene.scene_id)));
        }
        assert!(c11_4_grid_track_sizing_runtime_source("unknown;throw 1").is_err());
    }
}
