use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};

use crate::{ImageRequest, ImageRequestError};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const SVG: &str = "http://www.w3.org/2000/svg";

#[test]
fn image_request_requires_a_connected_html_img_with_a_nonempty_src() {
    let mut empty_document = spinon_core::HostDocument::new().unwrap();
    let unknown = empty_document.reserve_node_handle().unwrap();
    assert_eq!(
        ImageRequest::from_document(&empty_document.snapshot(), unknown),
        Err(ImageRequestError::UnknownElement)
    );

    let (document, image) =
        image_document(HTML, "img", Some("https://example.invalid/a.png"), true);
    let request = ImageRequest::from_document(&document.snapshot(), image).unwrap();
    assert_eq!(
        request.src().to_string_lossy(),
        "https://example.invalid/a.png"
    );

    let (document, image) =
        image_document(HTML, "img", Some("https://example.invalid/a.png"), false);
    assert_eq!(
        ImageRequest::from_document(&document.snapshot(), image),
        Err(ImageRequestError::DetachedElement)
    );

    let (document, image) = image_document(SVG, "img", Some("https://example.invalid/a.png"), true);
    assert_eq!(
        ImageRequest::from_document(&document.snapshot(), image),
        Err(ImageRequestError::NotHtmlImage)
    );

    let (document, image) = image_document_with_src_name(
        HTML,
        "img",
        Some("https://example.invalid/a.png"),
        true,
        AttributeName::new(Some("urn:example".to_owned()), "src").unwrap(),
    );
    assert_eq!(
        ImageRequest::from_document(&document.snapshot(), image),
        Err(ImageRequestError::MissingSource)
    );

    let (document, image) =
        image_document(HTML, "image", Some("https://example.invalid/a.png"), true);
    assert_eq!(
        ImageRequest::from_document(&document.snapshot(), image),
        Err(ImageRequestError::NotHtmlImage)
    );

    let (document, image) = image_document(HTML, "img", None, true);
    assert_eq!(
        ImageRequest::from_document(&document.snapshot(), image),
        Err(ImageRequestError::MissingSource)
    );

    let (document, image) = image_document(HTML, "img", Some(""), true);
    assert_eq!(
        ImageRequest::from_document(&document.snapshot(), image),
        Err(ImageRequestError::EmptySource)
    );

    let max_src = "😀".repeat(524_288);
    let (document, image) = image_document(HTML, "img", Some(&max_src), true);
    assert_eq!(
        ImageRequest::from_document(&document.snapshot(), image)
            .unwrap()
            .src()
            .code_units()
            .len(),
        1_048_576
    );

    let long_src = "😀".repeat(524_289);
    let (document, image) = image_document(HTML, "img", Some(&long_src), true);
    assert!(matches!(
        ImageRequest::from_document(&document.snapshot(), image),
        Err(ImageRequestError::SourceTooLong {
            actual: 1_048_578,
            maximum: 1_048_576,
        })
    ));
}

fn image_document(
    namespace: &str,
    local_name: &str,
    src: Option<&str>,
    connected: bool,
) -> (HostDocument, HostNodeHandle) {
    image_document_with_src_name(
        namespace,
        local_name,
        src,
        connected,
        AttributeName::new(None, "src").unwrap(),
    )
}

fn image_document_with_src_name(
    namespace: &str,
    local_name: &str,
    src: Option<&str>,
    connected: bool,
    src_name: AttributeName,
) -> (HostDocument, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(91).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: image,
            namespace: namespace.to_owned(),
            local_name: local_name.to_owned(),
        });
    if let Some(src) = src {
        batch.push(DocumentOperation::SetAttribute {
            node: image,
            name: src_name,
            value: src.to_owned().into(),
        });
    }
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    if connected {
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: image,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (document, image)
}
