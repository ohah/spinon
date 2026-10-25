const AHEM: &[u8] = include_bytes!("../../../../../../../tests/fixtures/resources/c15/Ahem.ttf");

#[path = "orthogonal_rows/assertions.rs"]
mod assertions;
#[path = "orthogonal_rows/scene.rs"]
mod scene;
#[path = "orthogonal_rows/snapshot.rs"]
mod snapshot;

use assertions::assert_runtime_scene;
use scene::SCENES;

#[test]
fn orthogonal_direct_text_image_rows_match_all_runtime_scenes() {
    assert_eq!(SCENES.len(), 4);
    for scene in SCENES {
        assert_runtime_scene(scene);
    }
}
