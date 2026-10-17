use crate::runtime_session::{SpinonRuntimeSession, spinon_runtime_session_free};
use crate::runtime_ua_cascade::{
    SpinonRuntimeRootHtmlCanvas, SpinonRuntimeRootHtmlEnvironment,
    spinon_runtime_session_resolve_root_html_canvas,
    spinon_runtime_session_set_root_html_environment,
};
use spinon_runtime::RuntimeUaCascadeState;
use spinon_runtime::{ROOT_HTML_SOURCE_MAX_BYTES, RuntimeSession, TaskPriority};
use std::ffi::c_char;
use std::time::Duration;

const ROOT_HTML_ASSET_ASSERTION: &str = r#"(() => {
  const elementChildren = parent => {
    const result = [];
    for (let child = parent.firstChild; child !== null; child = child.nextSibling) {
      if (child.nodeType === 1) result.push(child);
    }
    return result;
  };
  const root = document.documentElement;
  const head = document.head;
  const body = document.body;
  const headElements = head === null ? [] : elementChildren(head);
  const bodyElements = body === null ? [] : elementChildren(body);
  const viewport = headElements.find(element =>
    element.localName === "meta" && element.getAttribute("name") === "viewport");
  const scheme = headElements.find(element =>
    element.localName === "meta" && element.getAttribute("name") === "color-scheme");
  const style = headElements.find(element => element.localName === "style");
  const main = bodyElements.find(element => element.localName === "main");
  const script = bodyElements.find(element => element.localName === "script");
  if (!root || root.localName !== "html" || root.getAttribute("lang") !== "ko" ||
      !head || head.parentNode !== root || !body || body.parentNode !== root ||
      !viewport || viewport.getAttribute("content") !== "width=device-width, initial-scale=1" ||
      !scheme || scheme.getAttribute("content") !== "light dark" ||
      !style || style.textContent !== ":root { color-scheme: light dark; }" ||
      !main || main.id !== "root-asset" || main.getAttribute("data-fixture") !== "c04-12" ||
      main.textContent !== "앱 HTML asset" || !script ||
      globalThis.__c04_12_asset_script_ran !== undefined) {
    throw new Error("C04.12 platform root HTML asset assertion failed");
  }
})()"#;

struct ViewportExpectation {
    name: &'static str,
    content: &'static str,
    layout_width: f32,
    layout_height: f32,
    visual_width: f32,
    visual_height: f32,
    page_scale: f32,
    media_height: &'static str,
}

const ROOT_HTML_VIEWPORT_EXPECTATIONS: [ViewportExpectation; 3] = [
    ViewportExpectation {
        name: "scale-half",
        content: "width=device-width, initial-scale=0.5",
        layout_width: 780.0,
        layout_height: 1688.0,
        visual_width: 780.0,
        visual_height: 1688.0,
        page_scale: 0.5,
        media_height: "123px",
    },
    ViewportExpectation {
        name: "scale-two",
        content: "width=device-width, initial-scale=2",
        layout_width: 390.0,
        layout_height: 844.0,
        visual_width: 195.0,
        visual_height: 422.0,
        page_scale: 2.0,
        media_height: "456px",
    },
    ViewportExpectation {
        name: "fixed-width-500",
        content: "width=500, initial-scale=1",
        layout_width: 500.0,
        layout_height: 1082.0513,
        visual_width: 390.0,
        visual_height: 844.0,
        page_scale: 1.0,
        media_height: "123px",
    },
];

fn validate_root_html_viewport_matrix() -> Result<(), String> {
    for expected in ROOT_HTML_VIEWPORT_EXPECTATIONS {
        let source = format!(
            "<!doctype html><html><head><meta name='viewport' content='{}'><style>@media (min-height: 1000px) {{ main {{ height: 123px; }} }} @media (max-height: 900px) {{ main {{ height: 456px; }} }}</style></head><body><main>viewport</main></body></html>",
            expected.content
        );
        let (session, _) = RuntimeSession::new_with_root_html(&source)
            .map_err(|error| format!("{} session: {error}", expected.name))?;
        let c_session = Box::into_raw(Box::new(session)).cast::<SpinonRuntimeSession>();
        let ffi_values = (|| {
            let mut environment = SpinonRuntimeRootHtmlEnvironment::default();
            let status = unsafe {
                spinon_runtime_session_set_root_html_environment(
                    c_session,
                    1170.0,
                    2532.0,
                    3.0,
                    0,
                    &mut environment,
                )
            };
            if status != 0 {
                return Err(format!(
                    "{} environment ABI returned {status}",
                    expected.name
                ));
            }

            let mut canvas = SpinonRuntimeRootHtmlCanvas::default();
            let status = unsafe {
                spinon_runtime_session_resolve_root_html_canvas(
                    c_session,
                    1170.0,
                    2532.0,
                    3.0,
                    0,
                    3_000,
                    &mut canvas,
                )
            };
            if status != 0 {
                return Err(format!(
                    "{} Canvas environment ABI returned {status}",
                    expected.name
                ));
            }

            let values = |environment: SpinonRuntimeRootHtmlEnvironment| {
                [
                    environment.viewport_width_css_px,
                    environment.viewport_height_css_px,
                    environment.visual_viewport_width_css_px,
                    environment.visual_viewport_height_css_px,
                    environment.page_scale,
                ]
            };
            let session = unsafe { &*c_session.cast::<RuntimeSession>() };
            let _layout_terminal = session.wait_for_layout_snapshot(Duration::from_secs(3));
            let cascade = session.ua_cascade_snapshot();
            if cascade.state != RuntimeUaCascadeState::Ready
                || !cascade.completed.as_ref().is_some_and(|completed| {
                    completed.roots.iter().any(|root| {
                        root.styles.elements.iter().any(|element| {
                            element
                                .properties
                                .get("height")
                                .is_some_and(|height| height == expected.media_height)
                        })
                    })
                })
            {
                return Err(format!(
                    "{} layout-height media query mismatch: expected {}",
                    expected.name, expected.media_height
                ));
            }
            if canvas.cascade_environment_revision != environment.environment_revision {
                return Err(format!(
                    "{} Canvas environment revision mismatch: set={} canvas={}",
                    expected.name,
                    environment.environment_revision,
                    canvas.cascade_environment_revision
                ));
            }
            Ok((values(environment), values(canvas.environment)))
        })();
        unsafe { spinon_runtime_session_free(c_session) };
        let (environment_values, canvas_values) = ffi_values?;
        let actual = [
            environment_values[0],
            environment_values[1],
            environment_values[2],
            environment_values[3],
            environment_values[4],
        ];
        let required = [
            expected.layout_width,
            expected.layout_height,
            expected.visual_width,
            expected.visual_height,
            expected.page_scale,
        ];
        if actual
            .into_iter()
            .zip(required)
            .any(|(actual, required)| (actual - required).abs() > 0.001)
            || canvas_values
                .into_iter()
                .zip(required)
                .any(|(actual, required)| (actual - required).abs() > 0.001)
        {
            return Err(format!(
                "{} viewport mismatch: layout={}x{} visual={}x{} scale={}",
                expected.name,
                environment_values[0],
                environment_values[1],
                environment_values[2],
                environment_values[3],
                environment_values[4]
            ));
        }
    }
    Ok(())
}

/// 실제 V8 DOM facade에서 root HTML의 document accessor를 확인하는 opt-in fixture입니다.
///
/// # Safety
/// `output`은 `output_capacity` bytes를 쓸 수 있어야 합니다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spinon_c04_12_root_html_accessors_fixture(
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return -1;
    }
    let html = "<!doctype html><html lang='ko'><head><meta name='viewport' content='width=device-width, initial-scale=1'><meta name='color-scheme' content='light dark'><style>:root { color-scheme: light dark; }</style><script>globalThis.__c04_12_root_html_script_ran = true;</script></head><body><main id='app'>루트 HTML</main></body></html>";
    let (session, startup_report) = match RuntimeSession::new_with_root_html(html) {
        Ok(value) => value,
        Err(error) => {
            crate::write_report(output, output_capacity, &error);
            return -1;
        }
    };
    let response = session.eval(
        "(() => { const root = document.documentElement; const head = document.head; const body = document.body; const viewport = head && head.firstChild; const scheme = viewport && viewport.nextSibling; const style = scheme && scheme.nextSibling; const script = style && style.nextSibling; const main = body && body.firstChild; if (!root || root !== document.documentElement || root.localName !== 'html' || root.getAttribute('lang') !== 'ko' || !head || head !== document.head || head.parentNode !== root || !viewport || viewport.localName !== 'meta' || viewport.getAttribute('name') !== 'viewport' || viewport.getAttribute('content') !== 'width=device-width, initial-scale=1' || !scheme || scheme.getAttribute('name') !== 'color-scheme' || scheme.getAttribute('content') !== 'light dark' || !style || style.localName !== 'style' || style.textContent !== ':root { color-scheme: light dark; }' || !script || script.localName !== 'script' || !body || body !== document.body || body.parentNode !== root || !main || main.localName !== 'main' || main.id !== 'app' || main.textContent !== '루트 HTML' || globalThis.__c04_12_root_html_script_ran !== undefined) throw new Error('C04.12 root HTML DOM accessor assertion failed'); const previousRoot = root; document.removeChild(previousRoot); const replacement = document.createElement('main'); document.appendChild(replacement); if (document.documentElement !== replacement || document.head !== null || document.body !== null) throw new Error('documentElement must follow the first document element'); })()",
        TaskPriority::UserVisible,
    );
    if response.status != 0 {
        let _ = crate::write_report(output, output_capacity, &response.report);
        return response.status;
    }
    let report = format!(
        "status=0 c04_12_root_html_accessors=PASS startup={}",
        startup_report.replace(' ', "_")
    );
    if crate::write_report(output, output_capacity, &report) {
        0
    } else {
        -3
    }
}

/// 플랫폼 bundle에서 읽은 HTML asset을 실제 V8 세션에 전달하는 opt-in fixture입니다.
///
/// # Safety
/// `source`는 `source_length` bytes를 읽을 수 있어야 하고 `output`은
/// `output_capacity` bytes를 쓸 수 있어야 합니다. 두 범위는 서로 겹치지 않아야 합니다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spinon_c04_12_root_html_asset_fixture(
    source: *const c_char,
    source_length: usize,
    output: *mut c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return -1;
    }
    if source_length > ROOT_HTML_SOURCE_MAX_BYTES {
        let _ = crate::write_report(
            output,
            output_capacity,
            "루트 HTML asset이 허용 크기를 초과했습니다",
        );
        return -1;
    }
    let source_bytes = if source_length == 0 {
        &[][..]
    } else if source.is_null() {
        let _ = crate::write_report(
            output,
            output_capacity,
            "루트 HTML asset 포인터가 비었습니다",
        );
        return -1;
    } else {
        unsafe { std::slice::from_raw_parts(source.cast::<u8>(), source_length) }
    };
    let source = match std::str::from_utf8(source_bytes) {
        Ok(source) => source,
        Err(_) => {
            let _ = crate::write_report(
                output,
                output_capacity,
                "루트 HTML asset은 UTF-8이어야 합니다",
            );
            return -1;
        }
    };
    let (session, startup_report) = match RuntimeSession::new_with_root_html(source) {
        Ok(value) => value,
        Err(error) => {
            let _ = crate::write_report(output, output_capacity, &error);
            return -1;
        }
    };
    let response = session.eval(ROOT_HTML_ASSET_ASSERTION, TaskPriority::UserVisible);
    if response.status != 0 {
        let _ = crate::write_report(output, output_capacity, &response.report);
        return response.status;
    }
    drop(session);
    if let Err(error) = validate_root_html_viewport_matrix() {
        let _ = crate::write_report(output, output_capacity, &error);
        return -1;
    }
    let report = format!(
        "status=0 c04_12_root_html_asset=PASS c04_12_viewport=PASS c04_12_environment_abi=PASS cases=3 fixed_width=500x1082 visual=390x844 startup={}",
        startup_report.replace(' ', "_")
    );
    if crate::write_report(output, output_capacity, &report) {
        0
    } else {
        -3
    }
}

#[cfg(test)]
mod tests {
    use super::spinon_c04_12_root_html_asset_fixture;
    use std::ffi::CStr;

    #[test]
    fn asset_fixture_rejects_non_utf8_without_starting_a_session() {
        let source = [0xff_u8];
        let mut output = [0_u8; 128];
        let status = unsafe {
            spinon_c04_12_root_html_asset_fixture(
                source.as_ptr().cast(),
                source.len(),
                output.as_mut_ptr().cast(),
                output.len(),
            )
        };

        assert_eq!(status, -1);
        let report = unsafe { CStr::from_ptr(output.as_ptr().cast()) }
            .to_str()
            .unwrap();
        assert_eq!(report, "루트 HTML asset은 UTF-8이어야 합니다");
    }

    #[test]
    fn asset_fixture_rejects_oversized_source_before_reading_it() {
        let mut output = [0_u8; 128];
        let status = unsafe {
            spinon_c04_12_root_html_asset_fixture(
                std::ptr::null(),
                spinon_runtime::ROOT_HTML_SOURCE_MAX_BYTES + 1,
                output.as_mut_ptr().cast(),
                output.len(),
            )
        };

        assert_eq!(status, -1);
        let report = unsafe { CStr::from_ptr(output.as_ptr().cast()) }
            .to_str()
            .unwrap();
        assert_eq!(report, "루트 HTML asset이 허용 크기를 초과했습니다");
    }

    #[test]
    fn asset_fixture_rejects_null_source_with_nonzero_length() {
        let mut output = [0_u8; 128];
        let status = unsafe {
            spinon_c04_12_root_html_asset_fixture(
                std::ptr::null(),
                1,
                output.as_mut_ptr().cast(),
                output.len(),
            )
        };

        assert_eq!(status, -1);
        let report = unsafe { CStr::from_ptr(output.as_ptr().cast()) }
            .to_str()
            .unwrap();
        assert_eq!(report, "루트 HTML asset 포인터가 비었습니다");
    }
}
