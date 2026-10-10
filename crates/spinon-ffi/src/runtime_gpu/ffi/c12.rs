use super::super::{
    RUNTIME_CSS_C12_1_POSITION_FIXTURE_SOURCE, RUNTIME_CSS_C12_2_ABSOLUTE_BLOCK_FIXTURE_SOURCE,
    SpinonRuntimeGpuHost, raw_host,
};
use super::{ERR_ARGUMENT, write_host_status};
use std::ffi::c_char;

const C12_1_POSITION_INVENTORY: &str =
    include_str!("../../../../../tests/fixtures/css/c12/position-static-relative-inventory.json");

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

#[cfg(test)]
mod tests {
    use super::position_state_source;

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
}
