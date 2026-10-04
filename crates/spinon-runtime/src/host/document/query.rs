use super::HostDocumentBridge;
use spinon_core::{AttributeName, HostNodeKind, HostParent};
use std::ffi::{c_char, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::slice;

const CALLBACK_OK: i32 = 0;
const CALLBACK_INVALID: i32 = -1;
const CALLBACK_REJECTED: i32 = -2;
const CALLBACK_PANIC: i32 = -3;
const CALLBACK_BUFFER_TOO_SMALL: i32 = 1;
const MAX_QUERY_NAME_UNITS: usize = 1024;
const MAX_QUERY_OUTPUT_UNITS: usize = 16_777_216;
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

pub(crate) const QUERY_NODE_INFO: i32 = 1;
pub(crate) const QUERY_PARENT: i32 = 2;
pub(crate) const QUERY_CHILD_COUNT: i32 = 3;
pub(crate) const QUERY_CHILD_AT: i32 = 4;
pub(crate) const QUERY_NEXT_SIBLING: i32 = 5;
pub(crate) const QUERY_TEXT_CONTENT: i32 = 6;
pub(crate) const QUERY_ATTRIBUTE: i32 = 7;
pub(crate) const QUERY_NEXT_ID: i32 = 8;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct SpinonDocumentQuery {
    pub(crate) kind: i32,
    pub(crate) node_id: i32,
    pub(crate) index: i32,
    pub(crate) name: *const u16,
    pub(crate) name_length: usize,
    pub(crate) output: *mut u16,
    pub(crate) output_capacity: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct SpinonDocumentQueryResult {
    pub(crate) exists: i32,
    pub(crate) value: i32,
    pub(crate) output_length: usize,
}

pub(crate) type DocumentQueryCallback = unsafe extern "C" fn(
    *mut c_void,
    *const SpinonDocumentQuery,
    *mut SpinonDocumentQueryResult,
    *mut c_char,
    usize,
) -> i32;

fn write_error(output: *mut c_char, capacity: usize, message: &str) {
    if output.is_null() || capacity == 0 {
        return;
    }
    let max = capacity - 1;
    let mut length = message.len().min(max);
    while !message.is_char_boundary(length) {
        length -= 1;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(message.as_ptr(), output.cast::<u8>(), length);
        *output.add(length) = 0;
    }
}

unsafe fn query_name(query: &SpinonDocumentQuery) -> Result<String, String> {
    if query.name_length > MAX_QUERY_NAME_UNITS {
        return Err("조회 이름이 허용 길이를 초과했습니다".to_owned());
    }
    if query.name_length == 0 {
        return Ok(String::new());
    }
    if query.name.is_null() {
        return Err("조회 이름 포인터가 비어 있습니다".to_owned());
    }
    let units = unsafe { slice::from_raw_parts(query.name, query.name_length) };
    String::from_utf16(units).map_err(|_| "조회 이름에 짝이 맞지 않는 UTF-16이 있습니다".to_owned())
}

fn node_id(
    bridge: &HostDocumentBridge,
    handle: spinon_core::HostNodeHandle,
) -> Result<i32, String> {
    bridge
        .external_id(handle)
        .ok_or_else(|| "문서 노드의 연결 키를 찾지 못했습니다".to_owned())
}

fn read_query(
    bridge: &mut HostDocumentBridge,
    query: &SpinonDocumentQuery,
) -> Result<(SpinonDocumentQueryResult, Vec<u16>), String> {
    let mut result = SpinonDocumentQueryResult::default();
    let mut output = Vec::new();
    match query.kind {
        QUERY_NEXT_ID => {
            result.value = bridge.next_external_id()?;
        }
        QUERY_NODE_INFO => {
            if query.node_id <= 0 {
                return Err("노드 정보 조회에는 양의 노드 ID가 필요합니다".to_owned());
            }
            let handle = bridge
                .handle(query.node_id)
                .ok_or_else(|| "알 수 없는 노드 연결 키입니다".to_owned())?;
            let node = bridge
                .document
                .node(handle)
                .ok_or_else(|| "문서 snapshot에서 노드를 찾지 못했습니다".to_owned())?;
            result.exists = 1;
            match node.kind() {
                HostNodeKind::Element(element) => {
                    result.value = 1;
                    let name = if element.namespace() == HTML_NAMESPACE {
                        element.local_name().to_ascii_uppercase()
                    } else {
                        element.local_name().to_owned()
                    };
                    output = name.encode_utf16().collect();
                }
                HostNodeKind::Text(_) => {
                    result.value = 3;
                    output = "#text".encode_utf16().collect();
                }
            }
        }
        QUERY_PARENT => {
            if query.node_id <= 0 {
                return Err("부모 조회에는 양의 노드 ID가 필요합니다".to_owned());
            }
            let handle = bridge
                .handle(query.node_id)
                .ok_or_else(|| "알 수 없는 노드 연결 키입니다".to_owned())?;
            if let Some(parent) = bridge.document.parent(handle) {
                result.exists = 1;
                result.value = match parent {
                    HostParent::Root => 0,
                    HostParent::Node(parent) => node_id(bridge, parent)?,
                };
            }
        }
        QUERY_CHILD_COUNT => {
            if query.node_id == 0 {
                result.value = bridge.document.root_children().count() as i32;
            } else {
                let handle = bridge
                    .handle(query.node_id)
                    .ok_or_else(|| "알 수 없는 노드 연결 키입니다".to_owned())?;
                result.value = bridge
                    .document
                    .children(handle)
                    .map_or(0, |children| children.count() as i32);
            }
            result.exists = 1;
        }
        QUERY_CHILD_AT => {
            if query.index < 0 {
                return Err("자식 인덱스가 음수입니다".to_owned());
            }
            let document = &bridge.document;
            let child = if query.node_id == 0 {
                document.root_children().nth(query.index as usize)
            } else {
                let handle = bridge
                    .handle(query.node_id)
                    .ok_or_else(|| "알 수 없는 노드 연결 키입니다".to_owned())?;
                document
                    .children(handle)
                    .and_then(|mut children| children.nth(query.index as usize))
            };
            if let Some(child) = child {
                result.exists = 1;
                result.value = node_id(bridge, child)?;
            }
        }
        QUERY_NEXT_SIBLING => {
            if query.node_id <= 0 {
                return Err("형제 조회에는 양의 노드 ID가 필요합니다".to_owned());
            }
            let handle = bridge
                .handle(query.node_id)
                .ok_or_else(|| "알 수 없는 노드 연결 키입니다".to_owned())?;
            if let Some(next) = bridge.document.next_sibling(handle) {
                result.exists = 1;
                result.value = node_id(bridge, next)?;
            }
        }
        QUERY_TEXT_CONTENT => {
            if query.node_id <= 0 {
                return Err("textContent 조회에는 양의 노드 ID가 필요합니다".to_owned());
            }
            let handle = bridge
                .handle(query.node_id)
                .ok_or_else(|| "알 수 없는 노드 연결 키입니다".to_owned())?;
            let text = bridge
                .document
                .text_content(handle)
                .ok_or_else(|| "문서 snapshot에서 textContent를 만들지 못했습니다".to_owned())?;
            output.extend_from_slice(text.code_units());
            result.exists = 1;
        }
        QUERY_ATTRIBUTE => {
            if query.node_id <= 0 {
                return Err("속성 조회에는 양의 노드 ID가 필요합니다".to_owned());
            }
            let name = unsafe { query_name(query)? };
            let name = AttributeName::new(None, name)
                .ok_or_else(|| "속성 이름이 잘못되었습니다".to_owned())?;
            let handle = bridge
                .handle(query.node_id)
                .ok_or_else(|| "알 수 없는 노드 연결 키입니다".to_owned())?;
            let node = bridge
                .document
                .node(handle)
                .ok_or_else(|| "문서 snapshot에서 노드를 찾지 못했습니다".to_owned())?;
            if let HostNodeKind::Element(element) = node.kind()
                && let Some(value) = element.attribute(&name)
            {
                result.exists = 1;
                output.extend_from_slice(value.code_units());
            }
        }
        unknown => return Err(format!("알 수 없는 문서 조회 종류입니다: {unknown}")),
    }
    if output.len() > MAX_QUERY_OUTPUT_UNITS {
        return Err("문서 조회 결과가 허용 길이를 초과했습니다".to_owned());
    }
    result.output_length = output.len();
    Ok((result, output))
}

/// Rust 문서 상태를 읽고 facade 노드 ID를 예약하는 V8 callback입니다.
///
/// # Safety
/// `user_data`는 owner 실행기에서 유일하게 접근 가능한 `HostDocumentBridge`여야 합니다.
/// query/result와 선택적 UTF-16·오류 buffer는 호출 중 유효해야 하고, output capacity는
/// output buffer의 실제 크기와 같아야 합니다.
pub(crate) unsafe extern "C" fn query_callback(
    user_data: *mut c_void,
    query: *const SpinonDocumentQuery,
    output: *mut SpinonDocumentQueryResult,
    error_output: *mut c_char,
    error_capacity: usize,
) -> i32 {
    if user_data.is_null() || query.is_null() || output.is_null() {
        write_error(
            error_output,
            error_capacity,
            "문서 조회 callback 인자가 잘못되었습니다",
        );
        return CALLBACK_INVALID;
    }
    let query = unsafe { &*query };
    let bridge = unsafe { &mut *user_data.cast::<HostDocumentBridge>() };
    let result = catch_unwind(AssertUnwindSafe(|| read_query(bridge, query)));
    match result {
        Ok(Ok((result, value))) => {
            if query.output_capacity < value.len() || (!value.is_empty() && query.output.is_null())
            {
                unsafe { output.write(result) };
                return CALLBACK_BUFFER_TOO_SMALL;
            }
            if !value.is_empty() {
                unsafe {
                    std::ptr::copy_nonoverlapping(value.as_ptr(), query.output, value.len());
                }
            }
            unsafe { output.write(result) };
            CALLBACK_OK
        }
        Ok(Err(error)) => {
            write_error(error_output, error_capacity, &error);
            CALLBACK_REJECTED
        }
        Err(_) => {
            write_error(
                error_output,
                error_capacity,
                "문서 조회 callback에서 패닉을 복구했습니다",
            );
            CALLBACK_PANIC
        }
    }
}

#[cfg(test)]
#[path = "query_tests.rs"]
mod tests;
