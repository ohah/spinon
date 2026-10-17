use super::{
    attach_hostdocument_text_paint, fixture_document, layout_fixture, validate_layout_snapshot,
};
use spinon_core::{EnvironmentRevision, StyleRevision};
use spinon_render::RuntimeRenderKey;

fn key() -> RuntimeRenderKey {
    let document = spinon_core::HostDocument::new().unwrap().snapshot();
    RuntimeRenderKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
}

#[test]
fn hostdocument_text_layout_is_the_runtime_paint_source() {
    let (scene, report) = attach_hostdocument_text_paint(key()).unwrap();
    assert_eq!(scene.viewport_css_px().width(), 320.0);
    assert_eq!(scene.viewport_css_px().height(), 240.0);
    assert_eq!(scene.boxes().len(), 2);
    assert_eq!(scene.boxes()[1].frame_css_px().width(), 15.0);
    assert_eq!(scene.boxes()[1].frame_css_px().height(), 10.0);
    assert_eq!(scene.texts().len(), 1);
    assert_eq!(scene.texts()[0].node_id(), scene.boxes()[1].node_id());
    assert_eq!(scene.texts()[0].origin_x_css_px(), 0.0);
    assert_eq!(scene.texts()[0].origin_y_css_px(), 0.0);
    assert_eq!(scene.texts()[0].glyphs().len(), 2);
    assert!(report.contains("layout_source=fixture_owned_host_document"));
    assert!(report.contains("owner_frame_css_px=0,0,15,10"));
    assert!(report.contains("text_width_css_px=20"));
    assert!(report.contains("overflow_css_px=5"));
    assert!(report.contains("hostdocument_text_samples_css_px="));
    let encoded = report
        .split_once("hostdocument_text_samples_css_px=")
        .unwrap()
        .1;
    let samples = encoded
        .split(';')
        .map(|entry| {
            let (name, coordinates) = entry.split_once(':').unwrap();
            let (x, y) = coordinates.split_once(',').unwrap();
            (name, x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    assert_eq!(samples.len(), 6);
    assert!(samples.iter().all(|(_, x, y)| *x < 320 && *y < 240));
    assert!(samples[..2].iter().all(|(_, x, _)| *x < 15));
    assert!(samples[2..].iter().all(|(_, x, _)| *x >= 15));
    assert_eq!(
        samples
            .iter()
            .map(|(_, x, y)| (*x, *y))
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        6,
    );
}

#[test]
fn hostdocument_source_and_text_revision_mismatch_fail_before_paint() {
    let fixture = fixture_document().unwrap();
    let (input, mut output, text_snapshot, _text_engine) = layout_fixture(&fixture).unwrap();
    let frame = output.frames[&fixture.paragraph.id()];
    output.revision = input.revision().with_intrinsic_size_revision(
        spinon_core::IntrinsicSizeRevision::INITIAL
            .checked_next()
            .unwrap(),
    );
    assert!(
        validate_layout_snapshot(&fixture, &input, &output, &text_snapshot, frame)
            .unwrap_err()
            .contains("revision")
    );
}
