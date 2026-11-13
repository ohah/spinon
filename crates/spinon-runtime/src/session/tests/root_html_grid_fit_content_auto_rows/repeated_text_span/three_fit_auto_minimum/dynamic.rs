use super::*;
use std::time::Duration;

const TOPOLOGY_HEIGHT_BASE: usize = 320;
const MIXED_MINIMUM_HEIGHT_BASE: usize = 400;
const MIXED_LAYOUT_WIDTH_BASE: usize = 1280;

pub(super) fn topology_index(topology: &str) -> usize {
    match topology {
        "span2Six-forward" => 0,
        "span2Six-reverse" => 1,
        "span2Span3Six-forward" => 2,
        "span2Span3Six-reverse" => 3,
        other => panic!("unexpected C11.4 three-fit topology {other:?}"),
    }
}

pub(super) fn variant_items(variant: usize) -> &'static [(&'static str, u64, u64)] {
    match variant {
        0 => &[
            ("A", 2, 2),
            ("B", 3, 2),
            ("C", 4, 2),
            ("D", 5, 2),
            ("E", 6, 2),
        ],
        1 => &[
            ("E", 6, 2),
            ("D", 5, 2),
            ("C", 4, 2),
            ("B", 3, 2),
            ("A", 2, 2),
        ],
        2 => &[("A", 2, 2), ("B", 3, 2), ("C", 4, 3), ("D", 5, 3)],
        3 => &[("D", 5, 3), ("C", 4, 3), ("B", 3, 2), ("A", 2, 2)],
        _ => panic!("exactly four row-span variants exist"),
    }
}

pub(super) fn payload_text(id: &str) -> &'static str {
    match id {
        "A" => "AAAA BBBB CCCC DDDD EEEE FFFF",
        "B" => "AAAAAA BBBBB CCCC DDD",
        "C" => "AAAAA BBBB CCC DDDD",
        "D" => "AAAA BBB CCCCC DDDD",
        "E" => "AAA BBBB CCCCCC",
        other => panic!("unexpected three-fit text item {other:?}"),
    }
}

pub(super) fn variant_color(variant: usize, id: &str, payload: bool) -> OpaqueCssSrgb {
    let base = if payload {
        color_for_payload(id)
    } else {
        color_for_item(id)
    };
    OpaqueCssSrgb::new(
        base.red(),
        base.green(),
        base.blue() + u8::try_from(variant).expect("four topology colors fit in one channel"),
    )
}

pub(super) fn css_color(color: OpaqueCssSrgb) -> String {
    format!("rgb({},{},{})", color.red(), color.green(), color.blue())
}

pub fn source_for_track_list(track_list: &str) -> String {
    assert!(!track_list.trim().is_empty(), "track list is not empty");
    let mut style = format!(
        "* {{ box-sizing:border-box; }} html,body {{ margin:0; padding:0; }} \
         #grid {{ display:grid; width:40px; height:220px; grid-template-columns:40px; grid-template-rows:10px; grid-auto-rows:{}; grid-auto-flow:row; align-content:start; align-items:normal; justify-content:start; justify-items:normal; direction:ltr; overflow:visible; margin:0; padding:0; row-gap:0; column-gap:0; }} \
         #grid .item {{ display:block; width:auto; height:auto; min-height:0; margin:0; padding:0; overflow:visible; grid-column:1 / 2; }} \
         #grid .empty {{ min-height:0; height:10px; background-color:rgb(11,17,23); }} \
         #grid .payload {{ display:block; width:auto; height:auto; margin:0; padding:0; overflow:visible; font-family:monospace; font-size:10px; font-style:normal; font-weight:400; line-height:10px; letter-spacing:0; word-spacing:0; white-space:normal; word-break:normal; direction:ltr; }} \
         #grid .variant {{ display:none; }} \
         @media (prefers-color-scheme: dark) {{ #grid .variant {{ min-height:auto; }} }}",
        track_list
    );
    for variant in 0..4 {
        for id in ["A", "B", "C", "D", "E"] {
            style.push_str(&format!(
                "#grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ background-color:{}; }} \
                 #grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] .payload[data-payload=\"{id}\"] {{ background-color:{}; }}",
                css_color(variant_color(variant, id, false)),
                css_color(variant_color(variant, id, true))
            ));
        }
    }
    for active_profile in 0..8 {
        let active_variant = active_profile / 2;
        let align = if active_profile % 2 == 0 {
            "start"
        } else {
            "stretch"
        };
        let viewport_height = TOPOLOGY_HEIGHT_BASE + active_profile;
        style.push_str(&format!("@media (min-height:{viewport_height}px) {{"));
        style.push_str(&format!("#grid {{ align-content:{align}; }}"));
        for variant in 0..4 {
            let display = if variant == active_variant {
                "block"
            } else {
                "none"
            };
            style.push_str(&format!(
                "#grid .variant[data-variant=\"{variant}\"] {{ display:{display}; }}"
            ));
        }
        style.push('}');
    }

    let mut children = String::from(
        "<div id=\"grid\"><div class=\"item empty\" data-item=\"empty\" style=\"grid-row:1 / 2\"></div>",
    );
    for variant in 0..4 {
        for (id, start, span) in variant_items(variant) {
            children.push_str(&format!(
                "<div class=\"item variant\" data-variant=\"{variant}\" data-item=\"{id}\" style=\"grid-row:{start} / span {span}\"><div class=\"payload\" data-payload=\"{id}\">{}</div></div>",
                payload_text(id)
            ));
        }
    }
    children.push_str("</div>");
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><style>{style}</style></head><body>{children}</body></html>"
    )
}

pub fn source_for_six_fit_mixed_minimum_track_list(track_list: &str) -> String {
    assert!(!track_list.trim().is_empty(), "track list is not empty");
    let mut style = format!(
        "* {{ box-sizing:border-box; }} html,body {{ margin:0; padding:0; }} \
         #grid {{ display:grid; width:40px; height:220px; grid-template-columns:40px; grid-template-rows:10px; grid-auto-rows:{}; grid-auto-flow:row; align-content:start; align-items:normal; justify-content:start; justify-items:normal; direction:ltr; overflow:visible; margin:0; padding:0; row-gap:0; column-gap:0; }} \
         #grid .item {{ display:block; width:auto; height:auto; min-height:0; margin:0; padding:0; overflow:visible; grid-column:1 / 2; }} \
         #grid .empty {{ min-height:0; height:10px; background-color:rgb(11,17,23); }} \
         #grid .payload {{ display:block; width:auto; height:auto; margin:0; padding:0; overflow:visible; font-family:monospace; font-size:10px; font-style:normal; font-weight:400; line-height:10px; letter-spacing:0; word-spacing:0; white-space:normal; word-break:normal; direction:ltr; }} \
         #grid .variant {{ display:none; }}",
        track_list
    );
    for variant in 0..4 {
        for id in ["A", "B", "C", "D", "E"] {
            style.push_str(&format!(
                " #grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ background-color:{}; }} \
                  #grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] .payload[data-payload=\"{id}\"] {{ background-color:{}; }}",
                css_color(variant_color(variant, id, false)),
                css_color(variant_color(variant, id, true))
            ));
        }
    }
    for mask in 0..32 {
        let height = MIXED_MINIMUM_HEIGHT_BASE + mask;
        style.push_str(&format!(" @media (min-height:{height}px) {{"));
        for variant in 0..4 {
            let ids: &[&str] = if variant < 2 {
                &["A", "B", "C", "D", "E"]
            } else {
                &["A", "B", "C", "D"]
            };
            let variant_mask = if variant < 2 { mask } else { mask & 0b1111 };
            for id in ids {
                style.push_str(&format!(
                    " #grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ min-height:0; }}"
                ));
            }
            for (bit, id) in ids.iter().enumerate() {
                if variant_mask & (1 << bit) != 0 {
                    style.push_str(&format!(
                        " #grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ min-height:auto; }}"
                    ));
                }
            }
        }
        style.push_str(" }");
    }
    for layout_profile in 0..8 {
        let width = MIXED_LAYOUT_WIDTH_BASE + layout_profile;
        let variant = layout_profile / 2;
        let alignment = if layout_profile % 2 == 0 {
            "start"
        } else {
            "stretch"
        };
        style.push_str(&format!(" @media (min-width:{width}px) {{"));
        style.push_str(&format!(" #grid {{ align-content:{alignment}; }}"));
        for reset_variant in 0..4 {
            let display = if reset_variant == variant {
                "block"
            } else {
                "none"
            };
            style.push_str(&format!(
                " #grid .variant[data-variant=\"{reset_variant}\"] {{ display:{display}; }}"
            ));
        }
        style.push_str(" }");
    }

    let mut children = String::from(
        "<div id=\"grid\"><div class=\"item empty\" data-item=\"empty\" style=\"grid-row:1 / 2\"></div>",
    );
    for variant in 0..4 {
        for (id, start, span) in variant_items(variant) {
            children.push_str(&format!(
                "<div class=\"item variant\" data-variant=\"{variant}\" data-item=\"{id}\" style=\"grid-row:{start} / span {span}\"><div class=\"payload\" data-payload=\"{id}\">{}</div></div>",
                payload_text(id)
            ));
        }
    }
    children.push_str("</div>");
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><style>{style}</style></head><body>{children}</body></html>"
    )
}

fn mixed_minimum_mask(scene: &Value, variant: usize) -> usize {
    let ids: &[&str] = if variant < 2 {
        &["A", "B", "C", "D", "E"]
    } else {
        &["A", "B", "C", "D"]
    };
    let auto_items = scene["autoItems"]
        .as_array()
        .expect("mixed minimum scene records auto item IDs");
    let mut mask = 0;
    for (bit, id) in ids.iter().enumerate() {
        if auto_items.iter().any(|item| item.as_str() == Some(id)) {
            mask |= 1 << bit;
        }
    }
    assert_eq!(
        scene["minimumMask"].as_u64(),
        Some(mask as u64),
        "semantic item IDs match the documented minimum mask"
    );
    assert!(
        auto_items
            .iter()
            .all(|item| { item.as_str().is_some_and(|id| ids.contains(&id)) }),
        "mask contains only topology item IDs"
    );
    mask
}

pub fn configure_six_fit_mixed_minimum_scene(
    session: &super::super::super::RuntimeSession,
    record: &Value,
) {
    use spinon_style::CssMediaEnvironment;

    let scene = &record["scene"];
    let variant = topology_index(scene["topology"].as_str().unwrap());
    let alignment = match scene["alignContent"].as_str().unwrap() {
        "start" => 0,
        "stretch" => 1,
        other => panic!("unsupported alignment {other:?}"),
    };
    let mask = mixed_minimum_mask(scene, variant);
    let width = MIXED_LAYOUT_WIDTH_BASE + variant * 2 + alignment;
    let height = MIXED_MINIMUM_HEIGHT_BASE + mask;
    session
        .set_root_html_environment(
            width as f32,
            height as f32,
            1.0,
            CssMediaEnvironment::MOBILE,
        )
        .expect("dynamic mixed-minimum matrix environment selects a bounded scene");
}

pub fn configure_scene(session: &super::super::super::RuntimeSession, record: &Value) {
    use spinon_style::{CssColorScheme, CssMediaEnvironment};

    let scene = &record["scene"];
    let align_index = match scene["alignContent"].as_str().unwrap() {
        "start" => 0,
        "stretch" => 1,
        other => panic!("unsupported alignment {other:?}"),
    };
    let viewport_height = TOPOLOGY_HEIGHT_BASE
        + topology_index(scene["topology"].as_str().unwrap()) * 2
        + align_index;
    let mut media = CssMediaEnvironment::MOBILE;
    media.color_scheme = if scene["minimum"] == "auto" {
        CssColorScheme::Dark
    } else {
        CssColorScheme::Light
    };
    session
        .set_root_html_environment(1280.0, viewport_height as f32, 1.0, media)
        .expect("dynamic matrix environment should select one supported scene");
}

pub fn observe_current(
    session: &super::super::super::RuntimeSession,
    record: &Value,
    label: &str,
) -> SceneResult {
    let scene = &record["scene"];
    let variant = topology_index(scene["topology"].as_str().unwrap());
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    let cascade_snapshot = session.ua_cascade_snapshot();
    let cascade_debug = (layout.state != crate::session::RuntimeLayoutState::Ready)
        .then_some(cascade_snapshot.completed.as_ref())
        .flatten()
        .map(|snapshot| {
            snapshot
                .roots
                .iter()
                .flat_map(|root| root.styles.elements.iter())
                .map(|style| {
                    (
                        style.node_id,
                        style.properties.get("display").cloned(),
                        style.properties.get("grid-auto-rows").cloned(),
                        style.properties.get("min-height").cloned(),
                    )
                })
                .collect::<Vec<_>>()
        });
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "{label}: layout={:?}; diagnostics={:?}; cascade={cascade:?}",
        layout.error,
        layout.diagnostics,
        cascade = cascade_debug
    );
    let completed = layout.completed.as_ref().expect("layout is published");
    let render = completed
        .render_snapshot
        .as_ref()
        .expect("render output is published");
    let cascade = cascade_snapshot
        .completed
        .expect("same Root HTML revision publishes computed styles");
    assert_eq!(completed.key, cascade.key, "{label}: revision ownership");
    super::super::assert_optional_track_trace(completed, record, label);

    let styles = &cascade.roots[0].styles.elements;
    let grid = styles
        .iter()
        .find(|style| style.grid_layout.is_some())
        .expect("one parsed Grid owner exists");
    assert_eq!(
        grid.grid_layout.as_ref().unwrap().auto_rows,
        ComputedCssGridAutoRows::TrackList(computed_tracks(scene)),
        "{label}: authored track order and cap assignment"
    );
    assert_eq!(
        grid.properties.get("align-content").map(String::as_str),
        scene["alignContent"].as_str(),
        "{label}: alignment"
    );
    let rows = row_values(record);
    let mut item_frames = BTreeMap::new();
    let mut payload_frames = BTreeMap::new();
    for item in record["items"].as_array().unwrap() {
        let id = item["id"].as_str().unwrap();
        let item_color = if id == "empty" {
            color_for_item(id)
        } else {
            variant_color(variant, id, false)
        };
        let actual = frame_for_paint(render, item_color);
        let expected = frame(&item["frame"]);
        assert_frame(actual, expected, &format!("{label}: item {id}"));
        assert!(item_frames.insert(id.to_owned(), actual).is_none());
        let expected_auto_minimum = if let Some(auto_items) = scene["autoItems"].as_array() {
            auto_items.iter().any(|item| item.as_str() == Some(id))
        } else {
            id != "empty" && scene["minimum"] == "auto"
        };
        let expected_minimum = if id == "empty" || !expected_auto_minimum {
            spinon_style::ComputedCssDimension::LengthPx(0.0)
        } else {
            spinon_style::ComputedCssDimension::Auto
        };
        assert_eq!(
            style_for_color(styles, item_color, label)
                .layout_dimensions
                .min_height,
            expected_minimum,
            "{label}: item {id} minimum"
        );
        if id == "empty" {
            continue;
        }
        let payload_frame = frame(&item["payloadFrame"]);
        let payload_color = variant_color(variant, id, true);
        let actual_payload = frame_for_paint(render, payload_color);
        assert_frame(
            actual_payload,
            payload_frame,
            &format!("{label}: payload {id}"),
        );
        assert!(
            payload_frames
                .insert(id.to_owned(), actual_payload)
                .is_none(),
            "{label}: one payload frame per text item"
        );
        let payload_style = style_for_color(styles, payload_color, label);
        let runtime_text = render
            .texts()
            .iter()
            .find(|text| text.node_id() == payload_style.node_id)
            .unwrap_or_else(|| panic!("{label}: payload {id} text is rendered"));
        let runtime_lines = runtime_text
            .glyphs()
            .iter()
            .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        assert_eq!(runtime_lines, item["lineRects"].as_array().unwrap().len());
        let span_height = item_span_height(record, item, &rows, label);
        assert!(
            (actual[3] - span_height).abs() <= 0.01,
            "{label}: item {id} frame matches its used row span"
        );
        let has_content_based_auto_minimum = if let Some(auto_items) = scene["autoItems"].as_array()
        {
            auto_items.iter().any(|item| item.as_str() == Some(id))
                && eligible_for_auto_minimum(record, id)
        } else {
            scene["minimum"] == "auto" && eligible_for_auto_minimum(record, id)
        };
        if has_content_based_auto_minimum {
            assert!(
                actual[3] + 0.01 >= actual_payload[3],
                "{label}: eligible item {id} preserves its payload floor"
            );
        }
    }
    if let Some(auto_items) = scene["autoItems"].as_array() {
        let diagnostics = completed
            .grid_track_sizing_diagnostics
            .as_ref()
            .expect("mixed-mask fixture exposes Grid contribution traces");
        let row_pass = diagnostics.traces[0]
            .passes
            .iter()
            .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
            .max_by_key(|pass| pass.sequence)
            .expect("mixed-mask trace has a row sizing pass");
        for placement in scene["items"]
            .as_array()
            .expect("mixed-mask scene records row placements")
        {
            let id = placement[0].as_str().unwrap();
            let span =
                u16::try_from(placement[2].as_u64().unwrap()).expect("bounded row span fits u16");
            let item_style = style_for_color(styles, variant_color(variant, id, false), label);
            let minimum_contributions = row_pass
                .contributions
                .iter()
                .filter(|contribution| {
                    contribution.item == item_style.node_id
                        && contribution.kind
                            == spinon_layout::LayoutGridTrackSizingContributionKind::Minimum
                })
                .collect::<Vec<_>>();
            assert_eq!(
                minimum_contributions.len(),
                1,
                "{label}: item {id} contributes one minimum trace"
            );
            let contribution = minimum_contributions[0];
            assert_eq!(contribution.span, span, "{label}: item {id} span trace");
            let uses_auto_minimum = auto_items.iter().any(|item| item.as_str() == Some(id));
            let eligible = eligible_for_auto_minimum(record, id);
            let source_matches = if uses_auto_minimum && eligible {
                matches!(
                    contribution.minimum_source,
                    Some(
                        spinon_layout::LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                            spans_auto_min_track: true,
                            spans_multiple_tracks: true,
                            spans_flexible_max_track: false,
                            content_based: true,
                            ..
                        }
                    )
                )
            } else if uses_auto_minimum {
                matches!(
                    contribution.minimum_source,
                    Some(
                        spinon_layout::LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                            spans_auto_min_track: false,
                            spans_multiple_tracks: true,
                            spans_flexible_max_track: false,
                            content_based: false,
                            ..
                        }
                    )
                ) && contribution.value.abs() <= 0.01
            } else {
                matches!(
                    contribution.minimum_source,
                    Some(
                        spinon_layout::LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum
                    )
                )
            };
            assert!(
                source_matches,
                "{label}: item {id} trace source {:?} matches its computed min-height mask",
                contribution.minimum_source
            );
            if uses_auto_minimum && eligible {
                let payload_height = frame(
                    &record["items"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|item| item["id"] == id)
                        .expect("automatic-minimum item has an oracle record")["payloadFrame"],
                )[3];
                assert!(
                    contribution.value + 0.01 >= payload_height,
                    "{label}: eligible item {id} minimum contribution preserves its payload floor"
                );
            }
        }
    }
    SceneResult {
        rows,
        item_frames,
        payload_frames,
    }
}
