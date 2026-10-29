use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::LayoutError;
use spinon_style::{CssMediaEnvironment, CssViewport, StyloDocumentView};

use crate::{
    StyleLayoutError, StyleLayoutOutput,
    compute_runtime_block_flex_grid_template_implicit_named_lines_style_layout,
    compute_runtime_block_flex_grid_template_style_layout,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

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
            DocumentChangeBatch::new(OwnerId::new(11_233).unwrap(), document.document_revision());
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
        self.compute_with_implicit_named_lines(false)
    }

    fn compute_with_implicit_named_lines(
        &self,
        allow_implicit_named_lines: bool,
    ) -> Result<StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), self.root)
                .unwrap();
        let viewport = CssViewport {
            width_css_px: 240.0,
            height_css_px: 100.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        if allow_implicit_named_lines {
            compute_runtime_block_flex_grid_template_implicit_named_lines_style_layout(
                &snapshot,
                &view,
                self.root,
                &[],
                viewport,
                StyleRevision::INITIAL,
            )
        } else {
            compute_runtime_block_flex_grid_template_style_layout(
                &snapshot,
                &view,
                self.root,
                &[],
                viewport,
                StyleRevision::INITIAL,
            )
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

fn sized_area_item(area: &str, width: u32, height: u32) -> String {
    format!(
        "box-sizing:border-box;width:{width}px;height:{height}px;grid-area:{area};margin:0;padding:0;border:0"
    )
}

#[test]
fn explicit_areas_place_items_at_their_sparse_rectangles() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:240px;height:80px;grid-template-columns:80px 80px 80px;grid-template-rows:30px 50px;grid-template-areas:'head head .' 'nav main main';column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[
            ("head", &sized_area_item("head", 160, 30)),
            ("nav", &sized_area_item("nav", 80, 50)),
            ("main", &sized_area_item("main", 160, 50)),
        ],
    );
    let output = fixture.compute().unwrap();

    let frame = |id: &str| output.layout.frames[&fixture.nodes[id].id()];
    let head = frame("head");
    let nav = frame("nav");
    let main = frame("main");
    assert_eq!(
        (head.x, head.y, head.width, head.height),
        (0.0, 0.0, 160.0, 30.0)
    );
    assert_eq!(
        (nav.x, nav.y, nav.width, nav.height),
        (0.0, 30.0, 80.0, 50.0)
    );
    assert_eq!(
        (main.x, main.y, main.width, main.height),
        (80.0, 30.0, 160.0, 50.0)
    );
}

#[test]
fn auto_sized_area_item_stretches_to_its_named_area() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:40px;grid-template-columns:40px 80px 40px;grid-template-rows:40px;grid-template-areas:'. hero .';column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[("hero", "grid-area:hero;margin:0;padding:0;border:0")],
    );
    let output = fixture.compute().unwrap();
    let frame = output.layout.frames[&fixture.nodes["hero"].id()];

    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (40.0, 0.0, 80.0, 40.0)
    );
}

#[test]
fn bare_area_name_uses_first_area_alias_while_numeric_ident_uses_named_line() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:40px;grid-template-columns:[hero-start] 40px [hero] 40px [hero-end] 80px;grid-template-rows:40px;grid-template-areas:'. hero .';column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[
            (
                "bare",
                "box-sizing:border-box;width:80px;height:40px;grid-row:1 / 2;grid-column-start:hero;grid-column-end:hero;margin:0;padding:0;border:0",
            ),
            (
                "numbered",
                "box-sizing:border-box;width:120px;height:40px;grid-row:1 / 2;grid-column-start:1 hero;grid-column-end:4;margin:0;padding:0;border:0",
            ),
        ],
    );
    let output = fixture.compute().unwrap();
    let bare = output.layout.frames[&fixture.nodes["bare"].id()];
    let numbered = output.layout.frames[&fixture.nodes["numbered"].id()];

    assert_eq!((bare.x, bare.width), (0.0, 80.0));
    assert_eq!((numbered.x, numbered.width), (40.0, 120.0));
}

#[test]
fn bare_ident_without_template_area_falls_back_to_first_named_line() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:120px;height:90px;grid-template-columns:40px [panel] 40px [panel] 40px;grid-template-rows:30px [panel] 30px [panel] 30px;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "panel-item",
            "box-sizing:border-box;width:20px;height:20px;grid-area:panel;margin:0;padding:0;border:0",
        )],
    );
    let output = fixture.compute().unwrap();
    let frame = output.layout.frames[&fixture.nodes["panel-item"].id()];

    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (40.0, 30.0, 20.0, 20.0)
    );
}

#[test]
fn explicit_line_pairs_form_implicit_areas_on_both_axes() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:80px;grid-template-columns:[box-start] 40px [box-end] 120px;grid-template-rows:[box-start] 30px [box-end] 50px;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[("implicit", &sized_area_item("box", 40, 30))],
    );
    let output = fixture.compute().unwrap();
    let frame = output.layout.frames[&fixture.nodes["implicit"].id()];

    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (0.0, 0.0, 40.0, 30.0)
    );
}

#[test]
fn mismatched_area_dimensions_are_rejected_before_layout_output() {
    for areas in ["'a b c'", "'. . .'", "'a' 'b'"] {
        let fixture = Fixture::new(
            &format!(
                "display:grid;box-sizing:border-box;width:80px;height:40px;grid-template-columns:40px 40px;grid-template-rows:40px;grid-template-areas:{areas};margin:0;padding:0;border:0"
            ),
            &[("item", "width:20px;height:20px;margin:0;padding:0;border:0")],
        );
        let error = fixture.compute().unwrap_err();

        assert!(
            matches!(
                error,
                StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property: "grid-template-areas",
                    ..
                } if node == fixture.root.id()
            ),
            "{areas}: {error}"
        );
    }
}

#[test]
fn unknown_bare_names_and_forward_named_spans_extend_fixed_tracks() {
    let grid = "display:grid;box-sizing:border-box;width:160px;height:120px;grid-template-columns:30px 40px;grid-template-rows:10px 20px;grid-auto-rows:5px;grid-auto-columns:15px;grid-auto-flow:row;margin:0;padding:0;border:0";
    let cases = [
        (
            "both-row",
            "box-sizing:border-box;width:30px;height:5px;grid-row:nonExistentArea / nonExistentArea;grid-column:1;margin:0;padding:0;border:0",
            (0.0, 35.0, 30.0, 5.0),
        ),
        (
            "column-span",
            "box-sizing:border-box;width:45px;height:10px;grid-row:1;grid-column:nonExistentArea / span 3;margin:0;padding:0;border:0",
            (85.0, 0.0, 45.0, 10.0),
        ),
        (
            "row-named-span",
            "box-sizing:border-box;width:30px;height:25px;grid-row:nonExistentArea / span 5 firstRow;grid-column:1;margin:0;padding:0;border:0",
            (0.0, 35.0, 30.0, 25.0),
        ),
        (
            "column-span-one",
            "box-sizing:border-box;width:15px;height:10px;grid-row:1;grid-column:nonExistentArea / span 1;margin:0;padding:0;border:0",
            (85.0, 0.0, 15.0, 10.0),
        ),
    ];

    for (id, item_style, expected) in cases {
        let fixture = Fixture::new(grid, &[(id, item_style)]);
        let output = fixture.compute_with_implicit_named_lines(true).unwrap();
        let frame = output.layout.frames[&fixture.nodes[id].id()];
        assert_eq!(
            (frame.x, frame.y, frame.width, frame.height),
            expected,
            "{id}"
        );
    }
}

#[test]
fn forward_named_span_counts_explicit_matches_before_adding_trailing_tracks() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:120px;height:30px;grid-template-columns:[edge] 10px [edge] 20px [edge] 30px;grid-template-rows:30px;grid-auto-columns:7px;grid-auto-rows:10px;grid-auto-flow:row;margin:0;padding:0;border:0",
        &[(
            "span",
            "box-sizing:border-box;width:74px;height:30px;grid-row:1;grid-column:1 / span 4 edge;margin:0;padding:0;border:0",
        )],
    );

    let output = fixture.compute_with_implicit_named_lines(true).unwrap();
    let frame = output.layout.frames[&fixture.nodes["span"].id()];

    assert_eq!((frame.x, frame.width), (0.0, 74.0));
}

#[test]
fn backward_named_span_creates_leading_implicit_lines() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:30px;height:20px;grid-template-columns:[edge] 10px [edge] 10px [edge] 10px;grid-template-rows:20px;grid-auto-columns:7px;grid-auto-rows:10px;grid-auto-flow:row;margin:0;padding:0;border:0",
        &[(
            "span",
            "width:10px;height:20px;grid-row:1;grid-column:span 4 edge / 3;margin:0;padding:0;border:0",
        )],
    );

    let output = fixture.compute_with_implicit_named_lines(true).unwrap();
    let frame = output.layout.frames[&fixture.nodes["span"].id()];

    assert_eq!((frame.x, frame.width), (0.0, 10.0));
}

#[test]
fn signed_named_line_implicit_matrix_matches_frozen_chrome_frames() {
    let cases = [
        (
            "positive-missing-occurrences",
            "[known-start] 30px 40px [known-end]",
            "3 ghost / 4 ghost",
            (107.0, 7.0),
        ),
        (
            "negative-missing-occurrences",
            "[known-start] 30px 40px [known-end]",
            "-1 ghost / -2 ghost",
            (0.0, 15.0),
        ),
        ("backward-span-one", "100px", "span ghost / 4", (0.0, 129.0)),
        (
            "backward-span-two",
            "100px",
            "span 2 ghost / 4",
            (0.0, 144.0),
        ),
        (
            "backward-span-partial-explicit",
            "[ghost] 40px [other] 100px",
            "span 2 ghost / 4",
            (0.0, 162.0),
        ),
        ("forward-span-two", "50px", "4 / span 2 ghost", (72.0, 22.0)),
        (
            "backward-span-uses-authored-name",
            "30px [ghost] 40px",
            "span 1 ghost / 4",
            (30.0, 55.0),
        ),
    ];

    for (id, columns, placement, expected) in cases {
        let fixture = Fixture::new(
            &format!(
                "display:grid;box-sizing:border-box;width:320px;height:20px;grid-template-columns:{columns};grid-template-rows:20px;grid-auto-rows:20px;grid-auto-columns:15px 7px;grid-auto-flow:row;gap:0;margin:0;padding:0;border:0"
            ),
            &[(
                id,
                &format!(
                    "box-sizing:border-box;width:auto;height:auto;min-width:0;min-height:0;grid-row:1 / 2;grid-column:{placement};margin:0;padding:0;border:0"
                ),
            )],
        );
        let output = fixture.compute_with_implicit_named_lines(true).unwrap();
        let frame = output.layout.frames[&fixture.nodes[id].id()];

        assert_eq!((frame.x, frame.width), expected, "{id}");
        assert_eq!((frame.y, frame.height), (0.0, 20.0), "{id}");
    }
}

#[test]
fn opt_in_does_not_accept_unmarked_numeric_implicit_lines() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:30px;grid-template-columns:40px 40px;grid-template-rows:30px;grid-auto-columns:10px;grid-auto-rows:10px;margin:0;padding:0;border:0",
        &[(
            "numeric",
            "width:10px;height:10px;grid-row:1;grid-column:1 / 5;margin:0;padding:0;border:0",
        )],
    );

    let error = fixture.compute_with_implicit_named_lines(true).unwrap_err();

    assert!(matches!(
        error,
        StyleLayoutError::Layout(LayoutError::InvalidGridPlacement {
            node,
            property: "grid-column-end",
            ..
        }) if node == fixture.nodes["numeric"].id()
    ));
}

#[test]
fn opt_in_does_not_accept_unmarked_numeric_span_extent_in_a_sibling() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:30px;grid-template-columns:40px 40px;grid-template-rows:30px;grid-auto-columns:10px;grid-auto-rows:10px;margin:0;padding:0;border:0",
        &[
            (
                "implicit",
                "width:10px;height:10px;grid-row:missing / missing;grid-column:1;margin:0;padding:0;border:0",
            ),
            (
                "numeric-span",
                "width:10px;height:10px;grid-row:1;grid-column:1 / span 5;margin:0;padding:0;border:0",
            ),
        ],
    );

    assert!(matches!(
        fixture.compute_with_implicit_named_lines(true),
        Err(StyleLayoutError::Layout(LayoutError::InvalidGridPlacement {
            node,
            property: "grid-column-end",
            ..
        })) if node == fixture.nodes["numeric-span"].id()
    ));
}

#[test]
fn opt_in_does_not_accept_unmarked_leading_span_extent() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:30px;grid-template-columns:40px 40px;grid-template-rows:30px;grid-auto-columns:10px;grid-auto-rows:10px;margin:0;padding:0;border:0",
        &[
            (
                "implicit",
                "width:10px;height:10px;grid-row:missing / missing;grid-column:1;margin:0;padding:0;border:0",
            ),
            (
                "numeric-span",
                "width:10px;height:10px;grid-row:1;grid-column:span 5 / -1;margin:0;padding:0;border:0",
            ),
        ],
    );

    assert!(matches!(
        fixture.compute_with_implicit_named_lines(true),
        Err(StyleLayoutError::Layout(LayoutError::InvalidGridPlacement {
            node,
            property: "grid-column-start",
            ..
        })) if node == fixture.nodes["numeric-span"].id()
    ));
}

#[test]
fn opt_in_does_not_accept_unmarked_last_line_auto_extent() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:30px;grid-template-columns:40px 40px;grid-template-rows:30px;grid-auto-columns:10px;grid-auto-rows:10px;margin:0;padding:0;border:0",
        &[
            (
                "implicit",
                "width:10px;height:10px;grid-row:missing / missing;grid-column:1;margin:0;padding:0;border:0",
            ),
            (
                "numeric-auto",
                "width:10px;height:10px;grid-row:1;grid-column:3 / auto;margin:0;padding:0;border:0",
            ),
        ],
    );

    assert!(matches!(
        fixture.compute_with_implicit_named_lines(true),
        Err(StyleLayoutError::Layout(LayoutError::InvalidGridPlacement {
            node,
            property: "grid-column-end",
            ..
        })) if node == fixture.nodes["numeric-auto"].id()
    ));
}

#[test]
fn implicit_track_profile_requires_fixed_tracks_and_definite_item_placement() {
    let auto_track = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:30px;grid-template-columns:40px 40px;grid-template-rows:30px;grid-auto-columns:10px;grid-auto-rows:auto;margin:0;padding:0;border:0",
        &[(
            "implicit",
            "width:10px;height:10px;grid-row:missing / missing;grid-column:1;margin:0;padding:0;border:0",
        )],
    );
    assert!(matches!(
        auto_track.compute_with_implicit_named_lines(true),
        Err(StyleLayoutError::Layout(LayoutError::UnsupportedGridAutoRows {
            node,
            ..
        })) if node == auto_track.root.id()
    ));

    let auto_item = Fixture::new(
        "display:grid;box-sizing:border-box;width:80px;height:30px;grid-template-columns:40px 40px;grid-template-rows:30px;grid-auto-columns:10px;grid-auto-rows:10px;margin:0;padding:0;border:0",
        &[
            (
                "implicit",
                "width:10px;height:10px;grid-row:missing / missing;grid-column:1;margin:0;padding:0;border:0",
            ),
            (
                "auto",
                "width:10px;height:10px;grid-column:1;margin:0;padding:0;border:0",
            ),
        ],
    );
    assert!(matches!(
        auto_item.compute_with_implicit_named_lines(true),
        Err(StyleLayoutError::Layout(LayoutError::GridImplicitTrackRequired(node)))
            if node == auto_item.root.id()
    ));
}

#[test]
fn unknown_template_area_preserves_known_area_sibling_and_overflows_root() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:120px;grid-template-columns:80px 80px;grid-template-rows:60px 60px;grid-template-areas:'known .' '. .';grid-auto-rows:5px;grid-auto-columns:15px;grid-auto-flow:row;margin:0;padding:0;border:0",
        &[
            (
                "known-area",
                "grid-area:known;width:80px;height:60px;margin:0;padding:0;border:0",
            ),
            (
                "unknown-area",
                "grid-area:not-authored;width:15px;height:5px;margin:0;padding:0;border:0",
            ),
        ],
    );
    let output = fixture.compute_with_implicit_named_lines(true).unwrap();
    let known = output.layout.frames[&fixture.nodes["known-area"].id()];
    let unknown = output.layout.frames[&fixture.nodes["unknown-area"].id()];

    assert_eq!(
        (known.x, known.y, known.width, known.height),
        (0.0, 0.0, 80.0, 60.0)
    );
    assert_eq!(
        (unknown.x, unknown.y, unknown.width, unknown.height),
        (175.0, 125.0, 15.0, 5.0)
    );
}

#[test]
fn existing_template_wrapper_still_rejects_unknown_bare_names() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:160px;height:120px;grid-template-columns:30px 40px;grid-template-rows:10px 20px;grid-auto-rows:5px;grid-auto-columns:15px;margin:0;padding:0;border:0",
        &[(
            "unknown",
            "width:30px;height:5px;grid-row:missing / missing;grid-column:1;margin:0;padding:0;border:0",
        )],
    );

    let error = fixture.compute().unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::UnsupportedNamedGridLine { node, .. }
            if node == fixture.nodes["unknown"].id()
    ));
    assert!(fixture.compute_with_implicit_named_lines(true).is_ok());
}
