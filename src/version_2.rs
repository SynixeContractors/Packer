use std::path::Path;

use regex::Regex;
use serde_json::{Map, Value};
use synixe_model::missions::Mission;

lazy_static::lazy_static! {
    static ref REGEX_NAME: Regex = Regex::new(r#"(?m)OnLoadName = "(.+?)";"#).unwrap();
    static ref REGEX_SUMMARY: Regex = Regex::new(r#"(?m)OnLoadMission = "(.+?)";"#).unwrap();
    static ref REGEX_TYPE: Regex = Regex::new(r"(?m)synixe_type = (\d);").unwrap();

    static ref REGEX_BRIEF_EMPLOYER: Regex = Regex::new(r#"(?ms)Employer", "(.+?)"]];"#).unwrap();
    static ref REGEX_BRIEF_SITUATION: Regex = Regex::new(r#"(?ms)Situation", "(.+?)"]];"#).unwrap();
    static ref REGEX_BRIEF_MISSION: Regex = Regex::new(r#"(?ms)Mission", "(.+?)"]];"#).unwrap();
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

    let path = source.join(format!("{}/{}/edit_me/briefing.sqf", dir, id));

    let briefing = if path.exists() {
        // Read briefing.sqf
        let briefing_sqf = std::fs::read_to_string(&path).unwrap();

        REGEX_BRIEF_EMPLOYER.captures(&briefing_sqf).map_or_else(
            || {
                format!(
                    "**{}**\n\n**Mission**{}",
                    name,
                    REGEX_BRIEF_MISSION
                        .captures(&briefing_sqf)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                )
            },
            |employer| {
                format!(
                    "**{}**\n\n**Employer**{}\n\n**Situation**{}\n\n**Mission**{}",
                    name,
                    employer.get(1).unwrap().as_str(),
                    REGEX_BRIEF_SITUATION
                        .captures(&briefing_sqf)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                    REGEX_BRIEF_MISSION
                        .captures(&briefing_sqf)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str(),
                )
            },
        )
    } else {
        String::new()
    };

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
        briefing: Value::Object({
            let mut map = Map::new();
            map.insert("old".to_string(), Value::String(briefing));
            map
        }),
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
