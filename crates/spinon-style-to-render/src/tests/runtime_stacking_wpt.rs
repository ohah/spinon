use super::runtime_stacking::{fixture, ids};

const ROOT: &str =
    "display:block;position:relative;width:120px;height:100px;background-color:#ffffff";

#[test]
fn wpt_css2_z_index_stack_001_orders_distinct_positive_levels() {
    let scene = fixture(
        &[
            ROOT,
            "position:absolute;left:30px;top:30px;width:60px;height:60px;z-index:2;background-color:#ffa500",
            "position:absolute;left:60px;top:40px;width:60px;height:60px;z-index:3;background-color:#ffff00",
            "position:absolute;left:0;top:0;width:60px;height:60px;z-index:1;background-color:#0000ff",
        ],
        &[None, Some(0), Some(0), Some(0)],
    )
    .expect("WPT z-index-stack-001 subset");

    // WPT: blue(z=1), orange(z=2), yellow(z=3), bottom to top.
    assert_eq!(ids(&scene), [1, 4, 2, 3]);
}

#[test]
fn wpt_css2_z_index_abspos_003_paints_negative_child_after_context_background() {
    let scene = fixture(
        &[
            ROOT,
            "position:absolute;left:10px;top:10px;width:80px;height:70px;z-index:0;background-color:#ff0000",
            "position:absolute;left:0;top:0;width:80px;height:70px;z-index:-1;background-color:#008000",
        ],
        &[None, Some(0), Some(1)],
    )
    .expect("WPT z-index-abspos-003 subset");

    // WPT의 0 context 배경은 음수 자손을 가리지 않는다.
    assert_eq!(ids(&scene), [1, 2, 3]);
}

#[test]
fn wpt_flexbox_items_as_stacking_contexts_keeps_descendant_inside_item() {
    let scene = fixture(
        &[
            "display:flex;position:relative;width:120px;height:100px;align-items:flex-start;background-color:#ffffff",
            "position:static;flex:0 0 100px;height:60px;margin-right:-60px;z-index:1;background-color:#0000ff",
            "position:absolute;left:0;top:0;width:100px;height:60px;z-index:9999;background-color:#ffff00",
            "position:static;flex:0 0 100px;height:60px;z-index:2;background-color:#008000",
        ],
        &[None, Some(0), Some(1), Some(0)],
    )
    .expect("WPT flexbox-items-as-stacking-contexts subset");

    // 두 번째 Flex item(z=2)이 첫 번째 item의 큰 자손 z-index보다 위에 와야 한다.
    assert_eq!(ids(&scene), [1, 2, 3, 4]);
}

#[test]
fn wpt_flexbox_paint_ordering_002_uses_order_for_equal_z_index_ties() {
    let scene = fixture(
        &[
            "display:flex;position:relative;width:120px;height:100px;align-items:flex-start;background-color:#ffffff",
            "position:static;order:100;flex:0 0 100px;height:60px;margin-right:-60px;z-index:0;background-color:#ff0000",
            "position:static;order:0;flex:0 0 100px;height:60px;z-index:1;background-color:#0000ff",
            "position:static;order:-1;flex:0 0 100px;height:60px;z-index:1;background-color:#008000",
        ],
        &[None, Some(0), Some(0), Some(0)],
    )
    .expect("WPT flexbox-paint-ordering-002 subset");

    // z=1 항목끼리는 order 순서가 tie-break이고, 둘 다 z=0 항목보다 위다.
    assert_eq!(ids(&scene), [1, 2, 4, 3]);
}
