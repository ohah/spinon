use super::*;

#[test]
fn cache_key_uses_exact_viewport_bits_and_all_media_inputs() {
    let fixture = Fixture::new("display:flex;width:120px;height:80px");
    let snapshot = fixture.snapshot();
    let base = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot: Arc::clone(&snapshot),
        viewport: CssViewport::C04_FIXTURE,
    };
    let mut changed_bits = WorkRequest {
        key: base.key,
        snapshot: Arc::clone(&snapshot),
        viewport: base.viewport,
    };
    let base_cache_key = RuntimeCalculationCacheKey::for_request(&base);
    changed_bits.viewport.width_css_px = f32::from_bits(base.viewport.width_css_px.to_bits() + 1);
    assert_ne!(
        base_cache_key,
        RuntimeCalculationCacheKey::for_request(&changed_bits)
    );

    changed_bits.viewport = base.viewport;
    changed_bits.viewport.height_css_px = f32::from_bits(base.viewport.height_css_px.to_bits() + 1);
    assert_ne!(
        base_cache_key,
        RuntimeCalculationCacheKey::for_request(&changed_bits)
    );

    changed_bits.viewport = base.viewport;
    changed_bits.viewport.device_scale_factor =
        f32::from_bits(base.viewport.device_scale_factor.to_bits() + 1);
    assert_ne!(
        base_cache_key,
        RuntimeCalculationCacheKey::for_request(&changed_bits)
    );

    let media_variants = [
        CssMediaEnvironment {
            color_scheme: spinon_style::CssColorScheme::Dark,
            ..base.viewport.media_environment
        },
        CssMediaEnvironment {
            primary_pointer: spinon_style::CssPrimaryPointer::Coarse,
            ..base.viewport.media_environment
        },
        CssMediaEnvironment {
            primary_hover: false,
            ..base.viewport.media_environment
        },
        CssMediaEnvironment {
            all_pointers: spinon_style::CssPointerCapabilities {
                coarse: true,
                ..base.viewport.media_environment.all_pointers
            },
            ..base.viewport.media_environment
        },
        CssMediaEnvironment {
            all_pointers: spinon_style::CssPointerCapabilities {
                fine: false,
                ..base.viewport.media_environment.all_pointers
            },
            ..base.viewport.media_environment
        },
        CssMediaEnvironment {
            all_pointers: spinon_style::CssPointerCapabilities {
                hover: false,
                ..base.viewport.media_environment.all_pointers
            },
            ..base.viewport.media_environment
        },
    ];
    for media_environment in media_variants {
        changed_bits.viewport = base.viewport;
        changed_bits.viewport.media_environment = media_environment;
        assert_ne!(
            base_cache_key,
            RuntimeCalculationCacheKey::for_request(&changed_bits)
        );
    }

    changed_bits.viewport = base.viewport;
    changed_bits.key.style_revision += 1;
    assert_ne!(
        base_cache_key,
        RuntimeCalculationCacheKey::for_request(&changed_bits)
    );

    changed_bits.key = base.key;
    changed_bits.key.environment_revision += 1;
    assert_ne!(
        base_cache_key,
        RuntimeCalculationCacheKey::for_request(&changed_bits)
    );

    changed_bits.key = base.key;
    changed_bits.key.render_tree_revision += 1;
    assert_ne!(
        base_cache_key,
        RuntimeCalculationCacheKey::for_request(&changed_bits)
    );

    changed_bits.key = base.key;
    changed_bits.key.generation += 1;
    assert_ne!(
        base_cache_key,
        RuntimeCalculationCacheKey::for_request(&changed_bits)
    );
}
