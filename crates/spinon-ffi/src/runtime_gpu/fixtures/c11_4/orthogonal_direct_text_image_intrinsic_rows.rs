#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Track {
    MinContent,
    MaxContent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Text,
    Image,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Order {
    Forward,
    Reversed,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    track: Track,
    order: Order,
}

const PNG_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";

const SCENES: [Scene; 4] = [
    Scene {
        id: "direct-orthogonal-row-min-content-forward",
        track: Track::MinContent,
        order: Order::Forward,
    },
    Scene {
        id: "direct-orthogonal-row-min-content-reversed",
        track: Track::MinContent,
        order: Order::Reversed,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-forward",
        track: Track::MaxContent,
        order: Order::Forward,
    },
    Scene {
        id: "direct-orthogonal-row-max-content-reversed",
        track: Track::MaxContent,
        order: Order::Reversed,
    },
];

pub(crate) fn runtime_source(scene_id: &str) -> Option<String> {
    SCENES
        .iter()
        .find(|scene| scene.id == scene_id)
        .copied()
        .map(scene_source)
}

fn scene_source(scene: Scene) -> String {
    let stem = scene
        .id
        .strip_prefix("direct-orthogonal-row-")
        .unwrap_or("");
    let root_id = format!("c114-orthogonal-{stem}-grid");
    let text_id = format!("c114-orthogonal-{stem}-text");
    let image_id = format!("c114-orthogonal-{stem}-image");
    let row = match scene.track {
        Track::MinContent => "min-content",
        Track::MaxContent => "max-content",
    };
    let mut source = format!(
        r#"(() => {{
const root = document.createElement('div');
root.setAttribute('id', {root_id:?});
root.setAttribute('data-node', '');
root.setAttribute('style', 'box-sizing:border-box;display:grid;width:74px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:0;direction:ltr;writing-mode:horizontal-tb;align-items:normal;align-content:start;justify-items:normal;justify-content:start;grid-template-columns:50px 24px;grid-template-rows:{row};background-color:#101820');
document.appendChild(root);
const nodeIds = [{root_id:?}];
"#
    );

    for (index, kind) in scene.item_order().into_iter().enumerate() {
        let declaration = match kind {
            Kind::Text => format!(
                r#"const item{index} = document.createElement('div');
item{index}.setAttribute('id', {text_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:1 / span 1;grid-row:1;writing-mode:vertical-rl;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d;color:#fff');
item{index}.appendChild(document.createTextNode('XX XXXXX'));
"#
            ),
            Kind::Image => format!(
                r#"const item{index} = document.createElement('img');
item{index}.setAttribute('id', {image_id:?});
item{index}.setAttribute('data-node', '');
item{index}.setAttribute('src', {PNG_URL:?});
item{index}.setAttribute('alt', '');
item{index}.setAttribute('style', 'box-sizing:border-box;display:block;width:auto;height:auto;min-width:auto;min-height:auto;margin:0;padding:0;border:0;grid-column:2 / span 1;grid-row:1;background-color:#2864dc');
"#
            ),
        };
        source.push_str(&declaration);
        let item_id = match kind {
            Kind::Text => &text_id,
            Kind::Image => &image_id,
        };
        source.push_str(&format!(
            "root.appendChild(item{index});\nnodeIds.push({item_id:?});\n"
        ));
    }

    source.push_str(&format!(
        "return {{ sceneId:{:?}, rootId:{root_id:?}, nodeIds }};\n}})()",
        scene.id
    ));
    source
}

impl Scene {
    fn item_order(self) -> [Kind; 2] {
        match self.order {
            Order::Forward => [Kind::Text, Kind::Image],
            Order::Reversed => [Kind::Image, Kind::Text],
        }
    }
}

impl Scene {
    #[cfg(test)]
    fn stem(self) -> &'static str {
        self.id.strip_prefix("direct-orthogonal-row-").unwrap_or("")
    }

    #[cfg(test)]
    fn node_id(self, kind: Kind) -> String {
        let suffix = match kind {
            Kind::Text => "text",
            Kind::Image => "image",
        };
        format!("c114-orthogonal-{}-{suffix}", self.stem())
    }
}

#[cfg(test)]
mod tests {
    use super::{Kind, Order, PNG_URL, SCENES, Track, runtime_source};

    #[test]
    fn exposes_only_the_four_fixed_orthogonal_row_scenes() {
        assert_eq!(SCENES.len(), 4);
        for (index, scene) in SCENES.iter().enumerate() {
            assert!(SCENES[..index].iter().all(|prior| prior.id != scene.id));
            let source = runtime_source(scene.id).expect("known orthogonal row scene");
            assert!(source.len() < 4 * 1024);
            assert!(source.contains("width:74px;height:auto"));
            assert!(source.contains("grid-template-columns:50px 24px"));
            assert!(source.contains("writing-mode:horizontal-tb"));
            assert!(source.contains("align-items:normal;align-content:start"));
            assert!(source.contains("justify-items:normal;justify-content:start"));
            assert!(source.contains("column-gap:0;row-gap:0"));
            assert!(source.contains("XX XXXXX"));
            assert!(source.contains("writing-mode:vertical-rl"));
            assert!(source.contains("line-break:auto"));
            assert!(source.contains("white-space-collapse:collapse;text-wrap-mode:wrap"));
            assert!(source.contains(PNG_URL));
            assert!(source.contains(&format!("sceneId:{:?}", scene.id)));
            assert!(source.contains(&format!("c114-orthogonal-{}-grid", scene.stem())));
            assert!(source.contains(&format!("c114-orthogonal-{}-text", scene.stem())));
            assert!(source.contains(&format!("c114-orthogonal-{}-image", scene.stem())));
            assert_eq!(source.matches("grid-row:1").count(), 2);
            assert_eq!(source.matches("document.createElement('img')").count(), 1);
            assert_eq!(source.matches("document.createElement('div')").count(), 2);
            assert_eq!(source.matches("document.createTextNode(").count(), 1);
            assert_eq!(source.matches("nodeIds.push(").count(), 2);
            assert!(!source.contains("justify-self:"));
            assert!(!source.contains("align-self:"));
            assert_eq!(source.matches("writing-mode:vertical-rl").count(), 1);
            assert!(scene.item_order().contains(&Kind::Text));
            assert!(scene.item_order().contains(&Kind::Image));
            assert!(source.contains(
                "min-width:auto;min-height:auto;margin:0;padding:0;border:0;grid-column:2 / span 1;grid-row:1;background-color:#2864dc"
            ));

            let expected_row = match scene.track {
                Track::MinContent => "grid-template-rows:min-content",
                Track::MaxContent => "grid-template-rows:max-content",
            };
            assert!(source.contains(expected_row));

            let text_position = source
                .find(&format!(
                    "setAttribute('id', {:?})",
                    scene.node_id(Kind::Text)
                ))
                .unwrap();
            let image_position = source
                .find(&format!(
                    "setAttribute('id', {:?})",
                    scene.node_id(Kind::Image)
                ))
                .unwrap();
            match scene.order {
                Order::Forward => assert!(text_position < image_position),
                Order::Reversed => assert!(image_position < text_position),
            }
        }
        assert!(runtime_source("direct-orthogonal-row-min-content-typo").is_none());
    }
}
