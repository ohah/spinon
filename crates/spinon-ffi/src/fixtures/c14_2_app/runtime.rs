use std::sync::Arc;

use spinon_core::{
    AttributeName, EnvironmentRevision, HostDocumentSnapshot, HostNodeHandle, HostNodeKind,
    IntrinsicSizeRevision, StyleRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::LayoutSourceRevision;
use spinon_runtime::{RuntimeSession, TaskPriority};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};
use spinon_style_to_layout::compute_runtime_style_layout_with_intrinsic_size_snapshot;

use super::oracle::{OracleCase, json_number, json_size, json_string, within_frame};
use super::{HTML_NAMESPACE, IMAGE_URL, PNG};

struct LiveCase {
    id: String,
    parent: Option<HostNodeHandle>,
    image: HostNodeHandle,
}

pub(super) struct LiveDocument {
    _runtime: RuntimeSession,
    snapshot: HostDocumentSnapshot,
    root: HostNodeHandle,
    cases: Vec<LiveCase>,
}

pub(super) struct DocumentReport {
    pub(super) cases: Vec<serde_json::Value>,
    pub(super) matched_count: usize,
    pub(super) source_stamp_matches: bool,
    pub(super) summary: serde_json::Value,
}

pub(super) fn create_live_document(
    source: &str,
    mappings: &[serde_json::Value],
    flex: bool,
) -> Result<LiveDocument, String> {
    let (runtime, _) = RuntimeSession::new().map_err(|error| format!("live V8 생성: {error}"))?;
    let eval = runtime.eval(source, TaskPriority::UserVisible);
    if eval.status != 0 {
        return Err(format!(
            "live V8 C14.2 {} document script 실패 status={} report={}",
            if flex { "Flex" } else { "Block" },
            eval.status,
            eval.report
        ));
    }
    let snapshot = runtime
        .snapshot_document_for_c14_1_fixture()
        .map_err(|error| format!("live V8 HostDocument snapshot: {error}"))?;
    let roots = snapshot.root_children().collect::<Vec<_>>();
    let [root] = roots.as_slice() else {
        return Err(format!("live V8 C14.2 root count is {} not 1", roots.len()));
    };
    let root = *root;
    ensure_element(&snapshot, root, "main", "cases")?;
    let children = snapshot
        .children(root)
        .ok_or_else(|| "live V8 C14.2 main children missing".to_owned())?
        .collect::<Vec<_>>();
    if children.len() != mappings.len() {
        return Err(format!(
            "live V8 C14.2 {} child count mismatch: expected={} actual={}",
            if flex { "Flex" } else { "Block" },
            mappings.len(),
            children.len()
        ));
    }
    let mut cases = Vec::with_capacity(mappings.len());
    for (mapping, child) in mappings.iter().zip(children) {
        let id = json_string(mapping, "id")?.to_owned();
        let (parent, image) = if flex {
            ensure_element(&snapshot, child, "div", &id)?;
            if attribute(&snapshot, child, "class").as_deref()
                != Some(json_string(mapping, "parentClass")?)
            {
                return Err(format!("{id} live-V8 flex parent class mismatch"));
            }
            let parent_style = json_string(mapping, "parentStyle")?;
            if attribute(&snapshot, child, "style").unwrap_or_default() != parent_style {
                return Err(format!("{id} live-V8 flex parent style mismatch"));
            }
            let descendants = snapshot
                .children(child)
                .ok_or_else(|| format!("{id} flex parent child list missing"))?
                .collect::<Vec<_>>();
            let [image] = descendants.as_slice() else {
                return Err(format!("{id} flex parent must have one img child"));
            };
            let image = *image;
            ensure_element(&snapshot, image, "img", &format!("{id}-image"))?;
            (Some(child), image)
        } else {
            ensure_element(&snapshot, child, "img", &id)?;
            (None, child)
        };
        if !snapshot.is_connected(image)
            || attribute(&snapshot, image, "src").as_deref() != Some(IMAGE_URL)
        {
            return Err(format!(
                "{id} live-V8 img is detached or has unexpected src"
            ));
        }
        let expected_style = json_string(mapping, "style")?;
        let actual_style = attribute(&snapshot, image, "style").unwrap_or_default();
        if actual_style != expected_style {
            return Err(format!(
                "{id} live-V8 img style differs from fixed fixture input"
            ));
        }
        cases.push(LiveCase { id, parent, image });
    }
    Ok(LiveDocument {
        _runtime: runtime,
        snapshot,
        root,
        cases,
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn layout_document(
    name: &str,
    live: &LiveDocument,
    cases: &[OracleCase],
    inventory: &serde_json::Value,
    source_html: &str,
    viewport: CssViewport,
) -> Result<DocumentReport, String> {
    if live.cases.len() != cases.len() {
        return Err(format!("{name} live and oracle case counts differ"));
    }
    let document = &live.snapshot;
    let host_source = LayoutSourceRevision::HostDocument {
        generation: document.generation(),
        document: document.document_revision(),
        render_tree: document.render_tree_revision(),
    };
    let mut completions = Vec::with_capacity(cases.len());
    let mut request_sources_match = true;
    for (live_case, oracle_case) in live.cases.iter().zip(cases) {
        if live_case.id != oracle_case.id {
            return Err(format!(
                "{name} case identity/order mismatch at {}",
                oracle_case.id
            ));
        }
        let request = ImageRequest::from_document(document, live_case.image)
            .map_err(|error| format!("{} image request: {error}", live_case.id))?;
        if request.src().to_string_lossy() != IMAGE_URL {
            return Err(format!("{} image request source mismatch", live_case.id));
        }
        request_sources_match &= request.source() == host_source;
        completions.push(
            request
                .decode(PNG)
                .map_err(|error| format!("{} PNG decode: {error}", live_case.id))?,
        );
    }
    let intrinsic_revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| format!("{name} intrinsic revision exhausted"))?;
    let resources = ImageResourceSnapshot::new(document, intrinsic_revision, completions)
        .map_err(|error| format!("{name} resource snapshot: {error}"))?;
    if resources.images().len() != cases.len() {
        return Err(format!("{name} resource snapshot image count mismatch"));
    }
    let intrinsic = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| format!("{name} intrinsic layout snapshot: {error}"))?;
    let intrinsic_source_matches = resources.source() == host_source
        && intrinsic.source() == resources.source()
        && request_sources_match;
    let intrinsic_revision_matches =
        intrinsic.revision() == resources.revision() && resources.revision() == intrinsic_revision;
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(document.clone()), live.root)
            .map_err(|error| format!("{name} Stylo document: {error}"))?;
    let style_revision = StyleRevision::INITIAL;
    let stylesheet = StylesheetSource {
        id: format!("c14-2-live-v8-{name}"),
        base_url: format!("https://spinon.invalid/c14-2-{name}.css"),
        origin: CssOrigin::Author,
        css: extract_stylesheet(source_html)?,
    };
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        viewport,
        style_revision,
    )
    .map_err(|error| format!("{name} CSS cascade: {error}"))?;
    let style_stamp_matches = computed.generation == document.generation()
        && computed.document_revision == document.document_revision()
        && computed.render_tree_revision == document.render_tree_revision()
        && computed.style_revision == style_revision
        && computed.viewport.environment_revision == viewport.environment_revision
        && computed.viewport.width_css_px == viewport.width_css_px
        && computed.viewport.height_css_px == viewport.height_css_px
        && computed.viewport.device_scale_factor == viewport.device_scale_factor;
    let no_style_diagnostics = computed.diagnostics.is_empty();
    let align_items = live
        .cases
        .iter()
        .map(|case| {
            case.parent.and_then(|parent| {
                computed
                    .elements
                    .iter()
                    .find(|style| style.node_id == parent.id())
                    .and_then(|style| style.properties.get("align-items"))
                    .cloned()
            })
        })
        .collect::<Vec<_>>();
    let output = compute_runtime_style_layout_with_intrinsic_size_snapshot(
        document,
        live.root,
        computed,
        viewport,
        intrinsic.clone(),
    )
    .map_err(|error| format!("{name} CSS layout: {error}"))?;
    let expected_natural = json_size(
        inventory["resource"]
            .get("naturalSizeCssPx")
            .ok_or_else(|| "C14.2 expected PNG natural size missing".to_owned())?,
        "C14.2 expected PNG natural size",
    )?;
    let mut matched_count = 0usize;
    let mut report_cases = Vec::with_capacity(cases.len());
    for (index, (live_case, oracle_case)) in live.cases.iter().zip(cases).enumerate() {
        let frame = output
            .layout
            .frames
            .get(&live_case.image.id())
            .ok_or_else(|| format!("{} image frame missing", oracle_case.id))?;
        let actual_frame = [frame.x, frame.y, frame.width, frame.height];
        let image = resources
            .images()
            .get(&live_case.image.id())
            .ok_or_else(|| format!("{} PNG metadata missing", oracle_case.id))?;
        let natural_size = [
            image.dimensions().width() as f32,
            image.dimensions().height() as f32,
        ];
        let natural_matches = natural_size == expected_natural;
        let frame_matches = within_frame(
            actual_frame,
            oracle_case.expected_frame,
            oracle_case.tolerance,
        );
        let (parent_frame, parent_size_matches, align_items_matches) = match live_case.parent {
            Some(parent) => {
                let parent_frame = output
                    .layout
                    .frames
                    .get(&parent.id())
                    .ok_or_else(|| format!("{} flex parent frame missing", oracle_case.id))?;
                let actual_parent = [
                    parent_frame.x,
                    parent_frame.y,
                    parent_frame.width,
                    parent_frame.height,
                ];
                let expected_size = oracle_case.expected_parent_size.ok_or_else(|| {
                    format!("{} flex expected parent size missing", oracle_case.id)
                })?;
                let size_matches = actual_parent[2].is_finite()
                    && actual_parent[3].is_finite()
                    && (actual_parent[2] - expected_size[0]).abs() <= oracle_case.tolerance
                    && (actual_parent[3] - expected_size[1]).abs() <= oracle_case.tolerance;
                let actual_align = align_items.get(index).and_then(Option::as_deref);
                let align_matches = actual_align == oracle_case.expected_align_items.as_deref();
                (Some(actual_parent), size_matches, align_matches)
            }
            None => (None, true, true),
        };
        let stamps_match =
            intrinsic_source_matches && intrinsic_revision_matches && style_stamp_matches;
        let matches = natural_matches
            && frame_matches
            && parent_size_matches
            && align_items_matches
            && stamps_match
            && no_style_diagnostics;
        if matches {
            matched_count += 1;
        }
        let mut row = serde_json::json!({
            "id": oracle_case.id,
            "document_kind": name,
            "status": if matches { "ok" } else { "mismatch" },
            "document_generation": format!("{:?}", document.generation()),
            "document_revision": document.document_revision().get(),
            "render_tree_revision": document.render_tree_revision().get(),
            "style_revision": style_revision.get(),
            "environment_revision": viewport.environment_revision.get(),
            "intrinsic_size_revision": resources.revision().get(),
            "source_stamp_matches": stamps_match,
            "natural_size_css_px": natural_size,
            "expected_natural_size_css_px": expected_natural,
            "frame_css_px": actual_frame,
            "expected_frame_css_px": oracle_case.expected_frame,
            "expected_case_size_css_px": oracle_case.expected_size,
            "tolerance_css_px": oracle_case.tolerance,
            "natural_size_matches": natural_matches,
            "frame_matches": frame_matches,
            "parent_size_matches": parent_size_matches,
            "align_items_matches": align_items_matches,
            "style_diagnostics_empty": no_style_diagnostics,
        });
        if let Some(parent_frame) = parent_frame {
            row["parent_frame_css_px"] = serde_json::json!(parent_frame);
            row["expected_parent_size_css_px"] =
                serde_json::json!(oracle_case.expected_parent_size);
            row["expected_parent_align_items"] =
                serde_json::json!(oracle_case.expected_align_items);
            row["actual_parent_align_items"] = serde_json::json!(align_items[index]);
        }
        report_cases.push(row);
    }
    let source_stamp_matches = intrinsic_source_matches
        && intrinsic_revision_matches
        && style_stamp_matches
        && no_style_diagnostics;
    let summary = serde_json::json!({
        "kind": name,
        "case_count": cases.len(),
        "matched_count": matched_count,
        "document_generation": format!("{:?}", document.generation()),
        "document_revision": document.document_revision().get(),
        "render_tree_revision": document.render_tree_revision().get(),
        "style_revision": style_revision.get(),
        "environment_revision": viewport.environment_revision.get(),
        "intrinsic_size_revision": resources.revision().get(),
        "document_source": format!("{:?}", resources.source()),
        "intrinsic_source_matches": intrinsic_source_matches,
        "intrinsic_revision_matches": intrinsic_revision_matches,
        "style_stamp_matches": style_stamp_matches,
        "style_diagnostics_empty": no_style_diagnostics,
    });
    Ok(DocumentReport {
        cases: report_cases,
        matched_count,
        source_stamp_matches,
        summary,
    })
}

fn extract_stylesheet(html: &str) -> Result<String, String> {
    let (_, rest) = html
        .split_once("<style>")
        .ok_or_else(|| "C14.2 pinned fixture style start missing".to_owned())?;
    let (css, _) = rest
        .split_once("</style>")
        .ok_or_else(|| "C14.2 pinned fixture style end missing".to_owned())?;
    // C14.2의 `<main>`은 Chrome UA stylesheet에서 block입니다. 이 제한 런타임 profile은
    // HTML UA stylesheet를 로드하지 않으므로 같은 기본값을 fixture cascade에 명시합니다.
    Ok(format!("{css}\nmain {{ display: block; }}"))
}

pub(super) fn viewport(inventory: &serde_json::Value) -> Result<CssViewport, String> {
    let size = inventory
        .get("viewportCssPx")
        .ok_or_else(|| "C14.2 viewport missing".to_owned())?;
    Ok(CssViewport {
        width_css_px: json_number(size, "width")?,
        height_css_px: json_number(size, "height")?,
        device_scale_factor: json_number(inventory, "devicePixelRatio")?,
        environment_revision: EnvironmentRevision::default(),
        media_environment: CssMediaEnvironment::DESKTOP,
    })
}

fn ensure_element(
    document: &HostDocumentSnapshot,
    node: HostNodeHandle,
    local_name: &str,
    id: &str,
) -> Result<(), String> {
    let snapshot_node = document
        .node(node)
        .ok_or_else(|| format!("{id} HostDocument node missing"))?;
    let element = match snapshot_node.kind() {
        HostNodeKind::Element(element)
            if element.namespace() == HTML_NAMESPACE && element.local_name() == local_name =>
        {
            element
        }
        _ => return Err(format!("{id} live-V8 node is not HTML {local_name}")),
    };
    let id_attribute = AttributeName::new(None, "id".to_owned())
        .ok_or_else(|| "HTML id attribute name invalid".to_owned())?;
    if element
        .attribute(&id_attribute)
        .map(|value| value.to_string_lossy())
        .as_deref()
        != Some(id)
    {
        return Err(format!("{id} live-V8 node id mismatch"));
    }
    Ok(())
}

fn attribute(document: &HostDocumentSnapshot, node: HostNodeHandle, name: &str) -> Option<String> {
    let HostNodeKind::Element(element) = document.node(node)?.kind() else {
        return None;
    };
    let name = AttributeName::new(None, name.to_owned())?;
    element
        .attribute(&name)
        .map(|value| value.to_string_lossy())
}
