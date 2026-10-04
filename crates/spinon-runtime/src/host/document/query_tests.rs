use super::*;
use crate::host::document::{DocumentBatchOperation, MAX_PENDING_EXTERNAL_IDS};

fn query(
    bridge: &mut HostDocumentBridge,
    kind: i32,
    node_id: i32,
    index: i32,
    name: &[u16],
    output: &mut [u16],
) -> (i32, SpinonDocumentQueryResult) {
    let input = SpinonDocumentQuery {
        kind,
        node_id,
        index,
        name: if name.is_empty() {
            std::ptr::null()
        } else {
            name.as_ptr()
        },
        name_length: name.len(),
        output: if output.is_empty() {
            std::ptr::null_mut()
        } else {
            output.as_mut_ptr()
        },
        output_capacity: output.len(),
    };
    let mut result = SpinonDocumentQueryResult::default();
    let mut error = [0_i8; 256];
    let status = unsafe {
        query_callback(
            std::ptr::from_mut(bridge).cast(),
            &input,
            &mut result,
            error.as_mut_ptr(),
            error.len(),
        )
    };
    (status, result)
}

#[test]
fn committed_document_queries_return_utf16_and_stable_external_ids() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    bridge
        .commit(&[
            DocumentBatchOperation::CreateElement {
                id: 41,
                namespace: HTML_NAMESPACE.to_owned(),
                name: "div".to_owned(),
            },
            DocumentBatchOperation::CreateText {
                id: 42,
                data: vec![b'a' as u16, 0xd800],
            },
            DocumentBatchOperation::Append {
                parent: 0,
                node: 41,
            },
            DocumentBatchOperation::Append {
                parent: 41,
                node: 42,
            },
            DocumentBatchOperation::SetAttribute {
                node: 41,
                name: "id".to_owned(),
                value: "proof".encode_utf16().collect(),
            },
        ])
        .unwrap();

    let (status, element) = query(&mut bridge, QUERY_NODE_INFO, 41, 0, &[], &mut []);
    assert_eq!(status, CALLBACK_BUFFER_TOO_SMALL);
    assert_eq!(element.exists, 1);
    assert_eq!(element.value, 1);
    assert_eq!(element.output_length, 3);
    let mut short_name = [0xeeee_u16; 1];
    let (status, short_result) = query(&mut bridge, QUERY_NODE_INFO, 41, 0, &[], &mut short_name);
    assert_eq!(status, CALLBACK_BUFFER_TOO_SMALL);
    assert_eq!(short_result.output_length, 3);
    assert_eq!(short_name, [0xeeee]);
    let mut name = [0_u16; 3];
    let (status, _) = query(&mut bridge, QUERY_NODE_INFO, 41, 0, &[], &mut name);
    assert_eq!(status, CALLBACK_OK);
    assert_eq!(String::from_utf16(&name).unwrap(), "DIV");

    let (status, child) = query(&mut bridge, QUERY_CHILD_AT, 41, 0, &[], &mut []);
    assert_eq!(status, CALLBACK_OK);
    assert_eq!(child.value, 42);
    let (status, parent) = query(&mut bridge, QUERY_PARENT, 42, 0, &[], &mut []);
    assert_eq!(status, CALLBACK_OK);
    assert_eq!(parent.value, 41);
    let (status, text_children) = query(&mut bridge, QUERY_CHILD_COUNT, 42, 0, &[], &mut []);
    assert_eq!(status, CALLBACK_OK);
    assert_eq!(text_children.value, 0);
    let (status, text_first_child) = query(&mut bridge, QUERY_CHILD_AT, 42, 0, &[], &mut []);
    assert_eq!(status, CALLBACK_OK);
    assert_eq!(text_first_child.exists, 0);

    let mut text = [0_u16; 2];
    let (status, text_result) = query(&mut bridge, QUERY_TEXT_CONTENT, 41, 0, &[], &mut text);
    assert_eq!(status, CALLBACK_OK);
    assert_eq!(text_result.output_length, 2);
    assert_eq!(text, [b'a' as u16, 0xd800]);

    let mut attribute = [0_u16; 5];
    let (status, attribute_result) = query(
        &mut bridge,
        QUERY_ATTRIBUTE,
        41,
        0,
        &"id".encode_utf16().collect::<Vec<_>>(),
        &mut attribute,
    );
    assert_eq!(status, CALLBACK_OK);
    assert_eq!(attribute_result.exists, 1);
    assert_eq!(String::from_utf16(&attribute).unwrap(), "proof");

    let (status, next_id) = query(&mut bridge, QUERY_NEXT_ID, 0, 0, &[], &mut []);
    assert_eq!(status, CALLBACK_OK);
    assert_eq!(next_id.value, 43);
}

#[test]
fn next_id_query_reserves_ids_and_enforces_pending_limit() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    for expected in 1..=MAX_PENDING_EXTERNAL_IDS as i32 {
        let (status, result) = query(&mut bridge, QUERY_NEXT_ID, 0, 0, &[], &mut []);
        assert_eq!(status, CALLBACK_OK);
        assert_eq!(result.value, expected);
    }

    let input = SpinonDocumentQuery {
        kind: QUERY_NEXT_ID,
        ..SpinonDocumentQuery::default()
    };
    let mut result = SpinonDocumentQueryResult::default();
    let mut error = [0_i8; 256];
    let status = unsafe {
        query_callback(
            std::ptr::from_mut(&mut bridge).cast(),
            &input,
            &mut result,
            error.as_mut_ptr(),
            error.len(),
        )
    };
    assert_eq!(status, CALLBACK_REJECTED);
    let message = unsafe { std::ffi::CStr::from_ptr(error.as_ptr()) }.to_string_lossy();
    assert!(message.starts_with("QuotaExceededError:"));

    bridge
        .commit(&[DocumentBatchOperation::CreateText {
            id: 1,
            data: Vec::new(),
        }])
        .unwrap();
    let (status, result) = query(&mut bridge, QUERY_NEXT_ID, 0, 0, &[], &mut []);
    assert_eq!(status, CALLBACK_OK);
    assert_eq!(result.value, MAX_PENDING_EXTERNAL_IDS as i32 + 1);
}

#[test]
fn invalid_query_does_not_read_or_write_unchecked_buffers() {
    let mut bridge = HostDocumentBridge::new().unwrap();
    let invalid = SpinonDocumentQuery {
        kind: QUERY_ATTRIBUTE,
        node_id: 1,
        name: std::ptr::null(),
        name_length: 1,
        ..SpinonDocumentQuery::default()
    };
    let mut result = SpinonDocumentQueryResult::default();
    let mut error = [0_i8; 128];
    let status = unsafe {
        query_callback(
            std::ptr::from_mut(&mut bridge).cast(),
            &invalid,
            &mut result,
            error.as_mut_ptr(),
            error.len(),
        )
    };
    assert_eq!(status, CALLBACK_REJECTED);

    let status = unsafe {
        query_callback(
            std::ptr::null_mut(),
            &invalid,
            &mut result,
            error.as_mut_ptr(),
            error.len(),
        )
    };
    assert_eq!(status, CALLBACK_INVALID);

    let overlong_name = vec![b'a' as u16; MAX_QUERY_NAME_UNITS + 1];
    let invalid = SpinonDocumentQuery {
        name: overlong_name.as_ptr(),
        name_length: overlong_name.len(),
        ..invalid
    };
    let status = unsafe {
        query_callback(
            std::ptr::from_mut(&mut bridge).cast(),
            &invalid,
            &mut result,
            error.as_mut_ptr(),
            error.len(),
        )
    };
    assert_eq!(status, CALLBACK_REJECTED);
}
