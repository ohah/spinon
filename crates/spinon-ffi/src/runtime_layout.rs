use serde_json::{Value, json};
use spinon_runtime::{
    RuntimeLayoutCompleted, RuntimeLayoutFailure, RuntimeLayoutSnapshot, RuntimeSession,
    RuntimeUaCascadeKey,
};
use std::ffi::c_char;

const ERR_ARGUMENT: i32 = -1;

/// 현재 runtime layout 상태를 JSON으로 복사합니다. 계산 완료는 기다리지 않습니다.
///
/// # Safety
/// `session`은 아직 해제되지 않은 유효 세션 포인터여야 합니다. `required_capacity`는 쓰기
/// 가능한 `size_t`를 가리켜야 하며, `output`은 `output_capacity` 바이트를 쓸 수 있어야
/// 합니다. 두 출력 범위는 서로 겹치거나 session 저장 공간과 겹치면 안 되며, 호출 중
/// session free와 병행하면 안 됩니다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spinon_runtime_session_copy_layout_json(
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
    let snapshot = unsafe { &*session.cast::<RuntimeSession>() }.layout_snapshot();
    let json = layout_snapshot_json(&snapshot).to_string();
    let Some(required) = json.len().checked_add(1) else {
        unsafe { output.write(0) };
        return ERR_ARGUMENT;
    };
    unsafe { required_capacity.write(required) };
    let output = unsafe { std::slice::from_raw_parts_mut(output.cast::<u8>(), output_capacity) };
    crate::runtime_ua_cascade::copy_json(json.as_bytes(), output)
}

fn layout_snapshot_json(snapshot: &RuntimeLayoutSnapshot) -> Value {
    json!({
        "schema": "spinon.runtime.layout",
        "state": snapshot.state.as_str(),
        "requested": snapshot.requested.map(key_json),
        "completed": snapshot.completed.as_deref().map(layout_completed_json),
        "diagnostics": snapshot.diagnostics.iter().map(diagnostic_json).collect::<Vec<_>>(),
        "error": snapshot.error.as_ref().map(layout_error_json),
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

fn layout_completed_json(completed: &RuntimeLayoutCompleted) -> Value {
    json!({
        "key": key_json(completed.key),
        "unit": "css-px",
        "projectionDurationUs": completed.projection_duration_us,
        "frames": completed.frames.iter().map(|frame| json!({
            "nodeId": frame.node_id,
            "x": frame.x,
            "y": frame.y,
            "width": frame.width,
            "height": frame.height,
        })).collect::<Vec<_>>(),
    })
}

fn diagnostic_json(entry: &spinon_style::CascadeDiagnostic) -> Value {
    json!({
        "sourceId": entry.source_id,
        "nodeId": entry.node_id.map(|id| id.get()),
        "line": entry.diagnostic.line,
        "column": entry.diagnostic.column,
        "message": entry.diagnostic.message,
    })
}

fn layout_error_json(error: &RuntimeLayoutFailure) -> Value {
    json!({
        "code": error.code,
        "nodeId": error.node_id,
        "property": error.property,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        ERR_ARGUMENT, layout_completed_json, layout_snapshot_json,
        spinon_runtime_session_copy_layout_json,
    };
    use spinon_runtime::{
        RuntimeLayoutCompleted, RuntimeLayoutFailure, RuntimeLayoutFrame, RuntimeLayoutSnapshot,
        RuntimeLayoutState, RuntimeUaCascadeKey,
    };

    #[test]
    fn layout_json_uses_separate_schema_and_stable_failure_fields() {
        let json = layout_snapshot_json(&RuntimeLayoutSnapshot {
            state: RuntimeLayoutState::Failed,
            requested: Some(RuntimeUaCascadeKey {
                generation: 1,
                document_revision: 2,
                render_tree_revision: 2,
                style_revision: 0,
                environment_revision: 0,
            }),
            completed: None,
            diagnostics: Vec::new(),
            error: Some(RuntimeLayoutFailure {
                code: "unsupported_inline_property",
                node_id: Some(3),
                property: Some("color".to_owned()),
            }),
        });
        assert_eq!(json["schema"], "spinon.runtime.layout");
        assert_eq!(json["state"], "failed");
        assert!(json["completed"].is_null());
        assert_eq!(json["error"]["code"], "unsupported_inline_property");
        assert_eq!(json["error"]["property"], "color");
        assert!(json["error"].get("message").is_none());
        assert!(json["error"].get("css").is_none());
    }

    #[test]
    fn layout_json_reports_css_pixels_and_ordered_frames() {
        let json = layout_completed_json(&RuntimeLayoutCompleted {
            key: RuntimeUaCascadeKey {
                generation: 1,
                document_revision: 2,
                render_tree_revision: 2,
                style_revision: 0,
                environment_revision: 0,
            },
            frames: vec![RuntimeLayoutFrame {
                node_id: 7,
                x: 1.0,
                y: 2.0,
                width: 30.0,
                height: 40.0,
            }],
            projection_duration_us: 12,
        });
        assert_eq!(json["unit"], "css-px");
        assert_eq!(json["projectionDurationUs"], 12);
        assert_eq!(json["frames"][0]["nodeId"], 7);
        assert_eq!(json["frames"][0]["x"], 1.0);
    }

    #[test]
    fn layout_json_abi_rejects_invalid_pointers_and_capacity() {
        assert_eq!(ERR_ARGUMENT, -1);
        assert_eq!(
            unsafe {
                spinon_runtime_session_copy_layout_json(
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                )
            },
            ERR_ARGUMENT
        );
    }
}
