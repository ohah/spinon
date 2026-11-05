use super::*;

#[test]
fn parsed_root_html_intrinsic_rows_accept_10000_tracks_and_reject_10001_early() {
    const MAX_TRACKS: usize = 10_000;

    let mut fixture = Fixture::new();
    let final_item = fixture
        .add_empty_items(MAX_TRACKS - 3)
        .expect("a full track list should add at least one direct item");
    let output = fixture
        .compute()
        .expect("the maximum supported implicit-row count should lay out");
    let final_frame = output.layout.frames[&final_item.id()];
    assert_eq!(
        [
            final_frame.x,
            final_frame.y,
            final_frame.width,
            final_frame.height
        ],
        [0.0, 10_060.0, 1.0, 1.0]
    );
    assert!(
        output.layout.grid_track_sizing_traces.is_empty(),
        "the oversized optional trace should be omitted without discarding computed frames"
    );

    let _ = fixture.add_empty_items(1);
    let error = match fixture.compute() {
        Err(error) => error,
        Ok(_) => panic!("the 10,001st row should fail before frame layout"),
    };
    match error {
        StyleLayoutError::UnsupportedComputedValue {
            property, value, ..
        } => {
            assert_eq!(property, "grid-auto-rows");
            assert!(value.contains("exceed the 10,000-track limit"));
        }
        other => panic!("expected the early Grid track limit rejection, got {other:?}"),
    }
}
