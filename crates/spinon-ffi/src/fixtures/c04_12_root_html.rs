use spinon_runtime::{RuntimeSession, TaskPriority};
use std::ffi::c_char;

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
