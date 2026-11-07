use super::*;

const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-max-content-precomparison.html"
);
const EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-intrinsic-auto-rows-max-content-precomparison-2026-11-07/expected-values.json"
);

#[test]
fn parsed_root_html_max_content_auto_rows_measure_text_and_preserve_overflow() {
    let expected: Value = serde_json::from_str(EXPECTED)
        .expect("max-content auto-row expected values should be valid JSON");
    let (session, layout) = run(SOURCE);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        cascade.state,
        crate::session::RuntimeUaCascadeState::Ready,
        "cascade error={:?}",
        cascade.error
    );
    let grids = cascade
        .completed
        .as_ref()
        .into_iter()
        .flat_map(|completed| completed.roots.iter())
        .flat_map(|root| root.styles.elements.iter())
        .filter_map(|element| element.grid_layout.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(grids.len(), 1, "the fixture should contain one Grid owner");
    assert_eq!(
        grids[0].auto_rows,
        spinon_style::ComputedCssGridAutoRows::TrackList(vec![
            spinon_style::ComputedCssGridTrackSize::MaxContent
        ]),
        "Root HTML cascade should preserve the requested typed track"
    );
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; computed grid={:?}",
        layout.error,
        grids
    );
    let completed = layout.completed.as_ref().expect("layout should complete");
    assert_eq!(
        completed.key,
        cascade
            .completed
            .as_ref()
            .expect("cascade should complete")
            .key,
        "style and layout revisions should match"
    );
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML route should publish paint boxes");
    let grid = painted_frame(scene, OpaqueCssSrgb::new(57, 67, 79), "max-content Grid");
    assert_frame_values(grid, [0.0, 0.0, 40.0, 40.0], "max-content Grid");

    for (id, color) in [
        ("first", OpaqueCssSrgb::new(242, 102, 94)),
        ("wrapped-text", OpaqueCssSrgb::new(91, 168, 245)),
        ("last", OpaqueCssSrgb::new(85, 199, 145)),
    ] {
        let expected_frame = expected["items"][id]
            .as_array()
            .expect("expected frame should be an array")
            .iter()
            .map(|component| {
                component
                    .as_f64()
                    .expect("expected frame component should be numeric") as f32
            })
            .collect::<Vec<_>>();
        let expected_frame: [f32; 4] = expected_frame
            .try_into()
            .expect("expected frame should have four components");
        let frame = painted_frame(scene, color, id);
        assert_frame_values(frame, expected_frame, id);
    }

    let text_item_node = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(color)
                if color == OpaqueCssSrgb::new(91, 168, 245))
        })
        .expect("wrapped text item should remain painted")
        .node_id();
    let text = scene
        .texts()
        .iter()
        .find(|text| text.node_id() == text_item_node)
        .expect("wrapped text item should publish text paint");
    assert_eq!(scene.texts().len(), 1, "one text owner is supported");
    assert_eq!(text.origin_x_css_px(), 0.0);
    assert_eq!(text.origin_y_css_px(), 10.0);
    assert_eq!(text.glyphs().len(), 6, "six visible A glyphs");
    let baselines = text
        .glyphs()
        .iter()
        .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(baselines.len(), 2, "text should occupy two lines");
    assert_eq!(text.source().generation().get(), completed.key.generation);
    assert_eq!(
        text.source().document_revision().get(),
        completed.key.document_revision
    );
    assert_eq!(
        text.source().render_tree_revision().get(),
        completed.key.render_tree_revision
    );
    assert_eq!(text.source().style_revision(), completed.key.style_revision);

    let last = painted_frame(
        scene,
        OpaqueCssSrgb::new(85, 199, 145),
        "max-content last item",
    );
    let visible_bottom = last[1] + last[3];
    assert_eq!(
        visible_bottom,
        expected["visibleBottomPx"].as_f64().unwrap() as f32
    );
    assert!(
        visible_bottom > grid[1] + grid[3],
        "visible overflow should remain"
    );
}

#[test]
fn parsed_root_html_max_content_auto_rows_reject_atomically_and_recover_same_session() {
    let source = SOURCE.replace(
        "    </style>",
        "    @media (prefers-color-scheme: dark) { .grid { grid-auto-rows: min-content 12px; } }\n    </style>",
    );
    assert_ne!(source, SOURCE, "the recovery rule should be inserted");
    let (session, initial) = run(&source);
    assert_eq!(initial.state, crate::session::RuntimeLayoutState::Ready);
    assert!(initial.completed.is_some());

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Dark;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("dark environment should be applied");
    let rejected = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(rejected.state, crate::session::RuntimeLayoutState::Failed);
    assert_eq!(
        rejected
            .error
            .as_ref()
            .and_then(|failure| failure.property.as_deref()),
        Some("grid-auto-rows")
    );
    assert!(
        rejected.completed.is_none(),
        "rejected mixed tracks must not retain a publishable partial layout"
    );

    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("light environment should restore the supported track");
    let recovered = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        recovered.state,
        crate::session::RuntimeLayoutState::Ready,
        "max-content layout should recover in the same session: {:?}",
        recovered.error
    );
    let scene = recovered
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("recovery should publish the full render snapshot");
    assert_frame_values(
        painted_frame(
            scene,
            OpaqueCssSrgb::new(85, 199, 145),
            "recovered max-content last item",
        ),
        [0.0, 30.0, 40.0, 34.0],
        "recovered max-content last item",
    );
}

#[test]
fn parsed_root_html_max_content_auto_rows_rejects_gaps_and_intrinsic_column_mixes() {
    for (label, source) in [
        (
            "nonzero-row-gap",
            SOURCE.replace("gap: 0;", "row-gap: 1px; column-gap: 0;"),
        ),
        (
            "mixed-max-content-track-list",
            SOURCE.replace(
                "grid-auto-rows: max-content;",
                "grid-auto-rows: max-content min-content;",
            ),
        ),
        (
            "max-content-with-intrinsic-explicit-column",
            SOURCE.replace(
                "grid-template-columns: 40px;",
                "grid-template-columns: min-content;",
            ),
        ),
    ] {
        assert_ne!(source, SOURCE, "{label}: source mutation should match");
        let (session, layout) = run(&source);
        assert_eq!(
            session.ua_cascade_snapshot().state,
            crate::session::RuntimeUaCascadeState::Ready,
            "{label}: CSS-valid value should reach bounded adapter validation"
        );
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{label}"
        );
        assert_eq!(
            layout
                .error
                .as_ref()
                .and_then(|failure| failure.property.as_deref()),
            Some("grid-auto-rows"),
            "{label}: report the property whose shape is unsupported"
        );
        assert!(
            layout.completed.is_none(),
            "{label}: do not publish partial output"
        );
    }
}
