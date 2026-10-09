use super::{
    ERR_ARGUMENT, ERR_OUTPUT_TOO_SMALL, spinon_runtime_session_copy_ua_cascade_json,
    spinon_runtime_session_set_ua_cascade_environment,
};
use serde_json::Value;
use std::ffi::{CStr, CString, c_char};
use std::thread;
use std::time::{Duration, Instant};

/// 실제 V8 DOM façade, 내부 환경 설정 C ABI와 JSON 조회 C ABI를 함께 확인하는 개발 fixture입니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트를 쓸 수 있는 버퍼여야 합니다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spinon_runtime_ua_cascade_probe(
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 || output_capacity > isize::MAX as usize {
        return ERR_ARGUMENT;
    }
    let mut create_report = [0 as c_char; 512];
    let session = unsafe {
        crate::runtime_session::spinon_runtime_session_new(
            create_report.as_mut_ptr(),
            create_report.len(),
        )
    };
    if session.is_null() {
        let message = format!(
            "status=-2 ua_cascade_probe=FAIL create={}",
            unsafe { CStr::from_ptr(create_report.as_ptr()) }.to_string_lossy()
        );
        crate::write_report(output, output_capacity, &message);
        return -2;
    }
    let create_report = unsafe { CStr::from_ptr(create_report.as_ptr()) }.to_string_lossy();
    let session_metrics = match session_metrics(&create_report) {
        Ok(metrics) => metrics,
        Err(error) => {
            unsafe { crate::runtime_session::spinon_runtime_session_free(session) };
            let report = format!("status=-7 ua_cascade_probe=FAIL reason={error}");
            crate::write_report(output, output_capacity, &report);
            return -7;
        }
    };

    let result = run_ua_cascade_probe(session, session_metrics);
    unsafe { crate::runtime_session::spinon_runtime_session_free(session) };
    let report = match result {
        Ok(report) => format!("status=0 ua_cascade_probe=PASS {report}"),
        Err(error) => format!("status=-7 ua_cascade_probe=FAIL reason={error}"),
    };
    if !crate::write_report(output, output_capacity, &report) {
        return ERR_OUTPUT_TOO_SMALL;
    }
    if report.starts_with("status=0") {
        0
    } else {
        -7
    }
}

fn run_ua_cascade_probe(
    session: *mut crate::runtime_session::SpinonRuntimeSession,
    session_metrics: SessionMetrics,
) -> Result<String, String> {
    let source = CString::new(
        "const uaDiv=document.createElement('div'); uaDiv.setAttribute('style','display:inline; color:nope;'); document.appendChild(uaDiv); const uaSpan=document.createElement('span'); document.appendChild(uaSpan); const uaButton=document.createElement('button'); document.appendChild(uaButton); const uaParagraph=document.createElement('p'); document.appendChild(uaParagraph); throw new Error('C04.8 commit before exception fixture');",
    )
    .map_err(|error| format!("fixture 문자열 오류: {error}"))?;
    let mut eval_report = [0 as c_char; 2048];
    let eval_status = unsafe {
        crate::runtime_session::spinon_runtime_session_eval(
            session,
            source.as_ptr(),
            eval_report.as_mut_ptr(),
            eval_report.len(),
        )
    };
    if eval_status != -4 {
        let report = unsafe { CStr::from_ptr(eval_report.as_ptr()) }.to_string_lossy();
        return Err(format!(
            "commit 후 예외 fixture가 기대 status=-4를 반환하지 않았습니다 status={eval_status} report={report}"
        ));
    }
    let eval_report = unsafe { CStr::from_ptr(eval_report.as_ptr()) }.to_string_lossy();
    let small_metrics = evaluation_metrics(&eval_report)?;

    let mut environment_revision = u64::MAX;
    let environment_status = unsafe {
        spinon_runtime_session_set_ua_cascade_environment(
            session,
            390.0,
            844.0,
            3.0,
            1,
            1,
            0,
            0b001,
            &mut environment_revision,
        )
    };
    if environment_status != 0 || environment_revision != 0 {
        return Err(format!(
            "CSS 환경 설정 실패 status={environment_status} revision={environment_revision}"
        ));
    }

    let small_snapshot = wait_for_ready_snapshot(session)?;
    let small_summary = verify_probe_result(&small_snapshot)?;
    let small_document_revision = small_snapshot["requested"]["documentRevision"]
        .as_u64()
        .ok_or_else(|| "small fixture document revision이 없습니다".to_owned())?;
    let small_compute_us = computation_duration_us(&small_snapshot)?;

    let large_source = CString::new(
        "const uaDetached=[]; for(let index=0; index<256; index+=1) uaDetached.push(document.createElement('div')); globalThis.__spinonC048Detached=uaDetached;",
    )
    .map_err(|error| format!("large fixture 문자열 오류: {error}"))?;
    let mut large_eval_report = [0 as c_char; 4096];
    let large_eval_status = unsafe {
        crate::runtime_session::spinon_runtime_session_eval(
            session,
            large_source.as_ptr(),
            large_eval_report.as_mut_ptr(),
            large_eval_report.len(),
        )
    };
    if large_eval_status != 0 {
        let report = unsafe { CStr::from_ptr(large_eval_report.as_ptr()) }.to_string_lossy();
        return Err(format!(
            "detached-node fixture가 기대 status=0을 반환하지 않았습니다 status={large_eval_status} report={report}"
        ));
    }
    let large_eval_report = unsafe { CStr::from_ptr(large_eval_report.as_ptr()) }.to_string_lossy();
    let large_metrics = evaluation_metrics(&large_eval_report)?;

    let large_snapshot = wait_for_ready_snapshot(session)?;
    let large_summary = verify_probe_result(&large_snapshot)?;
    let large_document_revision = large_snapshot["requested"]["documentRevision"]
        .as_u64()
        .ok_or_else(|| "large fixture document revision이 없습니다".to_owned())?;
    if large_document_revision <= small_document_revision {
        return Err("detached node 생성 뒤 document revision이 증가하지 않았습니다".to_owned());
    }
    if small_snapshot["completed"]["roots"] != large_snapshot["completed"]["roots"] {
        return Err("detached node가 기존 HostRoot cascade 결과를 바꿨습니다".to_owned());
    }
    let large_compute_us = computation_duration_us(&large_snapshot)?;
    Ok(format!(
        "{small_summary} {large_summary} css_worker_ready_us={} session_startup_us={} snapshot_clone_small_us={} snapshot_submit_small_us={} cascade_compute_small_us={small_compute_us} detached_nodes=256 snapshot_clone_large_us={} snapshot_submit_large_us={} cascade_compute_large_us={large_compute_us}",
        session_metrics.css_worker_ready_us,
        session_metrics.session_startup_us,
        small_metrics.snapshot_clone_us,
        small_metrics.snapshot_submit_us,
        large_metrics.snapshot_clone_us,
        large_metrics.snapshot_submit_us,
    ))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SessionMetrics {
    css_worker_ready_us: u64,
    session_startup_us: u64,
}

fn session_metrics(report: &str) -> Result<SessionMetrics, String> {
    if metric(report, "css_worker_per_session") != Some("1") {
        return Err("세션마다 CSS worker 하나가 시작되지 않았습니다".to_owned());
    }
    Ok(SessionMetrics {
        css_worker_ready_us: required_metric(report, "css_worker_ready_us")?,
        session_startup_us: required_metric(report, "session_startup_us")?,
    })
}

fn required_metric(report: &str, name: &str) -> Result<u64, String> {
    metric(report, name)
        .and_then(|value| value.parse().ok())
        .ok_or_else(|| format!("runtime report에 {name} 숫자 측정값이 없습니다"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EvaluationMetrics {
    snapshot_clone_us: u64,
    snapshot_submit_us: u64,
}

fn evaluation_metrics(report: &str) -> Result<EvaluationMetrics, String> {
    if metric(report, "host_document_changed") != Some("true")
        || metric(report, "ua_snapshot_submitted") != Some("true")
    {
        return Err("HostDocument 변경 뒤 snapshot이 제출되지 않았습니다".to_owned());
    }
    Ok(EvaluationMetrics {
        snapshot_clone_us: required_metric(report, "ua_snapshot_clone_us")?,
        snapshot_submit_us: required_metric(report, "ua_snapshot_submit_us")?,
    })
}

fn computation_duration_us(snapshot: &Value) -> Result<u64, String> {
    snapshot["completed"]["computationDurationUs"]
        .as_u64()
        .ok_or_else(|| "cascade 계산 시간이 JSON에 없습니다".to_owned())
}

fn wait_for_ready_snapshot(
    session: *mut crate::runtime_session::SpinonRuntimeSession,
) -> Result<Value, String> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let json = copy_session_json(session)?;
        let value: Value = serde_json::from_str(&json)
            .map_err(|error| format!("JSON snapshot parse 실패: {error}"))?;
        match value["state"].as_str() {
            Some("pending") => {
                if Instant::now() >= deadline {
                    return Err("CSS worker 완료 대기 5초 초과".to_owned());
                }
                thread::sleep(Duration::from_millis(1));
            }
            Some("ready") => return Ok(value),
            Some(state) => {
                return Err(format!(
                    "예상 밖 cascade 상태 state={state} error={}",
                    value["error"].as_str().unwrap_or("null")
                ));
            }
            None => return Err("JSON snapshot에 state가 없습니다".to_owned()),
        }
    }
}

fn metric<'a>(report: &'a str, name: &str) -> Option<&'a str> {
    let prefix = format!("{name}=");
    report
        .split_whitespace()
        .find_map(|field| field.strip_prefix(&prefix))
}

fn copy_session_json(
    session: *mut crate::runtime_session::SpinonRuntimeSession,
) -> Result<String, String> {
    let mut required = 0_usize;
    let mut probe = [0 as c_char; 1];
    let status = unsafe {
        spinon_runtime_session_copy_ua_cascade_json(
            session,
            probe.as_mut_ptr(),
            probe.len(),
            &mut required,
        )
    };
    if status != ERR_OUTPUT_TOO_SMALL || required <= probe.len() {
        return Err(format!(
            "JSON required capacity 조회 실패 status={status} required={required}"
        ));
    }
    let mut output = vec![0 as c_char; required];
    let status = unsafe {
        spinon_runtime_session_copy_ua_cascade_json(
            session,
            output.as_mut_ptr(),
            output.len(),
            &mut required,
        )
    };
    if status != 0 {
        return Err(format!("JSON snapshot 복사 실패 status={status}"));
    }
    Ok(unsafe { CStr::from_ptr(output.as_ptr()) }
        .to_string_lossy()
        .into_owned())
}

fn verify_probe_result(value: &Value) -> Result<String, String> {
    if value["schema"] != "spinon.runtime.ua-cascade.v1" {
        return Err("JSON schema 식별자가 다릅니다".to_owned());
    }
    let requested = &value["requested"];
    let completed = &value["completed"];
    if requested != &completed["key"] {
        return Err("requested와 completed revision key가 다릅니다".to_owned());
    }
    if requested["documentRevision"].as_u64().unwrap_or(0) == 0
        || requested["environmentRevision"].as_u64() != Some(0)
    {
        return Err("document/environment revision이 fixture 기대값과 다릅니다".to_owned());
    }
    let roots = completed["roots"]
        .as_array()
        .ok_or_else(|| "completed roots가 배열이 아닙니다".to_owned())?;
    if roots.len() != 4 {
        return Err(format!("root 개수가 4가 아닙니다: {}", roots.len()));
    }
    let display = |node_id: u64| -> Option<&str> {
        roots
            .iter()
            .find(|root| root["rootNodeId"].as_u64() == Some(node_id))?["elements"][0]["properties"]
            ["display"]
            .as_str()
    };
    let expected = [
        (1, "inline"),
        (2, "inline"),
        (3, "inline-block"),
        (4, "block"),
    ];
    for (node_id, expected_display) in expected {
        if display(node_id) != Some(expected_display) {
            return Err(format!(
                "root node {node_id} display mismatch: actual={:?} expected={expected_display}",
                display(node_id)
            ));
        }
    }
    if roots[0]["diagnostics"].as_array().is_none_or(Vec::is_empty) {
        return Err("잘못된 inline style CSS diagnostic이 없습니다".to_owned());
    }
    Ok(format!(
        "schema={} roots={} document_revision={} environment_revision={} ua_values=PASS inline_style=PASS diagnostics=PASS expected_js_exception=PASS",
        value["schema"].as_str().unwrap_or("missing"),
        roots.len(),
        requested["documentRevision"].as_u64().unwrap_or(0),
        requested["environmentRevision"]
            .as_u64()
            .unwrap_or(u64::MAX),
    ))
}

#[cfg(test)]
mod tests {
    use super::{evaluation_metrics, metric, required_metric, session_metrics};

    #[test]
    fn session_metrics_require_worker_and_both_startup_durations() {
        let metrics = session_metrics(
            "session=ready css_worker_per_session=1 css_worker_ready_us=12 session_startup_us=34",
        )
        .unwrap();
        assert_eq!(metrics.css_worker_ready_us, 12);
        assert_eq!(metrics.session_startup_us, 34);
        assert!(session_metrics("session=ready css_worker_per_session=0").is_err());
    }

    #[test]
    fn metric_reader_requires_an_exact_numeric_field() {
        assert_eq!(required_metric("x=7 y=8", "y"), Ok(8));
        assert!(required_metric("x=7 xy=8", "y").is_err());
        assert!(required_metric("x=nope", "x").is_err());
        assert_eq!(metric("x=7 y=8", "x"), Some("7"));
    }

    #[test]
    fn evaluation_metrics_require_a_committed_document_and_submission() {
        let metrics = evaluation_metrics(
            "host_document_changed=true ua_snapshot_submitted=true ua_snapshot_clone_us=3 ua_snapshot_submit_us=4",
        )
        .unwrap();
        assert_eq!(metrics.snapshot_clone_us, 3);
        assert_eq!(metrics.snapshot_submit_us, 4);
        assert!(evaluation_metrics("host_document_changed=false").is_err());
        assert!(evaluation_metrics(
            "host_document_changed=true ua_snapshot_submitted=false ua_snapshot_clone_us=3 ua_snapshot_submit_us=4"
        )
        .is_err());
    }
}
