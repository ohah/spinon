use super::*;

#[test]
fn auto_ratio_combination_fails_closed_instead_of_becoming_a_bare_ratio() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(7731).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML_NS.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "style").unwrap(),
            value: "aspect-ratio:auto 16 / 9".into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
    document.commit(batch).unwrap();
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::new(document.snapshot()), root)
            .unwrap();
    assert!(matches!(
        compute_runtime_flex_custom_properties_cascade_with_stylesheets(
            &view,
            &[],
            viewport(1.0),
            StyleRevision::INITIAL,
        ),
        Err(CssCascadeError::UnsupportedComputedAspectRatio { node, reason })
            if node == root.id() && reason.contains("content-box")
    ));
}

#[test]
fn min_max_constraints_with_aspect_ratio_fail_closed_for_each_axis() {
    for (constraint, property) in [
        ("min-width:10px", "min-width"),
        ("max-width:200px", "max-width"),
        ("min-height:10px", "min-height"),
        ("max-height:200px", "max-height"),
    ] {
        let style = format!("width:100px;aspect-ratio:2;{constraint}");
        assert!(
            matches!(
                compute_inline_root_layout(&style),
                Err(StyleLayoutError::UnsupportedAspectRatioConstraint {
                    node,
                    property: actual,
                }) if actual == property
                    && node.get() == 1
            ),
            "{constraint}가 지원 오류 없이 처리되었거나 잘못된 오류를 반환했습니다"
        );
    }
    assert!(matches!(
        compute_inline_flex_child_layout(
            "width:120px;min-height:70px;max-height:80px;aspect-ratio:2;flex-grow:0;flex-shrink:1;flex-basis:auto"
        ),
        Err(StyleLayoutError::UnsupportedAspectRatioConstraint {
            property: "min-height",
            ..
        })
    ));
}

#[test]
fn finite_css_ratio_operands_that_overflow_or_underflow_fail_closed() {
    for css_ratio in ["1e38 / 1e-38", "1e-38 / 1e38"] {
        let style = format!("width:100px;aspect-ratio:{css_ratio}");
        assert!(
            matches!(
                compute_inline_root_layout(&style),
                Err(StyleLayoutError::Cascade(
                    CssCascadeError::UnsupportedComputedAspectRatio { node, reason }
                )) if node.get() == 1 && reason.contains("유한한 양수")
            ),
            "표현 범위를 벗어난 비율 {css_ratio}를 거부하지 않았습니다"
        );
    }
}
