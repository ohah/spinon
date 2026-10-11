#[cfg(any(target_os = "android", target_os = "ios"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(any(target_os = "android", target_os = "ios", test))]
const MAX_STACK_SAMPLE_PAGES: usize = 8_192;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorkerStackBounds {
    pub(super) thread_id: u64,
    pub(super) reported_start_address: usize,
    pub(super) start_address: usize,
    pub(super) end_address: usize,
    pub(super) reported_stack_bytes: usize,
    pub(super) guard_size_bytes: usize,
    pub(super) requested_stack_bytes: usize,
    pub(super) page_size_bytes: usize,
}

#[cfg(any(target_os = "android", target_os = "ios"))]
unsafe extern "C" {
    fn spinon_c124_capture_current_thread_stack(
        thread_id: *mut u64,
        start_address: *mut usize,
        stack_size: *mut usize,
        guard_size: *mut usize,
        page_size: *mut usize,
        error_code: *mut i32,
    ) -> i32;
    fn spinon_c124_sample_stack_residency(
        start_address: usize,
        stack_size: usize,
        page_flags: *mut u8,
        page_capacity: usize,
        page_size: *mut usize,
        error_code: *mut i32,
    ) -> i32;
}

pub(super) fn capture(requested_stack_bytes: usize) -> Result<WorkerStackBounds, String> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let mut thread_id = 0;
        let mut reported_start_address = 0;
        let mut reported_stack_bytes = 0;
        let mut guard_size_bytes = 0;
        let mut page_size_bytes = 0;
        let mut error_code = 0;
        // SAFETY: the native wrapper writes only the supplied scalar outputs.
        let status = unsafe {
            spinon_c124_capture_current_thread_stack(
                &mut thread_id,
                &mut reported_start_address,
                &mut reported_stack_bytes,
                &mut guard_size_bytes,
                &mut page_size_bytes,
                &mut error_code,
            )
        };
        if status != 0 {
            return Err(format!(
                "현재 CSS worker의 pthread stack 범위를 읽지 못했습니다: native_error={error_code}"
            ));
        }
        let (start_address, end_address) = usable_stack_bounds(
            reported_start_address,
            reported_stack_bytes,
            guard_size_bytes,
        )?;
        let marker = 0_u8;
        let marker_address = (&marker as *const u8) as usize;
        validate_bounds(start_address, end_address, page_size_bytes, marker_address)?;
        return Ok(WorkerStackBounds {
            thread_id,
            reported_start_address,
            start_address,
            end_address,
            reported_stack_bytes,
            guard_size_bytes,
            requested_stack_bytes,
            page_size_bytes,
        });
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let _ = requested_stack_bytes;
        Err("CSS worker stack 관측은 Android·iOS에서만 지원합니다".to_owned())
    }
}

#[cfg(any(target_os = "android", target_os = "ios", test))]
fn usable_stack_bounds(
    reported_start_address: usize,
    reported_stack_bytes: usize,
    guard_size_bytes: usize,
) -> Result<(usize, usize), String> {
    let start_address = reported_start_address
        .checked_add(guard_size_bytes)
        .ok_or_else(|| "CSS worker pthread guard 범위에서 overflow했습니다".to_owned())?;
    let usable_stack_bytes = reported_stack_bytes
        .checked_sub(guard_size_bytes)
        .ok_or_else(|| "CSS worker pthread guard가 stack 범위를 초과했습니다".to_owned())?;
    if usable_stack_bytes == 0 {
        return Err("CSS worker pthread guard 뒤에 usable stack이 없습니다".to_owned());
    }
    let end_address = start_address
        .checked_add(usable_stack_bytes)
        .ok_or_else(|| "CSS worker stack 끝 주소가 overflow했습니다".to_owned())?;
    Ok((start_address, end_address))
}

#[cfg(any(target_os = "android", target_os = "ios", test))]
fn validate_bounds(
    start_address: usize,
    end_address: usize,
    page_size_bytes: usize,
    marker_address: usize,
) -> Result<usize, String> {
    if start_address >= end_address {
        return Err("CSS worker stack 주소 범위가 비어 있거나 역전됐습니다".to_owned());
    }
    if page_size_bytes == 0 || !page_size_bytes.is_power_of_two() {
        return Err(format!(
            "CSS worker page size가 유효하지 않습니다: start=0x{start_address:x} end=0x{end_address:x} page_size={page_size_bytes}"
        ));
    }
    if !(start_address..end_address).contains(&marker_address) {
        return Err("현재 CSS worker stack marker가 OS stack 범위 밖에 있습니다".to_owned());
    }
    let page_count = sample_page_range(start_address, end_address, page_size_bytes)?.2;
    if page_count == 0 || page_count > MAX_STACK_SAMPLE_PAGES {
        return Err(format!(
            "CSS worker stack page 수가 허용 범위를 벗어났습니다: {page_count}"
        ));
    }
    Ok(page_count)
}

#[cfg(any(target_os = "android", target_os = "ios", test))]
fn sample_page_range(
    start_address: usize,
    end_address: usize,
    page_size_bytes: usize,
) -> Result<(usize, usize, usize), String> {
    let first_full_page = start_address
        .checked_add(page_size_bytes - 1)
        .and_then(|address| address.checked_div(page_size_bytes))
        .and_then(|page| page.checked_mul(page_size_bytes))
        .ok_or_else(|| "CSS worker stack 시작 page 정렬에서 overflow했습니다".to_owned())?;
    let end_page = end_address
        .checked_add(page_size_bytes - 1)
        .and_then(|address| address.checked_div(page_size_bytes))
        .and_then(|page| page.checked_mul(page_size_bytes))
        .ok_or_else(|| "CSS worker stack 끝 page 정렬에서 overflow했습니다".to_owned())?;
    if first_full_page >= end_page {
        return Err("CSS worker stack 범위에 검사할 mincore page가 없습니다".to_owned());
    }
    let page_count = (end_page - first_full_page) / page_size_bytes;
    if page_count > MAX_STACK_SAMPLE_PAGES {
        return Err(format!(
            "CSS worker stack page 수가 허용 범위를 벗어났습니다: {page_count}"
        ));
    }
    Ok((first_full_page, end_page, page_count))
}

pub(super) fn sample_json(bounds: &WorkerStackBounds) -> Result<String, String> {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        validate_bounds(
            bounds.start_address,
            bounds.end_address,
            bounds.page_size_bytes,
            bounds.start_address,
        )?;
        let (sample_start_address, sample_end_address, page_count) = sample_page_range(
            bounds.start_address,
            bounds.end_address,
            bounds.page_size_bytes,
        )?;
        let sampled_stack_bytes = bounds.end_address - sample_start_address;
        let sampled_page_span_bytes = sample_end_address - sample_start_address;
        let unprobed_stack_prefix_bytes = sample_start_address - bounds.start_address;
        let sampled_tail_bytes = sample_end_address - bounds.end_address;
        let mut page_flags = Vec::new();
        page_flags
            .try_reserve_exact(page_count)
            .map_err(|error| format!("mincore 결과 buffer 할당 실패: {error}"))?;
        page_flags.resize(page_count, 0_u8);
        let mut observed_page_size = 0;
        let mut error_code = 0;
        // SAFETY: bounds were checked, and the output buffer has one byte per page.
        let status = unsafe {
            spinon_c124_sample_stack_residency(
                sample_start_address,
                sampled_page_span_bytes,
                page_flags.as_mut_ptr(),
                page_flags.len(),
                &mut observed_page_size,
                &mut error_code,
            )
        };
        if status != 0 {
            return Err(format!(
                "CSS worker stack mincore sample 실패: native_error={error_code}"
            ));
        }
        if observed_page_size != bounds.page_size_bytes {
            return Err(format!(
                "CSS worker page size가 실행 중 변경됐습니다: captured={} sampled={observed_page_size}",
                bounds.page_size_bytes
            ));
        }
        let resident_pages = page_flags.iter().filter(|flags| **flags & 1 != 0).count();
        let sampled_at_unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("sample timestamp 생성 실패: {error}"))?
            .as_millis();
        return Ok(format!(
            "{{\"schema\":\"spinon-c12-4-css-worker-stack-residency/v1\",\"status\":\"ok\",\"thread_id\":{},\"pthread_reported_stack_start_address\":\"0x{:x}\",\"pthread_reported_stack_bytes\":{},\"stack_guard_bytes\":{},\"stack_start_address\":\"0x{:x}\",\"stack_end_address\":\"0x{:x}\",\"requested_stack_bytes\":{},\"os_stack_range_bytes\":{},\"mincore_sample_start_address\":\"0x{:x}\",\"mincore_sample_end_address\":\"0x{:x}\",\"mincore_sampled_stack_bytes\":{},\"mincore_sampled_page_span_bytes\":{},\"mincore_unprobed_stack_prefix_bytes\":{},\"mincore_sampled_tail_bytes\":{},\"mincore_sampled_pages\":{},\"page_size_bytes\":{},\"mincore_resident_pages\":{},\"sampled_at_unix_ms\":{}}}",
            bounds.thread_id,
            bounds.reported_start_address,
            bounds.reported_stack_bytes,
            bounds.guard_size_bytes,
            bounds.start_address,
            bounds.end_address,
            bounds.requested_stack_bytes,
            bounds.end_address - bounds.start_address,
            sample_start_address,
            sample_end_address,
            sampled_stack_bytes,
            sampled_page_span_bytes,
            unprobed_stack_prefix_bytes,
            sampled_tail_bytes,
            page_count,
            bounds.page_size_bytes,
            resident_pages,
            sampled_at_unix_ms,
        ));
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let _ = bounds;
        Err("CSS worker stack residency는 Android·iOS에서만 지원합니다".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::{usable_stack_bounds, validate_bounds};

    #[test]
    fn reported_pthread_stack_guard_is_excluded_from_sampled_range() {
        assert_eq!(
            usable_stack_bounds(0x1000, 0x5000, 0x1000),
            Ok((0x2000, 0x6000))
        );
        assert!(usable_stack_bounds(usize::MAX, 0x2000, 0x1000).is_err());
        assert!(usable_stack_bounds(0x1000, 0x1000, 0x1000).is_err());
    }

    #[test]
    fn bounds_select_pages_intersecting_worker_stack_without_start_guard() {
        assert_eq!(validate_bounds(0x1000, 0x3000, 0x1000, 0x1800), Ok(2));
        assert!(validate_bounds(0x3000, 0x1000, 0x1000, 0x2000).is_err());
        assert_eq!(validate_bounds(0x1001, 0x3001, 0x1000, 0x1800), Ok(2));
        assert!(validate_bounds(0x1000, 0x3000, 0x1000, 0x3000).is_err());
    }

    #[test]
    fn bounds_reject_unbounded_page_work() {
        assert!(validate_bounds(0x1000, 0x1000 + 8193 * 0x1000, 0x1000, 0x1800).is_err());
        assert!(validate_bounds(0x1000, 0x3000, 0, 0x1800).is_err());
    }
}
