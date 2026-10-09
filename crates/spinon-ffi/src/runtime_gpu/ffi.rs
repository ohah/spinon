use super::{
    RUNTIME_AUTHOR_STYLESHEETS_FIXTURE_SOURCE, RUNTIME_GPU_FIXTURE_SOURCE, RuntimeGpuHost,
    SpinonRuntimeGpuHost, raw_host,
};
use std::ffi::{CStr, c_char, c_void};
use std::ptr;

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
/// Android native window에서 WGPU 표면을 생성합니다. 호출과 이후 표면 접근은 같은 render thread여야 합니다.
///
/// # Safety
/// `host`는 live GPU host이고 `native_window`는 renderer가 파괴될 때까지 유효해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_create_android(
    host: *mut SpinonRuntimeGpuHost,
    native_window: *mut c_void,
    width: u32,
    height: u32,
    backend: u32,
    output: *mut c_char,
    output_capacity: usize,
) -> *mut c_void {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let Some(host) = raw_host(host) else {
        crate::write_report(output, output_capacity, "runtime GPU host가 null입니다");
        return ptr::null_mut();
    };
    match unsafe { host.create_android(native_window, width, height, backend) } {
        Ok(report) => {
            if crate::write_report(output, output_capacity, &report) {
                host as *const RuntimeGpuHost as *mut c_void
            } else {
                let _ = host.destroy_renderer();
                ptr::null_mut()
            }
        }
        Err(error) => {
            crate::write_report(output, output_capacity, &error.to_string());
            ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
/// UIKit view를 읽어 WGPU surface를 준비합니다. UIKit 접근 때문에 반드시 메인 스레드에서 호출합니다.
///
/// # Safety
/// `host`는 live GPU host이고 `view`는 호출 동안 유효한 UIView여야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_prepare_uikit_surface(
    host: *mut SpinonRuntimeGpuHost,
    view: *mut c_void,
    backend: u32,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        crate::write_report(output, output_capacity, "runtime GPU host가 null입니다");
        return ERR_ARGUMENT;
    };
    let result = renderer_result(unsafe { host.prepare_uikit_surface(view, backend) });
    let status = write_host_coded_result(result, output, output_capacity);
    if status == ERR_OUTPUT {
        let _ = host.destroy_renderer();
    }
    status
}

#[unsafe(no_mangle)]
/// 메인 스레드에서 준비한 UIKit surface에 GPU 장치와 renderer를 만듭니다.
///
/// # Safety
/// `host`는 살아 있어야 하며 호출은 다른 renderer 생성·resize·draw·destroy와 직렬화해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_create_uikit(
    host: *mut SpinonRuntimeGpuHost,
    width: u32,
    height: u32,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    let result = renderer_result(host.create_uikit(width, height));
    let status = write_host_coded_result(result, output, output_capacity);
    if status == ERR_OUTPUT {
        let _ = host.destroy_renderer();
    }
    status
}

#[unsafe(no_mangle)]
/// UIKit용 GPU 장치 초기화 뒤 surface를 구성합니다. CAMetalLayer 속성 변경 때문에 메인 스레드에서 호출합니다.
///
/// # Safety
/// `host`는 살아 있어야 하며 호출은 renderer 생성·resize·draw·destroy와 직렬화해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_configure_uikit_surface(
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
    let result = renderer_result(host.configure_uikit_surface());
    write_host_coded_result(result, output, output_capacity)
}

#[unsafe(no_mangle)]
/// Renderer 크기를 변경하고 제출 대기 장면을 무효화합니다.
///
/// # Safety
/// `host` renderer는 살아 있어야 하며 호출은 해당 renderer의 draw·destroy와 직렬화해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_resize(
    host: *mut SpinonRuntimeGpuHost,
    width: u32,
    height: u32,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return ERR_ARGUMENT;
    };
    let result = host
        .resize(width, height)
        .map(|()| "surface resized and prior scene invalidated".to_owned());
    write_host_coded_result(result, output, output_capacity)
}

#[unsafe(no_mangle)]
/// 최신 runtime 장면을 제출합니다. 계산 대기는 하지 않으며 장면이 비어 있으면 배경만 지웁니다.
///
/// # Safety
/// `host` renderer는 살아 있어야 하며 호출은 해당 renderer의 resize·destroy와 직렬화해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_draw(
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
    write_host_coded_result(host.draw(), output, output_capacity)
}

#[cfg(feature = "c04-runtime-gpu-test-hooks")]
#[unsafe(no_mangle)]
/// 내부 검증 빌드에서 다음 WGPU draw 한 번의 오류 전파를 시험용으로 설정합니다.
///
/// # Safety
/// `host`는 살아 있어야 하며 호출은 draw·resize·destroy와 같은 render queue에서 직렬화해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_inject_next_draw_failure_for_test(
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
    write_host_coded_result(
        host.inject_next_draw_failure_for_test(),
        output,
        output_capacity,
    )
}

#[unsafe(no_mangle)]
/// platform surface generation을 무효화하고 대기 중인 surface 사용을 끝낸 뒤 render queue에서 WGPU renderer를 파괴합니다.
///
/// # Safety
/// `host`는 live GPU host여야 하며 호출은 draw·resize와 같은 render queue에서 직렬화해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_destroy_renderer(
    host: *mut SpinonRuntimeGpuHost,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    if let Some(host) = raw_host(host) {
        return write_host_coded_result(
            host.destroy_renderer()
                .map(|()| "renderer destroyed".to_owned()),
            output,
            output_capacity,
        );
    }
    ERR_ARGUMENT
}

#[unsafe(no_mangle)]
/// background executor 종료 후, render queue를 비운 다음 GPU host를 해제합니다.
///
/// # Safety
/// `host`는 null 또는 `spinon_runtime_gpu_host_new`가 반환한 live 포인터여야 합니다. 다른 host
/// 호출이 실행 중이지 않아야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_free(host: *mut SpinonRuntimeGpuHost) {
    if !host.is_null() {
        drop(unsafe { Box::from_raw(host.cast::<RuntimeGpuHost>()) });
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
