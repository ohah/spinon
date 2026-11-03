use super::RuntimeSession;
use std::time::Duration;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-auto-minimum-precomparison.html"
);

fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed Root HTML should initialize the default runtime");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("root metadata should configure the CSS environment");
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        let cascade = session.ua_cascade_snapshot();
        let layout = session.layout_snapshot();
        let layout_finished_for_request = layout.requested == cascade.requested
            && (layout.state == crate::session::RuntimeLayoutState::Failed
                || layout
                    .completed
                    .as_ref()
                    .is_some_and(|completed| Some(completed.key) == layout.requested));
        if cascade.state != super::RuntimeUaCascadeState::Pending && layout_finished_for_request {
            return (session, layout);
        }
        if std::time::Instant::now() >= deadline {
            return (session, layout);
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn frames_for_paint(
    scene: &spinon_render::RuntimeRenderSnapshot,
    expected: spinon_render::OpaqueCssSrgb,
) -> Vec<[f32; 4]> {
    scene
        .boxes()
        .iter()
        .filter_map(|render_box| match render_box.paint() {
            spinon_render::RuntimePaint::Opaque(color) if color == expected => {
                let frame = render_box.frame_css_px();
                Some([frame.x(), frame.y(), frame.width(), frame.height()])
            }
            _ => None,
        })
        .collect()
}

fn renamed_labels(source: &str) -> String {
    let mut renamed = source.to_owned();
    for (old, new) in [
        (".scene-label", ".label-v2"),
        (".scene", ".wrapper-v2"),
        (".root", ".grid-v2"),
        (".columns-auto", ".horizontal-auto-v2"),
        (".columns-zero", ".horizontal-zero-v2"),
        (".rows-auto", ".vertical-auto-v2"),
        (".rows-zero", ".vertical-zero-v2"),
        (".columns", ".horizontal-v2"),
        (".rows", ".vertical-v2"),
        (".item", ".cell-v2"),
        (".first", ".start-v2"),
        (".second", ".end-v2"),
        ("class=\"scene-label\"", "class=\"label-v2\""),
        (
            "class=\"scene columns\"",
            "class=\"wrapper-v2 horizontal-v2\"",
        ),
        ("class=\"scene rows\"", "class=\"wrapper-v2 vertical-v2\""),
        (
            "class=\"root columns-auto\"",
            "class=\"grid-v2 horizontal-auto-v2\"",
        ),
        (
            "class=\"root columns-zero\"",
            "class=\"grid-v2 horizontal-zero-v2\"",
        ),
        (
            "class=\"root rows-auto\"",
            "class=\"grid-v2 vertical-auto-v2\"",
        ),
        (
            "class=\"root rows-zero\"",
            "class=\"grid-v2 vertical-zero-v2\"",
        ),
        ("class=\"item first\"", "class=\"cell-v2 start-v2\""),
        ("class=\"item second\"", "class=\"cell-v2 end-v2\""),
        (" id=\"", " data-identity=\""),
    ] {
        renamed = renamed.replace(old, new);
    }
    renamed
}

fn assert_frame(actual: [f32; 4], expected: [f32; 4], name: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.02,
            "{name} frame component {component}: actual={actual:?}, expected={expected:?} ± 0.02"
        );
    }
}

fn assert_relative_frame(root: [f32; 4], item: [f32; 4], expected: [f32; 4], name: &str) {
    assert_frame(
        [item[0] - root[0], item[1] - root[1], item[2], item[3]],
        expected,
        name,
    );
}

fn assert_size(actual: [f32; 2], expected: [f32; 2], name: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.02,
            "{name} size component {component}: actual={actual:?}, expected={expected:?} ± 0.02"
        );
    }
}

#[test]
fn c11_1_root_html_auto_minimum_tracks_publish_border_and_overflow_geometry() {
    use spinon_render::OpaqueCssSrgb;

    let (session, layout) = run(SOURCE);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; cascade state={:?}; cascade error={:?}",
        layout.error,
        cascade.state,
        cascade.error
    );
    assert_eq!(cascade.state, super::RuntimeUaCascadeState::Ready);
    let completed = layout
        .completed
        .expect("automatic minimum scenes should publish through the default Root HTML route");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("the default route should publish a render scene");
    let roots = frames_for_paint(scene, OpaqueCssSrgb::new(238, 238, 238));
    let first_items = frames_for_paint(scene, OpaqueCssSrgb::new(23, 105, 170));
    let second_items = frames_for_paint(scene, OpaqueCssSrgb::new(227, 107, 44));
    assert_eq!(
        roots.len(),
        4,
        "one root box is expected per scene: {roots:?}"
    );
    assert_eq!(first_items.len(), 4, "one first item is expected per scene");
    assert_eq!(
        second_items.len(),
        4,
        "one second item is expected per scene"
    );

    let expected = [
        (
            [104.0, 44.0],
            [2.0, 2.0, 80.0, 40.0],
            [82.0, 2.0, 40.0, 40.0],
            18.0,
            "columns-auto-min",
        ),
        (
            [104.0, 44.0],
            [2.0, 2.0, 80.0, 40.0],
            [62.0, 2.0, 40.0, 40.0],
            0.0,
            "columns-zero-min-control",
        ),
        (
            [44.0, 104.0],
            [2.0, 2.0, 40.0, 80.0],
            [2.0, 82.0, 40.0, 40.0],
            18.0,
            "rows-auto-min",
        ),
        (
            [44.0, 104.0],
            [2.0, 2.0, 40.0, 80.0],
            [2.0, 62.0, 40.0, 40.0],
            0.0,
            "rows-zero-min-control",
        ),
    ];
    for (index, (root, first, second, outside, name)) in expected.into_iter().enumerate() {
        assert_size([roots[index][2], roots[index][3]], root, name);
        assert_relative_frame(roots[index], first_items[index], first, name);
        assert_relative_frame(roots[index], second_items[index], second, name);
        let right_or_bottom_overflow = if name.starts_with("columns") {
            second_items[index][0] + second_items[index][2] - roots[index][0] - roots[index][2]
        } else {
            second_items[index][1] + second_items[index][3] - roots[index][1] - roots[index][3]
        }
        .max(0.0);
        assert!(
            (right_or_bottom_overflow - outside).abs() <= 0.02,
            "{name} overflow beyond outer border: actual={right_or_bottom_overflow}, expected={outside}"
        );
    }

    let cascade_completed = cascade
        .completed
        .expect("computed Root HTML styles should remain available");
    assert_eq!(completed.key, cascade_completed.key);
}

#[test]
fn c11_1_root_html_unsupported_auto_minimum_track_does_not_publish_partial_layout() {
    let unsupported = SOURCE
        .replace("border-width: 2px;", "border-width: 0px;")
        .replace("minmax(auto, 1fr)", "minmax(auto, 40px)");
    let (session, layout) = run(&unsupported);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "valid but unsupported auto minimum track must fail layout closed; requested={:?}, cascade={:?}: {layout:#?}",
        layout.requested,
        cascade.requested
    );
    assert!(layout.completed.is_none());
    assert_eq!(cascade.state, super::RuntimeUaCascadeState::Ready);
    let cascade_completed = cascade
        .completed
        .expect("the unsupported layout track still has a computed cascade snapshot");
    assert_eq!(layout.requested, Some(cascade_completed.key));
}

#[test]
fn c11_1_root_html_auto_minimum_geometry_ignores_id_and_class_names() {
    use spinon_render::OpaqueCssSrgb;

    let (baseline_session, baseline_layout) = run(SOURCE);
    let renamed_source = renamed_labels(SOURCE);
    let (renamed_session, renamed_layout) = run(&renamed_source);
    for (session, layout) in [
        (&baseline_session, &baseline_layout),
        (&renamed_session, &renamed_layout),
    ] {
        assert_eq!(layout.state, crate::session::RuntimeLayoutState::Ready);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(cascade.state, super::RuntimeUaCascadeState::Ready);
        assert_eq!(
            layout.completed.as_ref().map(|completed| completed.key),
            cascade.completed.as_ref().map(|completed| completed.key)
        );
    }

    let baseline = baseline_layout
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("baseline Root HTML should publish a render snapshot");
    let renamed = renamed_layout
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("renamed Root HTML should publish a render snapshot");
    for color in [
        OpaqueCssSrgb::new(238, 238, 238),
        OpaqueCssSrgb::new(23, 105, 170),
        OpaqueCssSrgb::new(227, 107, 44),
    ] {
        assert_eq!(
            frames_for_paint(baseline, color),
            frames_for_paint(renamed, color)
        );
    }
}

#[test]
fn c11_1_root_html_zero_fraction_preserves_automatic_minimum_on_both_axes() {
    use spinon_render::OpaqueCssSrgb;

    let zero_fraction = SOURCE.replace("minmax(auto, 1fr)", "minmax(auto, 0fr)");
    let (session, layout) = run(&zero_fraction);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "zero-factor auto minimum should publish: error={:?}; cascade={:?}",
        layout.error,
        cascade.error
    );
    assert_eq!(cascade.state, super::RuntimeUaCascadeState::Ready);
    let completed = layout
        .completed
        .expect("zero-factor auto minimum should publish a complete layout");
    let cascade_completed = cascade
        .completed
        .expect("the zero-factor computed styles should remain available");
    assert_eq!(completed.key, cascade_completed.key);
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("the default Root HTML route should publish the zero-factor scene");
    let roots = frames_for_paint(scene, OpaqueCssSrgb::new(238, 238, 238));
    let first_items = frames_for_paint(scene, OpaqueCssSrgb::new(23, 105, 170));
    let second_items = frames_for_paint(scene, OpaqueCssSrgb::new(227, 107, 44));
    assert_eq!(roots.len(), 4);
    assert_eq!(first_items.len(), 4);
    assert_eq!(second_items.len(), 4);

    let expected = [
        (
            [104.0, 44.0],
            [2.0, 2.0, 80.0, 40.0],
            [82.0, 2.0, 40.0, 40.0],
            "columns-zero-fraction",
        ),
        (
            [104.0, 44.0],
            [2.0, 2.0, 80.0, 40.0],
            [62.0, 2.0, 40.0, 40.0],
            "columns-zero-min-control",
        ),
        (
            [44.0, 104.0],
            [2.0, 2.0, 40.0, 80.0],
            [2.0, 82.0, 40.0, 40.0],
            "rows-zero-fraction",
        ),
        (
            [44.0, 104.0],
            [2.0, 2.0, 40.0, 80.0],
            [2.0, 62.0, 40.0, 40.0],
            "rows-zero-min-control",
        ),
    ];
    for (index, (root, first, second, name)) in expected.into_iter().enumerate() {
        assert_size([roots[index][2], roots[index][3]], root, name);
        assert_relative_frame(roots[index], first_items[index], first, name);
        assert_relative_frame(roots[index], second_items[index], second, name);
        let overflow = if name.starts_with("columns") {
            second_items[index][0] + second_items[index][2] - roots[index][0] - roots[index][2]
        } else {
            second_items[index][1] + second_items[index][3] - roots[index][1] - roots[index][3]
        }
        .max(0.0);
        let expected_overflow = if name.ends_with("control") { 0.0 } else { 18.0 };
        assert!(
            (overflow - expected_overflow).abs() <= 0.02,
            "{name} overflow beyond the root border: actual={overflow}, expected={expected_overflow}"
        );
    }
}
