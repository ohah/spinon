use super::*;
use std::time::Duration;

fn assert_failed_without_partial_snapshot(
    layout: &crate::session::RuntimeLayoutSnapshot,
    label: &str,
) {
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{label}: unsupported style fails closed: {:?}",
        layout.error
    );
    assert!(
        layout.completed.is_none(),
        "{label}: no partial snapshot is published"
    );
    assert!(
        layout
            .error
            .as_ref()
            .is_some_and(|error| error.node_id.is_some() && error.property.is_some())
    );
}

#[test]
fn parsed_root_html_five_fit_mixed_minimum_rejects_auto_on_explicit_row_control_atomically() {
    use spinon_style::CssMediaEnvironment;

    let track_list = batches().keys().next().expect("five-fit track list exists");
    let source = source_for_six_fit_mixed_minimum_track_list(track_list).replace(
        "</style>",
        "@media (min-height:10000px) { #grid .empty { min-height:auto; } } </style>",
    );
    let (session, initial) = run(&source);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "control-zero profile initializes: {:?}",
        initial.error
    );

    session
        .set_root_html_environment(1280.0, 10_000.0, 1.0, CssMediaEnvironment::MOBILE)
        .expect("height query activates automatic minimum on the explicit-row control");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_failed_without_partial_snapshot(&rejected, "automatic-minimum control");

    session
        .set_root_html_environment(1280.0, 320.0, 1.0, CssMediaEnvironment::MOBILE)
        .expect("valid zero-mask profile restores after rejection");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "same-session recovery succeeds: {:?}",
        recovered.error
    );
    assert!(recovered.completed.is_some());
}
