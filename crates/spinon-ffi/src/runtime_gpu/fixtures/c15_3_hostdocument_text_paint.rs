mod layout;
mod raster;

use layout::{fixture_document, layout_fixture, validate_layout_snapshot};
use raster::{
    encode_sample_points, overflow_ink_samples, runtime_glyphs, validate_raster_snapshot,
};
use spinon_render::{
    CssClip, CssRect, CssSize, RuntimePaint, RuntimePaintCommand, RuntimeRenderBox,
    RuntimeRenderKey, RuntimeRenderSnapshot, RuntimeRenderText, RuntimeSrgbColor,
};

pub(super) const VIEWPORT_WIDTH: f32 = 320.0;
pub(super) const VIEWPORT_HEIGHT: f32 = 240.0;
pub(super) const DEVICE_SCALE_FACTOR: f32 = 1.0;

pub(crate) fn attach_hostdocument_text_paint(
    key: RuntimeRenderKey,
) -> Result<(RuntimeRenderSnapshot, String), String> {
    let fixture = fixture_document()?;
    let (input, output, text_snapshot, text_engine) = layout_fixture(&fixture)?;
    let paragraph_frame = *output
        .frames
        .get(&fixture.paragraph.id())
        .ok_or_else(|| "C15.3 HostDocument paragraph LayoutFrame이 없습니다".to_owned())?;
    validate_layout_snapshot(&fixture, &input, &output, &text_snapshot, paragraph_frame)?;

    let raster = text_engine
        .rasterize_glyphs(&text_snapshot.layout, DEVICE_SCALE_FACTOR)
        .map_err(|error| format!("C15.3 HostDocument layout snapshot raster 실패: {error}"))?;
    validate_raster_snapshot(&text_snapshot, &raster)?;
    let sample_points = overflow_ink_samples(&raster, paragraph_frame)?;
    let glyphs = runtime_glyphs(&raster)?;

    let viewport = CssSize::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)
        .map_err(|error| format!("C15.3 HostDocument viewport가 잘못되었습니다: {error}"))?;
    let root_frame = output
        .frames
        .get(&fixture.root.id())
        .copied()
        .ok_or_else(|| "C15.3 HostDocument root LayoutFrame이 없습니다".to_owned())?;
    let root = RuntimeRenderBox::new(
        fixture.root.id(),
        css_rect(root_frame, "root")?,
        RuntimePaint::Color(
            RuntimeSrgbColor::new(247.0 / 255.0, 249.0 / 255.0, 252.0 / 255.0, 1.0)
                .map_err(|error| format!("C15.3 HostDocument root 색이 잘못되었습니다: {error}"))?,
        ),
        0,
    );
    let paragraph = RuntimeRenderBox::new(
        fixture.paragraph.id(),
        css_rect(paragraph_frame, "paragraph")?,
        RuntimePaint::None,
        1,
    );
    let text = RuntimeRenderText::new(
        fixture.paragraph.id(),
        key,
        paragraph_frame.x,
        paragraph_frame.y,
        DEVICE_SCALE_FACTOR,
        RuntimeSrgbColor::new(9.0 / 255.0, 33.0 / 255.0, 112.0 / 255.0, 1.0)
            .map_err(|error| format!("C15.3 HostDocument text 색이 잘못되었습니다: {error}"))?,
        CssClip::unconstrained(),
        glyphs,
    )
    .map_err(|error| format!("C15.3 HostDocument RuntimeRenderText 생성 실패: {error}"))?;
    let scene = RuntimeRenderSnapshot::new(key, viewport, vec![root, paragraph])
        .map_err(|error| format!("C15.3 HostDocument runtime scene 생성 실패: {error}"))?
        .with_texts_and_paint_commands(
            vec![text],
            vec![
                RuntimePaintCommand::DrawBox { index: 0 },
                RuntimePaintCommand::DrawBox { index: 1 },
                RuntimePaintCommand::DrawText { index: 0 },
            ],
        )
        .map_err(|error| format!("C15.3 HostDocument text scene 연결 실패: {error}"))?;

    let report = format!(
        "c15_3_hostdocument_text_paint=attached layout_source=fixture_owned_host_document document_revision={} render_tree_revision={} layout_revision={:?} paragraph_node={} text_node={} owner_frame_css_px={},{},{},{} text={} text_width_css_px={} line_count={} glyph_advances_css_px=10,10 source_utf16=0-1,1-2 overflow_css_px=5 font=Ahem dpr=1 hostdocument_text_samples_css_px={}",
        fixture.snapshot.document_revision().get(),
        fixture.snapshot.render_tree_revision().get(),
        output.revision,
        fixture.paragraph.id(),
        fixture.text.id(),
        paragraph_frame.x,
        paragraph_frame.y,
        paragraph_frame.width,
        paragraph_frame.height,
        text_snapshot.layout.text,
        text_snapshot.layout.width,
        text_snapshot.layout.lines.len(),
        encode_sample_points(&sample_points),
    );
    Ok((scene, report))
}

fn css_rect(frame: spinon_layout::LayoutFrame, label: &str) -> Result<CssRect, String> {
    CssRect::new(frame.x, frame.y, frame.width, frame.height)
        .map_err(|error| format!("C15.3 HostDocument {label} rect가 잘못되었습니다: {error}"))
}

#[cfg(test)]
mod tests;
