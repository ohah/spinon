#[cfg(target_os = "macos")]
mod macos {
    use std::sync::Arc;

    use spinon_core::{
        AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    };
    use spinon_render_wgpu::readback_runtime_scene_samples;
    use spinon_style::{
        CssMediaEnvironment, CssViewport, StyloDocumentView,
        compute_c19_box_paint_opacity_cascade_with_stylesheets,
    };
    use spinon_style_to_layout::compute_runtime_style_layout;
    use spinon_style_to_render::{CurrentLayoutInputs, build_runtime_render_snapshot};

    const HTML: &str = "http://www.w3.org/1999/xhtml";

    #[test]
    fn nested_opacity_source_alpha_and_zero_opacity_keep_expected_pixels_and_hits() {
        let styles = [
            "display:block;position:relative;width:180px;height:40px;background-color:rgb(32 49 75)",
            "position:absolute;left:0;top:0;width:24px;height:24px;opacity:0.5",
            "position:absolute;left:0;top:0;width:24px;height:24px;opacity:0.5;background-color:rgb(0 255 0)",
            "position:absolute;left:32px;top:0;width:24px;height:24px;opacity:0.5;background-color:rgb(255 0 0 / 50%)",
            "position:absolute;left:64px;top:0;width:24px;height:24px;opacity:0;background-color:rgb(255 0 0)",
            "position:absolute;left:96px;top:0;width:24px;height:24px;opacity:0.5;background-color:rgb(255 0 0)",
            "position:absolute;left:0;top:0;width:24px;height:24px;background-color:rgb(0 0 255)",
            "position:absolute;left:128px;top:0;width:24px;height:24px;opacity:1;background-color:rgb(255 0 0)",
            "position:absolute;left:0;top:0;width:24px;height:24px;background-color:rgb(0 0 255)",
        ];
        let parents = [
            None,
            Some(0),
            Some(1),
            Some(0),
            Some(0),
            Some(0),
            Some(5),
            Some(0),
            Some(7),
        ];
        let mut document = HostDocument::new().expect("HostDocument");
        let nodes = styles
            .iter()
            .map(|_| document.reserve_node_handle().expect("node handle"))
            .collect::<Vec<_>>();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(193).expect("owner id"),
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
            batch.push(DocumentOperation::InsertBefore {
                parent: match parents[index] {
                    None => HostParent::Root,
                    Some(parent) => HostParent::Node(nodes[parent]),
                },
                node: *node,
                before: None,
            });
        }
        document.commit(batch).expect("document fixture commit");
        let snapshot = document.snapshot();
        let viewport = CssViewport {
            width_css_px: 180.0,
            height_css_px: 40.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let view =
            StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), nodes[0])
                .expect("Stylo view");
        let styles = compute_c19_box_paint_opacity_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            Default::default(),
        )
        .expect("C19.2 cascade");
        let layout = compute_runtime_style_layout(&snapshot, nodes[0], styles, viewport)
            .expect("C19.2 layout");
        let current =
            CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
        let scene = build_runtime_render_snapshot(&snapshot, nodes[0], &layout, current)
            .expect("C19.2 render snapshot");
        let samples = readback_runtime_scene_samples(
            &scene,
            180,
            40,
            &[(5, 5), (37, 5), (69, 5), (101, 5), (133, 5)],
        )
        .expect("C19.2 WGPU readback");

        assert_eq!(
            samples,
            [
                [24, 101, 56, 255],
                [88, 37, 56, 255],
                [32, 49, 75, 255],
                [16, 25, 165, 255],
                [0, 0, 255, 255],
            ]
        );
        assert_eq!(
            scene
                .hit_test_css_point(69.0, 5.0)
                .map(|render_box| render_box.node_id()),
            Some(nodes[4].id())
        );
    }

    #[test]
    fn opacity_group_preserves_clip_z_order_and_hit_testing() {
        let styles = [
            "display:block;position:relative;width:80px;height:80px;background-color:rgb(32 49 75)",
            "position:absolute;left:0;top:0;width:64px;height:64px;overflow:hidden;opacity:0.5",
            "position:absolute;left:40px;top:0;width:50px;height:64px;z-index:1;background-color:rgb(255 0 0)",
            "position:absolute;left:20px;top:20px;width:30px;height:30px;z-index:2;background-color:rgb(0 0 255)",
        ];
        let parents = [None, Some(0), Some(1), Some(1)];
        let mut document = HostDocument::new().expect("HostDocument");
        let nodes = styles
            .iter()
            .map(|_| document.reserve_node_handle().expect("node handle"))
            .collect::<Vec<_>>();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(194).expect("owner id"),
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
            batch.push(DocumentOperation::InsertBefore {
                parent: match parents[index] {
                    None => HostParent::Root,
                    Some(parent) => HostParent::Node(nodes[parent]),
                },
                node: *node,
                before: None,
            });
        }
        document.commit(batch).expect("document fixture commit");
        let snapshot = document.snapshot();
        let viewport = CssViewport {
            width_css_px: 80.0,
            height_css_px: 80.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let view =
            StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), nodes[0])
                .expect("Stylo view");
        let styles = compute_c19_box_paint_opacity_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            Default::default(),
        )
        .expect("C19.2 clip cascade");
        let layout = compute_runtime_style_layout(&snapshot, nodes[0], styles, viewport)
            .expect("C19.2 clip layout");
        let current =
            CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
        let scene = build_runtime_render_snapshot(&snapshot, nodes[0], &layout, current)
            .expect("C19.2 clip render snapshot");
        let samples =
            readback_runtime_scene_samples(&scene, 80, 80, &[(30, 30), (55, 10), (70, 10)])
                .expect("C19.2 clip WGPU readback");

        assert_eq!(
            samples,
            [[16, 25, 165, 255], [143, 25, 38, 255], [32, 49, 75, 255],]
        );
        assert_eq!(
            scene
                .hit_test_css_point(30.0, 30.0)
                .map(|render_box| render_box.node_id()),
            Some(nodes[3].id())
        );
        assert_eq!(
            scene
                .hit_test_css_point(70.0, 10.0)
                .map(|render_box| render_box.node_id()),
            Some(nodes[0].id())
        );
    }

    #[test]
    fn current_color_border_alpha_source_over_matches_chromium_sample() {
        let styles = [
            "display:block;position:relative;width:100px;height:100px;background-color:rgb(32 49 75)",
            "position:absolute;left:10px;top:10px;width:80px;height:80px;color:rgb(24 192 128 / 50%);border:8px solid currentColor",
        ];
        let parents = [None, Some(0)];
        let mut document = HostDocument::new().expect("HostDocument");
        let nodes = styles
            .iter()
            .map(|_| document.reserve_node_handle().expect("node handle"))
            .collect::<Vec<_>>();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(195).expect("owner id"),
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
            batch.push(DocumentOperation::InsertBefore {
                parent: match parents[index] {
                    None => HostParent::Root,
                    Some(parent) => HostParent::Node(nodes[parent]),
                },
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
        let view =
            StyloDocumentView::new_html_fragment_child_shared(Arc::new(snapshot.clone()), nodes[0])
                .expect("Stylo view");
        let styles = compute_c19_box_paint_opacity_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            Default::default(),
        )
        .expect("C19.2 border cascade");
        let layout = compute_runtime_style_layout(&snapshot, nodes[0], styles, viewport)
            .expect("C19.2 border layout");
        let current =
            CurrentLayoutInputs::for_host_document(&snapshot, Default::default(), viewport);
        let scene = build_runtime_render_snapshot(&snapshot, nodes[0], &layout, current)
            .expect("C19.2 border render snapshot");
        let samples = readback_runtime_scene_samples(&scene, 100, 100, &[(12, 12), (50, 50)])
            .expect("C19.2 border WGPU readback");

        assert_eq!(samples, [[28, 121, 102, 255], [32, 49, 75, 255]]);
    }
}
