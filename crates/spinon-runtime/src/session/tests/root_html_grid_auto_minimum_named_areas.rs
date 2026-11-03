use super::RuntimeSession;
use std::time::Duration;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-auto-minimum-named-area.html"
);

#[derive(Clone, Copy, Debug)]
enum Axis {
    Columns,
    Rows,
}

#[derive(Clone, Copy, Debug)]
enum Minimum {
    AutoOne,
    ZeroOne,
    AutoZero,
}

#[derive(Clone, Copy, Debug)]
struct Scene {
    axis: Axis,
    minimum: Minimum,
    root: [f32; 4],
    header: [f32; 4],
    side: [f32; 4],
    content: [f32; 4],
    overflow: f32,
}

const SCENES: [Scene; 6] = [
    Scene {
        axis: Axis::Columns,
        minimum: Minimum::AutoOne,
        root: [0.0, 0.0, 104.0, 44.0],
        header: [2.0, 2.0, 120.0, 20.0],
        side: [2.0, 22.0, 80.0, 20.0],
        content: [82.0, 22.0, 40.0, 20.0],
        overflow: 18.0,
    },
    Scene {
        axis: Axis::Columns,
        minimum: Minimum::ZeroOne,
        root: [0.0, 0.0, 104.0, 44.0],
        header: [2.0, 2.0, 100.0, 20.0],
        side: [2.0, 22.0, 80.0, 20.0],
        content: [62.0, 22.0, 40.0, 20.0],
        overflow: 0.0,
    },
    Scene {
        axis: Axis::Columns,
        minimum: Minimum::AutoZero,
        root: [0.0, 0.0, 104.0, 44.0],
        header: [2.0, 2.0, 120.0, 20.0],
        side: [2.0, 22.0, 80.0, 20.0],
        content: [82.0, 22.0, 40.0, 20.0],
        overflow: 18.0,
    },
    Scene {
        axis: Axis::Rows,
        minimum: Minimum::AutoOne,
        root: [0.0, 0.0, 84.0, 104.0],
        header: [2.0, 2.0, 80.0, 80.0],
        side: [2.0, 82.0, 40.0, 40.0],
        content: [42.0, 82.0, 40.0, 40.0],
        overflow: 18.0,
    },
    Scene {
        axis: Axis::Rows,
        minimum: Minimum::ZeroOne,
        root: [0.0, 0.0, 84.0, 104.0],
        header: [2.0, 2.0, 80.0, 80.0],
        side: [2.0, 62.0, 40.0, 40.0],
        content: [42.0, 62.0, 40.0, 40.0],
        overflow: 0.0,
    },
    Scene {
        axis: Axis::Rows,
        minimum: Minimum::AutoZero,
        root: [0.0, 0.0, 84.0, 104.0],
        header: [2.0, 2.0, 80.0, 80.0],
        side: [2.0, 82.0, 40.0, 40.0],
        content: [42.0, 82.0, 40.0, 40.0],
        overflow: 18.0,
    },
];

fn styles(scene: Scene) -> (String, [String; 3]) {
    let (width, height, columns, rows, item_sizes) = match (scene.axis, scene.minimum) {
        (Axis::Columns, Minimum::AutoOne) => (
            "100px",
            "40px",
            "minmax(auto,1fr) 40px",
            "20px 20px",
            [("auto", "20px"), ("80px", "20px"), ("40px", "20px")],
        ),
        (Axis::Columns, Minimum::ZeroOne) => (
            "100px",
            "40px",
            "minmax(0,1fr) 40px",
            "20px 20px",
            [("auto", "20px"), ("80px", "20px"), ("40px", "20px")],
        ),
        (Axis::Columns, Minimum::AutoZero) => (
            "100px",
            "40px",
            "minmax(auto,0fr) 40px",
            "20px 20px",
            [("auto", "20px"), ("80px", "20px"), ("40px", "20px")],
        ),
        (Axis::Rows, Minimum::AutoOne) => (
            "80px",
            "100px",
            "40px 40px",
            "minmax(auto,1fr) 40px",
            [("80px", "80px"), ("40px", "40px"), ("40px", "40px")],
        ),
        (Axis::Rows, Minimum::ZeroOne) => (
            "80px",
            "100px",
            "40px 40px",
            "minmax(0,1fr) 40px",
            [("80px", "80px"), ("40px", "40px"), ("40px", "40px")],
        ),
        (Axis::Rows, Minimum::AutoZero) => (
            "80px",
            "100px",
            "40px 40px",
            "minmax(auto,0fr) 40px",
            [("80px", "80px"), ("40px", "40px"), ("40px", "40px")],
        ),
    };
    let grid = format!(
        "display:grid;box-sizing:content-box;width:{width};height:{height};grid-template-columns:{columns};grid-template-rows:{rows};grid-template-areas:'header header' 'side content';gap:0;margin:0;padding:0;border-width:2px;border-style:solid;overflow:visible;background-color:#eeeeee;min-width:auto;min-height:auto;max-width:none;max-height:none"
    );
    let item = |id: &str, size: (&str, &str), color: &str| {
        format!(
            "display:block;box-sizing:content-box;width:{};height:{};min-width:auto;min-height:auto;max-width:none;max-height:none;grid-area:{id};margin:0;padding:0;border-width:0;overflow:visible;background-color:{color}",
            size.0, size.1
        )
    };
    (
        grid,
        [
            item("header", item_sizes[0], "#f4c95d"),
            item("side", item_sizes[1], "#78c59c"),
            item("content", item_sizes[2], "#70a8d8"),
        ],
    )
}

fn source_with_styles(grid: &str, items: &[String; 3]) -> String {
    let mut source = SOURCE.to_owned();
    for (id, style) in ["grid", "header", "side", "content"]
        .into_iter()
        .zip(std::iter::once(grid).chain(items.iter().map(String::as_str)))
    {
        let marker = format!("id=\"{id}\" style=\"");
        let start = source
            .find(&marker)
            .unwrap_or_else(|| panic!("fixture is missing {id} inline style"))
            + marker.len();
        let end = start
            + source[start..]
                .find('"')
                .unwrap_or_else(|| panic!("fixture {id} inline style is not closed"));
        source.replace_range(start..end, style);
    }
    source
}

fn source_for_scene(scene: Scene) -> String {
    let (grid, items) = styles(scene);
    source_with_styles(&grid, &items)
}

fn layout_for_source(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("the named-area fixture should initialize the default Root HTML runtime");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("Root HTML metadata should configure CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    (session, layout)
}

fn color_frames(
    scene: &spinon_render::RuntimeRenderSnapshot,
    color: spinon_render::OpaqueCssSrgb,
) -> Vec<[f32; 4]> {
    scene
        .boxes()
        .iter()
        .filter_map(|render_box| match render_box.paint() {
            spinon_render::RuntimePaint::Opaque(actual) if actual == color => {
                let frame = render_box.frame_css_px();
                Some([frame.x(), frame.y(), frame.width(), frame.height()])
            }
            _ => None,
        })
        .collect()
}

fn assert_frame(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.02,
            "{label} frame component {component}: actual={actual:?}, expected={expected:?} ± 0.02"
        );
    }
}

#[test]
fn root_html_named_area_auto_minimum_scenes_publish_geometry_and_overflow() {
    use spinon_render::OpaqueCssSrgb;

    for scene in SCENES {
        let source = source_for_scene(scene);
        let (session, layout) = layout_for_source(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene:?} failed: {layout:?}; cascade={:?}",
            session.ua_cascade_snapshot()
        );
        let completed = layout
            .completed
            .expect("a supported named-area scene should publish a complete layout");
        let cascade = session.ua_cascade_snapshot();
        let cascade_completed = cascade
            .completed
            .expect("the fixture should publish computed styles");
        assert_eq!(completed.key, cascade_completed.key);

        let render = completed
            .render_snapshot
            .as_ref()
            .expect("default Root HTML layout should publish paint");
        let root = color_frames(render, OpaqueCssSrgb::new(238, 238, 238));
        let header = color_frames(render, OpaqueCssSrgb::new(244, 201, 93));
        let side = color_frames(render, OpaqueCssSrgb::new(120, 197, 156));
        let content = color_frames(render, OpaqueCssSrgb::new(112, 168, 216));
        assert_eq!(root.len(), 1, "scene {scene:?}: {root:?}");
        assert_eq!(header.len(), 1, "scene {scene:?}: {header:?}");
        assert_eq!(side.len(), 1, "scene {scene:?}: {side:?}");
        assert_eq!(content.len(), 1, "scene {scene:?}: {content:?}");
        assert_frame(root[0], scene.root, "grid");
        assert_frame(header[0], scene.header, "header");
        assert_frame(side[0], scene.side, "side");
        assert_frame(content[0], scene.content, "content");
        let measured_overflow = match scene.axis {
            Axis::Columns => (content[0][0] + content[0][2] - root[0][0] - root[0][2]).max(0.0),
            Axis::Rows => (content[0][1] + content[0][3] - root[0][1] - root[0][3]).max(0.0),
        };
        assert!(
            (measured_overflow - scene.overflow).abs() <= 0.02,
            "scene {scene:?} outer-border overflow: actual={measured_overflow}"
        );
    }
}

#[test]
fn root_html_named_area_unsupported_revision_does_not_publish_partial_layout() {
    let (grid, items) = styles(SCENES[0]);
    let invalid_grid = grid.replace("minmax(auto,1fr)", "minmax(auto,40px)");
    let source = source_with_styles(&invalid_grid, &items);
    let (session, layout) = layout_for_source(&source);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(layout.state, crate::session::RuntimeLayoutState::Failed);
    assert!(layout.completed.is_none());
    assert_eq!(cascade.state, super::RuntimeUaCascadeState::Ready);
    let requested = cascade
        .completed
        .expect("the unsupported grid input should still compute styles")
        .key;
    assert_eq!(layout.requested, Some(requested));
}
