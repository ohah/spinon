fn assert_rejected(label: &str, source: &str) {
    let (session, layout) = super::super::run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "{label}: layout={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot().error
    );
    assert!(
        layout.completed.is_none(),
        "{label}: failure must not publish layout or render output"
    );
}

#[test]
fn parsed_root_html_one_fit_auto_minimum_rejects_out_of_profile_mutations() {
    let scene = super::auto_minimum_scenes()
        .into_iter()
        .find(|scene| {
            scene["fitPosition"].as_u64() == Some(1)
                && scene["cap"].as_str() == Some("20px")
                && scene["align"].as_str() == Some("start")
        })
        .expect("one-fit start scene exists");
    let source = super::source_for(&scene, true);
    let original_tracks = scene["computedAutoRows"].as_str().unwrap();
    let alter_tracks = |tracks: &str| super::super::replace_once(&source, original_tracks, tracks);

    let cases = [
        (
            "five fit tracks",
            alter_tracks(
                "fit-content(10px) fit-content(20%) fit-content(30px) fit-content(40px) fit-content(50px) min-content",
            ),
        ),
        (
            "wrong one-fit tail multiplicity",
            alter_tracks("fit-content(10px) min-content max-content max-content max-content auto"),
        ),
        (
            "zero fit tracks",
            alter_tracks("min-content min-content max-content max-content auto auto"),
        ),
        (
            "negative cap",
            alter_tracks("fit-content(-1px) min-content max-content max-content auto min-content"),
        ),
        (
            "calculated cap",
            alter_tracks(
                "fit-content(calc(10px + 10%)) min-content max-content max-content auto min-content",
            ),
        ),
        (
            "automatic minimum on explicit-row control",
            super::super::replace_once(
                &source,
                "#item-0 { min-height: 0; height: 10px;",
                "#item-0 { min-height: auto; height: 10px;",
            ),
        ),
        (
            "mixed zero and automatic text-item minima",
            super::super::replace_once(
                &source,
                "#item-2 { background-color: #5ba8f5; }",
                "#item-2 { min-height: 0; background-color: #5ba8f5; }",
            ),
        ),
        (
            "invalid fixed text-item minimum",
            super::super::replace_once(
                &source,
                "#item-2 { background-color: #5ba8f5; }",
                "#item-2 { min-height: 1px; background-color: #5ba8f5; }",
            ),
        ),
        (
            "scrollable first text item",
            super::super::replace_once(
                &source,
                "#item-1 { background-color: #f2665e; }",
                "#item-1 { overflow: auto; background-color: #f2665e; }",
            ),
        ),
        (
            "scrollable item block axis",
            super::super::replace_once(
                &source,
                "#item-1 { background-color: #f2665e; }",
                "#item-1 { overflow-y: scroll; background-color: #f2665e; }",
            ),
        ),
        (
            "hidden text-item overflow",
            super::super::replace_once(
                &source,
                "#item-1 { background-color: #f2665e; }",
                "#item-1 { overflow: hidden; background-color: #f2665e; }",
            ),
        ),
        (
            "scrollable Grid overflow",
            super::super::replace_once(&source, "overflow: visible;", "overflow: auto;"),
        ),
        (
            "indefinite Grid block size",
            super::super::replace_once(&source, "height: 220px;", "height: auto;"),
        ),
        (
            "wrong Grid width",
            super::super::replace_once(&source, "width: 40px;", "width: 41px;"),
        ),
        (
            "unsupported align-content",
            super::super::replace_once(&source, "align-content: start;", "align-content: center;"),
        ),
        (
            "unsupported flow",
            super::super::replace_once(&source, "grid-auto-flow: row;", "grid-auto-flow: column;"),
        ),
        (
            "wrong writing direction",
            super::super::replace_once(&source, "direction: ltr;", "direction: rtl;"),
        ),
        (
            "explicit placement on text item",
            super::super::replace_once(
                &source,
                "#item-1 { background-color: #f2665e; }",
                "#item-1 { grid-row: 2; background-color: #f2665e; }",
            ),
        ),
        (
            "missing ninth Grid item",
            super::super::replace_once(
                &source,
                "      <div id=\"item-8\" class=\"item\"><div id=\"text-8\" class=\"payload\">AA</div></div>\n",
                "",
            ),
        ),
        (
            "extra tenth Grid item",
            super::super::replace_once(
                &source,
                "    </div>\n  </body>",
                "      <div id=\"item-9\" class=\"item\"></div>\n    </div>\n  </body>",
            ),
        ),
        (
            "nested payload text node",
            super::super::replace_once(
                &source,
                "<div id=\"item-8\" class=\"item\"><div id=\"text-8\" class=\"payload\">AA</div></div>",
                "<div id=\"item-8\" class=\"item\"><div id=\"text-8\" class=\"payload\">AA<span>BB</span></div></div>",
            ),
        ),
        (
            "payload font mutation",
            super::super::replace_once(&source, "font-size: 10px;", "font-size: 11px;"),
        ),
    ];

    for (label, source) in cases {
        assert_rejected(label, &source);
    }
}
