use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};
use spinon_layout::{LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutSourceRevision};
use spinon_style::{
    ComputedStyleProfile, CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use crate::compute_runtime_style_layout_with_intrinsic_size_snapshot_for_c11_4_two_axis_grid_fixture;
use crate::{
    StyleLayoutError, StyleLayoutOutput, compute_runtime_block_flex_grid_template_style_layout,
    compute_runtime_block_flex_grid_track_sizing_style_layout,
    compute_runtime_style_layout_with_intrinsic_size_snapshot,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_calc.rs"]
mod cyclic_calc;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_fit_content.rs"]
mod cyclic_fit_content;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_calc_cross_axis_mixed_span_groups.rs"]
mod cyclic_minmax_calc_cross_axis_mixed_span_groups;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_calc_cross_axis_span_groups.rs"]
mod cyclic_minmax_calc_cross_axis_span_groups;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_calc_minimum.rs"]
mod cyclic_minmax_calc_minimum;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_calc_minimum_span.rs"]
mod cyclic_minmax_calc_minimum_span;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_calc_minimum_two_axis.rs"]
mod cyclic_minmax_calc_minimum_two_axis;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_calc_overlapping_rows.rs"]
mod cyclic_minmax_calc_overlapping_rows;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_calc_overlapping_spans.rs"]
mod cyclic_minmax_calc_overlapping_spans;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_calc_two_axis_spanning_text.rs"]
mod cyclic_minmax_calc_two_axis_spanning_text;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_mixed_text_image_span.rs"]
mod cyclic_minmax_mixed_text_image_span;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_minmax_replaced_image_span.rs"]
mod cyclic_minmax_replaced_image_span;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/cyclic_two_axis_multitrack.rs"]
mod cyclic_two_axis_multitrack;
#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
#[path = "c11_4_track_sizing_areas_tests/orthogonal_writing_mode.rs"]
mod orthogonal_writing_mode;

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    nodes: BTreeMap<String, HostNodeHandle>,
}

impl Fixture {
    fn new(root_style: &str, item_styles: &[(&str, &str)]) -> Self {
        Self::new_with_item_tag(root_style, item_styles, "div")
    }

    fn new_with_item_tag(root_style: &str, item_styles: &[(&str, &str)], item_tag: &str) -> Self {
        let tagged_items = item_styles
            .iter()
            .map(|(id, style)| (*id, item_tag, *style))
            .collect::<Vec<_>>();
        Self::new_with_item_tags(root_style, &tagged_items)
    }

    fn new_with_item_tags(root_style: &str, item_styles: &[(&str, &str, &str)]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let nodes = item_styles
            .iter()
            .map(|(id, _, _)| ((*id).to_owned(), document.reserve_node_handle().unwrap()))
            .collect::<BTreeMap<_, _>>();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_234).unwrap(), document.document_revision());
        append_element(&mut batch, root, HostParent::Root, "grid", root_style);
        for (id, tag, style) in item_styles {
            append_element_with_tag(
                &mut batch,
                nodes[*id],
                HostParent::Node(root),
                tag,
                id,
                style,
            );
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            nodes,
        }
    }

    fn new_nested_image(root_style: &str, image_style: &str) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let wrapper = document.reserve_node_handle().unwrap();
        let image = document.reserve_node_handle().unwrap();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_235).unwrap(), document.document_revision());
        append_element(&mut batch, root, HostParent::Root, "grid", root_style);
        append_element(
            &mut batch,
            wrapper,
            HostParent::Node(root),
            "wrapper",
            "margin:0;padding:0;border:0",
        );
        append_element_with_tag(
            &mut batch,
            image,
            HostParent::Node(wrapper),
            "img",
            "image",
            image_style,
        );
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            nodes: BTreeMap::from([("wrapper".to_owned(), wrapper), ("image".to_owned(), image)]),
        }
    }

    fn compute(&self) -> StyleLayoutOutput {
        self.try_compute().unwrap()
    }

    fn try_compute(&self) -> Result<StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), self.root)
                .unwrap();
        compute_runtime_block_flex_grid_track_sizing_style_layout(
            &snapshot,
            &view,
            self.root,
            &[],
            CssViewport {
                width_css_px: 240.0,
                height_css_px: 80.0,
                device_scale_factor: 1.0,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
            StyleRevision::INITIAL,
        )
    }

    fn compute_template(&self) -> Result<StyleLayoutOutput, StyleLayoutError> {
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
                width_css_px: 240.0,
                height_css_px: 80.0,
                device_scale_factor: 1.0,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            },
            StyleRevision::INITIAL,
        )
    }

    fn compute_with_image_intrinsic_size(&self) -> Result<StyleLayoutOutput, StyleLayoutError> {
        self.compute_with_image_intrinsic_sizes(&["image"])
    }

    fn compute_with_image_intrinsic_sizes(
        &self,
        image_ids: &[&str],
    ) -> Result<StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), self.root)
                .unwrap();
        let viewport = CssViewport {
            width_css_px: 240.0,
            height_css_px: 80.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let computed_styles =
            compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &view,
                &[],
                viewport,
                StyleRevision::INITIAL,
            )?;
        let intrinsic_sizes = LayoutIntrinsicSizeSnapshot::new(
            LayoutSourceRevision::HostDocument {
                generation: snapshot.generation(),
                document: snapshot.document_revision(),
                render_tree: snapshot.render_tree_revision(),
            },
            IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
            image_ids
                .iter()
                .map(|id| (self.nodes[*id].id(), LayoutIntrinsicSize::new(24.0, 16.0))),
        )?;
        compute_runtime_style_layout_with_intrinsic_size_snapshot(
            &snapshot,
            self.root,
            computed_styles,
            viewport,
            intrinsic_sizes,
        )
    }

    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    fn compute_with_image_intrinsic_sizes_and_two_axis_grid(
        &self,
        image_ids: &[&str],
    ) -> Result<StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), self.root)
                .unwrap();
        let viewport = CssViewport {
            width_css_px: 240.0,
            height_css_px: 80.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let computed_styles =
            compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &view,
                &[],
                viewport,
                StyleRevision::INITIAL,
            )?;
        let intrinsic_sizes = LayoutIntrinsicSizeSnapshot::new(
            LayoutSourceRevision::HostDocument {
                generation: snapshot.generation(),
                document: snapshot.document_revision(),
                render_tree: snapshot.render_tree_revision(),
            },
            IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
            image_ids
                .iter()
                .map(|id| (self.nodes[*id].id(), LayoutIntrinsicSize::new(24.0, 16.0))),
        )?;
        compute_runtime_style_layout_with_intrinsic_size_snapshot_for_c11_4_two_axis_grid_fixture(
            &snapshot,
            self.root,
            computed_styles,
            viewport,
            intrinsic_sizes,
        )
    }
}

fn append_element(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    parent: HostParent,
    id: &str,
    style: &str,
) {
    append_element_with_tag(batch, node, parent, "div", id, style);
}

fn append_element_with_tag(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    parent: HostParent,
    tag: &str,
    id: &str,
    style: &str,
) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: tag.to_owned(),
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

fn ratio_constraint_styles() -> (&'static str, &'static str) {
    (
        "display:grid;box-sizing:border-box;width:40px;height:20px;grid-template-columns:minmax(0,1fr);grid-template-rows:20px;align-items:start;align-content:start;justify-content:start;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        "display:block;box-sizing:border-box;width:auto;height:8px;aspect-ratio:3/2;max-height:4px;margin:0;padding:0;border:0",
    )
}

#[test]
fn track_sizing_maps_pref_disabled_inline_author_property_error() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:74px;height:auto;grid-template-columns:50px 24px;grid-template-rows:min-content;column-gap:0;row-gap:0;margin:0;padding:0;border:0;text-orientation:upright",
        &[(
            "item",
            "display:block;width:24px;height:16px;margin:0;padding:0;border:0",
        )],
    );

    let error = fixture.try_compute().unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::UnsupportedInlineProperty { node, property }
            if node == fixture.root.id() && property == "text-orientation"
    ));
}

#[test]
fn direct_html_image_ratio_constraints_are_scoped_to_track_sizing_profile() {
    let (root_style, image_style) = ratio_constraint_styles();
    let image = Fixture::new_with_item_tag(root_style, &[("image", image_style)], "img");
    let output = image.compute_with_image_intrinsic_size().unwrap();
    let image_frame = output.layout.frames[&image.nodes["image"].id()];

    assert_eq!(
        output.computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    assert_eq!((image_frame.width, image_frame.height), (6.0, 4.0));

    let other_grid_profile =
        Fixture::new_with_item_tag(root_style, &[("image", image_style)], "img");
    let other_profile_result = other_grid_profile.compute_template();
    assert!(
        matches!(
            &other_profile_result,
            Err(StyleLayoutError::UnsupportedInlineProperty {
                property,
                ..
            }) if property == "aspect-ratio"
        ),
        "unexpected other-profile result: {other_profile_result:?}"
    );
}

#[test]
fn ratio_constraints_remain_rejected_for_non_image_grid_items() {
    let (root_style, item_style) = ratio_constraint_styles();
    let fixture = Fixture::new(root_style, &[("item", item_style)]);

    assert!(matches!(
        fixture.try_compute(),
        Err(StyleLayoutError::UnsupportedAspectRatioConstraint {
            property: "max-height",
            ..
        })
    ));
}

#[test]
fn ratio_constraints_remain_rejected_for_nested_html_images() {
    let (root_style, image_style) = ratio_constraint_styles();
    let fixture = Fixture::new_nested_image(root_style, image_style);
    let result = fixture.try_compute();

    assert!(
        matches!(
            &result,
            Err(StyleLayoutError::UnsupportedAspectRatioConstraint {
                property: "max-height",
                ..
            })
        ),
        "unexpected nested-image result: {result:?}"
    );
}

#[test]
fn track_sizing_profile_keeps_named_area_placement() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:240px;height:80px;grid-template-columns:80px minmax(20px,1fr) 80px;grid-template-rows:30px 50px;grid-template-areas:'head head .' 'nav main main';column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[
            (
                "head",
                "box-sizing:border-box;width:160px;height:30px;grid-area:head;margin:0;padding:0;border:0",
            ),
            (
                "nav",
                "box-sizing:border-box;width:80px;height:50px;grid-area:nav;margin:0;padding:0;border:0",
            ),
            (
                "main",
                "box-sizing:border-box;width:160px;height:50px;grid-area:main;margin:0;padding:0;border:0",
            ),
        ],
    );
    let output = fixture.compute();

    assert_eq!(
        output.computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    for (id, expected) in [
        ("head", (0.0, 0.0, 160.0, 30.0)),
        ("nav", (0.0, 30.0, 80.0, 50.0)),
        ("main", (80.0, 30.0, 160.0, 50.0)),
    ] {
        let frame = output.layout.frames[&fixture.nodes[id].id()];
        assert_eq!(
            (frame.x, frame.y, frame.width, frame.height),
            expected,
            "area {id}"
        );
    }
}

#[test]
fn auto_repeat_template_areas_accept_bare_grid_area_placement_on_both_axes() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:116px;height:20px;grid-template-columns:20px repeat(auto-fill,20px) 20px;grid-template-rows:20px;grid-template-areas:'prefix hero hero hero suffix';column-gap:4px;row-gap:4px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        &[(
            "hero",
            "box-sizing:border-box;width:auto;height:auto;grid-area:hero;margin:0;padding:0;border:0",
        )],
    );
    let output = fixture.try_compute();
    assert!(
        output.is_ok(),
        "unexpected area placement result: {output:?}"
    );
    let output = output.unwrap();
    let frame = output.layout.frames[&fixture.nodes["hero"].id()];
    assert_eq!(
        (frame.x, frame.y, frame.width, frame.height),
        (24.0, 0.0, 68.0, 20.0)
    );
}

#[test]
fn cross_axis_fixed_repeat_and_auto_repeat_project_areas_on_either_axis() {
    let fixed_columns = Fixture::new(
        "display:grid;box-sizing:border-box;width:44px;height:76px;grid-template-columns:repeat(2,20px);grid-template-rows:repeat(auto-fill,20px);grid-template-areas:'hero .' 'hero .';column-gap:4px;row-gap:4px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        &[(
            "hero",
            "box-sizing:border-box;grid-area:hero;margin:0;padding:0;border:0",
        )],
    );
    let output = fixed_columns.try_compute().unwrap();
    let hero = output.layout.frames[&fixed_columns.nodes["hero"].id()];
    assert_eq!(
        (hero.x, hero.y, hero.width, hero.height),
        (0.0, 0.0, 20.0, 44.0)
    );

    let fixed_rows = Fixture::new(
        "display:grid;box-sizing:border-box;width:76px;height:44px;grid-template-columns:repeat(auto-fill,20px);grid-template-rows:repeat(2,20px);grid-template-areas:'hero hero' '. .';column-gap:4px;row-gap:4px;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        &[(
            "hero",
            "box-sizing:border-box;grid-area:hero;margin:0;padding:0;border:0",
        )],
    );
    let output = fixed_rows.try_compute().unwrap();
    let hero = output.layout.frames[&fixed_rows.nodes["hero"].id()];
    assert_eq!(
        (hero.x, hero.y, hero.width, hero.height),
        (0.0, 0.0, 44.0, 20.0)
    );
}

#[test]
fn cross_axis_fixed_repeat_rejects_area_dimension_mismatch_during_projection() {
    let cases = [
        (
            "fixed columns with too few area columns",
            "display:grid;box-sizing:border-box;width:44px;height:76px;grid-template-columns:repeat(2,20px);grid-template-rows:repeat(auto-fill,20px);grid-template-areas:'hero';column-gap:4px;row-gap:4px;margin:0;padding:0;border:0",
        ),
        (
            "fixed columns with too many area columns",
            "display:grid;box-sizing:border-box;width:44px;height:76px;grid-template-columns:repeat(2,20px);grid-template-rows:repeat(auto-fill,20px);grid-template-areas:'left middle right';column-gap:4px;row-gap:4px;margin:0;padding:0;border:0",
        ),
        (
            "fixed rows with too few area rows",
            "display:grid;box-sizing:border-box;width:76px;height:44px;grid-template-columns:repeat(auto-fill,20px);grid-template-rows:repeat(2,20px);grid-template-areas:'left right';column-gap:4px;row-gap:4px;margin:0;padding:0;border:0",
        ),
        (
            "fixed rows with too many area rows",
            "display:grid;box-sizing:border-box;width:76px;height:44px;grid-template-columns:repeat(auto-fill,20px);grid-template-rows:repeat(2,20px);grid-template-areas:'a b' 'c d' 'e f';column-gap:4px;row-gap:4px;margin:0;padding:0;border:0",
        ),
    ];
    for (case, root_style) in cases {
        let fixture = Fixture::new(root_style, &[]);
        assert!(
            matches!(
                fixture.try_compute(),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    node,
                    property: "grid-template-areas",
                    ..
                }) if node == fixture.root.id()
            ),
            "{case} must fail on the root during grid-template-areas projection"
        );
    }
}

#[test]
fn root_grid_max_content_is_scoped_to_the_track_sizing_profile() {
    let grid_style = "display:grid;box-sizing:border-box;width:max-content;height:max-content;grid-template-columns:40px 60px;grid-template-rows:10px 10px;column-gap:10%;row-gap:25%;margin:0;padding:0;border:0";
    let fixture = Fixture::new(grid_style, &[]);
    let output = fixture.compute();

    assert_eq!(
        output.computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    let frame = output.layout.frames[&fixture.root.id()];
    assert_eq!((frame.width, frame.height), (100.0, 20.0));

    let min_content = Fixture::new(
        "display:grid;box-sizing:border-box;width:min-content;height:auto;grid-template-columns:min-content;grid-template-rows:auto;column-gap:0;row-gap:0;margin:0;padding:0;border:0",
        &[(
            "item",
            "box-sizing:border-box;width:40px;height:10px;margin:0;padding:0;border:0",
        )],
    );
    let min_content_output = min_content.compute();
    let min_content_frame = min_content_output.layout.frames[&min_content.root.id()];
    assert_eq!(
        (min_content_frame.width, min_content_frame.height),
        (40.0, 10.0)
    );
    assert!(min_content.compute_template().is_err());

    let other_profile = Fixture::new(grid_style, &[]).compute_template();
    assert!(matches!(
        other_profile,
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::UnsupportedIntrinsicSizing { .. }
        ))
    ));

    let block = Fixture::new(
        "display:block;box-sizing:border-box;width:max-content;height:auto;margin:0;padding:0;border:0",
        &[],
    );
    assert!(matches!(
        block.try_compute(),
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::UnsupportedIntrinsicSizing { .. }
        ))
    ));
}

#[test]
fn track_sizing_profile_lays_out_css_auto_repeat_modes() {
    for (keyword, expected_width) in [("auto-fill", 21.25), ("auto-fit", 100.0)] {
        let fixture = Fixture::new(
            &format!(
                "display:grid;box-sizing:border-box;width:100px;height:30px;grid-template-columns:repeat({keyword},minmax(20px,1fr));grid-template-rows:30px;column-gap:5px;row-gap:0;margin:0;padding:0;border:0"
            ),
            &[(
                "item",
                "box-sizing:border-box;width:100%;height:20px;margin:0;padding:0;border:0",
            )],
        );
        let output = fixture.compute();
        let frame = output.layout.frames[&fixture.nodes["item"].id()];

        assert_eq!(frame.width, expected_width, "{keyword}");
        assert_eq!(frame.height, 20.0, "{keyword}");
    }
}

#[test]
fn track_sizing_profile_projects_negative_lines_and_reverse_auto_track_lists() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:240px;height:20px;grid-template-columns:10px 20px;grid-template-rows:10px;grid-auto-columns:30px 40px 50px;column-gap:0;row-gap:0;align-items:start;align-content:start;justify-content:start;margin:0;padding:0;border:0",
        &[
            (
                "leading",
                "width:1px;height:1px;grid-row:1 / 2;grid-column:-5 / -2;margin:0;padding:0;border:0",
            ),
            (
                "explicit",
                "width:1px;height:1px;grid-row:1 / 2;grid-column:1 / 2;margin:0;padding:0;border:0",
            ),
            (
                "trailing",
                "width:1px;height:1px;grid-row:1 / 2;grid-column:5 / 6;margin:0;padding:0;border:0",
            ),
        ],
    );
    let output = fixture.compute();

    assert_eq!(
        output.computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    for (id, expected_x) in [("leading", 0.0), ("explicit", 90.0), ("trailing", 190.0)] {
        let frame = output.layout.frames[&fixture.nodes[id].id()];
        assert_eq!(frame.x, expected_x, "{id}");
    }
}

#[test]
fn track_sizing_profile_anchors_negative_lines_when_template_is_none() {
    let grid_style = "display:grid;box-sizing:border-box;width:240px;height:20px;grid-template-columns:none;grid-template-rows:10px;grid-auto-columns:30px 40px 50px;column-gap:0;row-gap:0;align-items:start;align-content:start;justify-content:start;margin:0;padding:0;border:0";
    let fixture = Fixture::new(
        grid_style,
        &[
            (
                "leading",
                "width:1px;height:1px;grid-row:1 / 2;grid-column:-2 / 1;margin:0;padding:0;border:0",
            ),
            (
                "trailing",
                "width:1px;height:1px;grid-row:1 / 2;grid-column:1 / 2;margin:0;padding:0;border:0",
            ),
        ],
    );
    let output = fixture.compute();

    assert_eq!(
        output.computed_styles.profile,
        ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    assert_eq!(output.layout.frames[&fixture.nodes["leading"].id()].x, 0.0);
    assert_eq!(
        output.layout.frames[&fixture.nodes["trailing"].id()].x,
        50.0
    );

    assert!(matches!(
        Fixture::new(grid_style, &[]).compute_template(),
        Err(StyleLayoutError::Cascade(
            spinon_style::CssCascadeError::UnsupportedComputedGridValue {
                property: "grid-template-columns",
                ..
            }
        ))
    ));
}

#[test]
fn auto_fit_wpt_accepts_multiple_percent_neighbors_and_named_implicit_lines() {
    let scenes = [
        (
            "multiple percentage prefixes",
            "display:grid;background-color:grey;border:2px solid magenta;width:200px;position:relative;justify-content:start;grid-auto-columns:157px;grid-auto-rows:25px;grid-template-columns:10px [foo] 20% [bar] repeat(auto-fit,[autofoo] 35px);column-gap:20px",
            "background-color:cyan;grid-column:notPresent / 3 autofoo",
            269.0,
            157.0,
        ),
        (
            "percentage neighbors and repeated names",
            "display:grid;background-color:grey;border:2px solid magenta;width:200px;position:relative;justify-content:start;grid-auto-columns:157px;grid-auto-rows:25px;grid-template-columns:[start] 20px [foo] 50% repeat(auto-fit,[bar] 20px [start foo]) [foo] 10% [end bar];column-gap:20px",
            "background-color:cyan;grid-column:foo / bar 2",
            42.0,
            180.0,
        ),
    ];
    for (name, root_style, item_style, expected_x, expected_width) in scenes {
        let fixture = Fixture::new(root_style, &[("wpt-item", item_style)]);
        let output = fixture
            .try_compute()
            .unwrap_or_else(|error| panic!("{name} must reach layout: {error:?}"));
        let frame = output.layout.frames[&fixture.nodes["wpt-item"].id()];
        assert_eq!(frame.x, expected_x, "{name} x");
        assert_eq!(frame.y, 2.0, "{name} y");
        assert_eq!(frame.width, expected_width, "{name} width");
        assert_eq!(frame.height, 25.0, "{name} height");
    }
}

#[test]
fn auto_fit_wpt_resolves_percentage_minmax_suffix_neighbors() {
    let root_style = "display:grid;background-color:grey;border:2px solid magenta;width:200px;position:relative;justify-content:start;grid-auto-columns:157px;grid-auto-rows:25px;grid-template-columns:[start] repeat(2,50px [a]) [middle] repeat(auto-fit,[autofoo] 15px [autobar]) minmax(5%,10%) [end]";
    let gap_root_style = format!("{root_style};grid-column-gap:20px");
    let scenes = [
        ("grid-column:1 / span 8", 2.0, 195.0),
        ("grid-column:a / autobar 2", 52.0, 80.0),
        ("grid-column:autofoo / end", 102.0, 95.0),
    ];
    for (item_style, expected_x, expected_width) in scenes {
        let fixture = Fixture::new(root_style, &[("wpt-item", item_style)]);
        let output = fixture.try_compute().unwrap_or_else(|error| {
            panic!("percentage minmax neighbor must resolve: {item_style}: {error:?}")
        });
        let frame = output.layout.frames[&fixture.nodes["wpt-item"].id()];
        assert_eq!(frame.x, expected_x, "{item_style} x");
        assert_eq!(frame.y, 2.0, "{item_style} y");
        assert_eq!(frame.width, expected_width, "{item_style} width");
        assert_eq!(frame.height, 25.0, "{item_style} height");
    }

    for (item_style, expected_x, expected_width) in [
        ("grid-column:1 / span 4", 2.0, 195.0),
        ("grid-column:autobar / -1", 142.0, 20.0),
    ] {
        let fixture = Fixture::new(&gap_root_style, &[("wpt-item", item_style)]);
        let output = fixture.try_compute().unwrap_or_else(|error| {
            panic!("gapped percentage minmax neighbor must resolve: {item_style}: {error:?}")
        });
        let frame = output.layout.frames[&fixture.nodes["wpt-item"].id()];
        assert_eq!(frame.x, expected_x, "{item_style} x");
        assert_eq!(frame.y, 2.0, "{item_style} y");
        assert_eq!(frame.width, expected_width, "{item_style} width");
        assert_eq!(frame.height, 25.0, "{item_style} height");
    }
}

#[path = "c11_4_track_sizing_areas_tests/auto_repeat_explicit_neighbors.rs"]
mod auto_repeat_explicit_neighbors;
#[path = "c11_4_track_sizing_areas_tests/fixed_repeat_line_names.rs"]
mod fixed_repeat_line_names;
