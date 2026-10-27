use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::{
    LayoutGridTrackSizingAxis, LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingTrace,
    LayoutSourceRevision,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const ROOT_ID: &str = "c114-grid-change-intrinsic-auto-repeat-grid";
const ITEM_ID: &str = "c114-grid-change-intrinsic-auto-repeat-item";
const PROBE_IDS: [&str; 4] = [
    "c114-grid-change-intrinsic-auto-repeat-a1",
    "c114-grid-change-intrinsic-auto-repeat-a2",
    "c114-grid-change-intrinsic-auto-repeat-a3",
    "c114-grid-change-intrinsic-auto-repeat-a4",
];
const PROBE_COLUMNS: [&str; 4] = ["1 / 2", "2 / 3", "3 / 4", "4 / 5"];
const TOLERANCE: f32 = 1.0 / 64.0;

#[derive(Clone, Copy)]
struct State {
    id: &'static str,
    root_width: &'static str,
    item_width: f32,
    repeat_count: u16,
    content_width: f32,
    columns: &'static [f32],
    probes: [[f32; 4]; 4],
}

const STATES: [State; 8] = [
    State {
        id: "max-content-100",
        root_width: "max-content",
        item_width: 100.0,
        repeat_count: 3,
        content_width: 100.0,
        columns: &[20.0, 20.0, 20.0, 40.0],
        probes: [
            [0.0, 0.0, 20.0, 5.0],
            [20.0, 0.0, 20.0, 5.0],
            [40.0, 0.0, 20.0, 5.0],
            [60.0, 0.0, 40.0, 5.0],
        ],
    },
    State {
        id: "max-content-80",
        root_width: "max-content",
        item_width: 80.0,
        repeat_count: 2,
        content_width: 80.0,
        columns: &[20.0, 20.0, 40.0],
        probes: [
            [0.0, 0.0, 20.0, 5.0],
            [20.0, 0.0, 20.0, 5.0],
            [40.0, 0.0, 40.0, 5.0],
            [80.0, 0.0, 0.0, 5.0],
        ],
    },
    State {
        id: "max-content-15",
        root_width: "max-content",
        item_width: 15.0,
        repeat_count: 1,
        content_width: 60.0,
        columns: &[20.0, 40.0],
        probes: [
            [0.0, 0.0, 20.0, 5.0],
            [20.0, 0.0, 40.0, 5.0],
            [60.0, 0.0, 0.0, 5.0],
            [0.0, 0.0, 60.0, 5.0],
        ],
    },
    State {
        id: "max-content-120",
        root_width: "max-content",
        item_width: 120.0,
        repeat_count: 4,
        content_width: 120.0,
        columns: &[20.0, 20.0, 20.0, 20.0, 40.0],
        probes: [
            [0.0, 0.0, 20.0, 5.0],
            [20.0, 0.0, 20.0, 5.0],
            [40.0, 0.0, 20.0, 5.0],
            [60.0, 0.0, 20.0, 5.0],
        ],
    },
    State {
        id: "min-content-100",
        root_width: "min-content",
        item_width: 100.0,
        repeat_count: 3,
        content_width: 100.0,
        columns: &[20.0, 20.0, 20.0, 40.0],
        probes: [
            [0.0, 0.0, 20.0, 5.0],
            [20.0, 0.0, 20.0, 5.0],
            [40.0, 0.0, 20.0, 5.0],
            [60.0, 0.0, 40.0, 5.0],
        ],
    },
    State {
        id: "min-content-80",
        root_width: "min-content",
        item_width: 80.0,
        repeat_count: 2,
        content_width: 80.0,
        columns: &[20.0, 20.0, 40.0],
        probes: [
            [0.0, 0.0, 20.0, 5.0],
            [20.0, 0.0, 20.0, 5.0],
            [40.0, 0.0, 40.0, 5.0],
            [80.0, 0.0, 0.0, 5.0],
        ],
    },
    State {
        id: "min-content-15",
        root_width: "min-content",
        item_width: 15.0,
        repeat_count: 1,
        content_width: 30.0,
        columns: &[20.0, 10.0],
        probes: [
            [0.0, 0.0, 20.0, 5.0],
            [20.0, 0.0, 10.0, 5.0],
            [30.0, 0.0, 0.0, 5.0],
            [0.0, 0.0, 30.0, 5.0],
        ],
    },
    State {
        id: "min-content-120",
        root_width: "min-content",
        item_width: 120.0,
        repeat_count: 4,
        content_width: 120.0,
        columns: &[20.0, 20.0, 20.0, 20.0, 40.0],
        probes: [
            [0.0, 0.0, 20.0, 5.0],
            [20.0, 0.0, 20.0, 5.0],
            [40.0, 0.0, 20.0, 5.0],
            [60.0, 0.0, 20.0, 5.0],
        ],
    },
];

struct Document {
    host: HostDocument,
    owner: OwnerId,
    root: HostNodeHandle,
    item: HostNodeHandle,
    probes: [HostNodeHandle; 4],
}

impl Document {
    fn new(first: State) -> Self {
        let mut host = HostDocument::new().unwrap();
        let owner = OwnerId::new(11_428).unwrap();
        let root = host.reserve_node_handle().unwrap();
        let item = host.reserve_node_handle().unwrap();
        let probes = std::array::from_fn(|_| host.reserve_node_handle().unwrap());
        let mut batch = DocumentChangeBatch::new(owner, host.document_revision());
        create_element(
            &mut batch,
            root,
            HostParent::Root,
            ROOT_ID,
            &root_style(first),
        );
        create_element(
            &mut batch,
            item,
            HostParent::Node(root),
            ITEM_ID,
            &item_style(first.item_width),
        );
        for (index, probe) in probes.iter().copied().enumerate() {
            create_element(
                &mut batch,
                probe,
                HostParent::Node(root),
                PROBE_IDS[index],
                &probe_style(PROBE_COLUMNS[index]),
            );
        }
        host.commit(batch).unwrap();
        Self {
            host,
            owner,
            root,
            item,
            probes,
        }
    }

    fn snapshot(&self) -> Arc<HostDocumentSnapshot> {
        Arc::new(self.host.snapshot())
    }

    fn apply(&mut self, state: State) {
        let mut batch = DocumentChangeBatch::new(self.owner, self.host.document_revision());
        set_style(&mut batch, self.root, &root_style(state));
        set_style(&mut batch, self.item, &item_style(state.item_width));
        self.host.commit(batch).unwrap();
    }
}

fn create_element(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    parent: HostParent,
    id: &str,
    style: &str,
) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    for (name, value) in [("id", id.to_owned()), ("style", style.to_owned())] {
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, name).unwrap(),
            value: value.into(),
        });
    }
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node,
        before: None,
    });
}

fn set_style(batch: &mut DocumentChangeBatch, node: HostNodeHandle, style: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.into(),
    });
}

fn root_style(state: State) -> String {
    format!(
        "display:grid;position:relative;width:{};min-width:30px;border:2px solid black;padding-top:10px;margin-bottom:10px;grid-auto-columns:20px;grid-template-columns:repeat(auto-fill,20px) minmax(min-content,40px);grid-template-rows:none",
        state.root_width
    )
}

fn item_style(width: f32) -> String {
    format!("grid-column:1 / -1;width:{width}px;height:30px")
}

fn probe_style(columns: &str) -> String {
    format!("height:5px;position:absolute;width:100%;grid-column:{columns}")
}

#[test]
fn c114_auto_repeat_tracks_recompute_after_intrinsic_item_changes_in_one_document() {
    let mut document = Document::new(STATES[0]);
    let mut previous_snapshot: Option<Arc<HostDocumentSnapshot>> = None;
    let mut previous_styles = None;
    let mut style_revision = StyleRevision::INITIAL;
    let mut previous_source = None;

    for (index, state) in STATES.iter().copied().enumerate() {
        if index > 0 {
            document.apply(state);
            style_revision = style_revision.checked_next().unwrap();
        }
        let snapshot = document.snapshot();
        if let Some(previous) = previous_snapshot.as_ref() {
            assert_eq!(
                snapshot.generation(),
                previous.generation(),
                "{} generation changed",
                state.id
            );
            assert!(
                snapshot.document_revision() > previous.document_revision(),
                "{} must use a newer document revision",
                state.id
            );
            assert!(
                snapshot.render_tree_revision() > previous.render_tree_revision(),
                "style attribute mutations must advance render-tree revision: {}",
                state.id,
            );
        }
        let source = LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        };
        if let Some(previous) = previous_source {
            assert_ne!(
                source, previous,
                "{} must not reuse source revision",
                state.id
            );
        }
        previous_source = Some(source);

        let invalidation = previous_snapshot.as_deref().map(|previous| {
            super::super::invalidation::classify_style_invalidation(Some(previous), &snapshot)
        });
        if let Some(invalidation) = invalidation.as_ref() {
            assert!(
                !invalidation.dirty_roots().is_empty(),
                "{} style mutation must invalidate a connected subtree",
                state.id
            );
        }
        let key =
            key_for_with_style_revision(&snapshot, EnvironmentRevision::INITIAL, style_revision);
        let request = WorkRequest {
            key,
            snapshot: Arc::clone(&snapshot),
            viewport: CssViewport {
                width_css_px: 1280.0,
                height_css_px: 818.0,
                device_scale_factor: 1.0,
                environment_revision: EnvironmentRevision::INITIAL,
                media_environment: CssMediaEnvironment::MOBILE,
            },
            previous_snapshot: previous_snapshot.clone(),
            force_full: index == 0,
            invalidation,
            previous_styles: previous_styles.clone(),
        };
        let calculation = compute_request_for_block_flex_grid_track_sizing(&request)
            .unwrap_or_else(|error| panic!("{} runtime cascade failed: {error}", state.id));
        let completed = calculation
            .layout
            .as_ref()
            .unwrap_or_else(|error| panic!("{} layout failed: {error:?}", state.id));
        assert_eq!(completed.key, key, "{} output revision", state.id);
        let diagnostics = completed
            .grid_track_sizing_diagnostics
            .as_ref()
            .unwrap_or_else(|| panic!("{} grid trace missing", state.id));
        assert_eq!(
            diagnostics.revision.source(),
            source,
            "{} trace source",
            state.id
        );
        assert_eq!(
            diagnostics.revision.style(),
            style_revision,
            "{} trace style",
            state.id
        );

        let trace = trace_for(diagnostics, document.root);
        let columns = trace
            .passes
            .iter()
            .rfind(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .unwrap_or_else(|| panic!("{} Columns trace missing", state.id));
        let used_columns = final_used_tracks(columns);
        assert_eq!(
            u16::try_from(used_columns.len() - 1).unwrap(),
            state.repeat_count,
            "{} auto-repeat count derived from final tracks minus its one suffix",
            state.id
        );
        assert_eq!(used_columns, state.columns, "{} used columns", state.id);

        let root_frame = frame(completed, document.root);
        assert_frame_near(
            state.id,
            "root border box",
            [
                root_frame.x,
                root_frame.y,
                root_frame.width,
                root_frame.height,
            ],
            [0.0, 0.0, state.content_width + 4.0, 44.0],
        );
        let content_origin = [root_frame.x + 2.0, root_frame.y + 2.0];
        let item_frame = frame(completed, document.item);
        assert_frame_near(
            state.id,
            "grid item relative to padding edge",
            [
                item_frame.x - content_origin[0],
                item_frame.y - content_origin[1],
                item_frame.width,
                item_frame.height,
            ],
            [0.0, 10.0, state.item_width, 30.0],
        );
        for (probe_index, probe) in document.probes.iter().copied().enumerate() {
            let probe_frame = frame(completed, probe);
            assert_frame_near(
                state.id,
                PROBE_IDS[probe_index],
                [
                    probe_frame.x - content_origin[0],
                    probe_frame.y - content_origin[1],
                    probe_frame.width,
                    probe_frame.height,
                ],
                state.probes[probe_index],
            );
        }

        previous_styles = Some(Arc::clone(&calculation.roots));
        previous_snapshot = Some(snapshot);
    }
}

fn trace_for(
    diagnostics: &RuntimeGridTrackSizingDiagnostics,
    root: HostNodeHandle,
) -> &LayoutGridTrackSizingTrace {
    diagnostics
        .traces
        .iter()
        .find(|trace| trace.node.get() == root.id().get())
        .unwrap_or_else(|| panic!("Grid trace for node {} missing", root.id().get()))
}

fn final_used_tracks(pass: &LayoutGridTrackSizingPassTrace) -> Vec<f32> {
    pass.stages
        .last()
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect()
}

fn frame(completed: &RuntimeLayoutCompleted, node: HostNodeHandle) -> RuntimeLayoutFrame {
    *completed
        .frames
        .iter()
        .find(|frame| frame.node_id == node.id().get())
        .unwrap_or_else(|| panic!("layout frame for node {} missing", node.id().get()))
}

fn assert_frame_near(state: &str, node: &str, actual: [f32; 4], expected: [f32; 4]) {
    for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= TOLERANCE,
            "{state} {node} frame component {index}: expected {expected}, got {actual}"
        );
    }
}
