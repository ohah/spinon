use super::*;

#[test]
fn c11_1_root_html_default_runtime_rejects_fraction_outside_its_bounded_profile() {
    let cases = [
        (
            "minmax track",
            "grid-template-columns",
            "130px",
            "40px",
            "minmax(0px, 1fr) 2fr",
            "40px",
            "0 10px",
            "",
            r#"<div class="a"></div><div class="b"></div>"#,
        ),
        (
            "named areas with fraction",
            "grid-template-areas",
            "120px",
            "70px",
            "1fr 2fr",
            "20px 50px",
            "0",
            r#"grid-template-areas: "header header" "side content";"#,
            r#"<div class="a" style="grid-area:header"></div><div class="b" style="grid-area:side"></div><div class="c" style="grid-area:content"></div>"#,
        ),
        (
            "indefinite flex row axis",
            "height",
            "40px",
            "auto",
            "40px",
            "1fr 2fr",
            "10px 0",
            "",
            r#"<div class="a"></div><div class="b"></div>"#,
        ),
        (
            "width min-size override",
            "min-width",
            "130px",
            "40px",
            "1fr 2fr",
            "40px",
            "0 10px",
            "min-width: 0;",
            r#"<div class="a"></div><div class="b"></div>"#,
        ),
        (
            "width max-size override",
            "max-width",
            "130px",
            "40px",
            "1fr 2fr",
            "40px",
            "0 10px",
            "max-width: 200px;",
            r#"<div class="a"></div><div class="b"></div>"#,
        ),
        (
            "aspect ratio override",
            "aspect-ratio",
            "130px",
            "40px",
            "1fr 2fr",
            "40px",
            "0 10px",
            "aspect-ratio: 2 / 1;",
            r#"<div class="a"></div><div class="b"></div>"#,
        ),
        (
            "percentage width basis",
            "width",
            "100%",
            "40px",
            "1fr 2fr",
            "40px",
            "0 10px",
            "",
            r#"<div class="a"></div><div class="b"></div>"#,
        ),
        (
            "math width basis",
            "width",
            "calc(100px + 30px)",
            "40px",
            "1fr 2fr",
            "40px",
            "0 10px",
            "",
            r#"<div class="a"></div><div class="b"></div>"#,
        ),
    ];

    for (name, property, width, height, columns, rows, gap, extra, items) in cases {
        let (session, layout) = run(&source(width, height, columns, rows, gap, extra, items));
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{name} must fail closed; layout={layout:?}; cascade={:?}",
            session.ua_cascade_snapshot()
        );
        let failure = layout
            .error
            .as_ref()
            .expect("failed layout has a diagnostic");
        let cascade_snapshot = session.ua_cascade_snapshot();
        if failure.code == "cascade_failed" {
            let cascade_error = cascade_snapshot
                .error
                .expect("unsupported CSS value should report its cascade boundary");
            assert!(cascade_error.contains(property), "{name}: {cascade_error}");
        } else {
            assert_eq!(failure.code, "unsupported_computed_value", "{name}");
            assert_eq!(
                failure.property.as_deref(),
                Some(property),
                "{name}: failure={failure:?}; cascade_error={:?}; computed={:?}",
                cascade_snapshot.error,
                cascade_snapshot.completed.as_ref().and_then(|completed| {
                    completed.roots[0]
                        .styles
                        .elements
                        .iter()
                        .find(|element| Some(element.node_id.get()) == failure.node_id)
                        .map(|element| {
                            (
                                element.properties.get("width"),
                                element.layout_dimensions,
                                element.layout_math_values.keys().collect::<Vec<_>>(),
                                element
                                    .grid_layout
                                    .as_ref()
                                    .map(|grid| grid.template_areas.clone()),
                            )
                        })
                })
            );
        }
        assert!(
            layout.completed.is_none(),
            "{name} must not publish a partial scene"
        );
    }
}
