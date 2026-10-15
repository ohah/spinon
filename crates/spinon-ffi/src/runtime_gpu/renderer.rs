use std::ffi::c_void;

use spinon_render_wgpu::{RuntimeRenderer, RuntimeRendererError};

use super::{ERR_ARGUMENT, ERR_LAYOUT, ERR_STALE, RuntimeGpuHost, lock};

impl RuntimeGpuHost {
    pub(super) unsafe fn create_android(
        &self,
        native_window: *mut c_void,
        width: u32,
        height: u32,
        backend: u32,
    ) -> Result<String, RuntimeRendererError> {
        let mut renderer_slot = lock(&self.renderer);
        if renderer_slot.is_some() || lock(&self.pending_uikit_surface).is_some() {
            return Err(RuntimeRendererError::InvalidArgument(
                "runtime GPU renderer가 이미 생성되었습니다",
            ));
        }
        let (renderer, report) =
            unsafe { RuntimeRenderer::new_android(native_window, width, height, backend) }?;
        *renderer_slot = Some(renderer);
        Ok(report)
    }

    pub(super) unsafe fn prepare_uikit_surface(
        &self,
        view: *mut c_void,
        backend: u32,
    ) -> Result<String, RuntimeRendererError> {
        let renderer_slot = match self.renderer.try_lock() {
            Ok(renderer) => renderer,
            Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => {
                return Err(RuntimeRendererError::InvalidArgument(
                    "runtime GPU renderer가 현재 surface를 사용 중입니다",
                ));
            }
        };
        if renderer_slot.is_some() {
            return Err(RuntimeRendererError::InvalidArgument(
                "runtime GPU renderer가 이미 생성되었습니다",
            ));
        }
        drop(renderer_slot);
        let mut pending_slot = lock(&self.pending_uikit_surface);
        if pending_slot.is_some() {
            return Err(RuntimeRendererError::InvalidArgument(
                "UIKit surface가 이미 준비되었습니다",
            ));
        }
        let surface = unsafe { RuntimeRenderer::prepare_uikit_surface(view, backend) }?;
        *pending_slot = Some(surface);
        Ok("UIKit surface가 준비되었습니다".to_owned())
    }

    pub(super) fn create_uikit(
        &self,
        width: u32,
        height: u32,
    ) -> Result<String, RuntimeRendererError> {
        let mut renderer_slot = lock(&self.renderer);
        if renderer_slot.is_some() {
            return Err(RuntimeRendererError::InvalidArgument(
                "runtime GPU renderer가 이미 생성되었습니다",
            ));
        }
        let pending = lock(&self.pending_uikit_surface).take().ok_or(
            RuntimeRendererError::InvalidArgument(
                "UIKit surface가 메인 스레드에서 준비되지 않았습니다",
            ),
        )?;
        #[cfg(any(
            feature = "c12-4-metal-surface-readback-test",
            feature = "c11-grid-metal-surface-pixels-test",
            feature = "c13-1-overflow-clip-fixture-test",
            feature = "c13-2-scroll-fixture-test"
        ))]
        let (renderer, report) = if self.surface_pixel_readback_test {
            RuntimeRenderer::initialize_uikit_surface_with_copy_src(pending, width, height)?
        } else {
            RuntimeRenderer::initialize_uikit_surface(pending, width, height)?
        };
        #[cfg(not(any(
            feature = "c12-4-metal-surface-readback-test",
            feature = "c11-grid-metal-surface-pixels-test",
            feature = "c13-1-overflow-clip-fixture-test",
            feature = "c13-2-scroll-fixture-test"
        )))]
        let (renderer, report) = RuntimeRenderer::initialize_uikit_surface(pending, width, height)?;
        *renderer_slot = Some(renderer);
        Ok(report)
    }

    pub(super) fn configure_uikit_surface(&self) -> Result<String, RuntimeRendererError> {
        let mut renderer_slot = lock(&self.renderer);
        let renderer = renderer_slot
            .as_mut()
            .ok_or(RuntimeRendererError::InvalidArgument(
                "runtime GPU renderer가 없습니다",
            ))?;
        renderer.configure_surface();
        Ok("UIKit surface가 메인 스레드에서 구성되었습니다".to_owned())
    }

    pub(super) fn resize(&self, width: u32, height: u32) -> Result<(), (i32, String)> {
        let sequence = self
            .invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        let result = lock(&self.renderer)
            .as_mut()
            .ok_or_else(|| (ERR_LAYOUT, "runtime GPU renderer가 없습니다".to_owned()))?
            .resize(width, height)
            .map_err(|error| {
                let status = if matches!(error, RuntimeRendererError::InvalidArgument(_)) {
                    ERR_ARGUMENT
                } else {
                    ERR_LAYOUT
                };
                (status, error.to_string())
            });
        #[cfg(feature = "c13-3-gesture-fixture-test")]
        if result.is_ok()
            && let Some(state) = &self.c13_3_gesture_state
            && let Ok(snapshot) = state.current_snapshot()
            && !self.presentation.publish(sequence, snapshot)
        {
            return Err((
                ERR_STALE,
                "C13.3 resize 뒤 최신 gesture snapshot 복구가 stale입니다".to_owned(),
            ));
        }
        result
    }

    pub(super) fn draw(&self) -> Result<String, (i32, String)> {
        let (sequence, scene) = self.presentation.lock_current();
        let snapshot = scene.as_ref().map(|published| published.snapshot.as_ref());
        let mut renderer_slot = lock(&self.renderer);
        let renderer = renderer_slot
            .as_mut()
            .ok_or_else(|| (ERR_LAYOUT, "runtime GPU renderer가 없습니다".to_owned()))?;
        let presented = renderer
            .draw_if(snapshot, || self.presentation_sequence() == sequence)
            .map_err(|error| (ERR_LAYOUT, error.to_string()))?;
        let surface_texture_size = renderer.last_surface_texture_size();
        let pixel_samples = renderer.take_surface_pixel_samples_for_test();
        drop(renderer_slot);
        if !presented {
            return Err((
                ERR_STALE,
                "새 화면 요청이 도착해 이전 장면 제출을 막았습니다".to_owned(),
            ));
        }
        let mut report = match scene.as_ref() {
            Some(scene) => {
                let viewport = scene.snapshot.viewport_css_px();
                let scroll_offsets = scene
                    .snapshot
                    .scroll_containers()
                    .iter()
                    .map(|owner| format!("{},{}", owner.offset_x_css_px(), owner.offset_y_css_px()))
                    .collect::<Vec<_>>()
                    .join(";");
                let root = scene.snapshot.boxes().first().map(|render_box| {
                    let frame = render_box.frame_css_px();
                    format!(" root_css_px={}x{}", frame.width(), frame.height())
                });
                format!(
                    "presented boxes={} environment_revision={} document_revision={} render_tree_revision={} style_revision={} scroll_state_revision={} scroll_offsets=[{}] generation={} viewport_css_px={}x{} surface_texture={}x{}{}",
                    scene.snapshot.boxes().len(),
                    scene.snapshot.key().environment_revision().get(),
                    scene.snapshot.key().document_revision().get(),
                    scene.snapshot.key().render_tree_revision().get(),
                    scene.snapshot.key().style_revision().get(),
                    scene.snapshot.key().scroll_state_revision().get(),
                    scroll_offsets,
                    scene.snapshot.key().generation().get(),
                    viewport.width(),
                    viewport.height(),
                    surface_texture_size.0,
                    surface_texture_size.1,
                    root.unwrap_or_default(),
                )
            }
            None => format!(
                "presented empty scene while runtime calculation is pending presentation_sequence={sequence}"
            ),
        };
        if let Some(samples) = pixel_samples {
            for sample in &samples {
                report.push_str(&format!(
                    " surface_pixel_x={} surface_pixel_y={} surface_pixel_rgba=#{:02x}{:02x}{:02x}{:02x}",
                    sample.x,
                    sample.y,
                    sample.rgba[0],
                    sample.rgba[1],
                    sample.rgba[2],
                    sample.rgba[3],
                ));
            }
            let sample_values = samples
                .iter()
                .map(|sample| {
                    serde_json::json!({
                        "x": sample.x,
                        "y": sample.y,
                        "rgba": sample.rgba,
                    })
                })
                .collect::<Vec<_>>();
            let sample_json = serde_json::to_string(&sample_values)
                .map_err(|error| (ERR_LAYOUT, format!("surface pixel JSON 변환 실패: {error}")))?;
            report.push_str(&format!(" surface_pixel_samples_json={sample_json}"));
        }
        Ok(report)
    }

    #[cfg(any(
        feature = "c12-4-metal-surface-readback-test",
        feature = "c11-grid-metal-surface-pixels-test",
        feature = "c13-1-overflow-clip-fixture-test",
        feature = "c13-2-scroll-fixture-test"
    ))]
    pub(super) fn request_surface_pixel_readback_for_test(
        &self,
        points: &[(u32, u32)],
    ) -> Result<String, String> {
        if !self.surface_pixel_readback_test {
            return Err("Metal surface pixel readback 전용 host가 아닙니다".to_owned());
        }
        let mut renderer_slot = lock(&self.renderer);
        let renderer = renderer_slot
            .as_mut()
            .ok_or_else(|| "runtime GPU renderer가 없습니다".to_owned())?;
        renderer
            .request_surface_pixel_readback_for_test(points)
            .map_err(|error| error.to_string())?;
        Ok(format!(
            "Metal surface 표본을 요청했습니다 count={}",
            points.len()
        ))
    }

    #[cfg(feature = "c04-runtime-gpu-test-hooks")]
    pub(super) fn inject_next_draw_failure_for_test(&self) -> Result<String, (i32, String)> {
        let mut renderer_slot = lock(&self.renderer);
        let renderer = renderer_slot
            .as_mut()
            .ok_or_else(|| (ERR_LAYOUT, "runtime GPU renderer가 없습니다".to_owned()))?;
        renderer.inject_next_draw_failure_for_test();
        Ok("시험용 다음 draw 실패를 설정했습니다".to_owned())
    }

    pub(super) fn destroy_renderer(&self) -> Result<(), (i32, String)> {
        let renderer = lock(&self.renderer).take();
        let pending = lock(&self.pending_uikit_surface).take();
        drop(renderer);
        drop(pending);
        Ok(())
    }
}
