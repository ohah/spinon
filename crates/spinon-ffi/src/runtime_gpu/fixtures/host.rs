use super::super::RuntimeGpuHost;
use spinon_runtime::RuntimeSession;

impl RuntimeGpuHost {
    pub(in crate::runtime_gpu) fn new_registered_properties_fixture()
    -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_registered_properties_fixture()?;
        Ok((Self::from_session(session), report))
    }

    pub(in crate::runtime_gpu) fn new_block_paint_fixture() -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_paint_fixture()?;
        Ok((Self::from_session(session), report))
    }

    pub(in crate::runtime_gpu) fn new_block_formatting_fixture() -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_formatting_fixture()?;
        Ok((Self::from_session_with_frame_report(session), report))
    }

    pub(in crate::runtime_gpu) fn new_c12_1_position_fixture() -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu()?;
        Ok((Self::from_session_with_frame_report(session), report))
    }

    pub(in crate::runtime_gpu) fn new_c12_2_absolute_block_fixture()
    -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_positioning_fixture()?;
        Ok((Self::from_session_with_frame_report(session), report))
    }

    pub(in crate::runtime_gpu) fn new_c12_3_fixed_fixture() -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_positioning_fixture()?;
        Ok((Self::from_session_with_frame_report(session), report))
    }

    pub(in crate::runtime_gpu) fn new_c12_4_stacking_fixture() -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_flex_stacking_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        Ok((host, report))
    }

    #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
    pub(in crate::runtime_gpu) fn new_c13_1_overflow_clip_fixture() -> Result<(Self, String), String>
    {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_flex_overflow_clip_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        host.include_c13_1_hit_test_report = true;
        Ok((host, report))
    }

    #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
    pub(in crate::runtime_gpu) fn new_c13_1_overflow_clip_border_fixture()
    -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_flex_overflow_clip_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        host.include_c13_1_border_hit_test_report = true;
        Ok((host, report))
    }

    pub(in crate::runtime_gpu) fn new_c11_1_grid_fixture() -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_flex_grid_fixture()?;
        Ok((Self::from_session_with_frame_report(session), report))
    }

    pub(in crate::runtime_gpu) fn new_c11_2_1_grid_placement_fixture()
    -> Result<(Self, String), String> {
        let (session, report) =
            RuntimeSession::new_runtime_gpu_block_flex_grid_placement_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        #[cfg(feature = "c11-grid-metal-surface-pixels-test")]
        {
            host.surface_pixel_readback_test = true;
        }
        Ok((host, report))
    }

    pub(in crate::runtime_gpu) fn new_c11_2_2_grid_auto_placement_fixture()
    -> Result<(Self, String), String> {
        let (session, report) =
            RuntimeSession::new_runtime_gpu_block_flex_grid_auto_placement_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        #[cfg(feature = "c11-grid-metal-surface-pixels-test")]
        {
            host.surface_pixel_readback_test = true;
        }
        Ok((host, report))
    }

    #[cfg(feature = "c11-2-3-grid-fixture-test")]
    pub(in crate::runtime_gpu) fn new_c11_2_3_grid_auto_placement_fixture()
    -> Result<(Self, String), String> {
        let (session, report) =
            RuntimeSession::new_runtime_gpu_block_flex_grid_dense_auto_placement_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        #[cfg(feature = "c11-grid-metal-surface-pixels-test")]
        {
            host.surface_pixel_readback_test = true;
        }
        Ok((host, report))
    }

    pub(in crate::runtime_gpu) fn new_c11_3_1_grid_repeat_fixture() -> Result<(Self, String), String>
    {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_flex_grid_template_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        #[cfg(feature = "c11-grid-metal-surface-pixels-test")]
        {
            host.surface_pixel_readback_test = true;
        }
        Ok((host, report))
    }

    pub(in crate::runtime_gpu) fn new_c11_3_2_grid_named_lines_fixture()
    -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_flex_grid_template_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        #[cfg(feature = "c11-grid-metal-surface-pixels-test")]
        {
            host.surface_pixel_readback_test = true;
        }
        Ok((host, report))
    }

    pub(in crate::runtime_gpu) fn new_c11_3_3_grid_template_areas_fixture()
    -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_block_flex_grid_template_fixture()?;
        let mut host = Self::from_session_with_frame_report(session);
        host.include_paint_order_report = true;
        #[cfg(feature = "c11-grid-metal-surface-pixels-test")]
        {
            host.surface_pixel_readback_test = true;
        }
        Ok((host, report))
    }

    #[cfg(feature = "c12-4-worker-stack-residency-test")]
    pub(in crate::runtime_gpu) fn css_worker_stack_residency_sample_json_for_test(
        &self,
    ) -> Result<String, String> {
        self.session
            .css_worker_stack_residency_sample_json_for_test()
    }

    #[cfg(feature = "c12-4-metal-surface-readback-test")]
    pub(in crate::runtime_gpu) fn new_c12_4_surface_readback_fixture()
    -> Result<(Self, String), String> {
        let (mut host, report) = Self::new_c12_4_stacking_fixture()?;
        host.surface_pixel_readback_test = true;
        Ok((host, report))
    }

    #[cfg(feature = "c11-grid-metal-surface-pixels-test")]
    pub(in crate::runtime_gpu) fn new_c11_1_grid_surface_pixel_readback_fixture()
    -> Result<(Self, String), String> {
        let (mut host, report) = Self::new_c11_1_grid_fixture()?;
        host.surface_pixel_readback_test = true;
        Ok((host, report))
    }

    pub(in crate::runtime_gpu) fn new_c10_3_5_positioned_flex_fixture()
    -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu()?;
        Ok((Self::from_session_with_frame_report(session), report))
    }
}
