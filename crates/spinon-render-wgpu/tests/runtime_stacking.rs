#[cfg(target_os = "macos")]
mod macos {
    use std::sync::Arc;

    use spinon_core::{
        AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    };
    use spinon_style::{
        CssMediaEnvironment, CssViewport, StyloDocumentView,
        compute_runtime_block_flex_stacking_cascade_with_stylesheets,
    };
    use spinon_style_to_layout::compute_runtime_style_layout;
    use spinon_style_to_render::{CurrentLayoutInputs, build_runtime_render_snapshot};

    use spinon_render_wgpu::readback_runtime_scene_samples;

    const HTML: &str = "http://www.w3.org/1999/xhtml";

    #[test]
    fn stacking_context_order_matches_gpu_overlap_pixels() {
        let styles = [
            "display:block;position:relative;width:320px;height:240px;background-color:#101827",
            "position:relative;left:20px;top:20px;width:120px;height:80px;background-color:#243b5a",
            "position:absolute;left:0;top:0;width:100px;height:70px;z-index:2;background-color:#38bdf8",
            "position:absolute;left:50px;top:50px;width:100px;height:70px;z-index:1;background-color:#28a745",
            "position:absolute;left:165px;top:20px;width:120px;height:80px;z-index:1;background-color:#f59e0b",
            "position:absolute;left:0;top:0;width:120px;height:80px;z-index:9999;background-color:#2866f6",
            "position:absolute;left:165px;top:20px;width:120px;height:80px;z-index:2;background-color:#22aa66",
            "display:flex;position:absolute;left:20px;top:125px;width:240px;height:90px;flex-direction:row;align-items:flex-start;justify-content:flex-start",
            "position:static;flex:0 0 150px;height:70px;margin-right:-100px;z-index:1;background-color:#a855f7",
            "position:static;flex:0 0 150px;height:70px;z-index:2;background-color:#e5484d",
        ];
        let parents = [
            None,
            Some(0),
            Some(1),
            Some(0),
            Some(0),
            Some(4),
            Some(0),
            Some(0),
            Some(7),
            Some(7),
        ];
        let mut document = HostDocument::new().expect("HostDocument");
        let nodes = styles
            .iter()
            .map(|_| document.reserve_node_handle().expect("node handle"))
            .collect::<Vec<_>>();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(124).expect("owner id"),
            document.document_revision(),
        );
        for (index, (node, style)) in nodes.iter().zip(styles).enumerate() {
            batch.push(DocumentOperation::CreateElement {
                node: *node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node: *node,
                name: AttributeName::new(None, "style").expect("style attribute"),
                value: style.to_owned().into(),
            });
            let parent = match parents[index] {
                None => HostParent::Root,
                Some(parent) => HostParent::Node(nodes[parent]),
            };
            batch.push(DocumentOperation::InsertBefore {
                parent,
                node: *node,
                before: None,
            });
        }
        document.commit(batch).expect("document fixture commit");

        let snapshot = document.snapshot();
        let viewport = CssViewport {
            width_css_px: 320.0,
            height_css_px: 240.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let view =
            StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), nodes[0])
                .expect("Stylo view");
        let styles = compute_runtime_block_flex_stacking_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            Default::default(),
        )
        .expect("C12.4 cascade");
        let layout = compute_runtime_style_layout(&snapshot, nodes[0], styles, viewport)
            .expect("C12.4 Taffy layout");
        let current =
            CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
        let scene = build_runtime_render_snapshot(&snapshot, nodes[0], &layout, current)
            .expect("C12.4 render snapshot");
        assert_eq!(
            scene
                .boxes()
                .iter()
                .map(|render_box| render_box.node_id().get())
                .collect::<Vec<_>>(),
            [1, 2, 8, 4, 5, 6, 9, 3, 7, 10]
        );

        let samples = readback_runtime_scene_samples(
            &scene,
            320,
            240,
            &[(60, 55), (170, 25), (100, 140), (300, 230)],
        )
        .expect("C12.4 WGPU overlap-pixel readback");
        assert_eq!(
            samples,
            [
                [56, 189, 248, 255],
                [34, 170, 102, 255],
                [229, 72, 77, 255],
                [16, 24, 39, 255],
            ]
        );
    }

    #[test]
    fn wpt_derived_stacking_cases_match_offscreen_pixels() {
        let styles = [
            "display:block;position:relative;width:120px;height:60px;background-color:#101827",
            "position:absolute;left:10px;top:10px;width:30px;height:30px;z-index:1;background-color:#2866f6",
            "position:absolute;left:10px;top:10px;width:30px;height:30px;z-index:2;background-color:#ffa500",
            "position:absolute;left:10px;top:10px;width:30px;height:30px;z-index:3;background-color:#ffff00",
            "position:absolute;left:45px;top:10px;width:30px;height:30px;z-index:0;background-color:#ff0000",
            "position:absolute;left:0;top:0;width:30px;height:30px;z-index:-1;background-color:#008000",
            "display:flex;position:absolute;left:80px;top:10px;width:40px;height:30px;flex-direction:row;align-items:flex-start;justify-content:flex-start",
            "position:static;flex:0 0 30px;height:30px;margin-right:-20px;z-index:1;background-color:#2866f6",
            "position:absolute;left:0;top:0;width:30px;height:30px;z-index:9999;background-color:#ffff00",
            "position:static;flex:0 0 30px;height:30px;z-index:2;background-color:#008000",
        ];
        let parents = [
            None,
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(4),
            Some(0),
            Some(6),
            Some(7),
            Some(6),
        ];
        let mut document = HostDocument::new().expect("HostDocument");
        let nodes = styles
            .iter()
            .map(|_| document.reserve_node_handle().expect("node handle"))
            .collect::<Vec<_>>();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(125).expect("owner id"),
            document.document_revision(),
        );
        for (index, (node, style)) in nodes.iter().zip(styles).enumerate() {
            batch.push(DocumentOperation::CreateElement {
                node: *node,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node: *node,
                name: AttributeName::new(None, "style").expect("style attribute"),
                value: style.to_owned().into(),
            });
            let parent = match parents[index] {
                None => HostParent::Root,
                Some(parent) => HostParent::Node(nodes[parent]),
            };
            batch.push(DocumentOperation::InsertBefore {
                parent,
                node: *node,
                before: None,
            });
        }
        document.commit(batch).expect("document fixture commit");

        let snapshot = document.snapshot();
        let viewport = CssViewport {
            width_css_px: 120.0,
            height_css_px: 60.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let view =
            StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), nodes[0])
                .expect("Stylo view");
        let styles = compute_runtime_block_flex_stacking_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            Default::default(),
        )
        .expect("C12.4 WPT-derived cascade");
        let layout = compute_runtime_style_layout(&snapshot, nodes[0], styles, viewport)
            .expect("C12.4 WPT-derived Taffy layout");
        let current =
            CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
        let scene = build_runtime_render_snapshot(&snapshot, nodes[0], &layout, current)
            .expect("C12.4 WPT-derived render snapshot");
        assert_eq!(scene.boxes().len(), nodes.len());

        let samples =
            readback_runtime_scene_samples(&scene, 120, 60, &[(20, 20), (50, 20), (95, 20)])
                .expect("C12.4 WPT-derived WGPU overlap-pixel readback");
        assert_eq!(
            samples,
            [[255, 255, 0, 255], [0, 128, 0, 255], [0, 128, 0, 255],]
        );
    }

    #[test]
    fn reverse_flex_paint_matches_all_pinned_chrome_overlap_pixels() {
        let cases = [
            ("row", [0, 0], [229, 72, 77, 255]),
            ("row", [1, 0], [40, 102, 246, 255]),
            ("row-reverse", [0, 0], [40, 102, 246, 255]),
            ("row-reverse", [1, 0], [229, 72, 77, 255]),
            ("column", [0, 0], [229, 72, 77, 255]),
            ("column", [1, 0], [40, 102, 246, 255]),
            ("column-reverse", [0, 0], [40, 102, 246, 255]),
            ("column-reverse", [1, 0], [229, 72, 77, 255]),
        ];
        for (direction, orders, expected) in cases {
            let column = direction.starts_with("column");
            let overlap_margin = match (column, direction.ends_with("reverse")) {
                (false, false) => "margin-right:-40px",
                (false, true) => "margin-left:-40px",
                (true, false) => "margin-bottom:-40px",
                (true, true) => "margin-top:-40px",
            };
            let item_size = if column {
                "flex:0 0 60px;width:100px;height:60px"
            } else {
                "flex:0 0 80px;width:80px;height:60px"
            };
            let styles = [
                format!(
                    "display:flex;position:relative;width:100px;height:100px;flex-direction:{direction};align-items:flex-start;justify-content:flex-start;background-color:#101827"
                ),
                format!(
                    "position:static;{item_size};{overlap_margin};order:{};background-color:#2866f6",
                    orders[0]
                ),
                format!(
                    "position:static;{item_size};{overlap_margin};order:{};background-color:#e5484d",
                    orders[1]
                ),
            ];

            let mut document = HostDocument::new().expect("HostDocument");
            let nodes = styles
                .iter()
                .map(|_| document.reserve_node_handle().expect("node handle"))
                .collect::<Vec<_>>();
            let mut batch = DocumentChangeBatch::new(
                OwnerId::new(126).expect("owner id"),
                document.document_revision(),
            );
            for (index, (node, style)) in nodes.iter().zip(styles).enumerate() {
                batch.push(DocumentOperation::CreateElement {
                    node: *node,
                    namespace: HTML.to_owned(),
                    local_name: "div".to_owned(),
                });
                batch.push(DocumentOperation::SetAttribute {
                    node: *node,
                    name: AttributeName::new(None, "style").expect("style attribute"),
                    value: style.into(),
                });
                let parent = if index == 0 {
                    HostParent::Root
                } else {
                    HostParent::Node(nodes[0])
                };
                batch.push(DocumentOperation::InsertBefore {
                    parent,
                    node: *node,
                    before: None,
                });
            }
            document.commit(batch).expect("document fixture commit");

            let snapshot = document.snapshot();
            let viewport = CssViewport {
                width_css_px: 100.0,
                height_css_px: 100.0,
                device_scale_factor: 1.0,
                environment_revision: Default::default(),
                media_environment: CssMediaEnvironment::MOBILE,
            };
            let view = StyloDocumentView::new_html_fragment_child_shared(
                Arc::new(snapshot.clone()),
                nodes[0],
            )
            .expect("Stylo view");
            let styles = compute_runtime_block_flex_stacking_cascade_with_stylesheets(
                &view,
                &[],
                viewport,
                Default::default(),
            )
            .expect("reverse Flex C12.4 cascade");
            let layout = compute_runtime_style_layout(&snapshot, nodes[0], styles, viewport)
                .expect("reverse Flex C12.4 Taffy layout");
            let current =
                CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
            let scene = build_runtime_render_snapshot(&snapshot, nodes[0], &layout, current)
                .expect("reverse Flex C12.4 render snapshot");
            assert_eq!(scene.boxes().len(), nodes.len());
            let sample_y = if column && direction.ends_with("reverse") {
                50
            } else {
                30
            };
            let actual = readback_runtime_scene_samples(&scene, 100, 100, &[(50, sample_y)])
                .expect("reverse Flex WGPU overlap-pixel readback");
            assert_eq!(
                actual,
                [expected],
                "direction={direction}, order={orders:?}"
            );
        }
    }
}
