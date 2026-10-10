use super::{
    ERR_ARGUMENT, ERR_OUTPUT, RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host, renderer_result,
    write_host_coded_result,
};
use std::ffi::{c_char, c_void};
use std::ptr;

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
