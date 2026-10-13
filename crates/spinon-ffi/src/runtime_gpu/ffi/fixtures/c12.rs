use super::super::super::RuntimeGpuHost;
use super::super::super::{
    RUNTIME_CSS_C12_1_POSITION_FIXTURE_SOURCE, RUNTIME_CSS_C12_2_ABSOLUTE_BLOCK_FIXTURE_SOURCE,
    RUNTIME_CSS_C12_4_STACKING_FIXTURE_SOURCE, SpinonRuntimeGpuHost, c12_3_fixed_runtime_script,
    raw_host,
};
use super::super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;
use std::ptr;

const C12_1_POSITION_INVENTORY: &str = include_str!(
    "../../../../../../tests/fixtures/css/c12/position-static-relative-inventory.json"
);

fn position_state_source(state: u32) -> Result<String, String> {
    let state_name = match state {
        1 => "target-relative",
        2 => "ancestor-relative",
        _ => return Err(format!("지원하지 않는 C12.1 상태 번호입니다: {state}")),
    };
    let inventory: serde_json::Value = serde_json::from_str(C12_1_POSITION_INVENTORY)
        .map_err(|error| format!("C12.1 fixture inventory를 읽지 못했습니다: {error}"))?;
    let mutation = inventory["mutations"]
        .as_array()
        .and_then(|mutations| {
            mutations
                .iter()
                .find(|mutation| mutation["state"].as_str() == Some(state_name))
        })
        .ok_or_else(|| format!("C12.1 상태 변경 정의가 없습니다: {state_name}"))?;
    let changes = mutation
        .get("changes")
        .ok_or_else(|| format!("C12.1 상태 변경 값이 없습니다: {state_name}"))?;
    let changes = serde_json::to_string(changes)
        .map_err(|error| format!("C12.1 상태 변경 값을 인코딩하지 못했습니다: {error}"))?;
    Ok(format!(
        "(() => {{ const changes = {changes}; const nodes = globalThis.__spinonC121NodeRefs; for (const [id, style] of Object.entries(changes)) {{ const element = nodes && nodes[id]; if (!element) throw new Error('C12.1 mutation target missing: ' + id); element.setAttribute('style', style); }} return true; }})()"
    ))
}

#[unsafe(no_mangle)]
/// C12.1 정적·상대 위치 CSS fixture를 V8·Stylo·Taffy 경로에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C12.1 runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c12_1_position_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(
        RUNTIME_CSS_C12_1_POSITION_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C12.2 Block absolute CSS fixture를 실제 V8·Stylo·Taffy 경로에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C12.2 runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c12_2_absolute_block_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(
        RUNTIME_CSS_C12_2_ABSOLUTE_BLOCK_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C12.3 inventory의 지원 viewport fixed subset을 실제 V8·Stylo·Taffy 경로에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C12.3 runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c12_3_fixed_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    let source = match c12_3_fixed_runtime_script() {
        Ok(source) => source,
        Err(error) => return write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    };
    match host.eval(&source, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C12.4 제한 Block/Flex 쌓임 맥락 fixture를 V8·Stylo·Taffy·WGPU 경로에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 C12.4 runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c12_4_stacking_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.eval(
        RUNTIME_CSS_C12_4_STACKING_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[cfg(feature = "c12-4-worker-stack-residency-test")]
#[unsafe(no_mangle)]
/// 실제 C12.4 CSS worker의 stack bounds와 순간 mincore residency를 테스트용으로 표본화합니다.
///
/// # Safety
/// `host`는 살아 있는 C12.4 runtime host여야 하며 `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_css_worker_stack_residency_for_test(
    host: *mut SpinonRuntimeGpuHost,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.css_worker_stack_residency_sample_json_for_test() {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err(error) => write_host_status(-13, error, output, output_capacity),
    }
}

#[cfg(feature = "c12-4-metal-surface-readback-test")]
#[unsafe(no_mangle)]
/// 같은 C12.4 host가 제출할 Metal surface frame에서 단일 표본을 요청합니다.
///
/// # Safety
/// `host`는 살아 있는 C12.4 surface-readback host이며 같은 render queue에서 직렬 호출해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c12_4_surface_pixel_for_test(
    host: *mut SpinonRuntimeGpuHost,
    x: u32,
    y: u32,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    match host.request_surface_pixel_readback_for_test(&[(x, y)]) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[cfg(feature = "c12-4-metal-surface-readback-test")]
#[unsafe(no_mangle)]
/// 같은 C12.4 surface frame에서 여러 표본 좌표를 요청합니다.
///
/// # Safety
/// `host`는 살아 있는 C12.4 surface-readback host여야 합니다. `sample_xy`는 `sample_count * 2`
///개의 읽기 가능한 `u32`를 호출 중 유지해야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_request_c12_4_surface_pixels_for_test(
    host: *mut SpinonRuntimeGpuHost,
    sample_xy: *const u32,
    sample_count: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(value_count) = sample_count.checked_mul(2) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C12.4 surface 표본 좌표 개수가 범위를 넘었습니다".to_owned(),
            output,
            output_capacity,
        );
    };
    if !(1..=64).contains(&sample_count) {
        return write_host_status(
            ERR_ARGUMENT,
            "C12.4 surface 표본 수는 1–64개여야 합니다".to_owned(),
            output,
            output_capacity,
        );
    }
    if sample_xy.is_null() {
        return write_host_status(
            ERR_ARGUMENT,
            "C12.4 surface 표본 좌표가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C12.4 surface-pixel host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    // SAFETY: 곱셈 overflow·표본 수 범위·null 입력을 위에서 확인했고, 포인터의 실제
    // 메모리 유효 범위는 FFI 호출자가 제공한 계약에 따른다.
    let values = unsafe { std::slice::from_raw_parts(sample_xy, value_count) };
    let mut points = Vec::new();
    if let Err(error) = points.try_reserve_exact(sample_count) {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C12.4 surface 표본 좌표 할당 실패: {error}"),
            output,
            output_capacity,
        );
    }
    points.extend(values.chunks_exact(2).map(|pair| (pair[0], pair[1])));
    match host.request_surface_pixel_readback_for_test(&points) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C12.1 fixture의 고정된 스타일 변경 상태를 같은 V8 문서에 적용하고 재계산합니다.
/// state 1은 `target-relative`, 2는 `ancestor-relative`입니다.
///
/// # Safety
/// `host`는 살아 있는 C12.1 runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c12_1_position_state(
    host: *mut SpinonRuntimeGpuHost,
    state: u32,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    let source = match position_state_source(state) {
        Ok(source) => source,
        Err(error) => return write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    };
    match host.eval(&source, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C12.1 위치 지정 runtime fixture 전용 V8 GPU host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c12_1_position_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c12_1_position_fixture() {
        Ok(host) => host,
        Err(error) => {
            crate::write_report(output, output_capacity, &error);
            return ptr::null_mut();
        }
    };
    if !crate::write_report(output, output_capacity, &report) {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(host)).cast::<SpinonRuntimeGpuHost>()
}

#[unsafe(no_mangle)]
/// C12.2 Block absolute positioning fixture 전용 V8 GPU host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c12_2_absolute_block_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c12_2_absolute_block_fixture() {
        Ok(host) => host,
        Err(error) => {
            crate::write_report(output, output_capacity, &error);
            return ptr::null_mut();
        }
    };
    if !crate::write_report(output, output_capacity, &report) {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(host)).cast::<SpinonRuntimeGpuHost>()
}

#[unsafe(no_mangle)]
/// C12.3 viewport fixed fixture 전용 V8 GPU host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c12_3_fixed_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c12_3_fixed_fixture() {
        Ok(host) => host,
        Err(error) => {
            crate::write_report(output, output_capacity, &error);
            return ptr::null_mut();
        }
    };
    if !crate::write_report(output, output_capacity, &report) {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(host)).cast::<SpinonRuntimeGpuHost>()
}

#[unsafe(no_mangle)]
/// C12.4 쌓임 맥락 fixture 전용 V8 GPU host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c12_4_stacking_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c12_4_stacking_fixture() {
        Ok(host) => host,
        Err(error) => {
            crate::write_report(output, output_capacity, &error);
            return ptr::null_mut();
        }
    };
    if !crate::write_report(output, output_capacity, &report) {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(host)).cast::<SpinonRuntimeGpuHost>()
}

#[cfg(feature = "c12-4-metal-surface-readback-test")]
#[unsafe(no_mangle)]
/// iOS Simulator Metal surface pixel 진단 전용 C12.4 host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c12_4_surface_readback_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c12_4_surface_readback_fixture() {
        Ok(host) => host,
        Err(error) => {
            crate::write_report(output, output_capacity, &error);
            return ptr::null_mut();
        }
    };
    if !crate::write_report(output, output_capacity, &report) {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(host)).cast::<SpinonRuntimeGpuHost>()
}

#[cfg(test)]
mod tests {
    use super::position_state_source;
    #[cfg(feature = "c12-4-metal-surface-readback-test")]
    use super::spinon_runtime_gpu_host_request_c12_4_surface_pixels_for_test;
    #[cfg(feature = "c12-4-metal-surface-readback-test")]
    use std::ffi::{CStr, c_char};
    #[cfg(feature = "c12-4-metal-surface-readback-test")]
    use std::ptr;

    #[test]
    fn state_scripts_are_derived_from_the_pinned_inventory() {
        let target = position_state_source(1).unwrap();
        let ancestor = position_state_source(2).unwrap();

        assert!(target.contains("dynamic-target"));
        assert!(target.contains("position:relative;left:15px;top:2px"));
        assert!(!target.contains("dynamic-parent"));
        assert!(target.contains("globalThis.__spinonC121NodeRefs"));
        assert!(!target.contains("getElementById"));
        assert!(ancestor.contains("dynamic-parent"));
        assert!(ancestor.contains("dynamic-nested-parent"));
        assert!(ancestor.contains("dynamic-target"));
    }

    #[test]
    fn unsupported_state_is_rejected_without_a_script() {
        assert!(position_state_source(0).is_err());
        assert!(position_state_source(3).is_err());
    }

    #[cfg(feature = "c12-4-metal-surface-readback-test")]
    #[test]
    fn c124_surface_pixel_ffi_rejects_invalid_pointer_and_count_shapes() {
        let mut output = [0 as c_char; 256];
        let points = [1_u32, 2];
        let cases = [
            (ptr::null(), 1, -1),
            (points.as_ptr(), 0, -1),
            (points.as_ptr(), 65, -1),
            (points.as_ptr(), usize::MAX, -1),
        ];
        for (sample_xy, sample_count, expected_status) in cases {
            let status = unsafe {
                spinon_runtime_gpu_host_request_c12_4_surface_pixels_for_test(
                    ptr::null_mut(),
                    sample_xy,
                    sample_count,
                    output.as_mut_ptr(),
                    output.len(),
                )
            };
            assert_eq!(status, expected_status);
            let report = unsafe { CStr::from_ptr(output.as_ptr()) }.to_string_lossy();
            assert!(!report.is_empty());
        }
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_request_c12_4_surface_pixels_for_test(
                    ptr::null_mut(),
                    points.as_ptr(),
                    1,
                    ptr::null_mut(),
                    256,
                )
            },
            -1
        );
    }
}
