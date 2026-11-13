use super::mixed_dynamic::{
    configure_three_fit_mixed_minimum_scene, source_for_three_fit_mixed_minimum_track_list,
};
use super::*;
use std::time::Duration;

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(source.contains(from), "fixture fragment missing: {from:?}");
    source.replacen(from, to, 1)
}

fn assert_rejected_atomically(name: &str, source: &str) {
    let (_session, layout) = run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{name} must fail closed: {:?}",
        layout.error
    );
    assert!(
        layout.completed.is_none(),
        "{name} must publish no partial layout"
    );
    assert!(
        layout
            .error
            .as_ref()
            .is_some_and(|error| { error.node_id.is_some() && error.property.is_some() })
    );
}

#[test]
fn parsed_root_html_three_fit_mixed_minimum_rejects_out_of_profile_track_list() {
    let record = super::mixed_matrix::records()
        .iter()
        .find(|record| {
            record["scene"]["topology"] == "span2Six-forward"
                && record["scene"]["minimumMask"] == 5
                && record["scene"]["alignContent"] == "start"
        })
        .expect("runtime sample includes a mixed partial mask");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let source = source_for_three_fit_mixed_minimum_track_list(track_list);
    let invalid_track_list = track_list.replacen("fit-content(0px)", "fit-content(1px)", 1);
    let invalid_cap = source.replace(
        "</style>",
        &format!(
            "@media (min-width:1290px) {{ #grid {{ grid-auto-rows:{invalid_track_list}; }} }} </style>"
        ),
    );
    let (session, initial) = run(&invalid_cap);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    session
        .set_root_html_environment(
            1290.0,
            405.0,
            1.0,
            spinon_style::CssMediaEnvironment::MOBILE,
        )
        .expect("activate cap outside exact 0px|10px|20% multiset");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert!(
        rejected.completed.is_none(),
        "invalid cap is rejected atomically"
    );

    let invalid_placement =
        replace_once(&source, "grid-auto-flow:row;", "grid-auto-flow:row dense;");
    assert_rejected_atomically("dense auto-placement", &invalid_placement);
}

#[test]
fn parsed_root_html_three_fit_mixed_minimum_rejects_control_and_invalid_item_then_recovers() {
    let record = super::mixed_matrix::records()
        .iter()
        .find(|record| {
            record["scene"]["topology"] == "span2Six-forward"
                && record["scene"]["minimumMask"] == 5
                && record["scene"]["alignContent"] == "start"
        })
        .expect("runtime sample includes a mixed partial mask");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let source = source_for_three_fit_mixed_minimum_track_list(track_list).replace(
        "</style>",
        "@media (min-height:10000px) { #grid .empty { min-height:auto; } } \
         @media (min-width:1287px) { #grid .item[data-variant=\"3\"][data-item=\"A\"] { min-height:1px; } } </style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "valid mixed-minimum source initializes: {:?}",
        initial.error
    );

    session
        .set_root_html_environment(
            1286.0,
            10_000.0,
            1.0,
            spinon_style::CssMediaEnvironment::MOBILE,
        )
        .expect("activate invalid explicit-row control minimum");
    let bad_control = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        bad_control.state,
        crate::session::RuntimeLayoutState::Failed
    );
    assert!(
        bad_control.completed.is_none(),
        "control rejection is atomic"
    );

    configure_three_fit_mixed_minimum_scene(&session, record);
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery after control rejection: {:?}",
        recovered.error
    );

    session
        .set_root_html_environment(
            1287.0,
            405.0,
            1.0,
            spinon_style::CssMediaEnvironment::MOBILE,
        )
        .expect("activate an item minimum outside 0px|auto");
    let bad_item = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(bad_item.state, crate::session::RuntimeLayoutState::Failed);
    assert!(bad_item.completed.is_none(), "item rejection is atomic");

    configure_three_fit_mixed_minimum_scene(&session, record);
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery after item rejection: {:?}",
        recovered.error
    );
}
