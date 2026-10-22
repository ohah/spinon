use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, OwnerId};
use spinon_layout::LayoutGridTrackSizingAxis;

#[path = "cyclic_minmax_replaced_image_span/assertions.rs"]
mod assertions;
use self::assertions::{
    assert_axis_tracks, assert_definite_control_stages, assert_frame, assert_intrinsic_span_stages,
    assert_replaced_contributions, assert_replaced_intrinsic_inputs,
};
use super::Fixture;

const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const TRACK: &str = "minmax(calc(10% + 2px),max-content)";

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    height: f32,
    column: &'static str,
    row: &'static str,
    span: u16,
    inline: f32,
}

const A: Item = Item {
    id: "image-a",
    height: 8.0,
    column: "1 / 3",
    row: "1 / 3",
    span: 2,
    inline: 12.0,
};
const B: Item = Item {
    id: "image-b",
    height: 12.0,
    column: "2 / 4",
    row: "1 / 3",
    span: 2,
    inline: 18.0,
};
const C: Item = Item {
    id: "image-c",
    height: 16.0,
    column: "1 / 4",
    row: "1 / 3",
    span: 3,
    inline: 24.0,
};
const FORWARD: [Item; 3] = [A, B, C];
const REVERSE: [Item; 3] = [C, B, A];

struct Case {
    id: &'static str,
    root_size: &'static str,
    order: &'static [Item; 3],
    columns: [f32; 3],
    rows: [f32; 2],
    root: [f32; 2],
    frames: [[f32; 4]; 3],
}

const INTRINSIC_FRAMES: [[f32; 4]; 3] = [
    [2.0, 2.0, 12.0, 8.0],
    [9.0, 2.0, 18.0, 12.0],
    [2.0, 2.0, 24.0, 16.0],
];
const DEFINITE_FRAMES: [[f32; 4]; 3] = [
    [2.0, 2.0, 12.0, 8.0],
    [14.0, 2.0, 18.0, 12.0],
    [2.0, 2.0, 24.0, 16.0],
];
const CASES: [Case; 4] = [
    Case {
        id: "intrinsic-forward",
        root_size: "min-content",
        order: &FORWARD,
        columns: [5.0, 8.0, 8.0],
        rows: [7.0, 7.0],
        root: [29.0, 20.0],
        frames: INTRINSIC_FRAMES,
    },
    Case {
        id: "intrinsic-reverse",
        root_size: "min-content",
        order: &REVERSE,
        columns: [5.0, 8.0, 8.0],
        rows: [7.0, 7.0],
        root: [29.0, 20.0],
        frames: INTRINSIC_FRAMES,
    },
    Case {
        id: "definite-forward",
        root_size: "80px",
        order: &FORWARD,
        columns: [10.0, 10.0, 10.0],
        rows: [10.0, 10.0],
        root: [84.0, 84.0],
        frames: DEFINITE_FRAMES,
    },
    Case {
        id: "definite-reverse",
        root_size: "80px",
        order: &REVERSE,
        columns: [10.0, 10.0, 10.0],
        rows: [10.0, 10.0],
        root: [84.0, 84.0],
        frames: DEFINITE_FRAMES,
    },
];

#[test]
fn cyclic_minmax_replaced_image_spans_match_all_four_fixed_scenes() {
    for case in CASES {
        let fixture = fixture(&case);
        let output = if case.root_size == "min-content" {
            fixture.compute_with_image_intrinsic_sizes_and_two_axis_grid(&[A.id, B.id, C.id])
        } else {
            fixture.compute_with_image_intrinsic_sizes(&[A.id, B.id, C.id])
        }
        .unwrap_or_else(|error| panic!("{}: {error}", case.id));
        let root = output.layout.frames[&fixture.root.id()];
        assert_frame(
            [root.width, root.height],
            case.root,
            case.id,
            "root border box",
        );

        for item in [A, B, C] {
            let frame = output.layout.frames[&fixture.nodes[item.id].id()];
            let expected = case.frames[match item.id {
                "image-a" => 0,
                "image-b" => 1,
                "image-c" => 2,
                _ => unreachable!(),
            }];
            assert_frame(
                [frame.x, frame.y, frame.width, frame.height],
                expected,
                case.id,
                item.id,
            );
        }

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .unwrap_or_else(|| panic!("{}: Grid track-sizing trace missing", case.id));
        assert_axis_tracks(
            trace,
            LayoutGridTrackSizingAxis::Columns,
            &case.columns,
            case.id,
        );
        assert_axis_tracks(trace, LayoutGridTrackSizingAxis::Rows, &case.rows, case.id);
        assert_replaced_intrinsic_inputs(&output, &fixture, &case);
        assert_replaced_contributions(&output, &fixture, trace, case.id);
        if case.root_size == "min-content" {
            assert_intrinsic_span_stages(trace, &fixture, &case);
        } else {
            assert_definite_control_stages(trace, &case);
        }
    }
}

fn fixture(case: &Case) -> Fixture {
    let root_style = format!(
        "box-sizing:content-box;display:grid;width:{};height:{};margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:2px;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(3,{});grid-template-rows:repeat(2,{});background-color:#101820",
        case.root_size, case.root_size, TRACK, TRACK,
    );
    let item_styles = case
        .order
        .iter()
        .map(|item| {
            (
                item.id,
                format!(
                    "box-sizing:content-box;display:block;width:auto;height:{}px;margin:0;padding:0;border:0;grid-column:{};grid-row:{};justify-self:auto;align-self:auto;background-color:#e5484d",
                    item.height, item.column, item.row,
                ),
            )
        })
        .collect::<Vec<_>>();
    let item_style_refs = item_styles
        .iter()
        .map(|(id, style)| (*id, style.as_str()))
        .collect::<Vec<_>>();
    let mut fixture = Fixture::new_with_item_tag(&root_style, &item_style_refs, "img");
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_234).unwrap(),
        fixture.document.document_revision(),
    );
    for item in case.order {
        batch.push(DocumentOperation::SetAttribute {
            node: fixture.nodes[item.id],
            name: AttributeName::new(None, "src").unwrap(),
            value: IMAGE_URL.to_owned().into(),
        });
    }
    fixture.document.commit(batch).unwrap();
    fixture
}
