use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{CssMediaEnvironment, CssViewport, StyloDocumentView};

use crate::{
    StyleLayoutError, StyleLayoutOutput, compute_runtime_block_flex_grid_template_style_layout,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const ROOT_STYLE: &str = "display:grid;box-sizing:border-box;width:160px;height:80px;grid-template-columns:[start] 40px [mid] 40px [mid end] 40px [finish] 40px [last];grid-template-rows:[row-start] 40px [row-end] 40px;column-gap:0;row-gap:0;margin:0;padding:0;border:0";

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    nodes: BTreeMap<String, HostNodeHandle>,
}

impl Fixture {
    fn new(root_style: &str, item_styles: &[(&str, &str)]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let nodes = item_styles
            .iter()
            .map(|(id, _)| ((*id).to_owned(), document.reserve_node_handle().unwrap()))
            .collect::<BTreeMap<_, _>>();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_232).unwrap(), document.document_revision());
        append_element(&mut batch, root, HostParent::Root, "grid", root_style);
        for (id, style) in item_styles {
            append_element(&mut batch, nodes[*id], HostParent::Node(root), id, style);
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            nodes,
        }
    }

    fn compute(&self) -> Result<StyleLayoutOutput, StyleLayoutError> {
        self.compute_with_revision(StyleRevision::INITIAL)
    }

    fn compute_with_revision(
        &self,
        style_revision: StyleRevision,
    ) -> Result<StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), self.root)
                .unwrap();
        compute_runtime_block_flex_grid_template_style_layout(
            &snapshot,
            &view,
            self.root,
            &[],
            CssViewport {
                width_css_px: 160.0,
                height_css_px: 80.0,
                device_scale_factor: 1.0,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
            style_revision,
        )
    }

    fn update_styles(&mut self, updates: &[(HostNodeHandle, String)]) {
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_232).unwrap(),
            self.document.document_revision(),
        );
        for (node, style) in updates {
            batch.push(DocumentOperation::SetAttribute {
                node: *node,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.clone().into(),
            });
        }
        self.document.commit(batch).unwrap();
    }

    fn nested_grid() -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let nested = document.reserve_node_handle().unwrap();
        let child = document.reserve_node_handle().unwrap();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_232).unwrap(), document.document_revision());
        append_element(&mut batch, root, HostParent::Root, "root", ROOT_STYLE);
        append_element(
            &mut batch,
            nested,
            HostParent::Node(root),
            "nested-grid",
            "display:grid;box-sizing:border-box;width:80px;height:40px;grid-template-columns:[start] 40px [inner] 40px [end];grid-template-rows:[top] 40px [bottom];grid-row:row-start / row-end;grid-column:start / mid 2;margin:0;padding:0;border:0",
        );
        append_element(
            &mut batch,
            child,
            HostParent::Node(nested),
            "nested-child",
            "box-sizing:border-box;width:40px;height:40px;grid-row:top / bottom;grid-column:inner / end;margin:0;padding:0;border:0",
        );
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            nodes: [
                ("nested-grid".to_owned(), nested),
                ("nested-child".to_owned(), child),
            ]
            .into_iter()
            .collect(),
        }
    }
}

fn append_element(
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
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "id").unwrap(),
        value: id.to_owned().into(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "style").unwrap(),
        value: style.to_owned().into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node,
        before: None,
    });
}

fn item_style(column: &str, width: u32) -> String {
    format!(
        "box-sizing:border-box;width:{width}px;height:40px;margin:0;padding:0;border:0;grid-row:row-start / row-end;grid-column:{column}"
    )
}

#[test]
fn positive_and_negative_named_occurrences_match_chrome_grid_coordinates() {
    let fixture = Fixture::new(
        ROOT_STYLE,
        &[
            ("positive", &item_style("mid 2 / last", 80)),
            ("negative", &item_style("mid -2 / 3", 40)),
        ],
    );
    let output = fixture.compute().unwrap();
    let positive = output.layout.frames[&fixture.nodes["positive"].id()];
    let negative = output.layout.frames[&fixture.nodes["negative"].id()];

    assert_eq!(
        (positive.x, positive.y, positive.width, positive.height),
        (80.0, 0.0, 80.0, 40.0)
    );
    assert_eq!(
        (negative.x, negative.y, negative.width, negative.height),
        (40.0, 0.0, 40.0, 40.0)
    );
}

#[test]
fn named_spans_search_from_the_opposite_definite_line_in_both_directions() {
    let fixture = Fixture::new(
        ROOT_STYLE,
        &[
            ("forward", &item_style("start / span 2 mid", 80)),
            ("backward", &item_style("span mid / last", 80)),
        ],
    );
    let output = fixture.compute().unwrap();
    let forward = output.layout.frames[&fixture.nodes["forward"].id()];
    let backward = output.layout.frames[&fixture.nodes["backward"].id()];

    assert_eq!((forward.x, forward.width), (0.0, 80.0));
    assert_eq!((backward.x, backward.width), (80.0, 80.0));
}

#[test]
fn named_span_without_opposite_definite_edge_uses_one_track_and_ignores_name() {
    let fixture = Fixture::new(
        ROOT_STYLE,
        &[("auto-span", &item_style("span absent-name / auto", 40))],
    );
    let output = fixture.compute().unwrap();
    let frame = output.layout.frames[&fixture.nodes["auto-span"].id()];
    assert_eq!((frame.x, frame.width), (0.0, 40.0));
}

#[test]
fn missing_case_mismatched_or_out_of_range_named_lines_fail_before_layout_output() {
    for column in [
        "missing / last",
        "MID / last",
        "mid 3 / last",
        "mid -3 / last",
    ] {
        let fixture = Fixture::new(ROOT_STYLE, &[("invalid", &item_style(column, 40))]);
        let error = fixture.compute().unwrap_err();
        assert!(
            matches!(error, StyleLayoutError::UnsupportedNamedGridLine { .. }),
            "{column}: {error}"
        );
    }
}

#[test]
fn reversed_identifier_integer_order_is_equivalent_after_stylo_cascade() {
    let fixture = Fixture::new(
        ROOT_STYLE,
        &[
            ("ident-first", &item_style("mid 2 / last", 80)),
            ("integer-first", &item_style("2 mid / last", 80)),
        ],
    );
    let output = fixture.compute().unwrap();
    let first = output.layout.frames[&fixture.nodes["ident-first"].id()];
    let second = output.layout.frames[&fixture.nodes["integer-first"].id()];
    assert_eq!(
        (first.x, first.y, first.width, first.height),
        (second.x, second.y, second.width, second.height)
    );
}

#[test]
fn named_line_metadata_is_resolved_from_the_immediate_nested_grid_owner() {
    let fixture = Fixture::nested_grid();
    let output = fixture.compute().unwrap();
    let grid = output.layout.frames[&fixture.nodes["nested-grid"].id()];
    let child = output.layout.frames[&fixture.nodes["nested-child"].id()];

    assert_eq!((grid.x, grid.width), (0.0, 80.0));
    assert_eq!(
        (child.x, child.y, child.width, child.height),
        (40.0, 0.0, 40.0, 40.0)
    );
}

#[test]
fn named_occurrence_counts_repeated_track_seam_as_one_physical_line() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:40px;grid-template-columns:repeat(2,[seam] 40px);grid-template-rows:[row-start] 40px [row-end];column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[("seam-item", &item_style("seam 2 / 3", 40))],
    );
    let output = fixture.compute().unwrap();
    let frame = output.layout.frames[&fixture.nodes["seam-item"].id()];

    assert_eq!((frame.x, frame.width), (40.0, 40.0));
}

#[test]
fn changed_grid_line_names_are_used_at_the_next_style_revision() {
    let mut fixture = Fixture::new(ROOT_STYLE, &[("renamed", &item_style("mid 2 / last", 80))]);
    let node = fixture.nodes["renamed"];
    let before = fixture.compute().unwrap();
    assert_eq!(before.layout.frames[&node.id()].x, 80.0);

    let renamed_root = ROOT_STYLE
        .replace("[mid]", "[new-mid]")
        .replace("[mid end]", "[new-mid end]");
    fixture.update_styles(&[
        (fixture.root, renamed_root),
        (node, item_style("new-mid 2 / last", 80)),
    ]);
    let next_revision = StyleRevision::INITIAL.checked_next().unwrap();
    let after = fixture.compute_with_revision(next_revision).unwrap();

    assert_eq!(after.computed_styles.style_revision, next_revision);
    assert_eq!(after.layout.frames[&node.id()].x, 80.0);
}

#[test]
fn named_endpoint_reversal_and_equality_follow_grid_conflict_rules() {
    let fixture = Fixture::new(
        ROOT_STYLE,
        &[
            ("reversed", &item_style("last / start", 160)),
            ("equal", &item_style("mid / mid", 40)),
        ],
    );
    let output = fixture.compute().unwrap();
    let reversed = output.layout.frames[&fixture.nodes["reversed"].id()];
    let equal = output.layout.frames[&fixture.nodes["equal"].id()];

    assert_eq!((reversed.x, reversed.width), (0.0, 160.0));
    assert_eq!((equal.x, equal.width), (40.0, 40.0));
}
