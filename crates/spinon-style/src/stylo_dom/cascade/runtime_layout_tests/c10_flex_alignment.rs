use super::super::CssViewport;
use super::view_for_inline_style;

#[test]
fn c1033_new_alignment_longhands_stay_out_of_runtime_block_profiles() {
    let (view, node) = view_for_inline_style(
        "display:block;align-self:center;align-content:end;justify-items:center;justify-self:end;place-content:center",
    );
    let flex = super::super::compute_runtime_flex_layout_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    for property in [
        "align-self",
        "align-content",
        "justify-items",
        "justify-self",
    ] {
        assert!(
            flex.elements[0].properties.contains_key(property),
            "Runtime Flex profile에서 {property}를 계산해야 합니다"
        );
    }

    let block = super::super::compute_runtime_block_paint_cascade(
        &view,
        CssViewport::C04_FIXTURE,
        Default::default(),
    )
    .unwrap();
    for property in [
        "align-self",
        "align-content",
        "justify-items",
        "justify-self",
    ] {
        assert!(
            !block.elements[0].properties.contains_key(property),
            "새 Flex 전용 {property}가 Block snapshot에 유입되면 안 됩니다"
        );
    }
    assert_eq!(
        super::super::first_unsupported_runtime_block_paint_inline_property(&view),
        Some((node, "align-self".to_owned()))
    );
}
