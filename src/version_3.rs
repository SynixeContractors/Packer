use std::path::Path;

use regex::Regex;
use serde_json::{Map, Value};
use synixe_model::missions::Mission;

lazy_static::lazy_static! {
    static ref REGEX_NAME: Regex = Regex::new(r#"(?m)OnLoadName = "(.+?)";"#).unwrap();
    static ref REGEX_SUMMARY: Regex = Regex::new(r#"(?m)OnLoadMission = "(.+?)";"#).unwrap();
    static ref REGEX_TYPE: Regex = Regex::new(r"(?m)synixe_type = (\d);").unwrap();
}

pub fn parse_mission(source: &Path, dir: &str, id: String) -> Mission {
    // Read description.ext
    let description_ext =
        std::fs::read_to_string(source.join(format!("{}/{}/edit_me/description.ext", dir, id)))
            .unwrap();

    let name = REGEX_NAME
        .captures(&description_ext)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();

    let mut briefing = Map::new();
    for title in ["situation", "mission", "objectives"] {
        let path = source.join(format!("{}/{}/edit_me/briefing/{}.html", dir, id, title));

        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap();

            briefing.insert(title.to_string(), Value::String(content));
        }
    }

    Mission {
        id,
        name,
        summary: REGEX_SUMMARY
            .captures(&description_ext)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string(),
        briefing: Value::Object(briefing),
        typ: REGEX_TYPE
            .captures(&description_ext)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap()
            .into(),
        play_count: None,
    }
}
