use spinon_render::{
    CssAxisClip, CssClip, CssSize, RuntimeGlyphBitmap, RuntimePaintCommand, RuntimeRenderBox,
    RuntimeRenderSnapshot, RuntimeRenderText, RuntimeScrollContainer, RuntimeSrgbColor,
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
const HORIZONTAL_TEXT_OWNER_FRAME: [f32; 4] = [41.0, 161.0, 40.0, 20.0];
const VERTICAL_TEXT_OWNER_FRAME: [f32; 4] = [101.0, 183.0, 28.0, 10.0];
const NESTED_TEXT_OWNER_FRAME: [f32; 4] = [262.0, 162.0, 40.0, 20.0];

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
    let horizontal_owner =
        find_text_owner(&scene, HORIZONTAL_TEXT_OWNER_FRAME, "horizontal scroll")?;
    let vertical_owner = find_text_owner(&scene, VERTICAL_TEXT_OWNER_FRAME, "vertical scroll")?;
    let nested_owner = find_text_owner(&scene, NESTED_TEXT_OWNER_FRAME, "nested scroll")?;
    let horizontal_scroll = find_scroll_container_by_port(&scene, 13.0, 20.0, "horizontal")?;
    let vertical_scroll = find_scroll_container_by_port(&scene, 28.0, 13.0, "vertical")?;
    let nested_outer_scroll = find_scroll_container_by_port(&scene, 28.0, 20.0, "nested outer")?;
    let nested_inner_scroll = find_scroll_container_by_port(&scene, 18.0, 20.0, "nested inner")?;
    require_scroll_ancestor(
        &scene,
        horizontal_owner.node_id(),
        horizontal_scroll.owner(),
        "horizontal",
    )?;
    require_scroll_ancestor(
        &scene,
        vertical_owner.node_id(),
        vertical_scroll.owner(),
        "vertical",
    )?;
    require_scroll_ancestor(
        &scene,
        nested_owner.node_id(),
        nested_outer_scroll.owner(),
        "nested outer",
    )?;
    require_scroll_ancestor(
        &scene,
        nested_owner.node_id(),
        nested_inner_scroll.owner(),
        "nested inner",
    )?;
    if nested_inner_scroll.parent() != Some(nested_outer_scroll.owner()) {
        return Err("C15.3 nested inner scroll parent가 outer owner와 다릅니다".to_owned());
    }
    if horizontal_scroll.max_offset_x_css_px() < 6.0
        || vertical_scroll.max_offset_y_css_px() < 18.0
        || nested_outer_scroll.max_offset_x_css_px() < 4.0
        || nested_inner_scroll.max_offset_x_css_px() < 5.0
    {
        return Err(
            "C15.3 scroll fixture의 scrollable extent가 acceptance offset보다 작습니다".to_owned(),
        );
    }
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
        ahem_bitmaps.clone(),
    )
    .map_err(|error| format!("C15.3 hidden overflow text snapshot 실패: {error}"))?;
    let horizontal_text = runtime_ahem_text(
        &scene,
        horizontal_owner,
        ahem_scale,
        &ahem_bitmaps,
        overflow_color,
    )?;
    let vertical_text = runtime_ahem_text(
        &scene,
        vertical_owner,
        ahem_scale,
        &ahem_bitmaps,
        overflow_color,
    )?;
    let nested_text = runtime_ahem_text(
        &scene,
        nested_owner,
        ahem_scale,
        &ahem_bitmaps,
        overflow_color,
    )?;
    let glyph_count = noto_text.glyphs().len();
    let overflow_glyph_count = visible_text.glyphs().len();
    let mut commands = scene.paint_commands().to_vec();
    let texts = vec![
        noto_text,
        visible_text,
        hidden_text,
        horizontal_text,
        vertical_text,
        nested_text,
    ];
    for index in 0..texts.len() {
        commands.push(RuntimePaintCommand::DrawText { index });
    }
    let attached_scene = scene
        .with_texts_and_paint_commands(texts, commands)
        .map_err(|error| format!("C15.3 runtime text scene 연결 실패: {error}"))?;
    let effective_hidden_clip = attached_scene.texts()[2].clip_css_px();
    if effective_hidden_clip != hidden_clip {
        return Err("C15.3 hidden text가 owner overflow clip과 일치하지 않습니다".to_owned());
    }
    if attached_scene.texts()[1].clip_css_px() != visible_owner.clip_css_px() {
        return Err("C15.3 visible text owner clip이 예상과 다릅니다".to_owned());
    }
    let hit_test_point = |point: (u32, u32)| (point.0 as f32 + 0.5, point.1 as f32 + 0.5);
    let visible_hit = hit_test_point(sample_points.visible);
    let hidden_inside_hit = hit_test_point(sample_points.hidden_inside);
    let hidden_outside_hit = hit_test_point(sample_points.hidden_outside);
    let visible_target = attached_scene
        .hit_test_css_point(visible_hit.0, visible_hit.1)
        .map(|render_box| render_box.node_id());
    let hidden_inside_target = attached_scene
        .hit_test_css_point(hidden_inside_hit.0, hidden_inside_hit.1)
        .map(|render_box| render_box.node_id());
    let hidden_outside_target = attached_scene
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
    let initial_revision = attached_scene.key().scroll_state_revision();
    let base_horizontal_origin = text_origin(&attached_scene, horizontal_owner.node_id())?;
    let base_vertical_origin = text_origin(&attached_scene, vertical_owner.node_id())?;
    let base_nested_origin = text_origin(&attached_scene, nested_owner.node_id())?;
    let scroll_start = attached_scene
        .with_scroll_offsets(
            initial_revision,
            &[
                (horizontal_scroll.owner(), 3.0, 0.0),
                (vertical_scroll.owner(), 0.0, 18.0),
                (nested_outer_scroll.owner(), 4.0, 0.0),
                (nested_inner_scroll.owner(), 5.0, 0.0),
            ],
        )
        .map_err(|error| format!("C15.3 초기 scroll offset 적용 실패: {error}"))?;
    require_text_origin(
        &scroll_start,
        horizontal_owner.node_id(),
        (base_horizontal_origin.0 - 3.0, base_horizontal_origin.1),
        "horizontal 3",
    )?;
    require_text_origin(
        &scroll_start,
        vertical_owner.node_id(),
        (base_vertical_origin.0, base_vertical_origin.1 - 18.0),
        "vertical 18",
    )?;
    require_text_origin(
        &scroll_start,
        nested_owner.node_id(),
        (base_nested_origin.0 - 9.0, base_nested_origin.1),
        "nested 4+5",
    )?;
    let scroll_middle = scroll_start
        .with_scroll_offsets(
            scroll_start.key().scroll_state_revision(),
            &[
                (horizontal_scroll.owner(), 6.0, 0.0),
                (vertical_scroll.owner(), 0.0, 7.0),
                (nested_outer_scroll.owner(), 2.0, 0.0),
                (nested_inner_scroll.owner(), 3.0, 0.0),
            ],
        )
        .map_err(|error| format!("C15.3 repeated scroll offset 적용 실패: {error}"))?;
    require_text_origin(
        &scroll_middle,
        horizontal_owner.node_id(),
        (base_horizontal_origin.0 - 6.0, base_horizontal_origin.1),
        "horizontal 6",
    )?;
    require_text_origin(
        &scroll_middle,
        vertical_owner.node_id(),
        (base_vertical_origin.0, base_vertical_origin.1 - 7.0),
        "vertical 7",
    )?;
    require_text_origin(
        &scroll_middle,
        nested_owner.node_id(),
        (base_nested_origin.0 - 5.0, base_nested_origin.1),
        "nested 2+3",
    )?;
    let scroll_reset = scroll_middle
        .with_scroll_offsets(
            scroll_middle.key().scroll_state_revision(),
            &[
                (horizontal_scroll.owner(), 0.0, 0.0),
                (vertical_scroll.owner(), 0.0, 0.0),
                (nested_outer_scroll.owner(), 0.0, 0.0),
                (nested_inner_scroll.owner(), 0.0, 0.0),
            ],
        )
        .map_err(|error| format!("C15.3 scroll reset 실패: {error}"))?;
    for (owner, origin, label) in [
        (
            horizontal_owner.node_id(),
            base_horizontal_origin,
            "horizontal reset",
        ),
        (
            vertical_owner.node_id(),
            base_vertical_origin,
            "vertical reset",
        ),
        (nested_owner.node_id(), base_nested_origin, "nested reset"),
    ] {
        require_text_origin(&scroll_reset, owner, origin, label)?;
    }
    let horizontal_sample = solid_glyph_sample(&scroll_start, horizontal_owner.node_id())?;
    let vertical_sample = solid_glyph_sample(&scroll_start, vertical_owner.node_id())?;
    let nested_sample = solid_glyph_sample(&scroll_start, nested_owner.node_id())?;
    for (owner, point, label) in [
        (horizontal_owner.node_id(), horizontal_sample, "horizontal"),
        (vertical_owner.node_id(), vertical_sample, "vertical"),
        (nested_owner.node_id(), nested_sample, "nested"),
    ] {
        let target = scroll_start
            .hit_test_css_point(point.0 as f32 + 0.5, point.1 as f32 + 0.5)
            .map(|render_box| render_box.node_id());
        if target != Some(owner) {
            return Err(format!(
                "C15.3 {label} scrolled Ahem hit owner가 맞지 않습니다: {target:?}"
            ));
        }
    }
    let scroll_report = format!(
        " scroll_origins_css_px=horizontal:{:.2},{:.2};vertical:{:.2},{:.2};nested:{:.2},{:.2} scroll_sequences=3>6>0;18>7>0;4+5>2+3>0+0 scroll_samples_css_px=horizontal:{},{};vertical:{},{};nested:{},{} scroll_hit_test=horizontal_owner;vertical_owner;nested_owner",
        scroll_start.texts()[3].origin_x_css_px(),
        scroll_start.texts()[3].origin_y_css_px(),
        scroll_start.texts()[4].origin_x_css_px(),
        scroll_start.texts()[4].origin_y_css_px(),
        scroll_start.texts()[5].origin_x_css_px(),
        scroll_start.texts()[5].origin_y_css_px(),
        horizontal_sample.0,
        horizontal_sample.1,
        vertical_sample.0,
        vertical_sample.1,
        nested_sample.0,
        nested_sample.1,
    );
    let scene = scroll_start;
    Ok((
        scene,
        format!(
            " c15_3_text=attached font=NotoSansKR glyphs={glyph_count} raster_scale={noto_scale} clip_right_css_px={clip_right} overflow_font=Ahem overflow_glyphs={overflow_glyph_count} overflow_clip_source=owner_render_box overflow_samples_css_px=visible:{},{};hidden_inside:{},{};hidden_outside:{},{} hit_test=visible_owner;hidden_inside_owner;hidden_outside_excludes_hidden_owner{scroll_report}",
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

fn runtime_ahem_text(
    scene: &RuntimeRenderSnapshot,
    owner: RuntimeRenderBox,
    scale: f32,
    glyphs: &[RuntimeGlyphBitmap],
    color: RuntimeSrgbColor,
) -> Result<RuntimeRenderText, String> {
    RuntimeRenderText::new(
        owner.node_id(),
        scene.key(),
        owner.frame_css_px().x(),
        owner.frame_css_px().y(),
        scale,
        color,
        CssClip::unconstrained(),
        glyphs.to_vec(),
    )
    .map_err(|error| format!("C15.3 scroll Ahem text snapshot 실패: {error}"))
}

fn find_scroll_container_by_port(
    scene: &RuntimeRenderSnapshot,
    width_css_px: f32,
    height_css_px: f32,
    label: &str,
) -> Result<RuntimeScrollContainer, String> {
    let matches = scene
        .scroll_containers()
        .iter()
        .copied()
        .filter(|container| {
            let port = container.scrollport_css_px();
            (port.width() - width_css_px).abs() <= 0.01
                && (port.height() - height_css_px).abs() <= 0.01
        })
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(format!(
            "C15.3 {label} scrollport {width_css_px}x{height_css_px} 검색 결과가 {}개입니다",
            matches.len()
        ));
    }
    Ok(matches[0])
}

fn require_scroll_ancestor(
    scene: &RuntimeRenderSnapshot,
    text_owner: spinon_core::NodeId,
    scroll_owner: spinon_core::NodeId,
    label: &str,
) -> Result<(), String> {
    if !scene
        .scroll_ancestors_for_node(text_owner)
        .is_some_and(|ancestors| ancestors.contains(&scroll_owner))
    {
        return Err(format!(
            "C15.3 {label} scroll owner가 text ancestor metadata에 없습니다"
        ));
    }
    Ok(())
}

fn text_origin(
    scene: &RuntimeRenderSnapshot,
    node_id: spinon_core::NodeId,
) -> Result<(f32, f32), String> {
    let Some(text) = scene.texts().iter().find(|text| text.node_id() == node_id) else {
        return Err(format!(
            "C15.3 runtime text origin owner가 없습니다: {node_id:?}"
        ));
    };
    Ok((text.origin_x_css_px(), text.origin_y_css_px()))
}

fn require_text_origin(
    scene: &RuntimeRenderSnapshot,
    node_id: spinon_core::NodeId,
    expected: (f32, f32),
    label: &str,
) -> Result<(), String> {
    let actual = text_origin(scene, node_id)?;
    if (actual.0 - expected.0).abs() > 0.01 || (actual.1 - expected.1).abs() > 0.01 {
        return Err(format!(
            "C15.3 {label} text origin이 base+offset 합과 다릅니다: expected={expected:?} actual={actual:?}"
        ));
    }
    Ok(())
}

fn solid_glyph_sample(
    scene: &RuntimeRenderSnapshot,
    node_id: spinon_core::NodeId,
) -> Result<(u32, u32), String> {
    let Some(text) = scene.texts().iter().find(|text| text.node_id() == node_id) else {
        return Err(format!(
            "C15.3 scroll text sample owner가 없습니다: {node_id:?}"
        ));
    };
    let viewport = scene.viewport_css_px();
    let scale = f64::from(text.device_scale_factor());
    for glyph in text.glyphs() {
        let left = f64::from(text.origin_x_css_px())
            + f64::from(glyph.origin_x_px() + glyph.left_px()) / scale;
        let top = f64::from(text.origin_y_css_px())
            + f64::from(glyph.baseline_y_px() + glyph.top_px()) / scale;
        for row in 0..glyph.height_px() {
            for column in 0..glyph.width_px() {
                let index = usize::try_from(
                    u64::from(row) * u64::from(glyph.width_px()) + u64::from(column),
                )
                .map_err(|_| "C15.3 scroll Ahem sample index 변환 실패".to_owned())?;
                if glyph.pixels()[index] != u8::MAX {
                    continue;
                }
                let center_x = left + (f64::from(column) + 0.5) / scale;
                let center_y = top + (f64::from(row) + 0.5) / scale;
                if center_x < 0.0
                    || center_y < 0.0
                    || center_x >= f64::from(viewport.width())
                    || center_y >= f64::from(viewport.height())
                    || !text
                        .clip_css_px()
                        .contains(center_x as f32, center_y as f32)
                {
                    continue;
                }
                return Ok((center_x.floor() as u32, center_y.floor() as u32));
            }
        }
    }
    Err(format!(
        "C15.3 {node_id:?} scrolled Ahem glyph mask에 clip 내부 opaque 표본이 없습니다"
    ))
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
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: fixture.font_family.to_owned(),
            font_size: fixture.font_size,
            line_height: fixture.line_height,
            white_space: WhiteSpaceMode::NoWrap,
            word_break: WordBreakMode::Normal,
            direction: TextDirection::Auto,
            line_break: LineBreakMode::Strict,
            ligatures: LigatureMode::Normal,
            letter_spacing: 0.0,
            text_indent: 0.0,
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
        CssAxisClip, CssClip, CssRect, CssSize, RuntimePaint, RuntimePaintCommand,
        RuntimeRenderBox, RuntimeRenderKey, RuntimeRenderSnapshot, RuntimeScrollClipLayer,
        RuntimeScrollContainer, RuntimeScrollRenderMetadata,
    };

    fn test_key() -> RuntimeRenderKey {
        let document = HostDocument::new().unwrap().snapshot();
        RuntimeRenderKey::new(
            document.generation(),
            document.document_revision(),
            document.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        )
    }

    fn fixture_scene(
        key: RuntimeRenderKey,
        visible_clip: Option<CssClip>,
        overlay: Option<NodeId>,
    ) -> RuntimeRenderSnapshot {
        let root = NodeId::new(1).unwrap();
        let visible = NodeId::new(2).unwrap();
        let hidden = NodeId::new(3).unwrap();
        let horizontal = NodeId::new(4).unwrap();
        let horizontal_text = NodeId::new(5).unwrap();
        let vertical = NodeId::new(6).unwrap();
        let vertical_text = NodeId::new(7).unwrap();
        let nested_outer = NodeId::new(8).unwrap();
        let nested_content = NodeId::new(9).unwrap();
        let nested_inner = NodeId::new(10).unwrap();
        let nested_text = NodeId::new(11).unwrap();
        let root_frame = CssRect::new(0.0, 0.0, 360.0, 240.0).unwrap();
        let visible_frame = CssRect::new(13.0, 81.0, 40.0, 10.0).unwrap();
        let hidden_frame = CssRect::new(13.0, 121.0, 40.0, 10.0).unwrap();
        let horizontal_frame = CssRect::new(40.0, 160.0, 15.0, 22.0).unwrap();
        let horizontal_text_frame = CssRect::new(41.0, 161.0, 40.0, 20.0).unwrap();
        let vertical_frame = CssRect::new(100.0, 160.0, 30.0, 15.0).unwrap();
        let vertical_text_frame = CssRect::new(101.0, 183.0, 28.0, 10.0).unwrap();
        let nested_outer_frame = CssRect::new(260.0, 160.0, 30.0, 22.0).unwrap();
        let nested_content_frame = CssRect::new(261.0, 161.0, 50.0, 20.0).unwrap();
        let nested_inner_frame = CssRect::new(261.0, 161.0, 20.0, 22.0).unwrap();
        let nested_text_frame = CssRect::new(262.0, 162.0, 40.0, 20.0).unwrap();
        let hidden_clip = CssClip::unconstrained()
            .with_x(CssAxisClip::new(13.0, 28.0).unwrap())
            .with_y(CssAxisClip::new(121.0, 131.0).unwrap());
        let visible_clip = visible_clip.unwrap_or_else(CssClip::unconstrained);
        let horizontal_port = CssRect::new(41.0, 161.0, 13.0, 20.0).unwrap();
        let vertical_port = CssRect::new(101.0, 161.0, 28.0, 13.0).unwrap();
        let nested_outer_port = CssRect::new(261.0, 161.0, 28.0, 20.0).unwrap();
        let nested_inner_port = CssRect::new(262.0, 162.0, 18.0, 20.0).unwrap();
        let clip_layer = |clip, ancestors| RuntimeScrollClipLayer::new(clip, clip, ancestors);
        let outer_clip = CssClip::unconstrained()
            .with_x(CssAxisClip::new(261.0, 289.0).unwrap())
            .with_y(CssAxisClip::new(161.0, 181.0).unwrap());
        let inner_clip = CssClip::unconstrained()
            .with_x(CssAxisClip::new(262.0, 280.0).unwrap())
            .with_y(CssAxisClip::new(162.0, 182.0).unwrap());
        let mut boxes = vec![
            RuntimeRenderBox::new(root, root_frame, RuntimePaint::None, 0),
            RuntimeRenderBox::new(visible, visible_frame, RuntimePaint::None, 1),
            RuntimeRenderBox::new(hidden, hidden_frame, RuntimePaint::None, 2),
            RuntimeRenderBox::new(horizontal, horizontal_frame, RuntimePaint::None, 3),
            RuntimeRenderBox::new(
                horizontal_text,
                horizontal_text_frame,
                RuntimePaint::None,
                4,
            ),
            RuntimeRenderBox::new(vertical, vertical_frame, RuntimePaint::None, 5),
            RuntimeRenderBox::new(vertical_text, vertical_text_frame, RuntimePaint::None, 6),
            RuntimeRenderBox::new(nested_outer, nested_outer_frame, RuntimePaint::None, 7),
            RuntimeRenderBox::new(nested_content, nested_content_frame, RuntimePaint::None, 8),
            RuntimeRenderBox::new(nested_inner, nested_inner_frame, RuntimePaint::None, 9),
            RuntimeRenderBox::new(nested_text, nested_text_frame, RuntimePaint::None, 10),
        ];
        if let Some(overlay) = overlay {
            boxes.push(RuntimeRenderBox::new(
                overlay,
                CssRect::new(29.0, 81.0, 24.0, 10.0).unwrap(),
                RuntimePaint::None,
                11,
            ));
        }
        let metadata = vec![
            RuntimeScrollRenderMetadata::new(root_frame, Vec::new(), Vec::new()),
            RuntimeScrollRenderMetadata::new(
                visible_frame,
                Vec::new(),
                vec![clip_layer(visible_clip, Vec::new())],
            ),
            RuntimeScrollRenderMetadata::new(
                hidden_frame,
                Vec::new(),
                vec![clip_layer(hidden_clip, Vec::new())],
            ),
            RuntimeScrollRenderMetadata::new(horizontal_frame, Vec::new(), Vec::new()),
            RuntimeScrollRenderMetadata::new(
                horizontal_text_frame,
                vec![horizontal],
                vec![clip_layer(
                    CssClip::unconstrained()
                        .with_x(CssAxisClip::new(41.0, 54.0).unwrap())
                        .with_y(CssAxisClip::new(161.0, 181.0).unwrap()),
                    Vec::new(),
                )],
            ),
            RuntimeScrollRenderMetadata::new(vertical_frame, Vec::new(), Vec::new()),
            RuntimeScrollRenderMetadata::new(
                vertical_text_frame,
                vec![vertical],
                vec![clip_layer(
                    CssClip::unconstrained()
                        .with_x(CssAxisClip::new(101.0, 129.0).unwrap())
                        .with_y(CssAxisClip::new(161.0, 174.0).unwrap()),
                    Vec::new(),
                )],
            ),
            RuntimeScrollRenderMetadata::new(nested_outer_frame, Vec::new(), Vec::new()),
            RuntimeScrollRenderMetadata::new(
                nested_content_frame,
                vec![nested_outer],
                vec![clip_layer(outer_clip, Vec::new())],
            ),
            RuntimeScrollRenderMetadata::new(
                nested_inner_frame,
                vec![nested_outer],
                vec![clip_layer(outer_clip, Vec::new())],
            ),
            RuntimeScrollRenderMetadata::new(
                nested_text_frame,
                vec![nested_inner, nested_outer],
                vec![
                    clip_layer(outer_clip, Vec::new()),
                    clip_layer(inner_clip, vec![nested_outer]),
                ],
            ),
        ];
        let mut metadata = metadata;
        if overlay.is_some() {
            metadata.push(RuntimeScrollRenderMetadata::new(
                CssRect::new(29.0, 81.0, 24.0, 10.0).unwrap(),
                Vec::new(),
                Vec::new(),
            ));
        }
        let scrolls = vec![
            RuntimeScrollContainer::new(horizontal, horizontal_port, 40.0, 20.0, 0.0, 0.0).unwrap(),
            RuntimeScrollContainer::new(vertical, vertical_port, 28.0, 32.0, 0.0, 0.0).unwrap(),
            RuntimeScrollContainer::new(nested_outer, nested_outer_port, 50.0, 20.0, 0.0, 0.0)
                .unwrap(),
            RuntimeScrollContainer::new(nested_inner, nested_inner_port, 40.0, 20.0, 0.0, 0.0)
                .unwrap()
                .with_parent(Some(nested_outer)),
        ];
        let commands = (0..boxes.len())
            .map(|index| RuntimePaintCommand::DrawBox { index })
            .collect();
        RuntimeRenderSnapshot::new_with_scroll_metadata_and_paint_commands(
            key,
            CssSize::new(360.0, 240.0).unwrap(),
            boxes,
            scrolls,
            metadata,
            commands,
        )
        .unwrap()
    }

    #[test]
    fn fixed_font_fixture_publishes_real_raster_as_last_paint_command() {
        let scene = fixture_scene(test_key(), None, None);
        let (scene, report) = attach_glyph_paint(scene).unwrap();
        assert_eq!(scene.texts().len(), 6);
        assert!(scene.texts().iter().all(|text| !text.glyphs().is_empty()));
        assert_eq!(scene.texts()[1].clip_css_px(), CssClip::unconstrained());
        assert_eq!(
            scene.texts()[2].clip_css_px(),
            CssClip::unconstrained()
                .with_x(CssAxisClip::new(13.0, 28.0).unwrap())
                .with_y(CssAxisClip::new(121.0, 131.0).unwrap())
        );
        assert_eq!(scene.texts()[3].origin_x_css_px(), 38.0);
        assert_eq!(scene.texts()[4].origin_y_css_px(), 165.0);
        assert_eq!(scene.texts()[5].origin_x_css_px(), 253.0);
        assert_eq!(
            scene.texts()[3].clip_css_px().x(),
            Some(CssAxisClip::new(41.0, 54.0).unwrap())
        );
        assert_eq!(
            scene.texts()[5].clip_css_px().x(),
            Some(CssAxisClip::new(261.0, 276.0).unwrap())
        );
        assert!(report.contains("c15_3_text=attached"));
        assert!(report.contains("overflow_font=Ahem"));
        assert!(report.contains("overflow_clip_source=owner_render_box"));
        assert!(report.contains("hidden_outside:"));
        assert!(report.contains(
            "hit_test=visible_owner;hidden_inside_owner;hidden_outside_excludes_hidden_owner"
        ));
        assert!(report.contains("scroll_sequences=3>6>0;18>7>0;4+5>2+3>0+0"));
        assert!(report.contains("scroll_hit_test=horizontal_owner;vertical_owner;nested_owner"));
        assert!(matches!(
            scene.paint_commands().last(),
            Some(RuntimePaintCommand::DrawText { index: 5 })
        ));
    }

    #[test]
    fn fixture_rejects_a_hidden_clip_on_the_visible_overflow_owner() {
        let visible_clip = CssClip::unconstrained()
            .with_x(CssAxisClip::new(13.0, 28.0).unwrap())
            .with_y(CssAxisClip::new(81.0, 91.0).unwrap());
        let scene = fixture_scene(test_key(), Some(visible_clip), None);

        let error = attach_glyph_paint(scene).unwrap_err();
        assert!(error.contains("visible overflow glyph의 event owner가 맞지 않습니다"));
    }

    #[test]
    fn fixture_does_not_let_visible_text_override_a_later_overlay_hit_target() {
        let overlay = NodeId::new(12).unwrap();
        let scene = fixture_scene(test_key(), None, Some(overlay));

        let error = attach_glyph_paint(scene).unwrap_err();
        assert_eq!(
            error,
            format!(
                "C15.3 visible overflow glyph의 event owner가 맞지 않습니다: Some({overlay:?})"
            )
        );
    }
}
