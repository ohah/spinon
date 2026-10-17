use spinon_render::{
    CssAxisClip, CssClip, RuntimeGlyphBitmap, RuntimePaintCommand, RuntimeRenderSnapshot,
    RuntimeRenderText, RuntimeSrgbColor,
};
use spinon_text_layout::{
    GlyphBitmapContent, LigatureMode, LineBreakMode, ParagraphInput, ParagraphStyle, TextDirection,
    TextLayoutEngine, TextSource, WhiteSpaceMode, WordBreakMode,
};
use std::sync::Arc;

const NOTO_SANS_KR: &[u8] =
    include_bytes!("../../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");
const FIXTURE_TEXT: &str = "C15 glyph paint coverage 한글 overflow sample";
const FIXTURE_FONT_SIZE_CSS_PX: f32 = 20.0;
const FIXTURE_DEVICE_SCALE_FACTOR: f32 = 1.0;

pub(crate) const RUNTIME_CSS_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c15/c15-3-wgpu-glyph-paint.js");

pub(crate) fn attach_glyph_paint(
    scene: RuntimeRenderSnapshot,
) -> Result<(RuntimeRenderSnapshot, String), String> {
    let Some(owner) = scene.boxes().first() else {
        return Err("C15.3 glyph paint fixture에 root render box가 없습니다".to_owned());
    };
    let owner_id = owner.node_id();
    let frame = owner.frame_css_px();
    let origin_x = frame.x() + 12.0;
    let origin_y = frame.y() + 12.0;
    let clip_right = (origin_x + 148.0).min(frame.x() + frame.width());
    if clip_right <= origin_x {
        return Err("C15.3 glyph paint fixture viewport가 clip 폭보다 작습니다".to_owned());
    }

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Noto Sans KR", NOTO_SANS_KR)
        .map_err(|error| format!("C15.3 Noto Sans KR 등록 실패: {error}"))?;
    let input = ParagraphInput {
        node: owner_id,
        sources: vec![TextSource {
            node: owner_id,
            inline_ancestors: Vec::new(),
            text: FIXTURE_TEXT.to_owned(),
        }],
        style: ParagraphStyle {
            font_family: "Noto Sans KR".to_owned(),
            font_size: FIXTURE_FONT_SIZE_CSS_PX,
            line_height: 26.0,
            white_space: WhiteSpaceMode::NoWrap,
            word_break: WordBreakMode::Normal,
            direction: TextDirection::Auto,
            line_break: LineBreakMode::Strict,
            ligatures: LigatureMode::Normal,
        },
        inline_styles: Default::default(),
        inline_run_styles: Default::default(),
    };
    let layout = engine
        .layout_paragraph(&input, frame.width())
        .map_err(|error| format!("C15.3 glyph shaping 실패: {error}"))?;
    let raster = engine
        .rasterize_glyphs(&layout, FIXTURE_DEVICE_SCALE_FACTOR)
        .map_err(|error| format!("C15.3 glyph raster 실패: {error}"))?;
    let mut bitmaps = Vec::new();
    bitmaps
        .try_reserve_exact(raster.glyphs.len())
        .map_err(|_| "C15.3 runtime glyph bitmap 목록을 할당하지 못했습니다".to_owned())?;
    for glyph in &raster.glyphs {
        let Some(bitmap) = &glyph.bitmap else {
            continue;
        };
        if bitmap.content != GlyphBitmapContent::AlphaMask {
            return Err(format!(
                "C15.3 runtime glyph {}가 AlphaMask가 아닙니다: {:?}",
                glyph.glyph_id, bitmap.content
            ));
        }
        bitmaps.push(
            RuntimeGlyphBitmap::new(
                bitmap.width_px,
                bitmap.height_px,
                glyph.origin_x_px,
                glyph.baseline_y_px,
                bitmap.left_px,
                bitmap.top_px,
                Arc::clone(&bitmap.pixels),
            )
            .map_err(|error| format!("C15.3 runtime glyph snapshot 실패: {error}"))?,
        );
    }
    if bitmaps.is_empty() {
        return Err("C15.3 glyph raster가 보이는 bitmap을 만들지 않았습니다".to_owned());
    }
    let text = RuntimeRenderText::new(
        owner_id,
        scene.key(),
        origin_x,
        origin_y,
        raster.device_scale_factor,
        RuntimeSrgbColor::new(0.035, 0.13, 0.44, 1.0)
            .map_err(|error| format!("C15.3 glyph paint 색이 유효하지 않습니다: {error}"))?,
        CssClip::unconstrained().with_x(
            CssAxisClip::new(origin_x, clip_right)
                .map_err(|error| format!("C15.3 glyph clip이 유효하지 않습니다: {error}"))?,
        ),
        bitmaps,
    )
    .map_err(|error| format!("C15.3 runtime text snapshot 실패: {error}"))?;
    let glyph_count = text.glyphs().len();
    let mut commands = scene.paint_commands().to_vec();
    commands.push(RuntimePaintCommand::DrawText { index: 0 });
    let scene = scene
        .with_texts_and_paint_commands(vec![text], commands)
        .map_err(|error| format!("C15.3 runtime text scene 연결 실패: {error}"))?;
    Ok((
        scene,
        format!(
            " c15_3_text=attached font=NotoSansKR glyphs={glyph_count} raster_scale={FIXTURE_DEVICE_SCALE_FACTOR} clip_right_css_px={clip_right}"
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::attach_glyph_paint;
    use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};
    use spinon_render::RuntimeRenderSnapshot;
    use spinon_render::{CssRect, CssSize, RuntimePaint, RuntimeRenderBox, RuntimeRenderKey};

    #[test]
    fn fixed_font_fixture_publishes_real_raster_as_last_paint_command() {
        let document = HostDocument::new().unwrap().snapshot();
        let key = RuntimeRenderKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        );
        let scene = RuntimeRenderSnapshot::new(
            key,
            CssSize::new(224.0, 72.0).unwrap(),
            vec![RuntimeRenderBox::new(
                NodeId::new(1).unwrap(),
                CssRect::new(0.0, 0.0, 224.0, 72.0).unwrap(),
                RuntimePaint::None,
                0,
            )],
        )
        .unwrap();
        let (scene, report) = attach_glyph_paint(scene).unwrap();
        assert_eq!(scene.texts().len(), 1);
        assert!(!scene.texts()[0].glyphs().is_empty());
        assert!(report.contains("c15_3_text=attached"));
        assert!(matches!(
            scene.paint_commands().last(),
            Some(spinon_render::RuntimePaintCommand::DrawText { index: 0 })
        ));
    }
}
