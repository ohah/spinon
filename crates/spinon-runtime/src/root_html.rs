use crate::ROOT_HTML_SOURCE_MAX_BYTES;
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute, ParseOpts, QualName, parse_document};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;

mod environment;

pub use environment::RootHtmlEnvironmentSnapshot;
pub(crate) use environment::{
    RootHtmlEnvironmentMetadata, resolve_root_html_canvas_background, resolve_root_html_environment,
};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const MAX_ROOT_HTML_NODES: usize = 65_536;
const MAX_ROOT_HTML_DEPTH: usize = 256;
const MAX_ROOT_HTML_ATTRIBUTES: usize = 16_384;
const MAX_RECORDED_PARSE_ERRORS: usize = 128;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParsedRootHtml {
    pub(crate) nodes: Vec<ParsedNode>,
    pub(crate) parse_error_count: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParsedNode {
    pub(crate) kind: ParsedNodeKind,
    pub(crate) children: Vec<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ParsedNodeKind {
    Document,
    Element {
        namespace: String,
        local_name: String,
        attributes: Vec<(String, String)>,
    },
    Text(String),
    Ignored,
}

impl ParsedRootHtml {
    pub(crate) fn parse(source: &str) -> Result<Self, String> {
        if source.len() > ROOT_HTML_SOURCE_MAX_BYTES {
            return Err(format!(
                "루트 HTML은 UTF-8 기준 {ROOT_HTML_SOURCE_MAX_BYTES}바이트까지 허용합니다"
            ));
        }

        let sink = HtmlTreeSink::new();
        let parsed =
            parse_document(sink, ParseOpts::default()).one(StrTendril::from_slice(source))?;
        Ok(parsed)
    }

    pub(crate) fn root_children(&self) -> &[usize] {
        &self.nodes[0].children
    }

    pub(crate) fn html_element(&self) -> Option<usize> {
        self.root_children().iter().copied().find(|index| {
            matches!(
                &self.nodes[*index].kind,
                ParsedNodeKind::Element { namespace, local_name, .. }
                    if namespace == HTML_NAMESPACE && local_name == "html"
            )
        })
    }
}

struct HtmlTreeSink {
    tree: RefCell<Tree>,
}

struct Tree {
    nodes: Vec<TreeNode>,
    template_contents: BTreeMap<usize, usize>,
    parse_error_count: u64,
    quirks_mode: Option<QuirksMode>,
}

struct TreeNode {
    kind: TreeNodeKind,
    parent: Option<usize>,
    children: Vec<usize>,
}

enum TreeNodeKind {
    Document,
    Element {
        name: Box<QualName>,
        attributes: Vec<Attribute>,
    },
    Text(String),
    Ignored,
}

impl HtmlTreeSink {
    fn new() -> Self {
        Self {
            tree: RefCell::new(Tree {
                nodes: vec![TreeNode {
                    kind: TreeNodeKind::Document,
                    parent: None,
                    children: Vec::new(),
                }],
                template_contents: BTreeMap::new(),
                parse_error_count: 0,
                quirks_mode: None,
            }),
        }
    }

    fn create_node(&self, kind: TreeNodeKind) -> usize {
        let mut tree = self.tree.borrow_mut();
        let handle = tree.nodes.len();
        tree.nodes.push(TreeNode {
            kind,
            parent: None,
            children: Vec::new(),
        });
        handle
    }

    fn append_child(&self, parent: usize, child: usize) {
        let mut tree = self.tree.borrow_mut();
        detach(&mut tree, child);
        tree.nodes[parent].children.push(child);
        tree.nodes[child].parent = Some(parent);
    }

    fn append_text(&self, parent: usize, text: &str) {
        let mut tree = self.tree.borrow_mut();
        if let Some(last) = tree.nodes[parent].children.last().copied()
            && let TreeNodeKind::Text(existing) = &mut tree.nodes[last].kind
        {
            existing.push_str(text);
            return;
        }
        let handle = tree.nodes.len();
        tree.nodes.push(TreeNode {
            kind: TreeNodeKind::Text(text.to_owned()),
            parent: Some(parent),
            children: Vec::new(),
        });
        tree.nodes[parent].children.push(handle);
    }

    fn append_before(&self, sibling: usize, child: usize) {
        let mut tree = self.tree.borrow_mut();
        let Some(parent) = tree.nodes[sibling].parent else {
            return;
        };
        detach(&mut tree, child);
        let Some(position) = tree.nodes[parent]
            .children
            .iter()
            .position(|candidate| *candidate == sibling)
        else {
            return;
        };
        tree.nodes[parent].children.insert(position, child);
        tree.nodes[child].parent = Some(parent);
    }

    fn append_text_before(&self, sibling: usize, text: &str) {
        let mut tree = self.tree.borrow_mut();
        let Some(parent) = tree.nodes[sibling].parent else {
            return;
        };
        let Some(position) = tree.nodes[parent]
            .children
            .iter()
            .position(|candidate| *candidate == sibling)
        else {
            return;
        };
        if position > 0 {
            let previous = tree.nodes[parent].children[position - 1];
            if let TreeNodeKind::Text(existing) = &mut tree.nodes[previous].kind {
                existing.push_str(text);
                return;
            }
        }
        let handle = tree.nodes.len();
        tree.nodes.push(TreeNode {
            kind: TreeNodeKind::Text(text.to_owned()),
            parent: Some(parent),
            children: Vec::new(),
        });
        tree.nodes[parent].children.insert(position, handle);
    }
}

impl TreeSink for HtmlTreeSink {
    type Handle = usize;
    type Output = Result<ParsedRootHtml, String>;
    type ElemName<'a> = &'a QualName;

    fn finish(self) -> Self::Output {
        let tree = self.tree.into_inner();
        let Tree {
            nodes: raw_nodes,
            parse_error_count,
            quirks_mode,
            ..
        } = tree;
        let nodes = raw_nodes
            .into_iter()
            .map(|node| {
                let kind = match node.kind {
                    TreeNodeKind::Document => ParsedNodeKind::Document,
                    TreeNodeKind::Element { name, attributes } => {
                        let mut parsed_attributes = Vec::with_capacity(attributes.len());
                        for attribute in attributes {
                            if !attribute.name.ns.to_string().is_empty() {
                                return Err("루트 HTML의 namespace 속성은 아직 지원하지 않습니다"
                                    .to_owned());
                            }
                            parsed_attributes.push((
                                attribute.name.local.to_string(),
                                attribute.value.to_string(),
                            ));
                        }
                        ParsedNodeKind::Element {
                            namespace: name.ns.to_string(),
                            local_name: name.local.to_string(),
                            attributes: parsed_attributes,
                        }
                    }
                    TreeNodeKind::Text(text) => ParsedNodeKind::Text(text),
                    TreeNodeKind::Ignored => ParsedNodeKind::Ignored,
                };
                Ok(ParsedNode {
                    kind,
                    children: node.children,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        validate_tree(&nodes)?;
        if quirks_mode.unwrap_or(QuirksMode::NoQuirks) != QuirksMode::NoQuirks {
            return Err(
                "quirks mode 루트 HTML은 현재 runtime profile에서 지원하지 않습니다".to_owned(),
            );
        }
        let parsed = ParsedRootHtml {
            nodes,
            parse_error_count,
        };
        if parsed.html_element().is_none() {
            return Err("HTML parser가 documentElement를 만들지 못했습니다".to_owned());
        }
        Ok(parsed)
    }

    fn parse_error(&self, _msg: Cow<'static, str>) {
        let mut tree = self.tree.borrow_mut();
        if tree.parse_error_count < MAX_RECORDED_PARSE_ERRORS as u64 {
            tree.parse_error_count += 1;
        }
    }

    fn get_document(&self) -> Self::Handle {
        0
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> Self::ElemName<'a> {
        let name = {
            let tree = self.tree.borrow();
            match &tree.nodes[*target].kind {
                TreeNodeKind::Element { name, .. } => &**name as *const QualName,
                _ => unreachable!("HTML tree builder requested a non-element name"),
            }
        };
        // SAFETY: element names are boxed at creation, never mutated or removed, and this
        // reference cannot outlive the sink; Vec growth moves only the Box handle.
        unsafe { &*name }
    }

    fn create_element(
        &self,
        name: QualName,
        attrs: Vec<Attribute>,
        _flags: ElementFlags,
    ) -> Self::Handle {
        self.create_node(TreeNodeKind::Element {
            name: Box::new(name),
            attributes: attrs,
        })
    }

    fn create_comment(&self, _text: StrTendril) -> Self::Handle {
        self.create_node(TreeNodeKind::Ignored)
    }

    fn create_pi(&self, _target: StrTendril, _data: StrTendril) -> Self::Handle {
        self.create_node(TreeNodeKind::Ignored)
    }

    fn append(&self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        match child {
            NodeOrText::AppendNode(child) => self.append_child(*parent, child),
            NodeOrText::AppendText(text) => self.append_text(*parent, &text),
        }
    }

    fn append_based_on_parent_node(
        &self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        if self.tree.borrow().nodes[*element].parent.is_some() {
            self.append_before_sibling(element, child);
        } else {
            self.append(prev_element, child);
        }
    }

    fn append_doctype_to_document(
        &self,
        _name: StrTendril,
        _public_id: StrTendril,
        _system_id: StrTendril,
    ) {
        let doctype = self.create_node(TreeNodeKind::Ignored);
        self.append_child(0, doctype);
    }

    fn get_template_contents(&self, target: &Self::Handle) -> Self::Handle {
        let mut tree = self.tree.borrow_mut();
        if let Some(fragment) = tree.template_contents.get(target) {
            return *fragment;
        }
        let fragment = tree.nodes.len();
        tree.nodes.push(TreeNode {
            kind: TreeNodeKind::Ignored,
            parent: None,
            children: Vec::new(),
        });
        tree.template_contents.insert(*target, fragment);
        fragment
    }

    fn same_node(&self, left: &Self::Handle, right: &Self::Handle) -> bool {
        left == right
    }

    fn set_quirks_mode(&self, mode: QuirksMode) {
        self.tree.borrow_mut().quirks_mode = Some(mode);
    }

    fn append_before_sibling(&self, sibling: &Self::Handle, new_node: NodeOrText<Self::Handle>) {
        match new_node {
            NodeOrText::AppendNode(child) => self.append_before(*sibling, child),
            NodeOrText::AppendText(text) => self.append_text_before(*sibling, &text),
        }
    }

    fn add_attrs_if_missing(&self, target: &Self::Handle, attrs: Vec<Attribute>) {
        let mut tree = self.tree.borrow_mut();
        let TreeNodeKind::Element {
            attributes: existing,
            ..
        } = &mut tree.nodes[*target].kind
        else {
            unreachable!("HTML tree builder adds attributes only to elements")
        };
        for attribute in attrs {
            if !existing.iter().any(|known| known.name == attribute.name) {
                existing.push(attribute);
            }
        }
    }

    fn remove_from_parent(&self, target: &Self::Handle) {
        detach(&mut self.tree.borrow_mut(), *target);
    }

    fn reparent_children(&self, node: &Self::Handle, new_parent: &Self::Handle) {
        let children = {
            let mut tree = self.tree.borrow_mut();
            let children = std::mem::take(&mut tree.nodes[*node].children);
            for child in &children {
                tree.nodes[*child].parent = None;
            }
            children
        };
        for child in children {
            self.append_child(*new_parent, child);
        }
    }
}

fn detach(tree: &mut Tree, node: usize) {
    let Some(parent) = tree.nodes[node].parent.take() else {
        return;
    };
    tree.nodes[parent].children.retain(|child| *child != node);
}

fn validate_tree(nodes: &[ParsedNode]) -> Result<(), String> {
    let Some(root) = nodes.first() else {
        return Err("HTML parser가 빈 document를 만들었습니다".to_owned());
    };
    if !matches!(root.kind, ParsedNodeKind::Document) {
        return Err("HTML parser document root가 잘못되었습니다".to_owned());
    }
    let mut stack = root
        .children
        .iter()
        .map(|child| (*child, 1usize))
        .collect::<Vec<_>>();
    let mut visited = 1usize;
    let mut attribute_count = 0usize;
    while let Some((node, depth)) = stack.pop() {
        if node >= nodes.len() {
            return Err("HTML parser가 잘못된 자식 핸들을 만들었습니다".to_owned());
        }
        visited = visited
            .checked_add(1)
            .ok_or_else(|| "루트 HTML node 수를 계산하지 못했습니다".to_owned())?;
        if visited > MAX_ROOT_HTML_NODES {
            return Err(format!(
                "루트 HTML은 최대 {MAX_ROOT_HTML_NODES}개 node까지 허용합니다"
            ));
        }
        if depth > MAX_ROOT_HTML_DEPTH {
            return Err(format!(
                "루트 HTML tree depth는 {MAX_ROOT_HTML_DEPTH}까지 허용합니다"
            ));
        }
        if let ParsedNodeKind::Element { attributes, .. } = &nodes[node].kind {
            attribute_count = attribute_count
                .checked_add(attributes.len())
                .ok_or_else(|| "루트 HTML attribute 수를 계산하지 못했습니다".to_owned())?;
            if attribute_count > MAX_ROOT_HTML_ATTRIBUTES {
                return Err(format!(
                    "루트 HTML은 최대 {MAX_ROOT_HTML_ATTRIBUTES}개 attribute까지 허용합니다"
                ));
            }
        }
        stack.extend(nodes[node].children.iter().map(|child| (*child, depth + 1)));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
