use super::super::three_fit_auto_minimum::{
    configure_six_fit_mixed_minimum_scene, source_for_six_fit_mixed_minimum_track_list,
};
use super::*;
use std::time::Duration;

fn assert_failed_without_partial_snapshot(
    layout: &crate::session::RuntimeLayoutSnapshot,
    label: &str,
) {
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{label}: unsupported style must fail closed: {:?}",
        layout.error
    );
    assert!(
        layout.completed.is_none(),
        "{label}: no partial output is published"
    );
    assert!(
        layout
            .error
            .as_ref()
            .is_some_and(|error| error.node_id.is_some() && error.property.is_some())
    );
}

#[test]
fn parsed_root_html_four_fit_mixed_minimum_rejects_control_and_invalid_item_then_recovers() {
    let record = super::records()
        .iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["topology"] == "span2Six-forward"
                && scene["minimumMask"] == 5
                && scene["alignContent"] == "start"
        })
        .expect("sample contains a multi-item automatic-minimum mask");
    assert_eq!(super::encoded_minimum_mask(&record["scene"]), 5);
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let source = source_for_six_fit_mixed_minimum_track_list(track_list).replace(
        "</style>",
        "@media (min-height:10000px) { #grid .empty { min-height:auto; } } \
         @media (min-width:1287px) { #grid .item[data-variant=\"3\"][data-item=\"A\"] { min-height:1px; } } </style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "four-fit mixed profile starts ready: {:?}",
        initial.error
    );

    session
        .set_root_html_environment(
            1280.0,
            10_000.0,
            1.0,
            spinon_style::CssMediaEnvironment::MOBILE,
        )
        .expect("activate invalid explicit-row control minimum");
    let bad_control = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_failed_without_partial_snapshot(&bad_control, "control auto");

    configure_six_fit_mixed_minimum_scene(&session, record);
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
        .expect("activate item minimum outside 0px|auto");
    let bad_item = session.wait_for_layout_snapshot(Duration::from_secs(5));
    let cascade = session.ua_cascade_snapshot();
    let styles = &cascade.completed.as_ref().unwrap().roots[0].styles.elements;
    assert!(
        styles.iter().any(|style| {
            style.background_color
                == Some(spinon_style::OpaqueCssSrgb {
                    red: 31,
                    green: 41,
                    blue: 54,
                })
                && style.layout_dimensions.min_height
                    == spinon_style::ComputedCssDimension::LengthPx(1.0)
        }),
        "active A item receives the invalid 1px minimum"
    );
    assert_failed_without_partial_snapshot(&bad_item, "item min-height:1px");

    configure_six_fit_mixed_minimum_scene(&session, record);
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery after invalid item rejection: {:?}",
        recovered.error
    );
}
