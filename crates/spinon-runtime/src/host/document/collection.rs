use super::{HostDocumentBridge, NodeStringUsage};
use spinon_core::HostNodeHandle;
use std::ffi::{c_char, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::slice;

const CALLBACK_OK: i32 = 0;
const CALLBACK_BUFFER_TOO_SMALL: i32 = 1;
const CALLBACK_INVALID: i32 = -1;
const CALLBACK_REJECTED: i32 = -2;
const CALLBACK_PANIC: i32 = -3;

pub(crate) type DocumentCollectCallback = unsafe extern "C" fn(
    *mut c_void,
    *const i32,
    usize,
    *mut i32,
    usize,
    *mut usize,
    *mut c_char,
    usize,
) -> i32;

impl HostDocumentBridge {
    #[cfg(test)]
    pub(super) fn collect_unreachable(
        &mut self,
        roots: &[HostNodeHandle],
    ) -> Result<usize, String> {
        Ok(self.collect_unreachable_with_ids(roots)?.len())
    }

    fn collect_unreachable_with_ids(
        &mut self,
        roots: &[HostNodeHandle],
    ) -> Result<Vec<i32>, String> {
        self.validate_collection_indexes()?;

        let plan = self
            .document
            .plan_collection(roots)
            .map_err(|error| error.to_string())?;
        let mut reclaimed_string_units = 0usize;
        for handle in plan.reclaimed_handles() {
            let external_id = self
                .external_ids
                .get(handle)
                .copied()
                .ok_or_else(|| "회수 대상의 외부 노드 ID가 없습니다".to_owned())?;
            if self.handles.get(&external_id) != Some(handle) {
                return Err("회수 대상의 양방향 노드 ID 매핑이 다릅니다".to_owned());
            }
            let usage = self
                .string_usage
                .get(&external_id)
                .and_then(NodeStringUsage::total_units)
                .ok_or_else(|| "회수 대상의 문자열 계수가 잘못되었습니다".to_owned())?;
            reclaimed_string_units = reclaimed_string_units
                .checked_add(usage)
                .ok_or_else(|| "회수 대상 문자열 계수가 넘쳤습니다".to_owned())?;
        }
        let remaining_string_units = self
            .string_units
            .checked_sub(reclaimed_string_units)
            .ok_or_else(|| "회수 뒤 문자열 계수가 음수가 됩니다".to_owned())?;
        let mut reclaimed_external_ids = Vec::new();
        reclaimed_external_ids
            .try_reserve_exact(plan.reclaimed_node_count())
            .map_err(|_| "회수 결과 ID 버퍼를 확보하지 못했습니다".to_owned())?;
        for handle in plan.reclaimed_handles() {
            reclaimed_external_ids.push(
                *self
                    .external_ids
                    .get(handle)
                    .ok_or_else(|| "회수 대상의 외부 노드 ID가 없습니다".to_owned())?,
            );
        }
        reclaimed_external_ids.sort_unstable();

        self.document
            .commit_collection(&plan)
            .map_err(|error| error.to_string())?;
        for handle in plan.reclaimed_handles() {
            let external_id = self
                .external_ids
                .remove(handle)
                .expect("회수 전에 검증한 외부 노드 ID 매핑");
            self.handles.remove(&external_id);
            self.string_usage.remove(&external_id);
        }
        self.string_units = remaining_string_units;
        Ok(reclaimed_external_ids)
    }

    fn validate_collection_indexes(&self) -> Result<(), String> {
        if self.document.node_count() != self.handles.len()
            || self.handles.len() != self.external_ids.len()
            || self.handles.len() != self.string_usage.len()
        {
            return Err("문서 회수 전에 노드 인덱스 수가 일치하지 않습니다".to_owned());
        }

        let mut total_string_units = 0usize;
        for (external_id, handle) in &self.handles {
            if *external_id <= 0
                || self.document.node(*handle).is_none()
                || self.external_ids.get(handle) != Some(external_id)
            {
                return Err("문서 회수 전에 노드 ID 매핑이 일치하지 않습니다".to_owned());
            }
            let units = self
                .string_usage
                .get(external_id)
                .and_then(NodeStringUsage::total_units)
                .ok_or_else(|| "문서 회수 전에 문자열 계수가 잘못되었습니다".to_owned())?;
            total_string_units = total_string_units
                .checked_add(units)
                .ok_or_else(|| "문서 회수 전 문자열 계수가 넘쳤습니다".to_owned())?;
        }
        if total_string_units != self.string_units {
            return Err("문서 회수 전 문자열 총계가 노드별 계수와 다릅니다".to_owned());
        }
        for (handle, external_id) in &self.external_ids {
            if self.handles.get(external_id) != Some(handle) {
                return Err("문서 회수 전에 역방향 노드 ID 매핑이 다릅니다".to_owned());
            }
        }
        Ok(())
    }
}

fn write_error(output: *mut c_char, capacity: usize, message: &str) {
    if output.is_null() || capacity == 0 {
        return;
    }
    let mut length = message.len().min(capacity - 1);
    while !message.is_char_boundary(length) {
        length -= 1;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(message.as_ptr(), output.cast::<u8>(), length);
        *output.add(length) = 0;
    }
}

unsafe fn collect_document(
    user_data: *mut c_void,
    roots: *const i32,
    root_count: usize,
    reclaimed_ids: *mut i32,
    reclaimed_capacity: usize,
    reclaimed_count: *mut usize,
) -> Result<(), (i32, String)> {
    if user_data.is_null() || reclaimed_count.is_null() {
        return Err((
            CALLBACK_INVALID,
            "문서 회수 인자가 비어 있습니다".to_owned(),
        ));
    }
    if root_count > 0 && roots.is_null() {
        return Err((
            CALLBACK_INVALID,
            "문서 회수 root 인자가 잘못되었습니다".to_owned(),
        ));
    }
    let bridge = unsafe { &mut *user_data.cast::<HostDocumentBridge>() };
    let required_capacity = usize::try_from(bridge.node_count())
        .map_err(|_| (CALLBACK_REJECTED, "문서 노드 수가 넘쳤습니다".to_owned()))?;
    if reclaimed_capacity < required_capacity {
        unsafe { *reclaimed_count = required_capacity };
        return Err((
            CALLBACK_BUFFER_TOO_SMALL,
            "문서 회수 결과 버퍼가 부족합니다".to_owned(),
        ));
    }
    if required_capacity > 0 && reclaimed_ids.is_null() {
        return Err((
            CALLBACK_INVALID,
            "문서 회수 결과 포인터가 비어 있습니다".to_owned(),
        ));
    }

    let root_ids = if root_count == 0 {
        &[][..]
    } else {
        unsafe { slice::from_raw_parts(roots, root_count) }
    };
    let mut sorted_root_ids = Vec::new();
    sorted_root_ids.try_reserve_exact(root_count).map_err(|_| {
        (
            CALLBACK_REJECTED,
            "문서 root 검증 버퍼를 확보하지 못했습니다".to_owned(),
        )
    })?;
    sorted_root_ids.extend_from_slice(root_ids);
    sorted_root_ids.sort_unstable();
    if sorted_root_ids.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err((
            CALLBACK_REJECTED,
            "문서 회수 root ID가 중복되었습니다".to_owned(),
        ));
    }

    let mut root_handles = Vec::new();
    root_handles.try_reserve_exact(root_count).map_err(|_| {
        (
            CALLBACK_REJECTED,
            "문서 root handle 버퍼를 확보하지 못했습니다".to_owned(),
        )
    })?;
    for id in root_ids {
        let handle = bridge.handle(*id).ok_or_else(|| {
            (
                CALLBACK_REJECTED,
                format!("존재하지 않는 wrapper root ID입니다: {id}"),
            )
        })?;
        root_handles.push(handle);
    }

    let reclaimed = bridge
        .collect_unreachable_with_ids(&root_handles)
        .map_err(|error| (CALLBACK_REJECTED, error))?;
    // callback 앞에서 buffer >= 현재 node_count를 확인했고, 이 호출만 bridge를
    // 변경하므로 collector 결과는 항상 검증한 buffer 안에 들어옵니다.
    unsafe {
        if !reclaimed.is_empty() {
            std::ptr::copy_nonoverlapping(reclaimed.as_ptr(), reclaimed_ids, reclaimed.len());
        }
        *reclaimed_count = reclaimed.len();
    }
    Ok(())
}

/// V8 약한 wrapper snapshot을 사용해 HostDocument의 도달 불가 노드를 회수합니다.
///
/// # Safety
/// `user_data`는 owner 실행기에서 유일하게 접근 가능한 `HostDocumentBridge`여야 합니다.
/// root·결과 buffer는 호출 중 유효해야 하며, 결과 용량은 현재 문서 노드 수 이상이어야 합니다.
pub(crate) unsafe extern "C" fn collect_callback(
    user_data: *mut c_void,
    roots: *const i32,
    root_count: usize,
    reclaimed_ids: *mut i32,
    reclaimed_capacity: usize,
    reclaimed_count: *mut usize,
    error_output: *mut c_char,
    error_capacity: usize,
) -> i32 {
    if !reclaimed_count.is_null() {
        unsafe { *reclaimed_count = 0 };
    }
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        collect_document(
            user_data,
            roots,
            root_count,
            reclaimed_ids,
            reclaimed_capacity,
            reclaimed_count,
        )
    }));
    match result {
        Ok(Ok(())) => CALLBACK_OK,
        Ok(Err((status, error))) => {
            write_error(error_output, error_capacity, &error);
            status
        }
        Err(_) => {
            write_error(
                error_output,
                error_capacity,
                "문서 회수 callback에서 panic이 발생했습니다",
            );
            CALLBACK_PANIC
        }
    }
}
