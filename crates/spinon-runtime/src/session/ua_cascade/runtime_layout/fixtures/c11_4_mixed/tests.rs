use super::{ItemKind, SCENES};

#[test]
fn exposes_four_fixed_mixed_text_image_scenes_and_independent_tracks() {
    assert_eq!(SCENES.len(), 4);
    for (index, scene) in SCENES.iter().enumerate() {
        assert!(SCENES[..index].iter().all(|prior| prior.id != scene.id));
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
    assert_eq!(SCENES[0].tracks, [25.0, 36.0, 36.0, 0.0]);
    assert_eq!(SCENES[1].tracks, SCENES[0].tracks);
    assert_eq!(
        SCENES[2].tracks,
        [158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0]
    );
    assert_eq!(SCENES[3].tracks, SCENES[2].tracks);
}
