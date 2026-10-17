use spinon_render::{
    CssAxisClip, CssClip, CssSize, RuntimeGlyphBitmap, RuntimePaintCommand, RuntimeRenderBox,
    RuntimeRenderSnapshot, RuntimeRenderText, RuntimeSrgbColor,
};
use spinon_text_layout::{
    GlyphBitmapContent, LigatureMode, LineBreakMode, ParagraphInput, ParagraphStyle, TextDirection,
    TextLayoutEngine, TextSource, WhiteSpaceMode, WordBreakMode,
};
use std::sync::Arc;

const NOTO_SANS_KR: &[u8] =
    include_bytes!("../../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");
const AHEM: &[u8] = include_bytes!("../../../../../tests/fixtures/resources/c15/Ahem.ttf");
const FIXTURE_TEXT: &str = "C15 glyph paint coverage 한글 overflow sample";
const OVERFLOW_TEXT: &str = "XX";
const FIXTURE_FONT_SIZE_CSS_PX: f32 = 20.0;
const OVERFLOW_FONT_SIZE_CSS_PX: f32 = 10.0;
const FIXTURE_DEVICE_SCALE_FACTOR: f32 = 1.0;
const VISIBLE_TEXT_OWNER_FRAME: [f32; 4] = [13.0, 81.0, 40.0, 10.0];
const HIDDEN_TEXT_OWNER_FRAME: [f32; 4] = [13.0, 121.0, 40.0, 10.0];

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
    let visible_owner = find_text_owner(&scene, VISIBLE_TEXT_OWNER_FRAME, "visible")?;
    let hidden_owner = find_text_owner(&scene, HIDDEN_TEXT_OWNER_FRAME, "hidden")?;
    let hidden_clip = hidden_owner.clip_css_px();
    let hidden_x_clip = hidden_clip
        .x()
        .ok_or_else(|| "C15.3 hidden text owner에 상위 x overflow clip이 없습니다".to_owned())?;
    let hidden_y_clip = hidden_clip
        .y()
        .ok_or_else(|| "C15.3 hidden text owner에 상위 y overflow clip이 없습니다".to_owned())?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Noto Sans KR", NOTO_SANS_KR)
        .map_err(|error| format!("C15.3 Noto Sans KR 등록 실패: {error}"))?;
    engine
        .register_font("Ahem", AHEM)
        .map_err(|error| format!("C15.3 Ahem 등록 실패: {error}"))?;
    let (noto_scale, noto_bitmaps) = rasterize_fixture_text(
        &mut engine,
        FixtureTextInput {
            owner: owner_id,
            content: FIXTURE_TEXT,
            font_family: "Noto Sans KR",
            font_size: FIXTURE_FONT_SIZE_CSS_PX,
            line_height: 26.0,
            available_width: frame.width(),
            label: "Noto Sans KR",
        },
    )?;
    let (ahem_scale, ahem_bitmaps) = rasterize_fixture_text(
        &mut engine,
        FixtureTextInput {
            owner: visible_owner.node_id(),
            content: OVERFLOW_TEXT,
            font_family: "Ahem",
            font_size: OVERFLOW_FONT_SIZE_CSS_PX,
            line_height: OVERFLOW_FONT_SIZE_CSS_PX,
            available_width: visible_owner.frame_css_px().width(),
            label: "Ahem visible overflow",
        },
    )?;
    if visible_owner.frame_css_px().x() != hidden_owner.frame_css_px().x() {
        return Err("C15.3 visible/hidden text owner의 x 원점이 다릅니다".to_owned());
    }
    if ahem_scale != FIXTURE_DEVICE_SCALE_FACTOR {
        return Err(format!(
            "C15.3 Ahem raster scale가 고정값과 다릅니다: {ahem_scale}"
        ));
    }
    let sample_points = overflow_sample_points(
        &ahem_bitmaps,
        (
            visible_owner.frame_css_px().x(),
            visible_owner.frame_css_px().y(),
        ),
        (
            hidden_owner.frame_css_px().x(),
            hidden_owner.frame_css_px().y(),
        ),
        ahem_scale,
        hidden_x_clip,
        hidden_y_clip,
        29.0,
        scene.viewport_css_px(),
    )?;
    let noto_text = RuntimeRenderText::new(
        owner_id,
        scene.key(),
        origin_x,
        origin_y,
        noto_scale,
        RuntimeSrgbColor::new(0.035, 0.13, 0.44, 1.0)
            .map_err(|error| format!("C15.3 Noto glyph paint 색이 유효하지 않습니다: {error}"))?,
        CssClip::unconstrained().with_x(
            CssAxisClip::new(origin_x, clip_right)
                .map_err(|error| format!("C15.3 Noto glyph clip이 유효하지 않습니다: {error}"))?,
        ),
        noto_bitmaps,
    )
    .map_err(|error| format!("C15.3 Noto runtime text snapshot 실패: {error}"))?;
    let overflow_color = RuntimeSrgbColor::new(0.035, 0.13, 0.44, 1.0)
        .map_err(|error| format!("C15.3 Ahem glyph paint 색이 유효하지 않습니다: {error}"))?;
    let visible_text = RuntimeRenderText::new(
        visible_owner.node_id(),
        scene.key(),
        visible_owner.frame_css_px().x(),
        visible_owner.frame_css_px().y(),
        ahem_scale,
        overflow_color,
        CssClip::unconstrained(),
        ahem_bitmaps.clone(),
    )
    .map_err(|error| format!("C15.3 visible overflow text snapshot 실패: {error}"))?;
    let hidden_text = RuntimeRenderText::new(
        hidden_owner.node_id(),
        scene.key(),
        hidden_owner.frame_css_px().x(),
        hidden_owner.frame_css_px().y(),
        ahem_scale,
        overflow_color,
        CssClip::unconstrained(),
        ahem_bitmaps,
    )
    .map_err(|error| format!("C15.3 hidden overflow text snapshot 실패: {error}"))?;
    let glyph_count = noto_text.glyphs().len();
    let overflow_glyph_count = visible_text.glyphs().len();
    let mut commands = scene.paint_commands().to_vec();
    let texts = vec![noto_text, visible_text, hidden_text];
    for index in 0..texts.len() {
        commands.push(RuntimePaintCommand::DrawText { index });
    }
    let scene = scene
        .with_texts_and_paint_commands(texts, commands)
        .map_err(|error| format!("C15.3 runtime text scene 연결 실패: {error}"))?;
    let effective_hidden_clip = scene.texts()[2].clip_css_px();
    if effective_hidden_clip != hidden_clip {
        return Err("C15.3 hidden text가 owner overflow clip과 일치하지 않습니다".to_owned());
    }
    if scene.texts()[1].clip_css_px() != visible_owner.clip_css_px() {
        return Err("C15.3 visible text owner clip이 예상과 다릅니다".to_owned());
    }
    let hit_test_point = |point: (u32, u32)| (point.0 as f32 + 0.5, point.1 as f32 + 0.5);
    let visible_hit = hit_test_point(sample_points.visible);
    let hidden_inside_hit = hit_test_point(sample_points.hidden_inside);
    let hidden_outside_hit = hit_test_point(sample_points.hidden_outside);
    let visible_target = scene
        .hit_test_css_point(visible_hit.0, visible_hit.1)
        .map(|render_box| render_box.node_id());
    let hidden_inside_target = scene
        .hit_test_css_point(hidden_inside_hit.0, hidden_inside_hit.1)
        .map(|render_box| render_box.node_id());
    let hidden_outside_target = scene
        .hit_test_css_point(hidden_outside_hit.0, hidden_outside_hit.1)
        .map(|render_box| render_box.node_id());
    if visible_target != Some(visible_owner.node_id()) {
        return Err(format!(
            "C15.3 visible overflow glyph의 event owner가 맞지 않습니다: {visible_target:?}"
        ));
    }
    if hidden_inside_target != Some(hidden_owner.node_id()) {
        return Err(format!(
            "C15.3 hidden 내부 glyph의 event owner가 맞지 않습니다: {hidden_inside_target:?}"
        ));
    }
    if hidden_outside_target == Some(hidden_owner.node_id()) {
        return Err("C15.3 hidden clip 밖 점이 text owner에 적중했습니다".to_owned());
    }
    Ok((
        scene,
        format!(
            " c15_3_text=attached font=NotoSansKR glyphs={glyph_count} raster_scale={noto_scale} clip_right_css_px={clip_right} overflow_font=Ahem overflow_glyphs={overflow_glyph_count} overflow_clip_source=owner_render_box overflow_samples_css_px=visible:{},{};hidden_inside:{},{};hidden_outside:{},{} hit_test=visible_owner;hidden_inside_owner;hidden_outside_excludes_hidden_owner",
            sample_points.visible.0,
            sample_points.visible.1,
            sample_points.hidden_inside.0,
            sample_points.hidden_inside.1,
            sample_points.hidden_outside.0,
            sample_points.hidden_outside.1,
        ),
    ))
}

fn find_text_owner(
    scene: &RuntimeRenderSnapshot,
    expected_frame: [f32; 4],
    mode: &str,
) -> Result<RuntimeRenderBox, String> {
    let matches: Vec<_> = scene
        .boxes()
        .iter()
        .copied()
        .filter(|render_box| {
            let frame = render_box.frame_css_px();
            [frame.x(), frame.y(), frame.width(), frame.height()]
                .into_iter()
                .zip(expected_frame)
                .all(|(actual, expected)| (actual - expected).abs() <= 0.01)
        })
        .collect();
    if matches.len() != 1 {
        return Err(format!(
            "C15.3 {mode} overflow text owner frame 검색 결과가 {}개입니다",
            matches.len()
        ));
    }
    Ok(matches[0])
}

struct FixtureTextInput<'a> {
    owner: spinon_core::NodeId,
    content: &'a str,
    font_family: &'a str,
    font_size: f32,
    line_height: f32,
    available_width: f32,
    label: &'a str,
}

fn rasterize_fixture_text(
    engine: &mut TextLayoutEngine,
    fixture: FixtureTextInput<'_>,
) -> Result<(f32, Vec<RuntimeGlyphBitmap>), String> {
    let paragraph = ParagraphInput {
        node: fixture.owner,
        sources: vec![TextSource {
            node: fixture.owner,
            inline_ancestors: Vec::new(),
            text: fixture.content.to_owned(),
        }],
        style: ParagraphStyle {
            font_family: fixture.font_family.to_owned(),
            font_size: fixture.font_size,
            line_height: fixture.line_height,
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
        .layout_paragraph(&paragraph, fixture.available_width)
        .map_err(|error| format!("C15.3 {} shaping 실패: {error}", fixture.label))?;
    let raster = engine
        .rasterize_glyphs(&layout, FIXTURE_DEVICE_SCALE_FACTOR)
        .map_err(|error| format!("C15.3 {} raster 실패: {error}", fixture.label))?;
    let mut bitmaps = Vec::new();
    bitmaps
        .try_reserve_exact(raster.glyphs.len())
        .map_err(|_| {
            format!(
                "C15.3 {} runtime glyph bitmap 목록을 할당하지 못했습니다",
                fixture.label
            )
        })?;
    for glyph in &raster.glyphs {
        let Some(bitmap) = &glyph.bitmap else {
            continue;
        };
        if bitmap.content != GlyphBitmapContent::AlphaMask {
            return Err(format!(
                "C15.3 {} runtime glyph {}가 AlphaMask가 아닙니다: {:?}",
                fixture.label, glyph.glyph_id, bitmap.content
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
            .map_err(|error| {
                format!(
                    "C15.3 {} runtime glyph snapshot 실패: {error}",
                    fixture.label
                )
            })?,
        );
    }
    if bitmaps.is_empty() {
        return Err(format!(
            "C15.3 {} raster가 보이는 bitmap을 만들지 않았습니다",
            fixture.label
        ));
    }
    Ok((raster.device_scale_factor, bitmaps))
}

struct OverflowSamplePoints {
    visible: (u32, u32),
    hidden_inside: (u32, u32),
    hidden_outside: (u32, u32),
}

#[allow(clippy::too_many_arguments)]
fn overflow_sample_points(
    glyphs: &[RuntimeGlyphBitmap],
    visible_origin: (f32, f32),
    hidden_origin: (f32, f32),
    scale: f32,
    hidden_x_clip: CssAxisClip,
    hidden_y_clip: CssAxisClip,
    outside_border_min_x: f32,
    viewport: CssSize,
) -> Result<OverflowSamplePoints, String> {
    let mut inside = None;
    let mut outside = None;
    let scale = f64::from(scale);
    for glyph in glyphs {
        let glyph_left =
            f64::from(hidden_origin.0) + f64::from(glyph.origin_x_px() + glyph.left_px()) / scale;
        let glyph_top =
            f64::from(hidden_origin.1) + f64::from(glyph.baseline_y_px() + glyph.top_px()) / scale;
        for row in 0..glyph.height_px() {
            for column in 0..glyph.width_px() {
                let index = usize::try_from(
                    u64::from(row) * u64::from(glyph.width_px()) + u64::from(column),
                )
                .map_err(|_| "C15.3 Ahem sample index 변환 실패".to_owned())?;
                if glyph.pixels()[index] != u8::MAX {
                    continue;
                }
                let center_x = glyph_left + (f64::from(column) + 0.5) / scale;
                let center_y = glyph_top + (f64::from(row) + 0.5) / scale;
                if center_y < f64::from(hidden_y_clip.min())
                    || center_y >= f64::from(hidden_y_clip.max())
                {
                    continue;
                }
                let x = center_x.floor();
                let y = center_y.floor();
                if x < 0.0
                    || y < 0.0
                    || x + 0.5 >= f64::from(viewport.width())
                    || y + 0.5 >= f64::from(viewport.height())
                {
                    continue;
                }
                let point = (x as u32, y as u32);
                let sample_center_x = x + 0.5;
                if sample_center_x >= f64::from(hidden_x_clip.min())
                    && sample_center_x < f64::from(hidden_x_clip.max())
                    && inside.is_none()
                {
                    inside = Some(point);
                }
                if sample_center_x >= f64::from(outside_border_min_x) && outside.is_none() {
                    outside = Some(point);
                }
                if inside.is_some() && outside.is_some() {
                    break;
                }
            }
            if inside.is_some() && outside.is_some() {
                break;
            }
        }
    }
    let hidden_inside = inside.ok_or_else(|| {
        "C15.3 Ahem glyph mask에 hidden clip 내부 비교 표본이 없습니다".to_owned()
    })?;
    let hidden_outside = outside.ok_or_else(|| {
        "C15.3 Ahem glyph mask에 parent border 바깥 비교 표본이 없습니다".to_owned()
    })?;
    let local_y = hidden_outside.1 as f32 + 0.5 - hidden_origin.1;
    let visible_y = (visible_origin.1 + local_y).floor();
    let local_x = hidden_outside.0 as f32 + 0.5 - hidden_origin.0;
    let visible_x = (visible_origin.0 + local_x).floor();
    if visible_x < 0.0
        || visible_y < 0.0
        || visible_x + 0.5 >= viewport.width()
        || visible_y + 0.5 >= viewport.height()
    {
        return Err("C15.3 visible overflow 비교 표본이 viewport 밖입니다".to_owned());
    }
    Ok(OverflowSamplePoints {
        visible: (visible_x as u32, visible_y as u32),
        hidden_inside,
        hidden_outside,
    })
}

#[cfg(test)]
mod tests {
    use super::attach_glyph_paint;
    use spinon_core::{EnvironmentRevision, HostDocument, NodeId, StyleRevision};
    use spinon_render::{
        CssAxisClip, CssClip, CssRect, CssSize, RuntimePaint, RuntimeRenderBox, RuntimeRenderKey,
        RuntimeRenderSnapshot,
    };

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
        let hidden_clip = CssClip::unconstrained()
            .with_x(CssAxisClip::new(13.0, 28.0).unwrap())
            .with_y(CssAxisClip::new(121.0, 131.0).unwrap());
        let scene = RuntimeRenderSnapshot::new(
            key,
            CssSize::new(360.0, 240.0).unwrap(),
            vec![
                RuntimeRenderBox::new(
                    NodeId::new(1).unwrap(),
                    CssRect::new(0.0, 0.0, 360.0, 240.0).unwrap(),
                    RuntimePaint::None,
                    0,
                ),
                RuntimeRenderBox::new(
                    NodeId::new(2).unwrap(),
                    CssRect::new(13.0, 81.0, 40.0, 10.0).unwrap(),
                    RuntimePaint::None,
                    1,
                ),
                RuntimeRenderBox::new(
                    NodeId::new(3).unwrap(),
                    CssRect::new(13.0, 121.0, 40.0, 10.0).unwrap(),
                    RuntimePaint::None,
                    2,
                )
                .with_clip(hidden_clip),
            ],
        )
        .unwrap();
        let (scene, report) = attach_glyph_paint(scene).unwrap();
        assert_eq!(scene.texts().len(), 3);
        assert!(!scene.texts()[0].glyphs().is_empty());
        assert!(!scene.texts()[1].glyphs().is_empty());
        assert!(!scene.texts()[2].glyphs().is_empty());
        assert_eq!(scene.texts()[1].clip_css_px(), CssClip::unconstrained());
        assert_eq!(scene.texts()[2].clip_css_px(), hidden_clip);
        assert!(report.contains("c15_3_text=attached"));
        assert!(report.contains("overflow_font=Ahem"));
        assert!(report.contains("overflow_clip_source=owner_render_box"));
        assert!(report.contains("hidden_outside:"));
        assert!(report.contains(
            "hit_test=visible_owner;hidden_inside_owner;hidden_outside_excludes_hidden_owner"
        ));
        assert!(matches!(
            scene.paint_commands().last(),
            Some(spinon_render::RuntimePaintCommand::DrawText { index: 2 })
        ));
    }

    #[test]
    fn fixture_rejects_a_hidden_clip_on_the_visible_overflow_owner() {
        let document = HostDocument::new().unwrap().snapshot();
        let key = RuntimeRenderKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        );
        let visible_clip = CssClip::unconstrained()
            .with_x(CssAxisClip::new(13.0, 28.0).unwrap())
            .with_y(CssAxisClip::new(81.0, 91.0).unwrap());
        let hidden_clip = CssClip::unconstrained()
            .with_x(CssAxisClip::new(13.0, 28.0).unwrap())
            .with_y(CssAxisClip::new(121.0, 131.0).unwrap());
        let scene = RuntimeRenderSnapshot::new(
            key,
            CssSize::new(360.0, 240.0).unwrap(),
            vec![
                RuntimeRenderBox::new(
                    NodeId::new(1).unwrap(),
                    CssRect::new(0.0, 0.0, 360.0, 240.0).unwrap(),
                    RuntimePaint::None,
                    0,
                ),
                RuntimeRenderBox::new(
                    NodeId::new(2).unwrap(),
                    CssRect::new(13.0, 81.0, 40.0, 10.0).unwrap(),
                    RuntimePaint::None,
                    1,
                )
                .with_clip(visible_clip),
                RuntimeRenderBox::new(
                    NodeId::new(3).unwrap(),
                    CssRect::new(13.0, 121.0, 40.0, 10.0).unwrap(),
                    RuntimePaint::None,
                    2,
                )
                .with_clip(hidden_clip),
            ],
        )
        .unwrap();

        let error = attach_glyph_paint(scene).unwrap_err();
        assert!(error.contains("visible overflow glyph의 event owner가 맞지 않습니다"));
    }

    #[test]
    fn fixture_does_not_let_visible_text_override_a_later_overlay_hit_target() {
        let document = HostDocument::new().unwrap().snapshot();
        let key = RuntimeRenderKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        );
        let hidden_clip = CssClip::unconstrained()
            .with_x(CssAxisClip::new(13.0, 28.0).unwrap())
            .with_y(CssAxisClip::new(121.0, 131.0).unwrap());
        let overlay = NodeId::new(4).unwrap();
        let scene = RuntimeRenderSnapshot::new(
            key,
            CssSize::new(360.0, 240.0).unwrap(),
            vec![
                RuntimeRenderBox::new(
                    NodeId::new(1).unwrap(),
                    CssRect::new(0.0, 0.0, 360.0, 240.0).unwrap(),
                    RuntimePaint::None,
                    0,
                ),
                RuntimeRenderBox::new(
                    NodeId::new(2).unwrap(),
                    CssRect::new(13.0, 81.0, 40.0, 10.0).unwrap(),
                    RuntimePaint::None,
                    1,
                ),
                RuntimeRenderBox::new(
                    NodeId::new(3).unwrap(),
                    CssRect::new(13.0, 121.0, 40.0, 10.0).unwrap(),
                    RuntimePaint::None,
                    2,
                )
                .with_clip(hidden_clip),
                RuntimeRenderBox::new(
                    overlay,
                    CssRect::new(29.0, 81.0, 24.0, 10.0).unwrap(),
                    RuntimePaint::None,
                    3,
                ),
            ],
        )
        .unwrap();

        let error = attach_glyph_paint(scene).unwrap_err();
        assert_eq!(
            error,
            format!(
                "C15.3 visible overflow glyph의 event owner가 맞지 않습니다: Some({overlay:?})"
            )
        );
    }
}
