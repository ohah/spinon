use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, TextParagraphDirection, WhiteSpaceMode};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_root_html_text_paint_cascade_with_stylesheets,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout_with_root_html_grid_text_profiles};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const AHEM: &[u8] = include_bytes!("../../../tests/fixtures/resources/c15/Ahem.ttf");

struct SceneSpec<'a> {
    grid_style: &'a str,
    short_style: &'a str,
    long_style: &'a str,
    long_tag: &'a str,
}

struct SceneHandles {
    grid: HostNodeHandle,
    items: [HostNodeHandle; 2],
}

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    scenes: Vec<SceneHandles>,
}

impl Fixture {
    fn new(root_style: &str, specs: &[SceneSpec<'_>]) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let scenes = specs
            .iter()
            .map(|_| SceneHandles {
                grid: document.reserve_node_handle().unwrap(),
                items: [
                    document.reserve_node_handle().unwrap(),
                    document.reserve_node_handle().unwrap(),
                ],
            })
            .collect::<Vec<_>>();
        let texts = scenes
            .iter()
            .map(|_| {
                [
                    document.reserve_node_handle().unwrap(),
                    document.reserve_node_handle().unwrap(),
                ]
            })
            .collect::<Vec<_>>();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_684).unwrap(), document.document_revision());
        create_element(&mut batch, root, HostParent::Root, "div", root_style);
        for ((spec, scene), text_nodes) in specs.iter().zip(&scenes).zip(texts) {
            create_element(
                &mut batch,
                scene.grid,
                HostParent::Node(root),
                "div",
                spec.grid_style,
            );
            for (index, (item, text)) in scene.items.iter().copied().zip(text_nodes).enumerate() {
                let tag = if index == 1 { spec.long_tag } else { "div" };
                let item_style = if index == 1 {
                    spec.long_style
                } else {
                    spec.short_style
                };
                create_element(
                    &mut batch,
                    item,
                    HostParent::Node(scene.grid),
                    tag,
                    item_style,
                );
                batch.push(DocumentOperation::CreateText {
                    node: text,
                    data: if index == 0 { "A" } else { "AAAA" }.into(),
                });
                batch.push(DocumentOperation::InsertBefore {
                    parent: HostParent::Node(item),
                    node: text,
                    before: None,
                });
            }
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            scenes,
        }
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let viewport = CssViewport {
            width_css_px: 400.0,
            height_css_px: 300.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let view = StyloDocumentView::new(snapshot.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        let computed_styles = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .map_err(StyleLayoutError::Cascade)?;
        let paragraph = ParagraphStyle {
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            white_space: WhiteSpaceMode::NoWrap,
            direction: TextParagraphDirection::Ltr,
            ..ParagraphStyle::default()
        };
        let paragraphs = self
            .scenes
            .iter()
            .flat_map(|scene| scene.items)
            .map(|item| (item.id(), paragraph.clone()))
            .collect::<BTreeMap<_, _>>();
        let mut text_engine = TextLayoutEngine::new_without_system_fonts();
        text_engine
            .register_font("Ahem", AHEM)
            .expect("the pinned Ahem font should register");
        compute_runtime_style_layout_with_root_html_grid_text_profiles(
            &snapshot,
            self.root,
            computed_styles,
            viewport,
            &paragraphs,
            &BTreeMap::new(),
            &text_engine,
        )
    }
}

fn create_element(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    parent: HostParent,
    tag: &str,
    style: &str,
) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: tag.to_owned(),
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

fn root_style(extra: &str) -> String {
    format!(
        "display:block;box-sizing:border-box;width:400px;height:300px;margin:0;padding:0;border:0;{extra}"
    )
}

fn grid_style(extra: &str) -> String {
    format!(
        "display:grid;box-sizing:content-box;width:70px;height:auto;grid-template-columns:10px 60px;grid-template-rows:auto;grid-auto-flow:row;gap:0;align-items:start;justify-items:start;align-content:normal;justify-content:normal;overflow:visible;margin:0;padding:0;border:0;{extra}"
    )
}

fn item_style(dimensions: &str, ratio: &str) -> String {
    format!(
        "display:block;box-sizing:content-box;margin:0;padding:0;border:0;overflow:visible;font-family:Ahem;font-size:10px;line-height:10px;white-space:nowrap;{dimensions}{ratio}"
    )
}

fn scene<'a>(grid_style: &'a str, short_style: &'a str, long_style: &'a str) -> SceneSpec<'a> {
    SceneSpec {
        grid_style,
        short_style,
        long_style,
        long_tag: "div",
    }
}

fn assert_frame(
    output: &crate::StyleLayoutOutput,
    node: HostNodeHandle,
    expected: [f32; 4],
    label: &str,
) {
    let frame = output.layout.frames[&node.id()];
    let actual = [frame.x, frame.y, frame.width, frame.height];
    assert_eq!(actual, expected, "{label}");
}

#[test]
fn parsed_root_html_direct_text_grid_items_transfer_numeric_ratio_on_either_axis() {
    let grid = grid_style("");
    let short = item_style("width:auto;height:auto;", "");
    let width_ratio = item_style("width:60px;height:auto;", "aspect-ratio:2 / 1;");
    let width_control = item_style("width:60px;height:auto;", "");
    let height_ratio = item_style("width:auto;height:30px;", "aspect-ratio:2 / 1;");
    let height_control = item_style("width:auto;height:30px;", "");
    let specs = [
        scene(&grid, &short, &width_ratio),
        scene(&grid, &short, &width_control),
        scene(&grid, &short, &height_ratio),
        scene(&grid, &short, &height_control),
    ];
    let fixture = Fixture::new(&root_style(""), &specs);
    let output = fixture
        .compute()
        .expect("the four fixed ratio scenes should project");
    let expected = [
        (
            [0.0, 0.0, 70.0, 30.0],
            [0.0, 0.0, 10.0, 10.0],
            [10.0, 0.0, 60.0, 30.0],
        ),
        (
            [0.0, 30.0, 70.0, 10.0],
            [0.0, 30.0, 10.0, 10.0],
            [10.0, 30.0, 60.0, 10.0],
        ),
        (
            [0.0, 40.0, 70.0, 30.0],
            [0.0, 40.0, 10.0, 10.0],
            [10.0, 40.0, 60.0, 30.0],
        ),
        (
            [0.0, 70.0, 70.0, 30.0],
            [0.0, 70.0, 10.0, 10.0],
            [10.0, 70.0, 40.0, 30.0],
        ),
    ];
    for (index, (scene, (grid, short, long))) in fixture.scenes.iter().zip(expected).enumerate() {
        assert_frame(&output, scene.grid, grid, &format!("scene {index} Grid"));
        assert_frame(
            &output,
            scene.items[0],
            short,
            &format!("scene {index} short item"),
        );
        assert_frame(
            &output,
            scene.items[1],
            long,
            &format!("scene {index} long item"),
        );
    }
}

#[test]
fn parsed_root_html_aspect_ratio_fails_closed_outside_the_fixed_owner_slice() {
    let grid = grid_style("");
    let short = item_style("width:auto;height:auto;", "");
    let width_control = item_style("width:60px;height:auto;", "");
    let zero_ratio = item_style("width:60px;height:auto;", "aspect-ratio:0 / 1;");
    let zero_ratio_fixture = Fixture::new(&root_style(""), &[scene(&grid, &short, &zero_ratio)]);
    assert!(matches!(
        zero_ratio_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    for (dimensions, ratio) in [
        ("width:60px;height:auto;", "1e-38 / 1"),
        ("width:auto;height:30px;", "1e38 / 1"),
        ("width:auto;height:80px;", "3 / 2"),
        ("width:auto;height:40px;", "1 / 2"),
    ] {
        let long = item_style(dimensions, &format!("aspect-ratio:{ratio};"));
        let fixture = Fixture::new(&root_style(""), &[scene(&grid, &short, &long)]);
        assert!(matches!(
            fixture.compute(),
            Err(StyleLayoutError::UnsupportedComputedValue {
                property: "aspect-ratio",
                ..
            })
        ));
    }

    let ratio = item_style("width:60px;height:auto;", "aspect-ratio:2 / 1;");
    let constrained = item_style(
        "width:60px;height:auto;max-height:40px;",
        "aspect-ratio:3 / 2;",
    );
    let constrained_fixture = Fixture::new(&root_style(""), &[scene(&grid, &short, &constrained)]);
    assert!(matches!(
        constrained_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let short_ratio = item_style("width:auto;height:auto;", "aspect-ratio:2 / 1;");
    let multiple_fixture = Fixture::new(&root_style(""), &[scene(&grid, &short_ratio, &ratio)]);
    assert!(matches!(
        multiple_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let named_area_grid = grid_style("grid-template-areas:'a b';");
    let named_area_fixture =
        Fixture::new(&root_style(""), &[scene(&named_area_grid, &short, &ratio)]);
    assert!(matches!(
        named_area_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let grid_ratio = grid_style("aspect-ratio:2 / 1;");
    let grid_ratio_fixture = Fixture::new(&root_style(""), &[scene(&grid_ratio, &short, &ratio)]);
    assert!(matches!(
        grid_ratio_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let block_root_ratio = root_style("aspect-ratio:2 / 1;");
    let no_ratio = item_style("width:60px;height:auto;", "");
    let root_ratio_fixture = Fixture::new(&block_root_ratio, &[scene(&grid, &short, &no_ratio)]);
    assert!(matches!(
        root_ratio_fixture.compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            property: "aspect-ratio",
            ..
        })
    ));

    let non_candidate_auto_row_grid =
        grid_style("width:71px;align-items:normal;justify-items:normal;");
    let non_candidate_fixture = Fixture::new(
        &root_style(""),
        &[scene(&non_candidate_auto_row_grid, &short, &width_control)],
    );
    assert!(matches!(
        non_candidate_fixture.compute(),
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::UnsupportedGridContainerStyle {
                property: "height",
                ..
            }
        ))
    ));

    for alignment in ["align-content:start;", "justify-content:start;"] {
        let non_default_grid = grid_style(alignment);
        let fixture = Fixture::new(&root_style(""), &[scene(&non_default_grid, &short, &ratio)]);
        assert!(matches!(
            fixture.compute(),
            Err(StyleLayoutError::UnsupportedComputedValue {
                property: "aspect-ratio",
                ..
            })
        ));
    }

    for alignment in ["align-self:start;", "justify-self:start;"] {
        let aligned_short = item_style(&format!("width:auto;height:auto;{alignment}"), "");
        let aligned_long = item_style(
            &format!("width:60px;height:auto;{alignment}"),
            "aspect-ratio:2 / 1;",
        );
        for (short_style, long_style) in [(&aligned_short, &ratio), (&short, &aligned_long)] {
            let fixture = Fixture::new(&root_style(""), &[scene(&grid, short_style, long_style)]);
            assert!(matches!(
                fixture.compute(),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    property: "aspect-ratio",
                    ..
                })
            ));
        }
    }
}

#[path = "c11_1_root_html_direct_text_aspect_ratio_tests/values.rs"]
mod values;
