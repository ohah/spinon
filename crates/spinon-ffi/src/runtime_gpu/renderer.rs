use std::ffi::c_void;

use spinon_render_wgpu::{RuntimeRenderer, RuntimeRendererError};

use super::{ERR_ARGUMENT, ERR_LAYOUT, ERR_STALE, RuntimeGpuHost, lock};

impl RuntimeGpuHost {
    #[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
    fn root_html_block_paint_surface_samples(
        viewport: spinon_render::CssSize,
        surface_width_px: u32,
        surface_height_px: u32,
    ) -> Result<Vec<(u32, u32)>, String> {
        const CSS_POINTS: [(u32, u32); 5] = [(10, 10), (10, 40), (10, 60), (200, 40), (10, 210)];
        if surface_width_px == 0 || surface_height_px == 0 {
            return Err("C04.12 surface pixel 표면 크기가 0입니다".to_owned());
        }
        let viewport_width = f64::from(viewport.width());
        let viewport_height = f64::from(viewport.height());
        if !viewport_width.is_finite()
            || viewport_width <= 0.0
            || !viewport_height.is_finite()
            || viewport_height <= 0.0
        {
            return Err("C04.12 surface pixel CSS viewport가 유효하지 않습니다".to_owned());
        }
        let mut points = Vec::new();
        points
            .try_reserve_exact(CSS_POINTS.len())
            .map_err(|_| "C04.12 surface pixel 표본 목록을 할당하지 못했습니다".to_owned())?;
        for (x_css_px, y_css_px) in CSS_POINTS {
            if f64::from(x_css_px) + 0.5 >= viewport_width
                || f64::from(y_css_px) + 0.5 >= viewport_height
            {
                return Err(format!(
                    "C04.12 surface pixel CSS 표본이 viewport 밖입니다: {x_css_px},{y_css_px}"
                ));
            }
            let x = ((f64::from(x_css_px) + 0.5) * f64::from(surface_width_px) / viewport_width)
                .floor();
            let y = ((f64::from(y_css_px) + 0.5) * f64::from(surface_height_px) / viewport_height)
                .floor();
            if x < 0.0
                || y < 0.0
                || x >= f64::from(surface_width_px)
                || y >= f64::from(surface_height_px)
            {
                return Err(format!(
                    "C04.12 surface pixel 표본이 drawable 밖입니다: {x},{y}"
                ));
            }
            points.push((x as u32, y as u32));
        }
        Ok(points)
    }

    #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
    pub(super) fn readback_c19_2_offscreen_samples(
        &self,
        width: u32,
        height: u32,
        points: &[(u32, u32)],
    ) -> Result<Vec<[u8; 4]>, String> {
        let (_, published) = self.presentation.lock_current();
        let snapshot = published
            .as_ref()
            .map(|scene| std::sync::Arc::clone(&scene.snapshot))
            .ok_or_else(|| {
                "C19.2 offscreen readback 전에 render scene이 publish되지 않았습니다".to_owned()
            })?;
        drop(published);
        spinon_render_wgpu::readback_runtime_scene_samples(&snapshot, width, height, points)
    }

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
        #[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
        let (renderer, report) = if self.root_html_block_paint_fixture {
            let (renderer, report) =
                unsafe { RuntimeRenderer::new_android(native_window, width, height, backend) }?;
            (
                renderer,
                format!("{report} surface_pixel_readback_path=AndroidPixelCopy"),
            )
        } else {
            #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
            let result = if self.c19_2_android_surface_readback_test {
                match unsafe {
                    RuntimeRenderer::new_android_with_copy_src(
                        native_window,
                        width,
                        height,
                        backend,
                        true,
                    )
                } {
                    Ok((renderer, report)) => {
                        (renderer, format!("{report} surface_copy_src=enabled"))
                    }
                    Err(readback_error) => {
                        let (renderer, report) = unsafe {
                            RuntimeRenderer::new_android(native_window, width, height, backend)
                        }?;
                        (
                            renderer,
                            format!(
                                "{report} surface_copy_src=unavailable reason={readback_error}"
                            ),
                        )
                    }
                }
            } else {
                unsafe { RuntimeRenderer::new_android(native_window, width, height, backend) }?
            };
            #[cfg(not(feature = "c19-2-box-paint-opacity-fixture-test"))]
            let result =
                unsafe { RuntimeRenderer::new_android(native_window, width, height, backend) }?;
            result
        };
        #[cfg(all(
            not(feature = "c04-12-root-html-block-paint-fixture-test"),
            feature = "c19-2-box-paint-opacity-fixture-test"
        ))]
        let (renderer, report) = if self.c19_2_android_surface_readback_test {
            match unsafe {
                RuntimeRenderer::new_android_with_copy_src(
                    native_window,
                    width,
                    height,
                    backend,
                    true,
                )
            } {
                Ok((renderer, report)) => (renderer, format!("{report} surface_copy_src=enabled")),
                Err(readback_error) => {
                    let (renderer, report) = unsafe {
                        RuntimeRenderer::new_android(native_window, width, height, backend)
                    }?;
                    (
                        renderer,
                        format!("{report} surface_copy_src=unavailable reason={readback_error}"),
                    )
                }
            }
        } else {
            unsafe { RuntimeRenderer::new_android(native_window, width, height, backend) }?
        };
        #[cfg(not(any(
            feature = "c04-12-root-html-block-paint-fixture-test",
            feature = "c19-2-box-paint-opacity-fixture-test"
        )))]
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
            feature = "c13-2-scroll-fixture-test",
            feature = "c18-1-image-paint-fixture-test",
            feature = "c18-2-alpha-image-compositing-fixture-test",
            feature = "c18-3-object-fit-position-fixture-test",
            feature = "c18-4-image-fixture-test",
            feature = "c19-2-box-paint-opacity-fixture-test",
            feature = "c15-3-wgpu-glyph-paint-fixture-test"
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
            feature = "c13-2-scroll-fixture-test",
            feature = "c18-1-image-paint-fixture-test",
            feature = "c18-2-alpha-image-compositing-fixture-test",
            feature = "c18-3-object-fit-position-fixture-test",
            feature = "c18-4-image-fixture-test",
            feature = "c19-2-box-paint-opacity-fixture-test",
            feature = "c15-3-wgpu-glyph-paint-fixture-test"
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
        let _sequence = self
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
            && !self.presentation.publish(_sequence, snapshot)
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
        #[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
        let mut root_html_block_paint_sample_points = None;
        #[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
        if self.root_html_block_paint_fixture
            && let Some(snapshot) = snapshot
        {
            let (surface_width_px, surface_height_px) = renderer.surface_dimensions();
            let points = Self::root_html_block_paint_surface_samples(
                snapshot.viewport_css_px(),
                surface_width_px,
                surface_height_px,
            )
            .map_err(|error| (ERR_ARGUMENT, error))?;
            #[cfg(target_os = "ios")]
            renderer
                .request_surface_pixel_readback_for_test(&points)
                .map_err(|error| (ERR_LAYOUT, error.to_string()))?;
            root_html_block_paint_sample_points = Some(points);
        }
        let presented = renderer
            .draw_if(snapshot, || self.presentation_sequence() == sequence)
            .map_err(|error| (ERR_LAYOUT, error.to_string()))?;
        let surface_texture_size = renderer.last_surface_texture_size();
        let image_draw_count = renderer.last_image_draw_count();
        let glyph_batch_draw_count = renderer.last_glyph_batch_draw_count();
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
        let runtime_image_count = snapshot.map_or(0, |scene| scene.images().len());
        let runtime_text_count = snapshot.map_or(0, |scene| scene.texts().len());
        report.push_str(&format!(
            " runtime_image_count={runtime_image_count} image_draw_count={image_draw_count} runtime_text_count={runtime_text_count} glyph_batch_draw_count={glyph_batch_draw_count}"
        ));
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
        #[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
        if let Some(points) = root_html_block_paint_sample_points {
            let encoded_points = points
                .iter()
                .map(|(x, y)| format!("{x},{y}"))
                .collect::<Vec<_>>()
                .join(";");
            report.push_str(&format!(" root_html_sample_surface_xy={encoded_points}"));
        }
        Ok(report)
    }

    #[cfg(any(
        feature = "c12-4-metal-surface-readback-test",
        feature = "c11-grid-metal-surface-pixels-test",
        feature = "c13-1-overflow-clip-fixture-test",
        feature = "c13-2-scroll-fixture-test",
        feature = "c18-1-image-paint-fixture-test",
        feature = "c18-2-alpha-image-compositing-fixture-test",
        feature = "c18-3-object-fit-position-fixture-test",
        feature = "c18-4-image-fixture-test",
        feature = "c19-2-box-paint-opacity-fixture-test",
        feature = "c15-3-wgpu-glyph-paint-fixture-test"
    ))]
    pub(super) fn request_surface_pixel_readback_for_test(
        &self,
        points: &[(u32, u32)],
    ) -> Result<String, String> {
        if !self.surface_pixel_readback_test {
            return Err("surface pixel readback 전용 host가 아닙니다".to_owned());
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

#[cfg(all(test, feature = "c04-12-root-html-block-paint-fixture-test"))]
mod root_html_block_paint_readback_tests {
    use super::RuntimeGpuHost;

    #[test]
    fn css_pixel_centers_map_to_each_surface_axis_without_a_fixed_dpr() {
        let viewport = spinon_render::CssSize::new(320.0, 240.0).unwrap();
        assert_eq!(
            RuntimeGpuHost::root_html_block_paint_surface_samples(viewport, 840, 630).unwrap(),
            [(27, 27), (27, 106), (27, 158), (526, 106), (27, 552)]
        );
        assert_eq!(
            RuntimeGpuHost::root_html_block_paint_surface_samples(viewport, 960, 720).unwrap(),
            [(31, 31), (31, 121), (31, 181), (601, 121), (31, 631)]
        );
    }

    #[test]
    fn invalid_surface_and_outside_css_samples_fail_closed() {
        let viewport = spinon_render::CssSize::new(320.0, 240.0).unwrap();
        assert!(RuntimeGpuHost::root_html_block_paint_surface_samples(viewport, 0, 720).is_err());
        let short_viewport = spinon_render::CssSize::new(200.0, 200.0).unwrap();
        assert!(
            RuntimeGpuHost::root_html_block_paint_surface_samples(short_viewport, 600, 600)
                .is_err()
        );
    }
}
