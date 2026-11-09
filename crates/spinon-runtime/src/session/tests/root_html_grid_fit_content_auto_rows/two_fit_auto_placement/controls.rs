use super::*;
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage as LengthPercentage,
    ComputedCssGridTrackSize as TrackSize,
};

#[test]
fn parsed_root_html_two_fit_auto_placement_accepts_unlisted_typed_caps() {
    for (label, track_list, expected_tracks, expected_rows) in [
        (
            "non-matrix px and percentage caps",
            "fit-content(7px) fit-content(13%) min-content min-content max-content auto",
            vec![
                TrackSize::FitContent(LengthPercentage::LengthPx(7.0)),
                TrackSize::FitContent(LengthPercentage::Percentage(0.13)),
                TrackSize::MinContent,
                TrackSize::MinContent,
                TrackSize::MaxContent,
                TrackSize::Auto,
            ],
            [10.0, 7.0, 20.0, 20.0, 30.0, 20.0, 20.0, 7.0, 10.0],
        ),
        (
            "over-container percentage cap",
            "fit-content(120%) fit-content(20%) min-content min-content max-content auto",
            vec![
                TrackSize::FitContent(LengthPercentage::Percentage(1.2)),
                TrackSize::FitContent(LengthPercentage::Percentage(0.2)),
                TrackSize::MinContent,
                TrackSize::MinContent,
                TrackSize::MaxContent,
                TrackSize::Auto,
            ],
            [10.0, 20.0, 20.0, 20.0, 30.0, 20.0, 20.0, 30.0, 10.0],
        ),
    ] {
        let source = replace_once(SOURCE, BASE_TRACK_LIST, track_list);
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{label}: {:?}; cascade={:?}",
            layout.error,
            session.ua_cascade_snapshot().error
        );
        let completed = layout.completed.as_ref().expect("complete control layout");
        let cascade = session
            .ua_cascade_snapshot()
            .completed
            .expect("control computed style");
        let grid = cascade.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("one control Grid");
        assert_eq!(
            grid.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(expected_tracks),
            "{label}: exact px/percentage type and order"
        );
        for (index, expected_height) in expected_rows.into_iter().enumerate() {
            let actual = frame_for_background(
                completed,
                &cascade.roots[0].styles.elements,
                ITEM_COLORS[index],
                label,
            );
            assert_frame(
                [actual[3], 0.0, 0.0, 0.0],
                [expected_height, 0.0, 0.0, 0.0],
                &format!("{label} item {index}"),
            );
        }
    }
}
