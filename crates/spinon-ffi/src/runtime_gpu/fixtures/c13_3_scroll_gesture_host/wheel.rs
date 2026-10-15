use super::state::C13_3GestureState;
use super::{ERR_LAYOUT, RuntimeGpuHost};
use spinon_render::{RuntimeRenderSnapshot, RuntimeScrollWheelSample};

pub(super) fn process_c13_3_wheel(
    host: &RuntimeGpuHost,
    state: &C13_3GestureState,
    snapshot: &RuntimeRenderSnapshot,
    sequence: u64,
    sample: RuntimeScrollWheelSample,
    coalesced_samples: u32,
) -> Result<String, (i32, String)> {
    let sample_epoch = sample.expected_key().style_revision().get();
    if sample.policy_epoch() != sample_epoch {
        return Ok(format!(
            "event=wheel sequence={sequence} rejected=sample-policy-epoch-mismatch sample_epoch={} policy_epoch={} target={}",
            sample.policy_epoch(),
            sample_epoch,
            sample.target().get()
        ));
    }
    let current_epoch = snapshot.key().style_revision().get();
    if sample.policy_epoch() != current_epoch {
        return Ok(format!(
            "event=wheel sequence={sequence} discarded=stale-policy-epoch sample_epoch={} current_epoch={} target={}",
            sample.policy_epoch(),
            current_epoch,
            sample.target().get()
        ));
    }
    if !state.is_latest_surface_generation(sample.surface_generation()) {
        return Ok(format!(
            "event=wheel sequence={sequence} discarded=stale-surface-generation sample_generation={} target={}",
            sample.surface_generation(),
            sample.target().get()
        ));
    }
    if !state.wheel_sample_matches_lineage(sample, snapshot.key()) {
        return Ok(format!(
            "event=wheel sequence={sequence} discarded=stale-snapshot-lineage basis_revision={} current_revision={} target={}",
            sample.basis_scroll_revision().get(),
            snapshot.key().scroll_state_revision().get(),
            sample.target().get()
        ));
    }

    let outcome = match snapshot.route_scroll_delta_for_target(
        snapshot.key().scroll_state_revision(),
        sample.target(),
        sample.delta_x_css_px(),
        sample.delta_y_css_px(),
    ) {
        Ok(outcome) => outcome,
        Err(error) => {
            return Ok(format!(
                "event=wheel sequence={sequence} rejected=router-error:{error} target={} basis_revision={}",
                sample.target().get(),
                sample.basis_scroll_revision().get()
            ));
        }
    };
    if !state.is_latest_surface_generation(sample.surface_generation()) {
        return Ok(format!(
            "event=wheel sequence={sequence} discarded=surface-generation-changed-during-route sample_generation={} target={}",
            sample.surface_generation(),
            sample.target().get()
        ));
    }
    let published = host.publish_c13_3_delta(
        state,
        &outcome,
        snapshot,
        sequence,
        sample.surface_generation(),
    )?;
    let owners = state.owners().map_err(|error| (ERR_LAYOUT, error))?;
    let details = super::c13_3_offsets(outcome.snapshot(), owners);
    Ok(format!(
        "event=wheel sequence={sequence} target={} point_css_px={:.2},{:.2} delta_css_px={:.2},{:.2} raw_delta={:.2},{:.2} unit={:?} source={:?} scale={:.3} momentum={:?} modifiers={} policy_epoch={} coalesced_samples={} published={published} {} result=[{}]",
        sample.target().get(),
        sample.point_x_css_px(),
        sample.point_y_css_px(),
        sample.delta_x_css_px(),
        sample.delta_y_css_px(),
        sample.raw_delta_x(),
        sample.raw_delta_y(),
        sample.source_unit(),
        sample.source(),
        sample.coordinate_scale(),
        sample.momentum_phase(),
        sample.modifier_flags(),
        sample.policy_epoch(),
        coalesced_samples,
        details,
        super::c13_3_delta_summary(&outcome)
    ))
}
