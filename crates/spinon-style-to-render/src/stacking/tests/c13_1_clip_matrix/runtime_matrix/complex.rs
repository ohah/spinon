use super::super::scene_for_viewport;
use super::{
    assert_clip_bounds, assert_frame_matches_observation, assert_hit_identities_match_chrome,
    assert_hit_ranges_match_observation, runtime_matrix_chrome_case,
};
#[test]
fn mixed_scrollable_axis_normalization_ignores_clip_margin_in_paint_and_hit() {
    let cases = [
        (
            "mixed-visible-hidden",
            "position:absolute;left:20px;top:588px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow-x:visible;overflow-y:hidden;background-color:#333333",
            Some((24.0, 136.0)),
            Some((592.0, 680.0)),
            Some((20.0, 140.0)),
            Some((588.0, 684.0)),
        ),
        (
            "mixed-clip-auto",
            "position:absolute;left:218px;top:588px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow-x:clip;overflow-y:auto;overflow-clip-margin:border-box 8px;background-color:#333333",
            Some((222.0, 334.0)),
            Some((592.0, 680.0)),
            Some((218.0, 338.0)),
            Some((588.0, 684.0)),
        ),
    ];

    for (case_id, parent_style, paint_x, paint_y, hit_x, hit_y) in cases {
        let (scene, nodes) = scene_for_viewport(
            &[
                "display:block;position:relative;width:1280px;height:1100px;background-color:#111111",
                parent_style,
                "width:140px;height:120px;background-color:#ff0000",
            ],
            &[None, Some(0), Some(1)],
            1100.0,
        );
        assert_clip_bounds(&scene, nodes[2], case_id, paint_x, paint_y, hit_x, hit_y);
        let oracle = runtime_matrix_chrome_case(case_id);
        assert_frame_matches_observation(&scene, nodes[2], case_id, &oracle);
        assert_hit_ranges_match_observation(&scene, nodes[2], case_id, &oracle);
        assert_hit_identities_match_chrome(&scene, nodes[2], nodes[0], nodes[1], None, case_id);
    }
}

#[test]
fn positioned_clip_container_matches_chrome_for_contained_absolute_descendant() {
    let (scene, nodes) = scene_for_viewport(
        &[
            "display:block;position:relative;width:1280px;height:1100px;background-color:#111111",
            "position:relative;left:416px;top:776px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow:clip;background-color:#333333",
            "position:absolute;left:84px;top:42px;box-sizing:border-box;width:42px;height:30px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1)],
        1100.0,
    );
    let target_node = nodes[2];
    assert_clip_bounds(
        &scene,
        target_node,
        "positioned-contained",
        Some((420.0, 532.0)),
        Some((780.0, 868.0)),
        Some((420.0, 532.0)),
        Some((780.0, 868.0)),
    );
    let oracle = runtime_matrix_chrome_case("positioned-contained");
    assert_frame_matches_observation(&scene, target_node, "positioned-contained", &oracle);
    assert_hit_ranges_match_observation(&scene, target_node, "positioned-contained", &oracle);
    assert_hit_identities_match_chrome(
        &scene,
        target_node,
        nodes[0],
        nodes[1],
        None,
        "positioned-contained",
    );
}

#[test]
fn static_clip_is_skipped_when_absolute_descendant_uses_outer_containing_block() {
    let (scene, nodes) = scene_for_viewport(
        &[
            "display:block;position:relative;width:1280px;height:1100px;background-color:#111111",
            "position:relative;left:20px;top:904px;box-sizing:border-box;width:180px;height:150px;padding-top:20px",
            "position:static;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow:clip;background-color:#333333",
            "position:absolute;left:94px;top:64px;box-sizing:border-box;width:42px;height:30px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1), Some(2)],
        1100.0,
    );
    let target_node = nodes[3];
    assert_clip_bounds(
        &scene,
        target_node,
        "positioned-escape",
        None,
        None,
        None,
        None,
    );
    let oracle = runtime_matrix_chrome_case("positioned-escape");
    assert_frame_matches_observation(&scene, target_node, "positioned-escape", &oracle);
    assert_hit_ranges_match_observation(&scene, target_node, "positioned-escape", &oracle);
    assert_hit_identities_match_chrome(
        &scene,
        target_node,
        nodes[0],
        nodes[2],
        None,
        "positioned-escape",
    );
}

#[test]
fn nested_clip_axes_intersect_for_paint_and_hit_against_chrome() {
    let (scene, nodes) = scene_for_viewport(
        &[
            "display:block;position:relative;width:1280px;height:1100px;background-color:#111111",
            "position:absolute;left:218px;top:924px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow:clip;background-color:#333333",
            "position:absolute;left:24px;top:26px;box-sizing:border-box;width:78px;height:68px;padding:4px;border:3px solid;overflow-x:clip;overflow-y:visible;background-color:#aaaaaa",
            "width:140px;height:120px;background-color:#ff0000",
        ],
        &[None, Some(0), Some(1), Some(2)],
        1100.0,
    );
    let target_node = nodes[3];
    assert_clip_bounds(
        &scene,
        target_node,
        "nested-clip",
        Some((249.0, 321.0)),
        Some((928.0, 1016.0)),
        Some((246.0, 324.0)),
        Some((928.0, 1016.0)),
    );
    let oracle = runtime_matrix_chrome_case("nested-clip");
    assert_frame_matches_observation(&scene, target_node, "nested-clip", &oracle);
    assert_hit_ranges_match_observation(&scene, target_node, "nested-clip", &oracle);
    assert_hit_identities_match_chrome(
        &scene,
        target_node,
        nodes[0],
        nodes[1],
        Some(nodes[2]),
        "nested-clip",
    );
}
