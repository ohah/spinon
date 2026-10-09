use super::super::spinon_runtime_session_set_ua_cascade_environment;
use crate::runtime_layout::spinon_runtime_session_copy_layout_json;
use serde_json::Value;
use std::ffi::{CStr, CString, c_char};
use std::thread;
use std::time::{Duration, Instant};

const ERR_OUTPUT_TOO_SMALL: i32 = -3;

pub(super) fn run_runtime_layout_fixture_probe() -> Result<String, String> {
    let mut create_report = [0 as c_char; 512];
    let session = unsafe {
        crate::runtime_session::spinon_runtime_session_new(
            create_report.as_mut_ptr(),
            create_report.len(),
        )
    };
    if session.is_null() {
        let report = unsafe { CStr::from_ptr(create_report.as_ptr()) }.to_string_lossy();
        return Err(format!("layout fixture runtime 생성 실패: {report}"));
    }

    let result = run_runtime_layout_fixture(session);
    unsafe { crate::runtime_session::spinon_runtime_session_free(session) };
    result
}

fn run_runtime_layout_fixture(
    session: *mut crate::runtime_session::SpinonRuntimeSession,
) -> Result<String, String> {
    let source = CString::new(concat!(
        "const root=document.createElement('div');",
        "root.setAttribute('id','root');",
        "root.setAttribute('style','display:flex;box-sizing:border-box;width:300px;height:140px;flex-direction:row;align-items:center;justify-content:flex-start;column-gap:7px;padding:5px');",
        "document.appendChild(root);",
        "const flexB=document.createElement('div');",
        "flexB.setAttribute('id','flex-b');",
        "flexB.setAttribute('style','display:flex;width:100px;height:80px;flex-direction:column;row-gap:4px;padding:2px');",
        "const flexA=document.createElement('div');",
        "flexA.setAttribute('id','flex-a');",
        "flexA.setAttribute('style','display:block;width:40px;height:30px;margin:2px;padding:3px');",
        "root.appendChild(flexA);",
        "root.appendChild(flexB);",
        "const blockChild=document.createElement('div');",
        "blockChild.setAttribute('id','block-child');",
        "blockChild.setAttribute('style','display:block;width:30px;height:11px;margin-top:3px');",
        "flexB.appendChild(blockChild);",
        "const hidden=document.createElement('div');",
        "hidden.setAttribute('id','hidden');",
        "hidden.setAttribute('style','display:none;width:20px;height:20px');",
        "flexB.appendChild(hidden);"
    ))
    .map_err(|error| format!("runtime layout fixture 문자열 오류: {error}"))?;
    let mut eval_report = [0 as c_char; 2048];
    let eval_status = unsafe {
        crate::runtime_session::spinon_runtime_session_eval(
            session,
            source.as_ptr(),
            eval_report.as_mut_ptr(),
            eval_report.len(),
        )
    };
    if eval_status != 0 {
        let report = unsafe { CStr::from_ptr(eval_report.as_ptr()) }.to_string_lossy();
        return Err(format!(
            "runtime layout DOM fixture가 기대 status=0을 반환하지 않았습니다 status={eval_status} report={report}"
        ));
    }

    let mut environment_revision = u64::MAX;
    let environment_status = unsafe {
        spinon_runtime_session_set_ua_cascade_environment(
            session,
            800.0,
            600.0,
            1.0,
            0,
            2,
            1,
            0b110,
            &mut environment_revision,
        )
    };
    if environment_status != 0 || environment_revision != 0 {
        return Err(format!(
            "runtime layout fixture 환경 설정 실패 status={environment_status} revision={environment_revision}"
        ));
    }

    let snapshot = wait_for_ready_layout(session)?;
    verify_runtime_layout_fixture(&snapshot)
}

fn wait_for_ready_layout(
    session: *mut crate::runtime_session::SpinonRuntimeSession,
) -> Result<Value, String> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let json = copy_layout_session_json(session)?;
        let value: Value = serde_json::from_str(&json)
            .map_err(|error| format!("runtime layout JSON parse 실패: {error}"))?;
        match value["state"].as_str() {
            Some("pending") => {
                if Instant::now() >= deadline {
                    return Err("runtime layout worker 완료 대기 5초 초과".to_owned());
                }
                thread::sleep(Duration::from_millis(1));
            }
            Some("ready") => return Ok(value),
            Some(state) => {
                return Err(format!(
                    "예상 밖 runtime layout 상태 state={state} error={}",
                    value["error"]
                ));
            }
            None => return Err("runtime layout JSON에 state가 없습니다".to_owned()),
        }
    }
}

fn verify_runtime_layout_fixture(value: &Value) -> Result<String, String> {
    if value["schema"] != "spinon.runtime.layout" {
        return Err("runtime layout JSON schema 식별자가 다릅니다".to_owned());
    }
    if value["requested"] != value["completed"]["key"] {
        return Err("runtime layout requested와 completed revision key가 다릅니다".to_owned());
    }
    if value["completed"]["unit"] != "css-px" {
        return Err("runtime layout 단위가 CSS px가 아닙니다".to_owned());
    }
    let frames = value["completed"]["frames"]
        .as_array()
        .ok_or_else(|| "runtime layout frames가 배열이 아닙니다".to_owned())?;
    if frames.len() != 5 {
        return Err(format!(
            "runtime layout frame 개수가 5가 아닙니다: {}",
            frames.len()
        ));
    }
    let expected = [
        (1, 0.0, 0.0, 300.0, 140.0),
        (3, 7.0, 52.0, 46.0, 36.0),
        (2, 62.0, 28.0, 104.0, 84.0),
        (4, 64.0, 33.0, 30.0, 11.0),
        (5, 0.0, 0.0, 0.0, 0.0),
    ];
    for (frame, (node_id, x, y, width, height)) in frames.iter().zip(expected) {
        if frame["nodeId"].as_u64() != Some(node_id) {
            return Err(format!(
                "runtime layout DOM preorder 불일치: expected node={node_id}, actual={}",
                frame["nodeId"]
            ));
        }
        for (field, actual, expected) in [
            ("x", frame["x"].as_f64(), x),
            ("y", frame["y"].as_f64(), y),
            ("width", frame["width"].as_f64(), width),
            ("height", frame["height"].as_f64(), height),
        ] {
            let Some(actual) = actual else {
                return Err(format!(
                    "runtime layout node={node_id} {field}가 숫자가 아닙니다"
                ));
            };
            if !actual.is_finite() || (actual - expected).abs() > 0.5 {
                return Err(format!(
                    "runtime layout node={node_id} {field} 불일치: actual={actual} Chromium={expected} 허용치=0.5 CSS px"
                ));
            }
        }
    }
    Ok(format!(
        "runtime_layout=PASS schema={} frames={} chromium_geometry=PASS document_revision={}",
        value["schema"].as_str().unwrap_or("missing"),
        frames.len(),
        value["requested"]["documentRevision"].as_u64().unwrap_or(0),
    ))
}

fn copy_layout_session_json(
    session: *mut crate::runtime_session::SpinonRuntimeSession,
) -> Result<String, String> {
    let mut required = 0_usize;
    let mut probe = [0 as c_char; 1];
    let status = unsafe {
        spinon_runtime_session_copy_layout_json(
            session,
            probe.as_mut_ptr(),
            probe.len(),
            &mut required,
        )
    };
    if status != ERR_OUTPUT_TOO_SMALL || required <= probe.len() {
        return Err(format!(
            "runtime layout JSON required capacity 조회 실패 status={status} required={required}"
        ));
    }
    let mut output = vec![0 as c_char; required];
    let status = unsafe {
        spinon_runtime_session_copy_layout_json(
            session,
            output.as_mut_ptr(),
            output.len(),
            &mut required,
        )
    };
    if status != 0 {
        return Err(format!(
            "runtime layout JSON snapshot 복사 실패 status={status}"
        ));
    }
    Ok(unsafe { CStr::from_ptr(output.as_ptr()) }
        .to_string_lossy()
        .into_owned())
}

#[cfg(test)]
mod tests {
    use super::verify_runtime_layout_fixture;
    use serde_json::{Value, json};

    fn runtime_layout_fixture_json() -> Value {
        json!({
            "schema": "spinon.runtime.layout",
            "state": "ready",
            "requested": {
                "generation": 1,
                "documentRevision": 2,
                "renderTreeRevision": 2,
                "styleRevision": 0,
                "environmentRevision": 0
            },
            "completed": {
                "key": {
                    "generation": 1,
                    "documentRevision": 2,
                    "renderTreeRevision": 2,
                    "styleRevision": 0,
                    "environmentRevision": 0
                },
                "unit": "css-px",
                "frames": [
                    {"nodeId": 1, "x": 0, "y": 0, "width": 300, "height": 140},
                    {"nodeId": 3, "x": 7, "y": 52, "width": 46, "height": 36},
                    {"nodeId": 2, "x": 62, "y": 28, "width": 104, "height": 84},
                    {"nodeId": 4, "x": 64, "y": 33, "width": 30, "height": 11},
                    {"nodeId": 5, "x": 0, "y": 0, "width": 0, "height": 0}
                ]
            }
        })
    }

    #[test]
    fn runtime_layout_probe_checks_chromium_geometry_and_dom_order() {
        let value = runtime_layout_fixture_json();
        let report = verify_runtime_layout_fixture(&value).unwrap();
        assert!(report.contains("runtime_layout=PASS"));

        let mut id_sorted = value.clone();
        id_sorted["completed"]["frames"] = json!([
            {"nodeId": 1, "x": 0, "y": 0, "width": 300, "height": 140},
            {"nodeId": 2, "x": 62, "y": 28, "width": 104, "height": 84},
            {"nodeId": 3, "x": 7, "y": 52, "width": 46, "height": 36},
            {"nodeId": 4, "x": 64, "y": 33, "width": 30, "height": 11},
            {"nodeId": 5, "x": 0, "y": 0, "width": 0, "height": 0}
        ]);
        assert!(verify_runtime_layout_fixture(&id_sorted).is_err());
    }

    #[test]
    fn runtime_layout_probe_rejects_stale_or_malformed_snapshots() {
        let mut stale = runtime_layout_fixture_json();
        stale["completed"]["key"]["environmentRevision"] = json!(1);
        assert!(verify_runtime_layout_fixture(&stale).is_err());

        let mut bad_frame = runtime_layout_fixture_json();
        bad_frame["completed"]["frames"][4]["width"] = json!(-1);
        assert!(verify_runtime_layout_fixture(&bad_frame).is_err());
    }
}
