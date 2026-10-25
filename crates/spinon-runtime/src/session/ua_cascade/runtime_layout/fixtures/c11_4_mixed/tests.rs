use super::{ItemKind, SCENES, SceneGrid};

#[test]
fn exposes_four_fixed_mixed_text_image_multispan_scenes_and_tracks() {
    let scenes = SCENES
        .iter()
        .filter(|scene| scene.grid == SceneGrid::SpanningMultiTrack)
        .collect::<Vec<_>>();
    assert_eq!(scenes.len(), 4);
    for (index, scene) in scenes.iter().enumerate() {
        assert!(scenes[..index].iter().all(|prior| prior.id != scene.id));
        assert!(matches!(scene.items.len(), 2 | 3));
        assert_eq!(
            scene
                .items
                .iter()
                .filter(|item| matches!(item.kind, ItemKind::Text(_)))
                .count(),
            if scene.items.len() == 2 { 1 } else { 2 }
        );
        assert_eq!(
            scene
                .items
                .iter()
                .filter(|item| matches!(item.kind, ItemKind::Image { .. }))
                .count(),
            1
        );
        assert_eq!(scene.columns.len(), 4);
        assert_eq!(scene.columns[3], 0.0);
        assert_eq!(scene.rows, [60.0, 60.0, 60.0]);
    }
    assert_eq!(scenes[0].columns, [25.0, 36.0, 36.0, 0.0]);
    assert_eq!(scenes[1].columns, scenes[0].columns);
    assert_eq!(
        scenes[2].columns,
        [158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0]
    );
    assert_eq!(scenes[3].columns, scenes[2].columns);
}

#[test]
fn exposes_four_direct_nonspanning_text_image_intrinsic_scenes() {
    let scenes = SCENES
        .iter()
        .filter(|scene| {
            matches!(
                scene.grid,
                SceneGrid::DirectMinContent | SceneGrid::DirectMaxContent
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(scenes.len(), 4);
    assert_eq!(
        scenes.iter().map(|scene| scene.id).collect::<Vec<_>>(),
        [
            "direct-min-content-forward",
            "direct-min-content-reversed",
            "direct-max-content-forward",
            "direct-max-content-reversed",
        ]
    );
    for (index, scene) in scenes.iter().enumerate() {
        assert!(scenes[..index].iter().all(|prior| prior.id != scene.id));
        assert_eq!(scene.items.len(), 2);
        assert!(scene.items.iter().all(|item| item.column_start == 1));
        assert!(scene.items.iter().all(|item| item.column_span == 1));
        assert_eq!(
            scene
                .items
                .iter()
                .filter(|item| matches!(item.kind, ItemKind::Text("XX XXXXX")))
                .count(),
            1
        );
        assert_eq!(
            scene
                .items
                .iter()
                .filter(|item| matches!(item.kind, ItemKind::Image { height: None }))
                .count(),
            1
        );
    }
    assert_eq!(scenes[0].grid, SceneGrid::DirectMinContent);
    assert_eq!(scenes[1].grid, SceneGrid::DirectMinContent);
    assert_eq!(scenes[2].grid, SceneGrid::DirectMaxContent);
    assert_eq!(scenes[3].grid, SceneGrid::DirectMaxContent);
    assert_eq!(scenes[0].columns, [50.0]);
    assert_eq!(scenes[1].columns, scenes[0].columns);
    assert_eq!(scenes[2].columns, [80.0]);
    assert_eq!(scenes[3].columns, scenes[2].columns);
    for scene in &scenes {
        assert_eq!(scene.rows, [24.0, 24.0]);
    }

    for scene in &scenes {
        for item in scene.items {
            match item.kind {
                ItemKind::Text(_) => assert_eq!(item.row, 1),
                ItemKind::Image { .. } => assert_eq!(item.row, 2),
            }
        }
    }
    for scene in [&scenes[0], &scenes[2]] {
        assert!(matches!(scene.items[0].kind, ItemKind::Text(_)));
    }
    for scene in [&scenes[1], &scenes[3]] {
        assert!(matches!(scene.items[0].kind, ItemKind::Image { .. }));
    }
}

#[test]
fn exposes_four_fixed_mixed_text_image_intrinsic_row_scenes() {
    let scenes = SCENES
        .iter()
        .filter(|scene| {
            matches!(
                scene.grid,
                SceneGrid::DirectMinContentRow | SceneGrid::DirectMaxContentRow
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(scenes.len(), 4);

    let expected = [
        (
            "direct-row-min-content-forward",
            SceneGrid::DirectMinContentRow,
            [
                "c114-direct-row-min-content-forward-text",
                "c114-direct-row-min-content-forward-image",
            ],
        ),
        (
            "direct-row-min-content-reversed",
            SceneGrid::DirectMinContentRow,
            [
                "c114-direct-row-min-content-reversed-image",
                "c114-direct-row-min-content-reversed-text",
            ],
        ),
        (
            "direct-row-max-content-forward",
            SceneGrid::DirectMaxContentRow,
            [
                "c114-direct-row-max-content-forward-text",
                "c114-direct-row-max-content-forward-image",
            ],
        ),
        (
            "direct-row-max-content-reversed",
            SceneGrid::DirectMaxContentRow,
            [
                "c114-direct-row-max-content-reversed-image",
                "c114-direct-row-max-content-reversed-text",
            ],
        ),
    ];

    for (scene, (id, grid, item_ids)) in scenes.iter().zip(expected) {
        assert_eq!(scene.id, id);
        assert_eq!(scene.grid, grid);
        assert_eq!(scene.columns, [50.0, 24.0]);
        assert_eq!(scene.rows, [20.0]);
        assert_eq!(
            scene.items.iter().map(|item| item.id).collect::<Vec<_>>(),
            item_ids
        );
        assert_eq!(scene.items.len(), 2);

        let text = scene
            .items
            .iter()
            .find(|item| matches!(item.kind, ItemKind::Text("XX XXXXX")))
            .expect("each row scene has the fixed Ahem text item");
        assert_eq!(text.column_start, 1);
        assert_eq!(text.column_span, 1);
        assert_eq!(text.row, 1);

        let image = scene
            .items
            .iter()
            .find(|item| matches!(item.kind, ItemKind::Image { height: None }))
            .expect("each row scene has the natural-size PNG item");
        assert_eq!(image.column_start, 2);
        assert_eq!(image.column_span, 1);
        assert_eq!(image.row, 1);
    }
}
