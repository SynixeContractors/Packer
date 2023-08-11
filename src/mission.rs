use std::path::Path;

use regex::Regex;
use synixe_model::missions::Mission;

lazy_static::lazy_static! {
    static ref REGEX_VERSION: Regex = Regex::new(r"(?m)synixe_template = (\d+?);").unwrap();
}

pub fn read_mission(source: &Path, dir: &str, id: String) -> Mission {
    let description_ext =
        std::fs::read_to_string(source.join(format!("{}/{}/do_not_edit/description.ext", dir, id)))
            .unwrap();

    let version = {
        let version = REGEX_VERSION
            .captures(&description_ext)
            .map(|c| c.get(1).expect("always in capture").as_str());
        version.map_or("2", |version| version)
    };

    match version {
        "2" => crate::version_2::parse_mission(source, dir, id),
        "3" => crate::version_3::parse_mission(source, dir, id),
        _ => panic!("Unknown version {}", version),
    }
}
