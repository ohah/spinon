# Root HTML fit-content 검증 경계 정정

계획 PR #1089의 Chrome 기준은 Ahem font resource를 HTML `@font-face`로 로드한다. 이 browser fixture는 독립 CSS oracle 용도이며 기본 Root HTML 앱의 font 환경이 아니다.

- [기본 Root HTML renderer](../../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/root_html_default_render.rs)는 `TextLayoutEngine::new_for_runtime()`를 만든다. `spinon-text-layout`에서 이 constructor는 Android/Apple에서는 system-font discovery를 켜고, 다른 host에서는 system font discovery를 끈다.
- [Ahem 등록 API](../../../../crates/spinon-text-layout/src/engine/fonts.rs)는 명시적 `register_font()` 호출을 제공한다. 기본 Root HTML renderer는 해당 호출을 하지 않는다.
- C11.3.3 Ahem fixture는 별도로 `TextLayoutEngine::new_without_system_fonts()`와 명시적 `register_font("Ahem", AHEM)`를 사용한다. 따라서 그 fixture의 exact intrinsic result는 기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 실행으로 보고할 수 없다.

계획 구현은 이 차이를 그대로 유지한다. Ahem을 쓰는 cap/floor intrinsic matrix는 `spinon-layout` test engine에서 검증한다. parsed Root HTML profile의 admission/projection은 `spinon-style-to-layout`에서 검증한다. Android/iOS default-route smoke는 font-independent fixed-size box fixture를 사용하며 `layout=ready`, track/frame, draw/present와 surface samples만 주장한다. 세 evidence class를 같은 Ahem run으로 합치지 않는다.
