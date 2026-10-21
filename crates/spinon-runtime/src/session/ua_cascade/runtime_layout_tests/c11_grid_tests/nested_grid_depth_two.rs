use super::*;

use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingTrace,
};

#[derive(Clone, Copy)]
struct Case {
    id: &'static str,
    depth: usize,
    constraint: &'static str,
    two_axis: bool,
    expected_width: f32,
    expected_height: f32,
    contribution: LayoutGridTrackSizingContributionKind,
}

const CASES: [Case; 10] = [
    Case {
        id: "nested-depth-one-min-content",
        depth: 1,
        constraint: "min-content",
        two_axis: false,
        expected_width: 70.0,
        expected_height: 20.0,
        contribution: LayoutGridTrackSizingContributionKind::MinContent,
    },
    Case {
        id: "nested-depth-two-min-content",
        depth: 2,
        constraint: "min-content",
        two_axis: false,
        expected_width: 70.0,
        expected_height: 20.0,
        contribution: LayoutGridTrackSizingContributionKind::MinContent,
    },
    Case {
        id: "nested-depth-one-max-content",
        depth: 1,
        constraint: "max-content",
        two_axis: false,
        expected_width: 100.0,
        expected_height: 10.0,
        contribution: LayoutGridTrackSizingContributionKind::MaxContent,
    },
    Case {
        id: "nested-depth-two-max-content",
        depth: 2,
        constraint: "max-content",
        two_axis: false,
        expected_width: 100.0,
        expected_height: 10.0,
        contribution: LayoutGridTrackSizingContributionKind::MaxContent,
    },
    Case {
        id: "nested-depth-one-min-content-two-axis",
        depth: 1,
        constraint: "min-content",
        two_axis: true,
        expected_width: 70.0,
        expected_height: 20.0,
        contribution: LayoutGridTrackSizingContributionKind::MinContent,
    },
    Case {
        id: "nested-depth-two-min-content-two-axis",
        depth: 2,
        constraint: "min-content",
        two_axis: true,
        expected_width: 70.0,
        expected_height: 20.0,
        contribution: LayoutGridTrackSizingContributionKind::MinContent,
    },
    Case {
        id: "nested-depth-one-max-content-two-axis",
        depth: 1,
        constraint: "max-content",
        two_axis: true,
        expected_width: 100.0,
        expected_height: 10.0,
        contribution: LayoutGridTrackSizingContributionKind::MaxContent,
    },
    Case {
        id: "nested-depth-two-max-content-two-axis",
        depth: 2,
        constraint: "max-content",
        two_axis: true,
        expected_width: 100.0,
        expected_height: 10.0,
        contribution: LayoutGridTrackSizingContributionKind::MaxContent,
    },
    Case {
        id: "nested-depth-three-min-content-two-axis",
        depth: 3,
        constraint: "min-content",
        two_axis: true,
        expected_width: 70.0,
        expected_height: 20.0,
        contribution: LayoutGridTrackSizingContributionKind::MinContent,
    },
    Case {
        id: "nested-depth-three-max-content-two-axis",
        depth: 3,
        constraint: "max-content",
        two_axis: true,
        expected_width: 100.0,
        expected_height: 10.0,
        contribution: LayoutGridTrackSizingContributionKind::MaxContent,
    },
];

#[derive(Clone, Copy, Debug)]
enum FixtureMutation {
    Valid,
    UnlistedDepthFour,
    WrongBranchIdentity,
    WrongIntrinsicStyle,
    ExtraRootChild,
}

#[test]
fn c114_nested_grid_depth_three_preserves_leaf_intrinsics_at_each_parent() {
    for (index, case) in CASES.iter().enumerate() {
        run_case(case, 11_800 + index as u64);
    }
}

#[test]
fn c114_nested_grid_depth_rejects_unlisted_and_malformed_topologies() {
    let case = &CASES[8];
    for (index, mutation) in [
        FixtureMutation::UnlistedDepthFour,
        FixtureMutation::WrongBranchIdentity,
        FixtureMutation::WrongIntrinsicStyle,
        FixtureMutation::ExtraRootChild,
    ]
    .into_iter()
    .enumerate()
    {
        let (snapshot, _, _, _) = fixture_with_mutation(case, 12_100 + index as u64, mutation);
        let request = request(snapshot);
        let result = compute_request_for_block_flex_grid_track_sizing(&request);
        let rejected = match result {
            Err(_) => true,
            Ok(calculation) => calculation.layout.is_err(),
        };
        assert!(rejected, "{mutation:?} topology must fail closed");
    }
}

fn run_case(case: &Case, owner: u64) {
    let (snapshot, grids, text_item, text_node) = fixture(case, owner);
    let request = request(snapshot);
    let calculation = compute_request_for_block_flex_grid_track_sizing(&request)
        .unwrap_or_else(|error| panic!("{}: calculation failed: {error:?}", case.id));
    let layout = calculation
        .layout
        .unwrap_or_else(|error| panic!("{}: layout failed: {error:?}", case.id));
    let frame = |node: HostNodeHandle| {
        layout
            .frames
            .iter()
            .find(|frame| frame.node_id == node.id().get())
            .unwrap_or_else(|| panic!("{}: missing frame for {}", case.id, node.id().get()))
    };

    for (index, grid) in grids.iter().enumerate() {
        assert_close(frame(*grid).x, 0.0, case.id, &format!("Grid {index} x"));
        assert_close(frame(*grid).y, 0.0, case.id, &format!("Grid {index} y"));
        assert_close(
            frame(*grid).width,
            case.expected_width,
            case.id,
            &format!("Grid {index} width"),
        );
        assert_close(
            frame(*grid).height,
            case.expected_height,
            case.id,
            &format!("Grid {index} height"),
        );
    }
    for (axis, expected) in [
        (LayoutGridTrackSizingAxis::Columns, case.expected_width),
        (LayoutGridTrackSizingAxis::Rows, case.expected_height),
    ] {
        for grid in &grids {
            let trace = layout
                .grid_track_sizing_diagnostics
                .as_ref()
                .and_then(|diagnostics| {
                    diagnostics
                        .traces
                        .iter()
                        .find(|trace| trace.node == grid.id())
                })
                .unwrap_or_else(|| {
                    panic!(
                        "{}: missing {axis:?} trace for Grid {}",
                        case.id,
                        grid.id().get()
                    )
                });
            assert_single_track(trace, axis, expected, case.id);
        }
    }
    assert_close(frame(text_item).x, 0.0, case.id, "leaf text x");
    assert_close(frame(text_item).y, 0.0, case.id, "leaf text y");
    assert_close(
        frame(text_item).width,
        case.expected_width,
        case.id,
        "leaf text width",
    );
    assert_close(
        frame(text_item).height,
        case.expected_height,
        case.id,
        "leaf text height",
    );

    let diagnostics = layout
        .grid_track_sizing_diagnostics
        .as_ref()
        .unwrap_or_else(|| panic!("{}: missing track sizing diagnostics", case.id));
    assert_eq!(
        diagnostics.traces.len(),
        grids.len(),
        "{}: Grid trace count",
        case.id
    );
    let leaf_inputs = diagnostics
        .item_inputs
        .iter()
        .filter(|input| input.node == text_item.id())
        .collect::<Vec<_>>();
    assert_eq!(
        leaf_inputs.len(),
        1,
        "{}: leaf text intrinsic input",
        case.id
    );
    let input = leaf_inputs[0];
    assert_eq!(input.grid_container, grids.last().unwrap().id());
    assert_eq!(input.source_order, 0);
    let LayoutGridItemIntrinsicSource::Text { sizes } = &input.source else {
        panic!("{}: leaf intrinsic source must be Ahem text", case.id);
    };
    assert_close(sizes.min_content(), 70.0, case.id, "text min-content");
    assert_close(
        sizes.min_content_height(),
        20.0,
        case.id,
        "text min-content height",
    );
    assert_close(sizes.max_content(), 100.0, case.id, "text max-content");
    assert_close(
        sizes.max_content_height(),
        10.0,
        case.id,
        "text max-content height",
    );
    assert_eq!(
        sizes.font_registry_revision().0,
        1,
        "{}: Ahem revision",
        case.id
    );
    assert_ne!(text_node, text_item);

    for (index, grid) in grids.iter().enumerate() {
        let child = grids.get(index + 1).copied().unwrap_or(text_item);
        if let Some(nested_grid) = grids.get(index + 1) {
            assert_close(
                frame(*nested_grid).x - frame(*grid).x,
                0.0,
                case.id,
                "nested Grid root-relative x",
            );
            assert_close(
                frame(*nested_grid).y - frame(*grid).y,
                0.0,
                case.id,
                "nested Grid root-relative y",
            );
        }
        let trace = diagnostics
            .traces
            .iter()
            .find(|trace| trace.node == grid.id())
            .unwrap();
        for (axis, expected) in [
            (LayoutGridTrackSizingAxis::Columns, case.expected_width),
            (LayoutGridTrackSizingAxis::Rows, case.expected_height),
        ] {
            let pass = trace.passes.iter().rfind(|pass| pass.axis == axis).unwrap();
            assert!(
                pass.contributions.iter().any(|contribution| {
                    contribution.item == child.id()
                        && contribution.kind == case.contribution
                        && contribution.span == 1
                        && (contribution.value - expected).abs() <= 1.0 / 64.0
                }),
                "{}: missing {:?} {axis:?} contribution from Grid {} to {}: {:#?}",
                case.id,
                case.contribution,
                child.id().get(),
                grid.id().get(),
                pass.contributions
            );
        }
    }
}

fn assert_single_track(
    trace: &LayoutGridTrackSizingTrace,
    axis: LayoutGridTrackSizingAxis,
    expected: f32,
    case: &str,
) {
    let passes = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == axis)
        .collect::<Vec<_>>();
    assert_eq!(
        passes.len(),
        1,
        "{case}: Grid {} {axis:?} pass count",
        trace.node.get()
    );
    let tracks = passes[0]
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: Grid {} missing final stage", trace.node.get()))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(
        tracks.len(),
        1,
        "{case}: Grid {} {axis:?} track count",
        trace.node.get()
    );
    assert_close(tracks[0], expected, case, &format!("{axis:?} final track"));
}

fn assert_close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{case}: {property} expected {expected}, got {actual}"
    );
}

fn fixture(
    case: &Case,
    owner: u64,
) -> (
    Arc<HostDocumentSnapshot>,
    Vec<HostNodeHandle>,
    HostNodeHandle,
    HostNodeHandle,
) {
    fixture_with_mutation(case, owner, FixtureMutation::Valid)
}

fn fixture_with_mutation(
    case: &Case,
    owner: u64,
    mutation: FixtureMutation,
) -> (
    Arc<HostDocumentSnapshot>,
    Vec<HostNodeHandle>,
    HostNodeHandle,
    HostNodeHandle,
) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new().unwrap();
    let grids = (0..=case.depth)
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let text_item = document.reserve_node_handle().unwrap();
    let text_node = document.reserve_node_handle().unwrap();
    let scene_id = if matches!(mutation, FixtureMutation::UnlistedDepthFour) {
        "nested-depth-four-min-content-two-axis"
    } else {
        case.id
    };
    let grid_ids = (0..=case.depth)
        .map(|index| {
            let level = if matches!(mutation, FixtureMutation::WrongBranchIdentity) && index == 2 {
                "unexpected"
            } else {
                match (case.depth, index) {
                    (_, 0) => "root",
                    (1, 1) | (2, 2) | (3, 3) => "inner",
                    (2, 1) | (3, 1) => "middle",
                    (3, 2) => "branch",
                    _ => panic!(
                        "unlisted nested Grid topology: depth {} index {index}",
                        case.depth
                    ),
                }
            };
            format!("c114-{scene_id}-{level}")
        })
        .collect::<Vec<_>>();
    let block_constraint = if case.two_axis {
        case.constraint
    } else {
        "auto"
    };
    let grid_style = format!(
        "box-sizing:border-box;display:grid;width:{};height:{};margin:0;padding:0;border:0;column-gap:0;row-gap:0;grid-template-columns:{};grid-template-rows:{};align-items:start;background-color:#e9f1ff",
        case.constraint, block_constraint, case.constraint, block_constraint
    );
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    for (index, (grid, id)) in grids.iter().zip(&grid_ids).enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: *grid,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        let style = if matches!(mutation, FixtureMutation::WrongIntrinsicStyle) && index == 2 {
            if case.constraint == "min-content" {
                grid_style.replace("min-content", "max-content")
            } else {
                grid_style.replace("max-content", "min-content")
            }
        } else {
            grid_style.clone()
        };
        for (name, value) in [
            ("id", id.clone()),
            ("data-node", String::new()),
            ("style", style),
        ] {
            batch.push(DocumentOperation::SetAttribute {
                node: *grid,
                name: AttributeName::new(None, name).unwrap(),
                value: value.into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent: if index == 0 {
                HostParent::Root
            } else {
                HostParent::Node(grids[index - 1])
            },
            node: *grid,
            before: None,
        });
    }
    let text_style = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#ffffff";
    batch.push(DocumentOperation::CreateElement {
        node: text_item,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    for (name, value) in [
        ("id", format!("c114-{scene_id}-text")),
        ("data-node", String::new()),
        ("style", text_style.to_owned()),
    ] {
        batch.push(DocumentOperation::SetAttribute {
            node: text_item,
            name: AttributeName::new(None, name).unwrap(),
            value: value.into(),
        });
    }
    batch.push(DocumentOperation::CreateText {
        node: text_node,
        data: "XX XXXXXXX".into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(*grids.last().unwrap()),
        node: text_item,
        before: None,
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(text_item),
        node: text_node,
        before: None,
    });
    if matches!(mutation, FixtureMutation::ExtraRootChild) {
        let extra_child = document.reserve_node_handle().unwrap();
        batch.push(DocumentOperation::CreateElement {
            node: extra_child,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: extra_child,
            name: AttributeName::new(None, "id").unwrap(),
            value: format!("c114-{scene_id}-unexpected").into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: extra_child,
            name: AttributeName::new(None, "data-node").unwrap(),
            value: String::new().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(grids[0]),
            node: extra_child,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), grids, text_item, text_node)
}

fn request(snapshot: Arc<HostDocumentSnapshot>) -> WorkRequest {
    WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 400.0,
            height_css_px: 200.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    }
}
