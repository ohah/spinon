use serde_json::{Value, json};
use spinon_runtime::{
    RuntimeSession, RuntimeUaCascadeCompleted, RuntimeUaCascadeKey, RuntimeUaCascadeSnapshot,
};
use spinon_style::{
    CssColorScheme, CssMediaEnvironment, CssPointerCapabilities, CssPrimaryPointer, CssViewport,
};
use std::ffi::c_char;

const ERR_ARGUMENT: i32 = -1;
const ERR_OUTPUT_TOO_SMALL: i32 = -3;
const VALID_POINTER_FLAGS: u32 = 0b111;

fn color_scheme_from_abi(value: i32) -> Option<CssColorScheme> {
    match value {
        0 => Some(CssColorScheme::Light),
        1 => Some(CssColorScheme::Dark),
        _ => None,
    }
}

fn primary_pointer_from_abi(value: i32) -> Option<CssPrimaryPointer> {
    match value {
        0 => Some(CssPrimaryPointer::None),
        1 => Some(CssPrimaryPointer::Coarse),
        2 => Some(CssPrimaryPointer::Fine),
        _ => None,
    }
}

fn boolean_from_abi(value: i32) -> Option<bool> {
    match value {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

fn media_environment_from_abi(
    color_scheme: i32,
    primary_pointer: i32,
    primary_hover: i32,
    all_pointer_flags: u32,
) -> Option<CssMediaEnvironment> {
    let color_scheme = color_scheme_from_abi(color_scheme)?;
    let primary_pointer = primary_pointer_from_abi(primary_pointer)?;
    let primary_hover = boolean_from_abi(primary_hover)?;
    if all_pointer_flags & !VALID_POINTER_FLAGS != 0 {
        return None;
    }
    let environment = CssMediaEnvironment {
        color_scheme,
        primary_pointer,
        primary_hover,
        all_pointers: CssPointerCapabilities {
            coarse: all_pointer_flags & 0b001 != 0,
            fine: all_pointer_flags & 0b010 != 0,
            hover: all_pointer_flags & 0b100 != 0,
        },
    };
    CssViewport {
        media_environment: environment,
        ..CssViewport::C04_FIXTURE
    }
    .validate()
    .ok()?;
    Some(environment)
}

/// 세션에 명시적인 CSS viewport와 media 환경을 전달합니다.
///
/// # Safety
/// `session`은 아직 해제되지 않은 유효 세션 포인터여야 하고, `environment_revision`은 쓰기
/// 가능한 `uint64_t`를 가리켜야 합니다. 이 호출과 session free를 병행하면 안 됩니다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spinon_runtime_session_set_ua_cascade_environment(
    session: *mut crate::runtime_session::SpinonRuntimeSession,
    width_css_px: f32,
    height_css_px: f32,
    device_scale_factor: f32,
    color_scheme: i32,
    primary_pointer: i32,
    primary_hover: i32,
    all_pointer_flags: u32,
    environment_revision: *mut u64,
) -> i32 {
    if session.is_null() || environment_revision.is_null() {
        return ERR_ARGUMENT;
    }
    let Some(media_environment) = media_environment_from_abi(
        color_scheme,
        primary_pointer,
        primary_hover,
        all_pointer_flags,
    ) else {
        return ERR_ARGUMENT;
    };
    match unsafe { &*session.cast::<RuntimeSession>() }.set_ua_cascade_environment(
        width_css_px,
        height_css_px,
        device_scale_factor,
        media_environment,
    ) {
        Ok(revision) => {
            unsafe { environment_revision.write(revision.get()) };
            0
        }
        Err(error) => error.status_code(),
    }
}

/// 현재 UA cascade 상태를 한 snapshot으로 JSON 직렬화해 복사합니다.
///
/// # Safety
/// `session`은 아직 해제되지 않은 유효 세션 포인터여야 합니다. `required_capacity`는 쓰기
/// 가능한 `size_t`를 가리켜야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야
/// 합니다. 호출 중 session free와 병행하면 안 됩니다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spinon_runtime_session_copy_ua_cascade_json(
    session: *mut crate::runtime_session::SpinonRuntimeSession,
    output: *mut c_char,
    output_capacity: usize,
    required_capacity: *mut usize,
) -> i32 {
    if session.is_null()
        || output.is_null()
        || required_capacity.is_null()
        || output_capacity == 0
        || output_capacity > isize::MAX as usize
    {
        return ERR_ARGUMENT;
    }
    let snapshot = unsafe { &*session.cast::<RuntimeSession>() }.ua_cascade_snapshot();
    let json = snapshot_json(&snapshot).to_string();
    let Some(required) = json.len().checked_add(1) else {
        unsafe { output.write(0) };
        return ERR_ARGUMENT;
    };
    unsafe { required_capacity.write(required) };
    let output = unsafe { std::slice::from_raw_parts_mut(output.cast::<u8>(), output_capacity) };
    copy_json(json.as_bytes(), output)
}

fn copy_json(json: &[u8], output: &mut [u8]) -> i32 {
    let Some(required) = json.len().checked_add(1) else {
        if let Some(first) = output.first_mut() {
            *first = 0;
        }
        return ERR_ARGUMENT;
    };
    if output.len() < required {
        if let Some(first) = output.first_mut() {
            *first = 0;
        }
        return ERR_OUTPUT_TOO_SMALL;
    }
    output[..json.len()].copy_from_slice(json);
    output[json.len()] = 0;
    0
}

fn snapshot_json(snapshot: &RuntimeUaCascadeSnapshot) -> Value {
    json!({
        "schema": "spinon.runtime.ua-cascade.v1",
        "state": snapshot.state.as_str(),
        "requested": snapshot.requested.map(key_json),
        "completed": snapshot.completed.as_deref().map(completed_json),
        "error": snapshot.error,
    })
}

fn key_json(key: RuntimeUaCascadeKey) -> Value {
    json!({
        "generation": key.generation,
        "documentRevision": key.document_revision,
        "renderTreeRevision": key.render_tree_revision,
        "styleRevision": key.style_revision,
        "environmentRevision": key.environment_revision,
    })
}

fn completed_json(completed: &RuntimeUaCascadeCompleted) -> Value {
    let roots = completed
        .roots
        .iter()
        .map(|root| {
            let elements = root
                .styles
                .elements
                .iter()
                .map(|element| {
                    json!({
                        "nodeId": element.node_id.get(),
                        "properties": element.properties,
                    })
                })
                .collect::<Vec<_>>();
            let diagnostics = root
                .styles
                .diagnostics
                .iter()
                .map(|entry| {
                    json!({
                        "sourceId": entry.source_id,
                        "nodeId": entry.node_id.map(|id| id.get()),
                        "line": entry.diagnostic.line,
                        "column": entry.diagnostic.column,
                        "message": entry.diagnostic.message,
                    })
                })
                .collect::<Vec<_>>();
            json!({
                "rootNodeId": root.root_node_id,
                "elements": elements,
                "diagnostics": diagnostics,
            })
        })
        .collect::<Vec<_>>();
    json!({
        "key": key_json(completed.key),
        "computationDurationUs": completed.computation_duration_us,
        "roots": roots,
    })
}

#[path = "runtime_ua_cascade/probe.rs"]
mod probe;

#[cfg(test)]
mod tests {
    use super::{
        ERR_ARGUMENT, ERR_OUTPUT_TOO_SMALL, copy_json, media_environment_from_abi, snapshot_json,
        spinon_runtime_session_copy_ua_cascade_json,
        spinon_runtime_session_set_ua_cascade_environment,
    };
    use spinon_runtime::{RuntimeUaCascadeKey, RuntimeUaCascadeSnapshot, RuntimeUaCascadeState};

    #[test]
    fn abi_environment_rejects_unknown_flags_and_non_boolean_values() {
        assert!(media_environment_from_abi(0, 1, 1, 0b110).is_none());
        assert!(media_environment_from_abi(0, 2, 2, 0b110).is_none());
        assert!(media_environment_from_abi(0, 2, 1, 0b1000).is_none());
        assert!(media_environment_from_abi(7, 2, 1, 0b110).is_none());
        assert!(media_environment_from_abi(0, 9, 1, 0b110).is_none());
    }

    #[test]
    fn abi_environment_preserves_valid_media_capabilities() {
        let environment = media_environment_from_abi(1, 1, 0, 0b001).unwrap();
        assert_eq!(environment.color_scheme, spinon_style::CssColorScheme::Dark);
        assert_eq!(
            environment.primary_pointer,
            spinon_style::CssPrimaryPointer::Coarse
        );
        assert!(environment.all_pointers.coarse);
        assert!(!environment.all_pointers.fine);
        assert!(!environment.all_pointers.hover);
    }

    #[test]
    fn state_json_always_has_stable_schema_and_no_implicit_request() {
        let json = snapshot_json(&RuntimeUaCascadeSnapshot {
            state: RuntimeUaCascadeState::NotConfigured,
            requested: None,
            completed: None,
            error: None,
        });
        assert_eq!(json["schema"], "spinon.runtime.ua-cascade.v1");
        assert_eq!(json["state"], "not_configured");
        assert!(json["requested"].is_null());
        assert!(json["completed"].is_null());
        assert!(json["error"].is_null());
    }

    #[test]
    fn failed_state_json_has_an_error_and_never_exposes_partial_roots() {
        let json = snapshot_json(&RuntimeUaCascadeSnapshot {
            state: RuntimeUaCascadeState::Failed,
            requested: Some(RuntimeUaCascadeKey {
                generation: 1,
                document_revision: 2,
                render_tree_revision: 2,
                style_revision: 0,
                environment_revision: 0,
            }),
            completed: None,
            error: Some("fixture error".to_owned()),
        });
        assert_eq!(json["state"], "failed");
        assert!(json["completed"].is_null());
        assert_eq!(json["error"], "fixture error");
    }

    #[test]
    fn output_short_buffer_contract_uses_nul_guard_and_capacity_status() {
        let json = b"{\"state\":\"pending\"}";
        let mut output = [0x7f_u8; 4];
        let status = copy_json(json, &mut output);
        assert_eq!(status, ERR_OUTPUT_TOO_SMALL);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn exact_capacity_copies_complete_json_and_terminator() {
        let json = b"{\"state\":\"pending\"}";
        let mut output = vec![0_u8; json.len() + 1];
        assert_eq!(copy_json(json, &mut output), 0);
        assert_eq!(&output[..json.len()], json);
        assert_eq!(output[json.len()], 0);
    }

    #[test]
    fn null_abi_inputs_have_documented_argument_status() {
        assert_eq!(ERR_ARGUMENT, -1);
        assert_eq!(
            unsafe {
                spinon_runtime_session_copy_ua_cascade_json(
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                )
            },
            ERR_ARGUMENT
        );
        assert_eq!(
            unsafe {
                spinon_runtime_session_set_ua_cascade_environment(
                    std::ptr::null_mut(),
                    390.0,
                    844.0,
                    3.0,
                    0,
                    1,
                    0,
                    1,
                    std::ptr::null_mut(),
                )
            },
            ERR_ARGUMENT
        );
    }
}
