use super::S04Scene;

impl S04Scene {
    pub(crate) fn hit_test_report(
        &self,
        expected_surface_generation: u64,
        surface_x: f32,
        surface_y: f32,
    ) -> Result<HitTestResult, HitTestFailure> {
        validate_hit_test_frame(
            self.surface_generation,
            self.last_submitted_surface_generation,
            self.frame_sequence,
            expected_surface_generation,
        )?;
        let Some((css_x, css_y)) = self
            .surface_mapping
            .css_point_from_surface(surface_x, surface_y)
            .map_err(|message| HitTestFailure { code: -4, message })?
        else {
            return Ok(HitTestResult::Miss(format!(
                "target=none reason=letterbox-or-viewport-edge surface_generation={} frame_sequence={} fixture={}",
                self.surface_generation,
                self.frame_sequence,
                self.snapshot.source().fixture_id,
            )));
        };
        let Some(render_box) = self.snapshot.hit_test_css_point(css_x, css_y).copied() else {
            return Ok(HitTestResult::Miss(format!(
                "target=none reason=no-box css=({css_x:.3},{css_y:.3}) surface_generation={} frame_sequence={} fixture={}",
                self.surface_generation,
                self.frame_sequence,
                self.snapshot.source().fixture_id,
            )));
        };
        Ok(HitTestResult::Hit(format!(
            "target=node_id={} paint_order={} css=({css_x:.3},{css_y:.3}) surface_generation={} frame_sequence={} fixture={}",
            render_box.node_id(),
            render_box.paint_order(),
            self.surface_generation,
            self.frame_sequence,
            self.snapshot.source().fixture_id,
        )))
    }
}

pub(crate) enum HitTestResult {
    Hit(String),
    Miss(String),
}

pub(crate) struct HitTestFailure {
    pub(crate) code: i32,
    pub(crate) message: String,
}

fn validate_hit_test_frame(
    current_surface_generation: u64,
    submitted_surface_generation: Option<u64>,
    frame_sequence: u64,
    expected_surface_generation: u64,
) -> Result<(), HitTestFailure> {
    if current_surface_generation != expected_surface_generation {
        return Err(HitTestFailure {
            code: -3,
            message: format!(
                "S04 stale surface generation: expected={expected_surface_generation} current={current_surface_generation}"
            ),
        });
    }
    if frame_sequence == 0 || submitted_surface_generation != Some(current_surface_generation) {
        return Err(HitTestFailure {
            code: -2,
            message: format!(
                "S04 surface generation {current_surface_generation}에 제출된 frame이 없습니다"
            ),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_hit_test_frame;

    #[test]
    fn hit_test_requires_current_surface_and_a_submitted_frame() {
        assert!(validate_hit_test_frame(4, Some(4), 1, 4).is_ok());
        assert!(validate_hit_test_frame(4, Some(4), 0, 4).is_err());
        assert!(validate_hit_test_frame(5, None, 2, 5).is_err());
        assert!(validate_hit_test_frame(5, Some(4), 2, 5).is_err());
        assert!(validate_hit_test_frame(5, Some(5), 2, 4).is_err());
    }
}
