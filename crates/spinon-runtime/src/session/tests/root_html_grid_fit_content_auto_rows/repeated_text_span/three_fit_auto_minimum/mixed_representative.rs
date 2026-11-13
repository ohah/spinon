use super::mixed_dynamic::{
    configure_three_fit_mixed_minimum_scene, source_for_three_fit_mixed_minimum_track_list,
};
use super::*;

const REPRESENTATIVE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-precomparison-2026-11-14/representative-scene-54448.json"
));
const SIMULATOR_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum-runtime-2026-11-14/root-html-mask16-simulator-fixture.html"
));

#[test]
fn parsed_root_html_three_fit_mixed_minimum_preserves_mask16_border_overflow_scene() {
    let record: Value =
        serde_json::from_str(REPRESENTATIVE).expect("representative Chrome scene is JSON");
    let scene = &record["scene"];
    assert_eq!(record["index"], 54_448);
    assert_eq!(scene["minimumMask"], 16);
    assert_eq!(scene["topology"], "span2Six-forward");
    assert_eq!(scene["autoItems"], serde_json::json!(["E"]));
    assert_eq!(
        scene["trackList"],
        "fit-content(20%) min-content auto fit-content(0px) fit-content(10px) max-content"
    );

    let (session, initial) = run(&source_for_three_fit_mixed_minimum_track_list(
        scene["trackList"].as_str().unwrap(),
    ));
    let cascade_error = session.ua_cascade_snapshot().error;
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "representative Root HTML source initializes: {:?}; cascade={cascade_error:?}",
        initial.error,
    );
    configure_three_fit_mixed_minimum_scene(&session, &record);
    let actual = super::observe_current(&session, &record, "mask-16 border-overflow scene");

    let grid = frame(&record["gridFrame"]);
    let content_bottom = grid[1] + grid[3] - 1.0;
    let d_payload = actual.payload_frames["D"];
    let d_border_overflow = d_payload[1] + d_payload[3] - content_bottom;
    assert!(
        (d_border_overflow - 10.0).abs() <= 0.01,
        "D remains min-height:0 and its payload paints 10px outside the red border: {d_border_overflow}"
    );
    let item_records = record["items"].as_array().unwrap();
    let d_record = item_records.iter().find(|item| item["id"] == "D").unwrap();
    let e_record = item_records.iter().find(|item| item["id"] == "E").unwrap();
    assert_eq!(d_record["minHeight"], "0px");
    assert_eq!(e_record["minHeight"], "auto");

    let e_item = actual.item_frames["E"];
    let e_payload = actual.payload_frames["E"];
    assert!(
        e_item[3] + 0.01 >= e_payload[3],
        "E retains its automatic-minimum payload floor"
    );
}

#[test]
fn simulator_root_html_fixture_matches_mask16_chrome_geometry() {
    let record: Value =
        serde_json::from_str(REPRESENTATIVE).expect("representative Chrome scene is JSON");
    let (session, initial) = run(SIMULATOR_FIXTURE);
    let cascade_error = session.ua_cascade_snapshot().error;
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "simulator fixture initializes: {:?}; cascade={cascade_error:?}",
        initial.error,
    );
    session
        .set_root_html_environment(
            1281.0,
            416.0,
            1.0,
            spinon_style::CssMediaEnvironment::MOBILE,
        )
        .expect("simulator fixture environment selects mask 16 and stretch");

    let actual = observe_current(&session, &record, "mask-16 simulator fixture");
    let grid = frame(&record["gridFrame"]);
    let content_bottom = grid[1] + grid[3] - 1.0;
    let d_payload = actual.payload_frames["D"];
    assert!(
        ((d_payload[1] + d_payload[3] - content_bottom) - 10.0).abs() <= 0.01,
        "D payload remains 10px beyond the red border"
    );
    assert!(
        actual.item_frames["E"][3] + 0.01 >= actual.payload_frames["E"][3],
        "E preserves the automatic-minimum payload floor"
    );

    let cascade = session
        .ua_cascade_snapshot()
        .completed
        .expect("the same Root HTML revision publishes computed styles");
    let grid_style = cascade.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one parsed Grid owner exists");
    let render = session
        .layout_snapshot()
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("the Root HTML revision publishes a render snapshot");
    let grid_box = render
        .boxes()
        .iter()
        .find(|render_box| render_box.node_id() == grid_style.node_id)
        .expect("the parsed Grid owner has a render box");
    let expected_red =
        spinon_render::RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(229, 72, 77));
    let border = grid_box.border();
    for side in [border.top(), border.right(), border.bottom(), border.left()] {
        assert_eq!(
            side.width_css_px(),
            1.0,
            "the Grid border is one CSS px; computed={:?}/{:?}; properties={:?}",
            grid_style.layout_border,
            grid_style.layout_border_paint,
            grid_style
                .properties
                .iter()
                .filter(|(name, _)| name.starts_with("border-") || name == &"color")
                .collect::<Vec<_>>(),
        );
        assert_eq!(side.paint(), expected_red, "the Grid border paints red");
    }
}
