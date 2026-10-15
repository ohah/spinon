use super::super::super::{RuntimeGpuHost, SpinonRuntimeGpuHost, raw_host};
use super::super::{ERR_ARGUMENT, write_host_status};
use crate::runtime_gpu::fixtures::c13_3_scroll_gesture_host::{C13_3GestureState, C13_3WheelInput};
use spinon_render::{
    RuntimeScrollGestureCancelReason, RuntimeScrollMomentumPhase, RuntimeScrollWheelSource,
    RuntimeScrollWheelUnit,
};
use std::collections::HashMap;
use std::ffi::c_char;
use std::ptr;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};

fn registry() -> &'static Mutex<HashMap<usize, Arc<C13_3GestureState>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<usize, Arc<C13_3GestureState>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock_registry() -> MutexGuard<'static, HashMap<usize, Arc<C13_3GestureState>>> {
    registry().lock().unwrap_or_else(PoisonError::into_inner)
}

fn state_for_handle(host: *mut SpinonRuntimeGpuHost) -> Option<Arc<C13_3GestureState>> {
    lock_registry().get(&(host as usize)).map(Arc::clone)
}

pub(in crate::runtime_gpu::ffi) fn unregister_host(host: usize) {
    lock_registry().remove(&host);
}

#[unsafe(no_mangle)]
/// C13.3 Android/iOS gesture 입력 전용 opt-in runtime fixture host를 만듭니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_new_c13_3_gesture_fixture(
    scenario_id: i32,
    output: *mut c_char,
    output_capacity: usize,
) -> *mut SpinonRuntimeGpuHost {
    if output.is_null() || output_capacity == 0 {
        return ptr::null_mut();
    }
    let (host, report) = match RuntimeGpuHost::new_c13_3_scroll_gesture_fixture(scenario_id) {
        Ok(host) => host,
        Err(error) => {
            crate::write_report(output, output_capacity, &error);
            return ptr::null_mut();
        }
    };
    let Some(state) = host.c13_3_gesture_state() else {
        crate::write_report(output, output_capacity, "C13.3 input state가 없습니다");
        return ptr::null_mut();
    };
    if !crate::write_report(output, output_capacity, &report) {
        return ptr::null_mut();
    }
    let handle = Box::into_raw(Box::new(host)).cast::<SpinonRuntimeGpuHost>();
    lock_registry().insert(handle as usize, state);
    handle
}

#[unsafe(no_mangle)]
/// 고정 C13.3 400×240 CSS scene을 runtime에서 계산하고 gesture snapshot을 준비합니다.
///
/// # Safety
/// `host`는 live C13.3 fixture host여야 합니다. 대기 허용 background executor에서 호출하고
/// 같은 host의 해제와 경합시키면 안 됩니다. `output`은 쓰기 가능해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_eval_c13_3_gesture_fixture(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 || layout_timeout_millis == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.3 gesture fixture host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    match host.eval_c13_3_scroll_gesture_fixture(layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, error)) => write_host_status(status, error, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// Active pan 중 fixture 전용 inline style 변경을 평가하고 새 snapshot을 게시합니다.
///
/// # Safety
/// `host`는 live C13.3 fixture host여야 하며 같은 host의 해제와 경합시키면 안 됩니다.
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_apply_c13_3_style_invalidation(
    host: *mut SpinonRuntimeGpuHost,
    layout_timeout_millis: u64,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 || layout_timeout_millis == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.3 style invalidation host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    match host.apply_c13_3_style_invalidation(layout_timeout_millis) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, error)) => write_host_status(status, error, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// Native callback에서 호출하는 bounded queue 삽입 함수입니다. Host를 역참조하지 않습니다.
///
/// `action`: 0 Begin, 1 PanBegan, 2 Move, 3 End, 4 Cancel. `reason`은 Cancel일 때 0 host,
/// 1 second pointer, 2 queue saturation, 3 surface lifecycle, 4 native recognizer failure입니다.
///
/// # Safety
/// `host`는 현재 C13.3 fixture host handle이어야 하고 host free와 경합하면 안 됩니다.
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_enqueue_c13_3_gesture(
    host: *mut SpinonRuntimeGpuHost,
    action: i32,
    surface_generation: u64,
    gesture_id: u64,
    pointer_id: u64,
    x_or_cumulative_x_css_px: f32,
    y_or_cumulative_y_css_px: f32,
    coordinate_scale: f32,
    reason: i32,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(state) = state_for_handle(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.3 gesture queue host handle이 유효하지 않습니다".to_owned(),
            output,
            output_capacity,
        );
    };
    let result = match action {
        0 => state.enqueue_begin(
            surface_generation,
            gesture_id,
            pointer_id,
            x_or_cumulative_x_css_px,
            y_or_cumulative_y_css_px,
            coordinate_scale,
        ),
        1 => state.enqueue_pan_began(
            surface_generation,
            gesture_id,
            pointer_id,
            coordinate_scale,
            x_or_cumulative_x_css_px,
            y_or_cumulative_y_css_px,
        ),
        2 => state.enqueue_move(
            surface_generation,
            gesture_id,
            pointer_id,
            coordinate_scale,
            x_or_cumulative_x_css_px,
            y_or_cumulative_y_css_px,
        ),
        3 | 4 => {
            let cancel_reason = match reason {
                0 => RuntimeScrollGestureCancelReason::HostCancelled,
                1 => RuntimeScrollGestureCancelReason::SecondPointer,
                2 => RuntimeScrollGestureCancelReason::InputQueueSaturated,
                3 => RuntimeScrollGestureCancelReason::SurfaceGenerationChanged,
                4 => RuntimeScrollGestureCancelReason::NativeRecognizerFailed,
                _ => {
                    return write_host_status(
                        ERR_ARGUMENT,
                        format!("C13.3 cancel reason이 올바르지 않습니다: {reason}"),
                        output,
                        output_capacity,
                    );
                }
            };
            state.enqueue_terminal(
                action == 3,
                surface_generation,
                gesture_id,
                pointer_id,
                cancel_reason,
            )
        }
        _ => Err(format!(
            "C13.3 gesture action이 올바르지 않습니다: {action}"
        )),
    };
    match result {
        Ok(report) => write_host_status(0, format!("C13.3 {report}"), output, output_capacity),
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// Fixture 전용 normalized CSS px wheel/trackpad sample을 bounded queue에 추가합니다.
///
/// `intent`: 0 scroll, 1 host가 분류한 magnification/zoom. 나머지 enum 값은 거부합니다.
///
/// # Safety
/// `host`는 현재 live C13.3 fixture host여야 하고 host free와 경합하면 안 됩니다.
/// `output`은 `output_capacity` 바이트를 쓸 수 있어야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_enqueue_c13_3_wheel(
    host: *mut SpinonRuntimeGpuHost,
    intent: i32,
    surface_generation: u64,
    point_x_css_px: f32,
    point_y_css_px: f32,
    delta_x_css_px: f32,
    delta_y_css_px: f32,
    raw_delta_x: f32,
    raw_delta_y: f32,
    source_unit: i32,
    coordinate_scale: f32,
    source: i32,
    momentum_phase: i32,
    modifier_flags: u32,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return ERR_ARGUMENT;
    }
    let Some(state) = state_for_handle(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.3 wheel fixture host handle이 유효하지 않습니다".to_owned(),
            output,
            output_capacity,
        );
    };
    let source_unit = match source_unit {
        0 => RuntimeScrollWheelUnit::CssPixels,
        1 => RuntimeScrollWheelUnit::AndroidPhysicalPixels,
        2 => RuntimeScrollWheelUnit::UIKitPoints,
        3 => RuntimeScrollWheelUnit::WheelLines,
        4 => RuntimeScrollWheelUnit::WheelPages,
        _ => {
            return write_host_status(
                ERR_ARGUMENT,
                format!("C13.3 wheel source unit이 올바르지 않습니다: {source_unit}"),
                output,
                output_capacity,
            );
        }
    };
    let source = match source {
        0 => RuntimeScrollWheelSource::MouseWheel,
        1 => RuntimeScrollWheelSource::TrackpadScroll,
        _ => {
            return write_host_status(
                ERR_ARGUMENT,
                format!("C13.3 wheel source가 올바르지 않습니다: {source}"),
                output,
                output_capacity,
            );
        }
    };
    let momentum_phase = match momentum_phase {
        0 => RuntimeScrollMomentumPhase::None,
        1 => RuntimeScrollMomentumPhase::Begin,
        2 => RuntimeScrollMomentumPhase::Change,
        3 => RuntimeScrollMomentumPhase::End,
        _ => {
            return write_host_status(
                ERR_ARGUMENT,
                format!("C13.3 wheel momentum phase가 올바르지 않습니다: {momentum_phase}"),
                output,
                output_capacity,
            );
        }
    };
    if !matches!(intent, 0 | 1) {
        return write_host_status(
            ERR_ARGUMENT,
            format!("C13.3 wheel intent가 올바르지 않습니다: {intent}"),
            output,
            output_capacity,
        );
    }
    match state.enqueue_wheel(C13_3WheelInput {
        scroll_intent: intent == 0,
        surface_generation,
        point_x_css_px,
        point_y_css_px,
        delta_x_css_px,
        delta_y_css_px,
        raw_delta_x,
        raw_delta_y,
        source_unit,
        coordinate_scale,
        source,
        momentum_phase,
        modifier_flags,
    }) {
        Ok(report) => {
            write_host_status(0, format!("C13.3 wheel {report}"), output, output_capacity)
        }
        Err(error) => write_host_status(ERR_ARGUMENT, error, output, output_capacity),
    }
}

#[unsafe(no_mangle)]
/// Background executor에서 ordered queue event를 session에 적용하고 snapshot을 게시합니다.
///
/// # Safety
/// `host`는 live C13.3 fixture host여야 합니다. 한 host에서는 consumer를 하나만 두고,
/// host 해제와 경합시키지 마세요. `output`은 쓰기 가능해야 합니다.
pub unsafe extern "C" fn spinon_runtime_gpu_host_process_c13_3_gesture_queue(
    host: *mut SpinonRuntimeGpuHost,
    max_events: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 || max_events == 0 {
        return ERR_ARGUMENT;
    }
    let Some(host) = raw_host(host) else {
        return write_host_status(
            ERR_ARGUMENT,
            "C13.3 gesture queue host가 null입니다".to_owned(),
            output,
            output_capacity,
        );
    };
    match host.process_c13_3_scroll_gesture_queue(max_events) {
        Ok(report) => write_host_status(0, report, output, output_capacity),
        Err((status, error)) => write_host_status(status, error, output, output_capacity),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        spinon_runtime_gpu_host_apply_c13_3_style_invalidation,
        spinon_runtime_gpu_host_enqueue_c13_3_gesture, spinon_runtime_gpu_host_enqueue_c13_3_wheel,
        spinon_runtime_gpu_host_eval_c13_3_gesture_fixture,
        spinon_runtime_gpu_host_new_c13_3_gesture_fixture,
        spinon_runtime_gpu_host_process_c13_3_gesture_queue,
    };
    use std::{
        ffi::{CStr, c_char},
        ptr,
    };

    #[test]
    fn c13_3_ffi_rejects_invalid_handles_and_actions() {
        let mut output = [0 as c_char; 256];
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_eval_c13_3_gesture_fixture(
                    ptr::null_mut(),
                    1000,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_apply_c13_3_style_invalidation(
                    ptr::null_mut(),
                    1000,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_enqueue_c13_3_gesture(
                    ptr::null_mut(),
                    9,
                    1,
                    1,
                    1,
                    0.0,
                    0.0,
                    1.0,
                    0,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_enqueue_c13_3_wheel(
                    ptr::null_mut(),
                    0,
                    1,
                    132.0,
                    112.0,
                    0.0,
                    40.0,
                    0.0,
                    40.0,
                    0,
                    1.0,
                    1,
                    0,
                    0,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert_eq!(
            unsafe {
                spinon_runtime_gpu_host_process_c13_3_gesture_queue(
                    ptr::null_mut(),
                    1,
                    output.as_mut_ptr(),
                    output.len(),
                )
            },
            -1
        );
        assert!(
            unsafe { spinon_runtime_gpu_host_new_c13_3_gesture_fixture(0, ptr::null_mut(), 0) }
                .is_null()
        );
        let unknown_scenario = unsafe {
            spinon_runtime_gpu_host_new_c13_3_gesture_fixture(99, output.as_mut_ptr(), output.len())
        };
        assert!(unknown_scenario.is_null());
        assert!(
            unsafe { CStr::from_ptr(output.as_ptr()) }
                .to_string_lossy()
                .contains("scenario id=99")
        );
    }
}
