use super::{DocumentBatchOperation, HostDocumentBridge, MAX_BATCH_OPERATIONS};
use crate::root_html::{ParsedNodeKind, ParsedRootHtml};

impl HostDocumentBridge {
    #[cfg(test)]
    pub(crate) fn new_with_root_html(source: &str) -> Result<Self, String> {
        Self::new_with_parsed_root_html(ParsedRootHtml::parse(source)?)
    }

    pub(crate) fn new_with_parsed_root_html(parsed: ParsedRootHtml) -> Result<Self, String> {
        let mut bridge = Self::new()?;
        let mut operations = Vec::with_capacity(MAX_BATCH_OPERATIONS);
        let mut pending = parsed
            .root_children()
            .iter()
            .rev()
            .map(|node| (*node, 0_i32))
            .collect::<Vec<_>>();

        while let Some((node_index, parent_id)) = pending.pop() {
            let node = parsed
                .nodes
                .get(node_index)
                .ok_or_else(|| "HTML tree에서 node를 찾지 못했습니다".to_owned())?;
            match &node.kind {
                ParsedNodeKind::Element {
                    namespace,
                    local_name,
                    attributes,
                } => {
                    let id = bridge.next_external_id()?;
                    push_operation(
                        &mut bridge,
                        &mut operations,
                        DocumentBatchOperation::CreateElement {
                            id,
                            namespace: namespace.clone(),
                            name: local_name.clone(),
                        },
                    )?;
                    for (name, value) in attributes {
                        push_operation(
                            &mut bridge,
                            &mut operations,
                            DocumentBatchOperation::SetAttribute {
                                node: id,
                                name: name.clone(),
                                value: value.encode_utf16().collect(),
                            },
                        )?;
                    }
                    push_operation(
                        &mut bridge,
                        &mut operations,
                        DocumentBatchOperation::Append {
                            parent: parent_id,
                            node: id,
                        },
                    )?;
                    pending.extend(node.children.iter().rev().map(|child| (*child, id)));
                }
                ParsedNodeKind::Text(text) => {
                    let id = bridge.next_external_id()?;
                    push_operation(
                        &mut bridge,
                        &mut operations,
                        DocumentBatchOperation::CreateText {
                            id,
                            data: text.encode_utf16().collect(),
                        },
                    )?;
                    push_operation(
                        &mut bridge,
                        &mut operations,
                        DocumentBatchOperation::Append {
                            parent: parent_id,
                            node: id,
                        },
                    )?;
                }
                ParsedNodeKind::Document | ParsedNodeKind::Ignored => {
                    pending.extend(node.children.iter().rev().map(|child| (*child, parent_id)));
                }
            }
        }
        flush(&mut bridge, &mut operations)?;
        Ok(bridge)
    }
}

fn push_operation(
    bridge: &mut HostDocumentBridge,
    operations: &mut Vec<DocumentBatchOperation>,
    operation: DocumentBatchOperation,
) -> Result<(), String> {
    operations.push(operation);
    if operations.len() == MAX_BATCH_OPERATIONS {
        flush(bridge, operations)?;
    }
    Ok(())
}

fn flush(
    bridge: &mut HostDocumentBridge,
    operations: &mut Vec<DocumentBatchOperation>,
) -> Result<(), String> {
    if !operations.is_empty() {
        bridge.commit(operations)?;
        operations.clear();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::HostDocumentBridge;
    use spinon_core::{AttributeName, HostNodeKind, HostParent};

    fn element_name(bridge: &HostDocumentBridge, handle: spinon_core::HostNodeHandle) -> String {
        match bridge.document.node(handle).unwrap().kind() {
            HostNodeKind::Element(element) => element.local_name().to_owned(),
            HostNodeKind::Text(_) => panic!("expected element"),
        }
    }

    fn child_named(
        bridge: &HostDocumentBridge,
        parent: spinon_core::HostNodeHandle,
        name: &str,
    ) -> spinon_core::HostNodeHandle {
        bridge
            .document
            .children(parent)
            .unwrap()
            .find(|child| element_name(bridge, *child) == name)
            .unwrap_or_else(|| panic!("missing {name} child"))
    }

    #[test]
    fn imports_complete_html_document_tree_and_head_metadata_without_render_projection() {
        let bridge = HostDocumentBridge::new_with_root_html(
            "<!doctype html><html lang='ko'><head><meta name='color-scheme' content='dark'></head><body><main id='app'>화면</main></body></html>",
        )
        .unwrap();
        let html = bridge.document.root_children().next().unwrap();
        let head = child_named(&bridge, html, "head");
        let body = child_named(&bridge, html, "body");
        let main = child_named(&bridge, body, "main");
        let lang = AttributeName::new(None, "lang").unwrap();
        let id = AttributeName::new(None, "id").unwrap();

        assert_eq!(element_name(&bridge, html), "html");
        let HostNodeKind::Element(html_element) = bridge.document.node(html).unwrap().kind() else {
            panic!("documentElement is not an element");
        };
        assert_eq!(html_element.namespace(), "http://www.w3.org/1999/xhtml");
        assert_eq!(
            html_element.attribute(&lang).unwrap().to_string_lossy(),
            "ko"
        );
        assert_eq!(bridge.document.parent(head), Some(HostParent::Node(html)));
        assert_eq!(bridge.document.parent(body), Some(HostParent::Node(html)));
        let HostNodeKind::Element(main_element) = bridge.document.node(main).unwrap().kind() else {
            panic!("main is not an element");
        };
        assert_eq!(
            main_element.attribute(&id).unwrap().to_string_lossy(),
            "app"
        );
        assert_eq!(
            bridge
                .document
                .text_content(main)
                .unwrap()
                .to_string_lossy(),
            "화면"
        );
    }

    #[test]
    fn rejects_html_parser_profiles_that_cannot_be_mirrored() {
        assert!(
            HostDocumentBridge::new_with_root_html("<html><body>quirks</body></html>").is_err()
        );
        assert!(
            HostDocumentBridge::new_with_root_html("<html lang='ko' xml:lang='ko'></html>")
                .is_err()
        );
    }

    #[test]
    fn template_contents_do_not_become_visible_host_document_children() {
        let bridge = HostDocumentBridge::new_with_root_html(
            "<!doctype html><html><head></head><body><template><span>inert</span></template><main>visible</main></body></html>",
        )
        .unwrap();
        let html = bridge.document.root_children().next().unwrap();
        let body = child_named(&bridge, html, "body");
        let children = bridge.document.children(body).unwrap().collect::<Vec<_>>();

        assert_eq!(children.len(), 2);
        assert_eq!(element_name(&bridge, children[0]), "template");
        assert_eq!(bridge.document.children(children[0]).unwrap().count(), 0);
        assert_eq!(element_name(&bridge, children[1]), "main");
        assert_eq!(
            bridge
                .document
                .text_content(children[1])
                .unwrap()
                .to_string_lossy(),
            "visible"
        );
    }
}
