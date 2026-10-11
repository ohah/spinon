use super::super::{
    RuntimeGpuHost, SpinonRuntimeGpuHost, c10_3_5_positioned_flex_fixture_source, raw_host,
};
use super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;
use std::ptr;

#[unsafe(no_mangle)]
/// C10.3.5 positioned Flex runtime fixture 전용 V8 GPU host를 생성합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c10_3_5_positioned_flex_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c10_3_5_positioned_flex_fixture() {
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
/// 고정 Chrome inventory의 C10.3.5 subset을 V8·Stylo·Taffy·WGPU 경로에서 실행합니다.
/// 결과에는 DOM source preorder frame이 포함됩니다.
///
/// # Safety
/// `host`는 살아 있는 C10.3.5 runtime host여야 합니다. 같은 host의 호출·해제와 경합시키면 안 됩니다.
/// 호출은 기다림 허용 background executor에서 해야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c10_3_5_positioned_flex_fixture(
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
    let source = c10_3_5_positioned_flex_fixture_source();
    match host.eval(&source, layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, report)) => write_host_status(status, report, output, output_capacity),
    }
}

#[cfg(test)]
mod tests {
    use super::c10_3_5_positioned_flex_fixture_source;

    #[test]
    fn runtime_fixture_embeds_the_same_geometry_inventory_as_the_chrome_oracle() {
        let source = c10_3_5_positioned_flex_fixture_source();
        for required in [
            "__spinonC1035FixtureInventory",
            "__spinonC1035RuntimeInventory",
            "c1035-runtime-root",
            "paint-order",
            "block-wrapper-static-position",
            "__spinonC1035RuntimeNodeIds",
        ] {
            assert!(
                source.contains(required),
                "runtime fixture misses {required}"
            );
        }
    }
}
