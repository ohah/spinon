use super::Fixture;
use spinon_layout::LayoutGridTrackSizingAxis;

struct Case {
    id: &'static str,
    root_style: &'static str,
    columns: &'static [f32],
    rows: &'static [f32],
    root: (f32, f32),
}

const ITEM_STYLE: &str = "display:block;box-sizing:border-box;width:24px;height:16px;min-width:0;min-height:0;margin:0;padding:0;border:0;grid-column:1;grid-row:1;background-color:#e5484d";

const CASES: [Case; 4] = [
    Case {
        id: "cyclic-calc-column",
        root_style: "display:grid;box-sizing:border-box;width:max-content;height:max-content;grid-template-columns:calc(50% + 4px);grid-template-rows:16px;column-gap:0;row-gap:0;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        columns: &[16.0],
        rows: &[16.0],
        root: (24.0, 16.0),
    },
    Case {
        id: "cyclic-calc-row",
        root_style: "display:grid;box-sizing:border-box;width:24px;height:max-content;grid-template-columns:24px;grid-template-rows:calc(50% + 2px);column-gap:0;row-gap:0;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        columns: &[24.0],
        rows: &[10.0],
        root: (24.0, 16.0),
    },
    Case {
        id: "cyclic-calc-two-axis",
        root_style: "display:grid;box-sizing:border-box;width:max-content;height:max-content;grid-template-columns:calc(50% + 4px);grid-template-rows:calc(50% + 2px);column-gap:0;row-gap:0;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        columns: &[16.0],
        rows: &[10.0],
        root: (24.0, 16.0),
    },
    Case {
        id: "definite-calc-control",
        root_style: "display:grid;box-sizing:border-box;width:40px;height:20px;grid-template-columns:calc(50% + 4px);grid-template-rows:calc(50% + 2px);column-gap:0;row-gap:0;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        columns: &[24.0],
        rows: &[12.0],
        root: (40.0, 20.0),
    },
];

#[test]
fn cyclic_calc_tracks_resolve_after_intrinsic_container_sizing() {
    for case in CASES {
        let fixture = Fixture::new(case.root_style, &[("item", ITEM_STYLE)]);
        let output = fixture.compute();
        let root = output.layout.frames[&fixture.root.id()];
        let item = output.layout.frames[&fixture.nodes["item"].id()];
        assert_eq!(
            (root.width, root.height),
            case.root,
            "{} root frame",
            case.id
        );
        assert_eq!(
            (item.x, item.y, item.width, item.height),
            (0.0, 0.0, 24.0, 16.0),
            "{} item frame",
            case.id
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Columns,
            case.columns,
            case.id,
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Rows,
            case.rows,
            case.id,
        );
    }
}

fn assert_axis_tracks(
    output: &crate::StyleLayoutOutput,
    grid: spinon_core::NodeId,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    scene: &str,
) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid)
        .unwrap_or_else(|| panic!("{scene}: missing Grid track sizing trace"));
    let pass = trace
        .passes
        .iter()
        .rfind(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} pass"));
    let stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} stage"));
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(
        tracks.len(),
        expected.len(),
        "{scene}: {axis:?} track count"
    );
    for (index, (track, expected)) in tracks.iter().zip(expected).enumerate() {
        assert!(
            (track.base_size - expected).abs() <= 0.05,
            "{scene}: {axis:?} track {} expected {expected}, got {}",
            index + 1,
            track.base_size,
        );
    }
}
