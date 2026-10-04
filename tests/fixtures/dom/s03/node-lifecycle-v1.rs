#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Node {
    pub id: u64,
    pub parent: Option<u64>,
    pub string_units: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TreeCase {
    pub name: &'static str,
    pub nodes: &'static [Node],
    pub host_root_children: &'static [u64],
    pub wrapper_roots: &'static [u64],
    pub external_roots: &'static [u64],
    pub expected_retained: &'static [u64],
}

const CONNECTED_NODES: &[Node] = &[
    Node {
        id: 1,
        parent: None,
        string_units: 10,
    },
    Node {
        id: 2,
        parent: Some(1),
        string_units: 6,
    },
    Node {
        id: 3,
        parent: Some(1),
        string_units: 7,
    },
];

const DETACHED_NODES: &[Node] = &[
    Node {
        id: 10,
        parent: None,
        string_units: 1,
    },
    Node {
        id: 11,
        parent: Some(10),
        string_units: 2,
    },
    Node {
        id: 12,
        parent: Some(10),
        string_units: 3,
    },
    Node {
        id: 13,
        parent: Some(12),
        string_units: 4,
    },
    Node {
        id: 20,
        parent: None,
        string_units: 5,
    },
];

pub const TREE_CASES: &[TreeCase] = &[
    TreeCase {
        name: "HostRoot 아래 자식과 후손은 모두 유지",
        nodes: CONNECTED_NODES,
        host_root_children: &[1],
        wrapper_roots: &[],
        external_roots: &[],
        expected_retained: &[1, 2, 3],
    },
    TreeCase {
        name: "root 없는 분리 연결 성분은 회수 가능",
        nodes: DETACHED_NODES,
        host_root_children: &[],
        wrapper_roots: &[],
        external_roots: &[],
        expected_retained: &[],
    },
    TreeCase {
        name: "후손 wrapper가 분리 연결 성분 전체를 유지",
        nodes: DETACHED_NODES,
        host_root_children: &[],
        wrapper_roots: &[13],
        external_roots: &[],
        expected_retained: &[10, 11, 12, 13],
    },
    TreeCase {
        name: "external root가 분리 연결 성분 전체를 유지",
        nodes: DETACHED_NODES,
        host_root_children: &[],
        wrapper_roots: &[],
        external_roots: &[12],
        expected_retained: &[10, 11, 12, 13],
    },
    TreeCase {
        name: "독립된 분리 연결 성분은 각각 회수",
        nodes: DETACHED_NODES,
        host_root_children: &[],
        wrapper_roots: &[13],
        external_roots: &[],
        expected_retained: &[10, 11, 12, 13],
    },
    TreeCase {
        name: "살아 있는 wrapper가 없어도 연결된 root 유지",
        nodes: CONNECTED_NODES,
        host_root_children: &[1],
        wrapper_roots: &[],
        external_roots: &[],
        expected_retained: &[1, 2, 3],
    },
];
