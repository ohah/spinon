use super::super::{axis_bounds, scene_for_viewport, target};
use super::{
    assert_frame_matches_observation, assert_hit_identities_match_chrome,
    assert_hit_ranges_match_observation, runtime_matrix_chrome_case,
};
#[test]
fn visible_axis_and_clip_margin_edges_match_chrome_frame_and_full_hit_ranges() {
    let cases = [
        (
            "visible",
            "position:absolute;left:20px;top:84px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow:visible;background-color:#333333",
            None,
            None,
            None,
            None,
        ),
        (
            "clip-x",
            "position:absolute;left:218px;top:84px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow-x:clip;overflow-y:visible;background-color:#333333",
            Some((222.0, 334.0)),
            None,
            Some((218.0, 338.0)),
            None,
        ),
        (
            "clip-both",
            "position:absolute;left:20px;top:252px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow:clip;background-color:#333333",
            Some((24.0, 136.0)),
            Some((256.0, 344.0)),
            Some((20.0, 140.0)),
            Some((252.0, 348.0)),
        ),
        (
            "clip-margin-padding",
            "position:absolute;left:218px;top:252px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow:clip;overflow-clip-margin:padding-box 8px;background-color:#333333",
            Some((214.0, 342.0)),
            Some((248.0, 352.0)),
            Some((214.0, 342.0)),
            Some((248.0, 352.0)),
        ),
        (
            "clip-margin-content",
            "position:absolute;left:416px;top:252px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid;overflow:clip;overflow-clip-margin:content-box 8px;background-color:#333333",
            Some((422.0, 530.0)),
            Some((258.0, 342.0)),
            Some((416.0, 536.0)),
            Some((252.0, 348.0)),
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
        let clipped_target = target(&scene, nodes[2]);
        assert_eq!(
            axis_bounds(clipped_target.clip_css_px(), true),
            paint_x,
            "{case_id} paint x"
        );
        assert_eq!(
            axis_bounds(clipped_target.clip_css_px(), false),
            paint_y,
            "{case_id} paint y"
        );
        assert_eq!(
            axis_bounds(clipped_target.hit_test_clip_css_px(), true),
            hit_x,
            "{case_id} hit x"
        );
        assert_eq!(
            axis_bounds(clipped_target.hit_test_clip_css_px(), false),
            hit_y,
            "{case_id} hit y"
        );
        let oracle = runtime_matrix_chrome_case(case_id);
        assert_frame_matches_observation(&scene, nodes[2], case_id, &oracle);
        assert_hit_ranges_match_observation(&scene, nodes[2], case_id, &oracle);
        assert_hit_identities_match_chrome(&scene, nodes[2], nodes[0], nodes[1], None, case_id);
    }
}
