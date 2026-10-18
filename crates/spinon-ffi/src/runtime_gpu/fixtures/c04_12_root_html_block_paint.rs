use super::super::RuntimeGpuHost;
use spinon_runtime::RuntimeSession;

const ROOT_HTML_BLOCK_PAINT_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c04/root-html-block-paint.html");

impl RuntimeGpuHost {
    pub(in crate::runtime_gpu) fn new_root_html_block_paint_fixture()
    -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_root_html_block_paint_fixture(
            ROOT_HTML_BLOCK_PAINT_SOURCE,
        )?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        host.root_html_block_paint_fixture = true;
        host.surface_pixel_readback_test = true;
        Ok((host, report))
    }
}
