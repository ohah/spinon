use super::{
    RUNTIME_AUTHOR_STYLESHEETS_FIXTURE_SOURCE, RUNTIME_GPU_FIXTURE_SOURCE, RuntimeGpuHost,
    SpinonRuntimeGpuHost, raw_host,
};
use std::ffi::{CStr, c_char};
use std::ptr;

mod c06;
mod c07;
mod c08;
mod c09;
mod c10;
mod c12;
mod surface;

const ERR_ARGUMENT: i32 = -1;
const ERR_OUTPUT: i32 = -3;

#[unsafe(no_mangle)]
/// 전용 V8 세션을 생성하고 C04.10 GPU paint profile을 사용합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new() {
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
/// C05.2 `@property` 검증 fixture 전용 CSS profile로 V8 GPU host를 생성합니다.
/// 일반 runtime host와 등록 속성 fixture profile은 서로 분리됩니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_registered_properties_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_registered_properties_fixture() {
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
/// C08 Block 흐름과 기본 배경 페인트 전용 runtime host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_block_paint_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_block_paint_fixture() {
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
/// C09.1 일반 Block 흐름 fixture 전용 V8 GPU host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_block_formatting_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_block_formatting_fixture() {
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
/// 화면 환경·surface 변경을 큐에 넣기 전에 현재 장면을 비동기로 무효화합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 합니다. 같은 host의 해제와 경합시키면 안 됩니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_begin_presentation_update(
    host: *mut SpinonRuntimeGpuHost,
) -> u64 {
    raw_host(host)
        .and_then(|host| host.invalidate_scene().ok())
        .unwrap_or(0)
}

#[unsafe(no_mangle)]
/// CSS viewport와 색상 체계를 설정하고 해당 장면 계산이 끝날 때까지 기다립니다.
/// 제한 시간은 CSS 레이아웃 결과 대기에만 적용하며, V8 JavaScript 실행을 취소하지 않습니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 합니다. 이 함수를 기다림 허용 background executor에서
/// 호출하고, 같은 host 해제와 경합시키면 안 됩니다. `output`은 쓰기 가능해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_set_environment(
    host: *mut SpinonRuntimeGpuHost,
    width_css_px: f32,
    height_css_px: f32,
    device_scale_factor: f32,
    dark: i32,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 || !matches!(dark, 0 | 1) {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    let result = host.set_environment(
        width_css_px,
        height_css_px,
        device_scale_factor,
        dark == 1,
        layout_timeout_millis,
    );
    write_host_coded_result(result, output, output_capacity)
}

#[unsafe(no_mangle)]
/// JavaScript를 동기 평가하고 같은 revision tuple의 layout 장면을 발행합니다.
/// 제한 시간은 평가 후 CSS 레이아웃 결과 대기에만 적용하며, JavaScript 실행을 취소하지 않습니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 합니다. `source`는 NUL 종료 UTF-8이어야 합니다. 이 함수를
/// background executor에서 호출하고, 같은 host 해제와 경합시키면 안 됩니다.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval(
    host: *mut SpinonRuntimeGpuHost,
    source: *const c_char,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if source.is_null() || output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    let source = match unsafe { CStr::from_ptr(source) }.to_str() {
        Ok(source) => source,
        Err(_) => return ERR_ARGUMENT,
    };
    match host.eval(source, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 저장소의 단일 C04.10 JavaScript fixture를 실제 V8 세션에서 실행합니다.
/// 제한 시간은 평가 후 CSS 레이아웃 결과 대기에만 적용하며, JavaScript 실행을 취소하지 않습니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_fixture(
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
    match host.eval(RUNTIME_GPU_FIXTURE_SOURCE, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 저장소의 C04.11 HTML style 요소 fixture를 실제 V8 세션에서 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_author_stylesheets_fixture(
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
        RUNTIME_AUTHOR_STYLESHEETS_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 현재 C04.10 fixture의 살아 있는 DOM 요소에 C05.1 사용자 지정 속성을 적용합니다.
///
/// V8 세션·HostDocument를 새로 만들지 않고 기존 `style` setter를 통해 layout·paint를
/// 갱신합니다. 제한 시간은 평가 후 CSS 결과 대기에만 적용합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_custom_properties_fixture(
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
        super::RUNTIME_CSS_CUSTOM_PROPERTIES_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 저장소의 C05.2 등록 사용자 지정 속성 fixture를 실제 V8·HostDocument 경로에서 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_registered_properties_fixture(
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
        super::RUNTIME_CSS_REGISTERED_PROPERTIES_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 저장소의 C05.3 detached-node fixture를 실제 V8·HostDocument 경로에서 한 단계 실행합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_runtime_result_cache_fixture(
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
        super::RUNTIME_CSS_RESULT_CACHE_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// 실제 V8 HostDocument에서 inline style 하위 트리 재계산 fixture를 한 단계 실행합니다.
///
/// 첫 호출은 단일 root와 좌우 branch를 만들고, 이후 호출은 왼쪽 branch의 사용자 지정 속성만
/// 번갈아 바꿉니다. 완료 보고에는 cascade 재계산·재사용·상속 문맥 계산 수가 포함됩니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_incremental_restyle_fixture(
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
        super::RUNTIME_CSS_INCREMENTAL_RESTYLE_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C06.1 percentage width·height·flex-basis를 실제 V8·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_percentage_dimensions_fixture(
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
        super::RUNTIME_CSS_PERCENTAGE_DIMENSIONS_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C06.2 percentage margin·padding·gap을 실제 V8·Stylo·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_spacing_percentages_fixture(
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
        super::RUNTIME_CSS_SPACING_PERCENTAGES_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C06.3 절대 CSS 길이 단위를 실제 V8·Stylo·Taffy runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_absolute_lengths_fixture(
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
        super::RUNTIME_CSS_ABSOLUTE_LENGTHS_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// C06.4 em/rem fixture를 실제 V8·Stylo·Taffy·WGPU runtime에서 평가합니다.
///
/// # Safety
/// `host`는 살아 있는 GPU host여야 하고 다른 host 호출·해제와 경합시키면 안 됩니다. 호출은
/// 기다림 허용 background executor에서 해야 합니다. `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_font_relative_units_fixture(
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
        super::RUNTIME_CSS_FONT_RELATIVE_UNITS_FIXTURE_SOURCE,
        layout_timeout_millis,
    ) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

fn write_host_coded_result(
    result: Result<String, (i32, String)>,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    match result {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

fn renderer_result(
    result: Result<String, super::RuntimeRendererError>,
) -> Result<String, (i32, String)> {
    result.map_err(|error| {
        let status = if matches!(error, super::RuntimeRendererError::InvalidArgument(_)) {
            ERR_ARGUMENT
        } else {
            -11
        };
        (status, error.to_string())
    })
}

fn write_host_status(
    status: i32,
    report: String,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if !crate::write_report(output, output_capacity, &report) {
        return ERR_OUTPUT;
    }
    status
}
