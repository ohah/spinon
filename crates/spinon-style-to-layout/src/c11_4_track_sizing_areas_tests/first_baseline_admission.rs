use super::{Fixture, StyleLayoutError};

#[test]
fn ordinary_track_sizing_projection_keeps_grid_first_baseline_closed() {
    let fixture = Fixture::new(
        "display:grid;box-sizing:border-box;width:max-content;height:auto;grid-template-columns:auto auto;grid-template-rows:auto;align-items:first baseline;margin:0;padding:0;border:0",
        &[
            (
                "small",
                "display:block;box-sizing:border-box;width:auto;height:auto;font-size:10px;line-height:10px;align-self:auto;margin:0;padding:0;border:0",
            ),
            (
                "large",
                "display:block;box-sizing:border-box;width:auto;height:auto;font-size:20px;line-height:20px;align-self:auto;margin:0;padding:0;border:0",
            ),
        ],
    );

    assert!(matches!(
        fixture.try_compute(),
        Err(StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "align-items",
            value,
        }) if node == fixture.root.id() && matches!(value.as_str(), "baseline" | "first baseline")
    ));
}
