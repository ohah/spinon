use super::dynamic::{css_color, payload_text, topology_index, variant_color, variant_items};
use super::*;
use std::collections::BTreeSet;

const MIXED_MINIMUM_HEIGHT_BASE: usize = 400;
const MIXED_LAYOUT_WIDTH_BASE: usize = 1280;

fn validate_track_list(track_list: &str) {
    let tracks = track_list.split_whitespace().collect::<Vec<_>>();
    assert_eq!(
        tracks.len(),
        6,
        "three-fit mixed track list has six members"
    );
    let fit_tracks = tracks
        .iter()
        .filter(|track| track.starts_with("fit-content(") && track.ends_with(')'))
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(
        fit_tracks.len(),
        3,
        "three-fit mixed track list has three fit tracks"
    );
    let caps = fit_tracks
        .iter()
        .map(|track| &track[12..track.len() - 1])
        .collect::<BTreeSet<_>>();
    assert_eq!(caps, BTreeSet::from(["0px", "10px", "20%"]));
    let tail = tracks
        .iter()
        .filter(|track| !track.starts_with("fit-content("))
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(tail, BTreeSet::from(["min-content", "max-content", "auto"]));
}

pub(super) fn source_for_three_fit_mixed_minimum_track_list(track_list: &str) -> String {
    validate_track_list(track_list);
    let mut style = format!(
        "* {{ box-sizing:border-box; }} html,body {{ margin:0; padding:0; }} \
         #grid {{ display:grid; width:42px; height:222px; border-width:1px; border-style:solid; color:#e5484d; grid-template-columns:40px; grid-template-rows:10px; grid-auto-rows:{}; grid-auto-flow:row; align-content:start; align-items:normal; justify-content:start; justify-items:normal; direction:ltr; overflow:visible; margin:0; padding:0; row-gap:0; column-gap:0; }} \
         #grid .item {{ display:block; width:auto; height:auto; min-height:0; margin:0; padding:0; overflow:visible; grid-column:1 / 2; }} \
         #grid .empty {{ min-height:0; height:10px; background-color:rgb(11,17,23); }} \
         #grid .payload {{ display:block; width:auto; height:auto; margin:0; padding:0; overflow:visible; font-family:monospace; font-size:10px; font-style:normal; font-weight:400; line-height:10px; letter-spacing:0; word-spacing:0; white-space:normal; word-break:normal; direction:ltr; }} \
         #grid .variant {{ display:none; }}",
        track_list
    );
    for variant in 0..4 {
        let ids: &[&str] = if variant < 2 {
            &["A", "B", "C", "D", "E"]
        } else {
            &["A", "B", "C", "D"]
        };
        for id in ids {
            style.push_str(&format!(
                "#grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ background-color:{}; }} \
                 #grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] .payload[data-payload=\"{id}\"] {{ background-color:{}; }}",
                css_color(variant_color(variant, id, false)),
                css_color(variant_color(variant, id, true))
            ));
        }
    }
    for mask in 0..32 {
        style.push_str(&format!(
            "@media (min-height:{}px) {{",
            MIXED_MINIMUM_HEIGHT_BASE + mask
        ));
        for variant in 0..4 {
            let ids: &[&str] = if variant < 2 {
                &["A", "B", "C", "D", "E"]
            } else {
                &["A", "B", "C", "D"]
            };
            let variant_mask = if variant < 2 { mask } else { mask & 0b1111 };
            for id in ids {
                style.push_str(&format!(
                    "#grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ min-height:0; }}"
                ));
            }
            for (bit, id) in ids.iter().enumerate() {
                if variant_mask & (1 << bit) != 0 {
                    style.push_str(&format!(
                        "#grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ min-height:auto; }}"
                    ));
                }
            }
        }
        style.push('}');
    }
    for layout_profile in 0..8 {
        let width = MIXED_LAYOUT_WIDTH_BASE + layout_profile;
        let variant = layout_profile / 2;
        let alignment = if layout_profile % 2 == 0 {
            "start"
        } else {
            "stretch"
        };
        style.push_str(&format!("@media (min-width:{width}px) {{"));
        style.push_str(&format!("#grid {{ align-content:{alignment}; }}"));
        for reset_variant in 0..4 {
            let display = if reset_variant == variant {
                "block"
            } else {
                "none"
            };
            style.push_str(&format!(
                "#grid .variant[data-variant=\"{reset_variant}\"] {{ display:{display}; }}"
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

pub(super) fn configure_three_fit_mixed_minimum_scene(
    session: &super::super::super::RuntimeSession,
    record: &Value,
) {
    use spinon_style::CssMediaEnvironment;

    let scene = &record["scene"];
    let topology = scene["topology"].as_str().unwrap();
    let variant = topology_index(topology);
    let alignment = match scene["alignContent"].as_str().unwrap() {
        "start" => 0,
        "stretch" => 1,
        other => panic!("unsupported three-fit mixed alignment {other:?}"),
    };
    let ids: &[&str] = if topology.starts_with("span2Six-") {
        &["A", "B", "C", "D", "E"]
    } else {
        &["A", "B", "C", "D"]
    };
    let mask = usize::try_from(scene["minimumMask"].as_u64().unwrap())
        .expect("three-fit minimum mask fits usize");
    let auto_items = scene["autoItems"]
        .as_array()
        .expect("scene records automatic-minimum item IDs")
        .iter()
        .map(|item| item.as_str().expect("automatic-minimum ID is a string"))
        .collect::<Vec<_>>();
    let expected_auto = ids
        .iter()
        .enumerate()
        .filter_map(|(bit, id)| (mask & (1 << bit) != 0).then_some(*id))
        .collect::<Vec<_>>();
    assert_eq!(
        auto_items, expected_auto,
        "semantic minimum mask matches IDs"
    );
    assert!(mask < (1 << ids.len()), "mask fits its selected topology");

    let width = MIXED_LAYOUT_WIDTH_BASE + variant * 2 + alignment;
    let height = MIXED_MINIMUM_HEIGHT_BASE + mask;
    session
        .set_root_html_environment(
            width as f32,
            height as f32,
            1.0,
            CssMediaEnvironment::MOBILE,
        )
        .expect("dynamic three-fit mixed environment selects a bounded scene");
}
