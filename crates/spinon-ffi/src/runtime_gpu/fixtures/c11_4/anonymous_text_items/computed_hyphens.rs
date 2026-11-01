#[derive(Clone, Copy, Debug, PartialEq)]
struct Scene {
    id: &'static str,
    text: &'static str,
    computed: &'static str,
    columns: &'static str,
    width: u8,
    inherited: bool,
}

const SCENES: [Scene; 21] = [
    scene(
        "hyphens-none-soft-min",
        "AB\u{00ad}CD",
        "none",
        "min-content",
        120,
        false,
    ),
    scene(
        "hyphens-none-soft-max",
        "AB\u{00ad}CD",
        "none",
        "max-content",
        120,
        false,
    ),
    scene(
        "hyphens-none-soft-fixed-30",
        "AB\u{00ad}CD",
        "none",
        "30px",
        30,
        false,
    ),
    scene(
        "hyphens-manual-soft-min",
        "AB\u{00ad}CD",
        "manual",
        "min-content",
        120,
        false,
    ),
    scene(
        "hyphens-manual-soft-max",
        "AB\u{00ad}CD",
        "manual",
        "max-content",
        120,
        false,
    ),
    scene(
        "hyphens-manual-soft-fixed-30",
        "AB\u{00ad}CD",
        "manual",
        "30px",
        30,
        false,
    ),
    scene(
        "hyphens-default-soft-fixed-30",
        "AB\u{00ad}CD",
        "",
        "30px",
        30,
        false,
    ),
    scene(
        "hyphens-none-visible-min",
        "AB-CD",
        "none",
        "min-content",
        120,
        false,
    ),
    scene(
        "hyphens-none-visible-fixed-30",
        "AB-CD",
        "none",
        "30px",
        30,
        false,
    ),
    scene(
        "hyphens-manual-visible-min",
        "AB-CD",
        "manual",
        "min-content",
        120,
        false,
    ),
    scene(
        "hyphens-manual-visible-fixed-30",
        "AB-CD",
        "manual",
        "30px",
        30,
        false,
    ),
    scene(
        "hyphens-none-unbroken-min",
        "ABCD",
        "none",
        "min-content",
        120,
        false,
    ),
    scene(
        "hyphens-manual-unbroken-min",
        "ABCD",
        "manual",
        "min-content",
        120,
        false,
    ),
    scene(
        "hyphens-none-multiple-min",
        "AB\u{00ad}CD\u{00ad}EF",
        "none",
        "min-content",
        120,
        false,
    ),
    scene(
        "hyphens-none-multiple-fixed-30",
        "AB\u{00ad}CD\u{00ad}EF",
        "none",
        "30px",
        30,
        false,
    ),
    scene(
        "hyphens-manual-multiple-min",
        "AB\u{00ad}CD\u{00ad}EF",
        "manual",
        "min-content",
        120,
        false,
    ),
    scene(
        "hyphens-manual-multiple-fixed-30",
        "AB\u{00ad}CD\u{00ad}EF",
        "manual",
        "30px",
        30,
        false,
    ),
    scene(
        "hyphens-inherited-none-fixed-30",
        "AB\u{00ad}CD",
        "none",
        "30px",
        30,
        true,
    ),
    scene(
        "hyphens-inherited-manual-fixed-30",
        "AB\u{00ad}CD",
        "manual",
        "30px",
        30,
        true,
    ),
    scene(
        "hyphens-auto-expected-reject",
        "AB\u{00ad}CD",
        "auto",
        "30px",
        30,
        false,
    ),
    scene(
        "hyphens-invalid-expected-reject",
        "AB\u{00ad}CD",
        "balanced",
        "30px",
        30,
        false,
    ),
];

const fn scene(
    id: &'static str,
    text: &'static str,
    computed: &'static str,
    columns: &'static str,
    width: u8,
    inherited: bool,
) -> Scene {
    Scene {
        id,
        text,
        computed,
        columns,
        width,
        inherited,
    }
}

pub(super) fn runtime_source(scene_id: &str) -> Option<String> {
    let scene = SCENES.iter().find(|scene| scene.id == scene_id)?;
    let root_id = format!("c114-{}-grid", scene.id);
    let wrapper_id = format!("c114-{}-parent", scene.id);
    let inherited_style = if scene.inherited {
        format!(
            "box-sizing:border-box;display:block;margin:0;padding:0;border:0;hyphens:{}",
            scene.computed
        )
    } else {
        "box-sizing:border-box;display:block;margin:0;padding:0;border:0".to_owned()
    };
    let wrapper_prefix = if scene.inherited {
        format!(
            "const wrapper = document.createElement('div');\nwrapper.setAttribute('id', {wrapper_id:?});\nwrapper.setAttribute('data-node', '');\nwrapper.setAttribute('style', {inherited_style:?});\ndocument.appendChild(wrapper);\n"
        )
    } else {
        String::new()
    };
    let mut declarations = vec![
        "box-sizing:border-box".to_owned(),
        "display:grid".to_owned(),
        format!("width:{}px", scene.width),
        "height:auto".to_owned(),
        "margin:0".to_owned(),
        "padding:0".to_owned(),
        "border:0".to_owned(),
        "gap:0".to_owned(),
        "align-items:start".to_owned(),
        "align-content:start".to_owned(),
        "justify-content:start".to_owned(),
        format!("grid-template-columns:{}", scene.columns),
        "grid-template-rows:auto".to_owned(),
        "font-family:Ahem".to_owned(),
        "font-size:10px".to_owned(),
        "font-weight:400".to_owned(),
        "font-style:normal".to_owned(),
        "line-height:10px".to_owned(),
        "white-space-collapse:collapse".to_owned(),
        "text-wrap-mode:wrap".to_owned(),
        "word-break:normal".to_owned(),
        "direction:ltr".to_owned(),
        "line-break:normal".to_owned(),
        "text-align:start".to_owned(),
        "overflow-wrap:normal".to_owned(),
        "text-transform:none".to_owned(),
        "letter-spacing:normal".to_owned(),
        "word-spacing:0px".to_owned(),
        "text-indent:0px".to_owned(),
        "color:#e5484d".to_owned(),
        "background-color:#101820".to_owned(),
    ];
    if !scene.inherited && !scene.computed.is_empty() {
        declarations.push(format!("hyphens:{}", scene.computed));
    }
    let style = declarations.join(";");
    let text = javascript_text_expression(scene.text);
    let append_target = if scene.inherited {
        "wrapper"
    } else {
        "document"
    };
    let node_ids = if scene.inherited {
        format!("[{wrapper_id:?}, {root_id:?}]")
    } else {
        format!("[{root_id:?}]")
    };
    Some(format!(
        r#"(() => {{
{wrapper_prefix}const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('lang', 'en');
root.setAttribute('style', {style:?});
{append_target}.appendChild(root);
root.appendChild(document.createTextNode({text}));
return {{ sceneId:{scene_id:?}, rootId:{root_id:?}, nodeIds:{node_ids} }};
}})()"#,
        scene_id = scene.id,
    ))
}

fn javascript_text_expression(text: &str) -> String {
    let mut parts = text.split('\u{00ad}');
    let mut expression = format!("{:?}", parts.next().unwrap_or_default());
    for part in parts {
        expression.push_str(" + String.fromCharCode(0x00ad) + ");
        expression.push_str(&format!("{part:?}"));
    }
    expression
}

#[cfg(test)]
mod tests {
    use super::{SCENES, runtime_source};
    use std::collections::BTreeSet;

    #[test]
    fn computed_hyphens_generates_the_fixed_direct_text_matrix() {
        let ids = SCENES.iter().map(|scene| scene.id).collect::<BTreeSet<_>>();
        assert_eq!(SCENES.len(), 21);
        assert_eq!(ids.len(), SCENES.len());
        for scene in SCENES {
            let source = runtime_source(scene.id).expect("allowlisted hyphens scene");
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("grid-template-columns:{};", scene.columns)));
            assert!(source.contains(&format!("width:{}px;", scene.width)));
            assert!(source.contains("root.setAttribute('lang', 'en')"));
            assert!(source.contains("font-family:Ahem"));
            assert_eq!(source.matches("document.createTextNode(").count(), 1);
            assert_eq!(
                source.matches("document.createElement('div')").count(),
                if scene.inherited { 2 } else { 1 }
            );
            if scene.computed == "" || scene.inherited {
                assert!(!source.contains("hyphens:") || scene.inherited);
            } else {
                assert!(source.contains(&format!("hyphens:{}", scene.computed)));
            }
            if scene.text.contains('\u{00ad}') {
                assert!(source.contains("String.fromCharCode(0x00ad)"));
            }
        }
        assert!(runtime_source("hyphens-language-auto").is_none());
    }
}
