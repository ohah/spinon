use super::*;

#[test]
fn parsed_root_html_five_fit_mixed_minimum_auto_tail_stretch_reconciles_rounding_residual() {
    let record = records()
        .iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["topology"] == "span2Span3Six-forward"
                && scene["minimumMask"] == 12
                && scene["alignContent"] == "stretch"
                && scene["fitPositions"] == serde_json::json!([1, 2, 3, 5, 6])
                && scene["tail"] == serde_json::json!(["auto"])
        })
        .expect("oracle sample contains the stretched mixed-mask remainder scene");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) = run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    let label = format!("five-fit mixed residual scene {}", record["sampleIndex"]);
    observe_current(&session, record, &label);
}

#[test]
fn parsed_root_html_five_fit_mixed_minimum_max_content_tail_stays_intrinsic_under_stretch() {
    let record = records()
        .iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["topology"] == "span2Six-forward"
                && scene["minimumMask"] == 1
                && scene["alignContent"] == "stretch"
                && scene["fitPositions"] == serde_json::json!([1, 2, 3, 4, 5])
                && scene["tail"] == serde_json::json!(["max-content"])
        })
        .expect("oracle sample contains the intrinsic max-content-tail scene");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) = run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    observe_current(&session, record, "five-fit mixed intrinsic tail scene");
}

#[test]
fn parsed_root_html_five_fit_mixed_minimum_max_content_tail_span_rounding_matches_chrome() {
    let record = records()
        .iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["topology"] == "span2Span3Six-forward"
                && scene["minimumMask"] == 9
                && scene["alignContent"] == "start"
                && scene["fitPositions"] == serde_json::json!([1, 2, 4, 5, 6])
                && scene["tail"] == serde_json::json!(["max-content"])
        })
        .expect("oracle sample contains the max-content-tail rounding case");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) = run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    observe_current(
        &session,
        record,
        "five-fit mixed max-content-tail span scene",
    );
}

#[test]
fn parsed_root_html_five_fit_mixed_minimum_auto_tail_span_rounding_matches_chrome() {
    let record = records()
        .iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["topology"] == "span2Span3Six-forward"
                && scene["minimumMask"] == 1
                && scene["alignContent"] == "start"
                && scene["fitPositions"] == serde_json::json!([2, 3, 4, 5, 6])
                && scene["tail"] == serde_json::json!(["auto"])
        })
        .expect("oracle sample contains the auto-tail span rounding case");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) = run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    observe_current(&session, record, "five-fit mixed auto-tail span scene");
}

#[test]
fn parsed_root_html_five_fit_mixed_minimum_later_span_keeps_layout_units() {
    let record = records()
        .iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["topology"] == "span2Span3Six-forward"
                && scene["minimumMask"] == 4
                && scene["alignContent"] == "start"
                && scene["fitPositions"] == serde_json::json!([2, 3, 4, 5, 6])
                && scene["tail"] == serde_json::json!(["auto"])
        })
        .expect("oracle sample contains the later-span automatic-minimum case");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) = run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    observe_current(&session, record, "five-fit mixed later-span minimum scene");
}

#[test]
fn parsed_root_html_five_fit_mixed_minimum_multiple_auto_spans_keep_rounding_order() {
    let record = records()
        .iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["topology"] == "span2Span3Six-forward"
                && scene["minimumMask"] == 12
                && scene["alignContent"] == "start"
                && scene["fitPositions"] == serde_json::json!([2, 3, 4, 5, 6])
                && scene["fitCaps"] == serde_json::json!(["30px", "40px", "20%", "10px", "0px"])
                && scene["tail"] == serde_json::json!(["auto"])
        })
        .expect("oracle sample contains the multiple automatic-minimum span case");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) = run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    observe_current(
        &session,
        record,
        "five-fit mixed multiple automatic-minimum spans",
    );
}

#[test]
fn parsed_root_html_five_fit_mixed_minimum_b_c_spans_match_layout_units() {
    let record = records()
        .iter()
        .find(|record| {
            let scene = &record["scene"];
            scene["topology"] == "span2Span3Six-forward"
                && scene["minimumMask"] == 6
                && scene["alignContent"] == "start"
                && scene["fitPositions"] == serde_json::json!([2, 3, 4, 5, 6])
                && scene["fitCaps"] == serde_json::json!(["0px", "10px", "40px", "30px", "20%"])
                && scene["tail"] == serde_json::json!(["auto"])
        })
        .expect("oracle sample contains the B and C automatic-minimum span case");
    let track_list = record["scene"]["trackList"].as_str().unwrap();
    let (session, initial) = run(&source_for_six_fit_mixed_minimum_track_list(track_list));
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    configure_six_fit_mixed_minimum_scene(&session, record);
    observe_current(
        &session,
        record,
        "five-fit mixed B and C automatic-minimum spans",
    );
}
