pub(super) const SOURCE: &str = r#"(() => {
const root = document.createElement('div');
root.setAttribute('id', 'c1133-fit-content-area-cap-grid');
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:200px;height:60px;margin:0;padding:0;border:0;gap:0;grid-template-columns:20px;grid-template-rows:60px;grid-template-areas:"fixed fit";grid-auto-columns:fit-content(40px);background-color:#101820');
document.appendChild(root);
const fixed = document.createElement('div');
fixed.setAttribute('id', 'c1133-fit-content-area-cap-fixed');
fixed.setAttribute('data-node', '');
fixed.setAttribute('style', 'box-sizing:border-box;width:20px;height:60px;margin:0;padding:0;border:0;background-color:#e5484d;grid-area:fixed');
root.appendChild(fixed);
const item = document.createElement('div');
item.setAttribute('id', 'c1133-fit-content-area-cap-item');
item.setAttribute('data-node', '');
item.setAttribute('style', 'box-sizing:border-box;height:auto;margin:0;padding:0;border:0;background-color:#30a46c;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space:normal;word-break:normal;grid-area:fit');
root.appendChild(item);
item.appendChild(document.createTextNode('AA AA AA AA'));
return { sceneId:'fit-content-area-cap', rootId:'c1133-fit-content-area-cap-grid', nodeIds:['c1133-fit-content-area-cap-grid','c1133-fit-content-area-cap-fixed','c1133-fit-content-area-cap-item'] };
})()"#;
