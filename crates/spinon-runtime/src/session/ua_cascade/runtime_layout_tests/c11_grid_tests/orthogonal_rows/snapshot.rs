use super::super::*;

use super::scene::Scene;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const PNG_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const ROOT_STYLE_PREFIX: &str = "box-sizing:border-box;display:grid;width:74px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:0;direction:ltr;writing-mode:horizontal-tb;align-items:normal;align-content:start;justify-items:normal;justify-content:start;grid-template-columns:50px 24px;grid-template-rows:";
const TEXT_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:1 / span 1;grid-row:1;writing-mode:vertical-rl;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d;color:#fff";
const IMAGE_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;border:0;grid-column:2 / span 1;grid-row:1;background-color:#2864dc";

pub(super) fn orthogonal_snapshot(
    scene: Scene,
) -> (
    Arc<HostDocumentSnapshot>,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let text_leaf = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_440).unwrap(), document.document_revision());
    let root_style = format!(
        "{ROOT_STYLE_PREFIX}{};background-color:#101820",
        if scene.min_content {
            "min-content"
        } else {
            "max-content"
        }
    );
    create_element(&mut batch, root, "div", &root_id(scene));
    set_attribute(&mut batch, root, "data-node", "");
    set_attribute(&mut batch, root, "style", &root_style);
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });

    for kind in if scene.reversed {
        [false, true]
    } else {
        [true, false]
    } {
        if kind {
            create_element(&mut batch, text, "div", &text_id(scene));
            set_attribute(&mut batch, text, "data-node", "");
            set_attribute(&mut batch, text, "style", TEXT_STYLE);
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(root),
                node: text,
                before: None,
            });
            batch.push(DocumentOperation::CreateText {
                node: text_leaf,
                data: "XX XXXXX".into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(text),
                node: text_leaf,
                before: None,
            });
        } else {
            create_element(&mut batch, image, "img", &image_id(scene));
            set_attribute(&mut batch, image, "data-node", "");
            set_attribute(&mut batch, image, "src", PNG_URL);
            set_attribute(&mut batch, image, "alt", "");
            set_attribute(&mut batch, image, "style", IMAGE_STYLE);
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(root),
                node: image,
                before: None,
            });
        }
    }
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), root, text, image)
}

fn root_id(scene: Scene) -> String {
    format!("c114-orthogonal-{}-grid", scene.stem())
}

fn text_id(scene: Scene) -> String {
    format!("c114-orthogonal-{}-text", scene.stem())
}

fn image_id(scene: Scene) -> String {
    format!("c114-orthogonal-{}-image", scene.stem())
}

fn create_element(batch: &mut DocumentChangeBatch, node: HostNodeHandle, tag: &str, id: &str) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: tag.to_owned(),
    });
    set_attribute(batch, node, "id", id);
}

fn set_attribute(batch: &mut DocumentChangeBatch, node: HostNodeHandle, name: &str, value: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, name).unwrap(),
        value: value.to_owned().into(),
    });
}
