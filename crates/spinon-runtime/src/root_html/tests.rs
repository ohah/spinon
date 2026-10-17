use super::{ParsedNodeKind, ParsedRootHtml};

fn local_name(document: &ParsedRootHtml, index: usize) -> &str {
    match &document.nodes[index].kind {
        ParsedNodeKind::Element { local_name, .. } => local_name,
        _ => panic!("node is not an element"),
    }
}

fn child_named(document: &ParsedRootHtml, parent: usize, name: &str) -> usize {
    document.nodes[parent]
        .children
        .iter()
        .copied()
        .find(|child| {
            matches!(
                &document.nodes[*child].kind,
                ParsedNodeKind::Element { local_name, .. } if local_name == name
            )
        })
        .unwrap_or_else(|| panic!("missing {name} child"))
}

#[test]
fn html5_document_mode_inserts_html_head_and_body_and_recovers_misnested_markup() {
    let document =
        ParsedRootHtml::parse("<!doctype html></p><title>title</title><p>one<div>two</div>three")
            .unwrap();
    let html = document.html_element().unwrap();
    let head = child_named(&document, html, "head");
    let body = child_named(&document, html, "body");

    assert_eq!(local_name(&document, html), "html");
    assert!(document.nodes[head].children.iter().any(|child| {
        matches!(
            &document.nodes[*child].kind,
            ParsedNodeKind::Element { local_name, .. } if local_name == "title"
        )
    }));
    assert!(document.nodes[body].children.iter().any(|child| {
        matches!(
            &document.nodes[*child].kind,
            ParsedNodeKind::Element { local_name, .. } if local_name == "div"
        )
    }));
    assert!(document.parse_error_count > 0);
}

#[test]
fn html5_tree_keeps_element_attribute_and_text_source_order() {
    let document = ParsedRootHtml::parse(
        "<!doctype html><html><head></head><body><p id='a'>first<b>second</b>third</p></body></html>",
    )
    .unwrap();
    let html = document.html_element().unwrap();
    let body = child_named(&document, html, "body");
    let paragraph = child_named(&document, body, "p");
    let attributes = match &document.nodes[paragraph].kind {
        ParsedNodeKind::Element { attributes, .. } => attributes,
        _ => unreachable!(),
    };
    assert_eq!(attributes, &[("id".to_owned(), "a".to_owned())]);
    let children = &document.nodes[paragraph].children;
    assert!(
        matches!(&document.nodes[children[0]].kind, ParsedNodeKind::Text(text) if text == "first")
    );
    assert_eq!(local_name(&document, children[1]), "b");
    assert!(
        matches!(&document.nodes[children[2]].kind, ParsedNodeKind::Text(text) if text == "third")
    );
}

#[test]
fn html5_tree_foster_parents_table_text_before_the_table() {
    let document = ParsedRootHtml::parse(
        "<!doctype html><html><body><table>foster<tr><td>cell</td></tr></table></body></html>",
    )
    .unwrap();
    let html = document.html_element().unwrap();
    let body = child_named(&document, html, "body");
    let children = &document.nodes[body].children;

    assert_eq!(children.len(), 2);
    assert!(
        matches!(&document.nodes[children[0]].kind, ParsedNodeKind::Text(text) if text == "foster")
    );
    assert_eq!(local_name(&document, children[1]), "table");
}

#[test]
fn html5_tree_rejects_quirks_mode_and_source_or_tree_budgets() {
    assert!(ParsedRootHtml::parse("<html><body>quirks</body></html>").is_err());
    let too_large = "x".repeat(1_048_577);
    assert!(ParsedRootHtml::parse(&too_large).is_err());
    let too_deep = format!(
        "<!doctype html>{}x{}",
        "<div>".repeat(257),
        "</div>".repeat(257)
    );
    assert!(ParsedRootHtml::parse(&too_deep).is_err());

    let attributes = (0..16_385)
        .map(|index| format!("a{index}='x'"))
        .collect::<Vec<_>>()
        .join(" ");
    let too_many_attributes =
        format!("<!doctype html><html><head></head><body><div {attributes}></div></body></html>");
    assert!(ParsedRootHtml::parse(&too_many_attributes).is_err());
}
