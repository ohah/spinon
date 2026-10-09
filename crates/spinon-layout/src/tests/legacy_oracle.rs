use super::*;
use std::collections::HashMap;
use std::ffi::CString;

#[allow(
    unexpected_cfgs,
    clippy::too_many_arguments,
    clippy::if_same_then_else
)]
#[rustfmt::skip]
#[path = "../../../../spikes/dynamic-tree/rust/tree.rs"]
mod legacy_tree;

extern "C" fn collect_legacy_frame(
    user_data: *mut std::ffi::c_void,
    id: i32,
    _tag: *const std::ffi::c_char,
    _text: *const std::ffi::c_char,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    let output = unsafe { &mut *user_data.cast::<BTreeMap<NodeId, crate::LayoutFrame>>() };
    output.insert(
        node_id(id as u64),
        crate::LayoutFrame {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        },
    );
}

struct LegacyTree(*mut legacy_tree::Tree);

impl Drop for LegacyTree {
    fn drop(&mut self) {
        unsafe { legacy_tree::boson_tree_free(self.0) };
    }
}

pub(super) fn legacy_frames(fixture: &Fixture) -> BTreeMap<NodeId, crate::LayoutFrame> {
    let tree = LegacyTree(legacy_tree::boson_tree_new());
    assert_eq!(fixture.viewport.width.fract(), 0.0);
    assert_eq!(fixture.viewport.height.fract(), 0.0);
    let mut parents = HashMap::<u64, (u64, i32)>::new();
    for parent in &fixture.nodes {
        for (order, &child) in parent.children.iter().enumerate() {
            parents.insert(child, (parent.id, order as i32));
        }
    }

    for node in &fixture.nodes {
        assert_eq!(
            node.style.direction, "ltr",
            "기존 엔진은 RTL을 지원하지 않습니다"
        );
        let (parent, order) = parents.get(&node.id).copied().unwrap_or((0, 0));
        let tag = CString::new(node.tag.as_str()).unwrap();
        assert_eq!(
            unsafe {
                legacy_tree::boson_tree_create(
                    tree.0,
                    node.id as i32,
                    parent as i32,
                    tag.as_ptr(),
                    order,
                )
            },
            0,
            "기존 엔진 fixture 노드 생성 실패: {}",
            node.id
        );
        let dimension_to_i32 = |dimension: &FixtureDimension| match dimension {
            FixtureDimension::Number(value) => {
                assert_eq!(
                    value.fract(),
                    0.0,
                    "기존 엔진 비교 fixture는 정수 길이만 사용합니다"
                );
                *value as i32
            }
            FixtureDimension::Keyword(keyword) if keyword == "auto" => -1,
            FixtureDimension::Keyword(keyword) => panic!("알 수 없는 fixture dimension: {keyword}"),
        };
        let padding = &node.style.padding;
        assert_eq!(padding.top, padding.right);
        assert_eq!(padding.top, padding.bottom);
        assert_eq!(padding.top, padding.left);
        assert_eq!(padding.top.fract(), 0.0);
        let gap = match node.style.flex_direction.as_str() {
            "row" => {
                assert_eq!(
                    node.style.row_gap, 0.0,
                    "기존 엔진 비교 fixture는 교차축 gap을 사용하지 않습니다"
                );
                node.style.column_gap
            }
            "column" => {
                assert_eq!(
                    node.style.column_gap, 0.0,
                    "기존 엔진 비교 fixture는 교차축 gap을 사용하지 않습니다"
                );
                node.style.row_gap
            }
            other => panic!("알 수 없는 fixture flexDirection: {other}"),
        };
        assert_eq!(gap.fract(), 0.0);
        assert_eq!(node.style.flex_grow.fract(), 0.0);
        assert_eq!(
            unsafe {
                legacy_tree::boson_tree_set_style(
                    tree.0,
                    node.id as i32,
                    dimension_to_i32(&node.style.width),
                    dimension_to_i32(&node.style.height),
                    padding.top as i32,
                    gap as i32,
                    node.style.flex_grow as i32,
                )
            },
            0,
            "기존 엔진 fixture 스타일 설정 실패: {}",
            node.id
        );
    }

    let mut output = BTreeMap::new();
    assert_eq!(
        unsafe {
            legacy_tree::boson_tree_layout(
                tree.0,
                fixture.viewport.width as i32,
                fixture.viewport.height as i32,
                collect_legacy_frame,
                (&mut output as *mut BTreeMap<NodeId, crate::LayoutFrame>).cast(),
            )
        },
        0
    );
    output
}
