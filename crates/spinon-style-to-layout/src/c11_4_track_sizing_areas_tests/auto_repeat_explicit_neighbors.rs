use spinon_layout::LayoutGridTrackSizingAxis;

use super::Fixture;

struct Case {
    id: &'static str,
    root_style: &'static str,
    items: &'static [(&'static str, &'static str)],
    axis: LayoutGridTrackSizingAxis,
    tracks: &'static [f32],
    collapsed: &'static [bool],
    root_size: (f32, f32),
    item_positions: &'static [(&'static str, f32, f32)],
}

const FILL_BOTH_ITEMS: &[(&str, &str)] = &[
    ("prefix", "width:12px;height:12px;grid-column:1;grid-row:1"),
    ("repeat", "width:12px;height:12px;grid-column:2;grid-row:1"),
    ("suffix", "width:12px;height:12px;grid-column:5;grid-row:1"),
];
const FIT_ONE_ITEMS: &[(&str, &str)] = &[
    ("repeat", "width:12px;height:12px;grid-column:2;grid-row:1"),
    ("suffix", "width:12px;height:12px;grid-column:5;grid-row:1"),
];
const FIT_SPAN_ITEMS: &[(&str, &str)] = &[
    (
        "span",
        "width:12px;height:12px;grid-column:2 / span 2;grid-row:1",
    ),
    ("suffix", "width:12px;height:12px;grid-column:5;grid-row:1"),
];
const PREFIX_ONLY_ITEMS: &[(&str, &str)] = &[
    ("prefix", "width:12px;height:12px;grid-column:1;grid-row:1"),
    ("repeat", "width:12px;height:12px;grid-column:2;grid-row:1"),
];
const SUFFIX_ONLY_ITEMS: &[(&str, &str)] = &[
    ("repeat", "width:12px;height:12px;grid-column:1;grid-row:1"),
    ("suffix", "width:12px;height:12px;grid-column:4;grid-row:1"),
];
const PATTERN_ITEMS: &[(&str, &str)] = &[
    ("prefix", "width:12px;height:12px;grid-column:1;grid-row:1"),
    (
        "repeat-one",
        "width:12px;height:12px;grid-column:2;grid-row:1",
    ),
    (
        "repeat-two",
        "width:12px;height:12px;grid-column:4;grid-row:1",
    ),
    ("suffix", "width:12px;height:12px;grid-column:6;grid-row:1"),
];
const INDEFINITE_ITEMS: &[(&str, &str)] = &[
    ("prefix", "width:12px;height:12px;grid-column:1;grid-row:1"),
    ("repeat", "width:12px;height:12px;grid-column:2;grid-row:1"),
    ("suffix", "width:12px;height:12px;grid-column:3;grid-row:1"),
];
const OVERFLOW_ITEMS: &[(&str, &str)] = &[
    ("repeat", "width:12px;height:12px;grid-column:2;grid-row:1"),
    ("suffix", "width:12px;height:12px;grid-column:3;grid-row:1"),
];
const ROW_ITEMS: &[(&str, &str)] = &[
    ("prefix", "width:12px;height:12px;grid-column:1;grid-row:1"),
    ("repeat", "width:12px;height:12px;grid-column:1;grid-row:2"),
    ("suffix", "width:12px;height:12px;grid-column:1;grid-row:5"),
];
const ROW_FIT_ITEMS: &[(&str, &str)] = &[
    ("repeat", "width:12px;height:12px;grid-column:1;grid-row:2"),
    ("suffix", "width:12px;height:12px;grid-column:1;grid-row:5"),
];
const NOT_COLLAPSED_5: &[bool] = &[false, false, false, false, false];
const FIT_ONE_COLLAPSED: &[bool] = &[false, false, true, true, false];
const FIT_SPAN_COLLAPSED: &[bool] = &[false, false, false, true, false];
const FIT_EMPTY_COLLAPSED: &[bool] = &[false, true, true, true, false];

#[test]
fn chrome155_auto_repeat_explicit_neighbor_matrix_preserves_tracks_and_frames() {
    let cases = [
        Case {
            id: "columns-auto-fill-both",
            root_style: "display:grid;box-sizing:border-box;width:200px;height:60px;grid-template-columns:20px repeat(auto-fill,minmax(40px,1fr)) 10px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: FILL_BOTH_ITEMS,
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[20.0, 46.0, 46.0, 46.0, 10.0],
            collapsed: NOT_COLLAPSED_5,
            root_size: (200.0, 60.0),
            item_positions: &[
                ("prefix", 0.0, 0.0),
                ("repeat", 28.0, 0.0),
                ("suffix", 190.0, 0.0),
            ],
        },
        Case {
            id: "columns-auto-fit-one",
            root_style: "display:grid;box-sizing:border-box;width:200px;height:60px;grid-template-columns:20px repeat(auto-fit,minmax(40px,1fr)) 10px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: FIT_ONE_ITEMS,
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[20.0, 154.0, 0.0, 0.0, 10.0],
            collapsed: FIT_ONE_COLLAPSED,
            root_size: (200.0, 60.0),
            item_positions: &[("repeat", 28.0, 0.0), ("suffix", 190.0, 0.0)],
        },
        Case {
            id: "columns-auto-fit-span-two",
            root_style: "display:grid;box-sizing:border-box;width:200px;height:60px;grid-template-columns:20px repeat(auto-fit,minmax(40px,1fr)) 10px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: FIT_SPAN_ITEMS,
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[20.0, 73.0, 73.0, 0.0, 10.0],
            collapsed: FIT_SPAN_COLLAPSED,
            root_size: (200.0, 60.0),
            item_positions: &[("span", 28.0, 0.0), ("suffix", 190.0, 0.0)],
        },
        Case {
            id: "columns-auto-fit-empty",
            root_style: "display:grid;box-sizing:border-box;width:200px;height:60px;grid-template-columns:20px repeat(auto-fit,minmax(40px,1fr)) 10px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: &[],
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[20.0, 0.0, 0.0, 0.0, 10.0],
            collapsed: FIT_EMPTY_COLLAPSED,
            root_size: (200.0, 60.0),
            item_positions: &[],
        },
        Case {
            id: "columns-auto-fill-prefix-only",
            root_style: "display:grid;box-sizing:border-box;width:200px;height:60px;grid-template-columns:20px repeat(auto-fill,minmax(40px,1fr));grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: PREFIX_ONLY_ITEMS,
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[20.0, 52.0, 52.0, 52.0],
            collapsed: &[false, false, false, false],
            root_size: (200.0, 60.0),
            item_positions: &[("prefix", 0.0, 0.0), ("repeat", 28.0, 0.0)],
        },
        Case {
            id: "columns-auto-fill-suffix-only",
            root_style: "display:grid;box-sizing:border-box;width:200px;height:60px;grid-template-columns:repeat(auto-fill,minmax(40px,1fr)) 10px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: SUFFIX_ONLY_ITEMS,
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[55.328125, 55.328125, 55.328125, 10.0],
            collapsed: &[false, false, false, false],
            root_size: (200.0, 60.0),
            item_positions: &[("repeat", 0.0, 0.0), ("suffix", 190.0, 0.0)],
        },
        Case {
            id: "columns-auto-fill-pattern",
            root_style: "display:grid;box-sizing:border-box;width:200px;height:60px;grid-template-columns:20px repeat(auto-fill,minmax(30px,1fr) 20px) 10px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: PATTERN_ITEMS,
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[20.0, 45.0, 20.0, 45.0, 20.0, 10.0],
            collapsed: &[false, false, false, false, false, false],
            root_size: (200.0, 60.0),
            item_positions: &[
                ("prefix", 0.0, 0.0),
                ("repeat-one", 28.0, 0.0),
                ("repeat-two", 109.0, 0.0),
                ("suffix", 190.0, 0.0),
            ],
        },
        Case {
            id: "columns-auto-fill-indefinite",
            root_style: "display:grid;box-sizing:border-box;width:max-content;height:60px;grid-template-columns:20px repeat(auto-fill,40px) 10px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: INDEFINITE_ITEMS,
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[20.0, 40.0, 10.0],
            collapsed: &[false, false, false],
            root_size: (86.0, 60.0),
            item_positions: &[
                ("prefix", 0.0, 0.0),
                ("repeat", 28.0, 0.0),
                ("suffix", 76.0, 0.0),
            ],
        },
        Case {
            id: "columns-auto-fill-max-size",
            root_style: "display:grid;box-sizing:border-box;width:max-content;max-width:200px;height:60px;grid-template-columns:20px repeat(auto-fill,minmax(40px,1fr)) 10px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: &[],
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[20.0, 40.0, 40.0, 40.0, 10.0],
            collapsed: &[false, false, false, false, false],
            root_size: (182.0, 60.0),
            item_positions: &[],
        },
        Case {
            id: "columns-auto-fill-min-size",
            root_style: "display:grid;box-sizing:border-box;width:max-content;min-width:200px;height:60px;grid-template-columns:20px repeat(auto-fill,minmax(40px,1fr)) 10px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: &[],
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[20.0, 40.0, 40.0, 40.0, 40.0, 10.0],
            collapsed: &[false, false, false, false, false, false],
            root_size: (230.0, 60.0),
            item_positions: &[],
        },
        Case {
            id: "columns-auto-repeat-overflow",
            root_style: "display:grid;box-sizing:border-box;width:40px;height:60px;grid-template-columns:30px repeat(auto-fill,40px) 20px;grid-template-rows:40px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: OVERFLOW_ITEMS,
            axis: LayoutGridTrackSizingAxis::Columns,
            tracks: &[30.0, 40.0, 20.0],
            collapsed: &[false, false, false],
            root_size: (40.0, 60.0),
            item_positions: &[("repeat", 38.0, 0.0), ("suffix", 86.0, 0.0)],
        },
        Case {
            id: "rows-auto-fill-both",
            root_style: "display:grid;box-sizing:border-box;width:50px;height:200px;grid-template-columns:40px;grid-template-rows:20px repeat(auto-fill,minmax(40px,1fr)) 10px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: ROW_ITEMS,
            axis: LayoutGridTrackSizingAxis::Rows,
            tracks: &[20.0, 46.0, 46.0, 46.0, 10.0],
            collapsed: NOT_COLLAPSED_5,
            root_size: (50.0, 200.0),
            item_positions: &[
                ("prefix", 0.0, 0.0),
                ("repeat", 0.0, 28.0),
                ("suffix", 0.0, 190.0),
            ],
        },
        Case {
            id: "rows-auto-fit-one",
            root_style: "display:grid;box-sizing:border-box;width:50px;height:200px;grid-template-columns:40px;grid-template-rows:20px repeat(auto-fit,minmax(40px,1fr)) 10px;column-gap:8px;row-gap:8px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
            items: ROW_FIT_ITEMS,
            axis: LayoutGridTrackSizingAxis::Rows,
            tracks: &[20.0, 154.0, 0.0, 0.0, 10.0],
            collapsed: FIT_ONE_COLLAPSED,
            root_size: (50.0, 200.0),
            item_positions: &[("repeat", 0.0, 28.0), ("suffix", 0.0, 190.0)],
        },
    ];

    for case in cases {
        let fixture = Fixture::new(case.root_style, case.items);
        let output = fixture.compute();
        let root = output.layout.frames[&fixture.root.id()];
        assert_close(root.width, case.root_size.0, case.id, "root.width");
        assert_close(root.height, case.root_size.1, case.id, "root.height");

        for (id, expected_x, expected_y) in case.item_positions {
            let frame = output.layout.frames[&fixture.nodes[*id].id()];
            assert_close(frame.x, *expected_x, case.id, &format!("{id}.x"));
            assert_close(frame.y, *expected_y, case.id, &format!("{id}.y"));
            assert_close(frame.width, 12.0, case.id, &format!("{id}.width"));
            assert_close(frame.height, 12.0, case.id, &format!("{id}.height"));
        }

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .unwrap_or_else(|| panic!("{}: missing track trace", case.id));
        let pass = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == case.axis)
            .last()
            .unwrap_or_else(|| panic!("{}: missing {:?} pass", case.id, case.axis));
        let stage = pass
            .stages
            .last()
            .unwrap_or_else(|| panic!("{}: missing final stage", case.id));
        let tracks = stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .collect::<Vec<_>>();
        assert_eq!(tracks.len(), case.tracks.len(), "{}: track count", case.id);
        assert_eq!(
            tracks.len(),
            case.collapsed.len(),
            "{}: collapse count",
            case.id
        );
        for (index, ((track, expected), collapsed)) in tracks
            .iter()
            .zip(case.tracks)
            .zip(case.collapsed)
            .enumerate()
        {
            assert!(
                (track.base_size - *expected).abs() <= 1.0 / 64.0,
                "{}: {:?}[{index}] expected {expected}, got {}",
                case.id,
                case.axis,
                track.base_size
            );
            assert_eq!(track.is_collapsed, *collapsed, "{}[{index}]", case.id);
        }
    }
}

fn assert_close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{case}: {property} expected {expected}, got {actual}"
    );
}
