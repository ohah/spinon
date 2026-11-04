const POSITIVE_SCENES: [&str; 8] = [
    "dom-cross-axis",
    "mixed-spans",
    "hidden-controls",
    "hidden-row-lock-qualification",
    "hidden-column-lock-qualification",
    "display-none-control",
    "reverse-order",
    "negative-line-control",
];
const REJECTION_SCENE: &str = "implicit-boundary-control";
const POSITIVE_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-column-flow-mixed-cross-axis-runtime.html"
);
const REJECTION_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-column-flow-mixed-cross-axis-implicit-boundary-runtime.html"
);

pub(crate) fn scene_source(scene_id: &str) -> Result<String, String> {
    if scene_id == REJECTION_SCENE {
        return Ok(REJECTION_SOURCE.to_owned());
    }
    if !POSITIVE_SCENES.contains(&scene_id) {
        return Err(format!(
            "알 수 없는 C11.1 Root HTML mixed column-flow scene ID: {scene_id}"
        ));
    }
    let start = format!("<!-- scene:{scene_id}:start -->");
    let end = format!("<!-- scene:{scene_id}:end -->");
    let fragment = POSITIVE_SOURCE
        .split_once(&start)
        .ok_or_else(|| format!("C11.1 Root HTML fixture에 시작 표식이 없습니다: {scene_id}"))?
        .1
        .split_once(&end)
        .ok_or_else(|| format!("C11.1 Root HTML fixture에 끝 표식이 없습니다: {scene_id}"))?
        .0;
    Ok(format!(
        "<!doctype html><html lang=\"ko\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"></head><body style=\"margin:0;padding:0;background-color:#ffffff\">{fragment}</body></html>"
    ))
}

#[cfg(test)]
mod tests {
    use super::{POSITIVE_SCENES, REJECTION_SCENE, scene_source};

    #[test]
    fn source_selector_returns_only_pinned_positive_and_rejection_scenes() {
        for scene_id in POSITIVE_SCENES {
            let source = scene_source(scene_id).unwrap();
            assert!(source.contains(&format!("id=\"grid-{scene_id}\"")));
            assert_eq!(source.matches("grid-auto-flow:column").count(), 1);
        }
        assert!(scene_source(REJECTION_SCENE).unwrap().contains("row-four"));
        assert!(scene_source("unknown;throw 1").is_err());
        assert!(scene_source("").is_err());
    }
}
