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
        assert_eq!(scene.tracks.len(), 4);
        assert_eq!(scene.tracks[3], 0.0);
    }
    assert_eq!(scenes[0].tracks, [25.0, 36.0, 36.0, 0.0]);
    assert_eq!(scenes[1].tracks, scenes[0].tracks);
    assert_eq!(
        scenes[2].tracks,
        [158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0]
    );
    assert_eq!(scenes[3].tracks, scenes[2].tracks);
}

#[test]
fn exposes_four_direct_nonspanning_text_image_intrinsic_scenes() {
    let scenes = SCENES
        .iter()
        .filter(|scene| scene.grid != SceneGrid::SpanningMultiTrack)
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
    assert_eq!(scenes[0].tracks, [50.0]);
    assert_eq!(scenes[1].tracks, scenes[0].tracks);
    assert_eq!(scenes[2].tracks, [80.0]);
    assert_eq!(scenes[3].tracks, scenes[2].tracks);

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
