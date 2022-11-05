use serde_json::Value;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;
use structopt::StructOpt;

use super::config;
use super::utils;

#[derive(Debug, StructOpt)]
pub struct Pull {}
impl Pull {
    pub fn run(&self, config_path: PathBuf, url: String) -> Result<String, String> {
        let file = File::open(config_path).expect("Could not open file.");
        let config: config::Config = serde_yaml::from_reader(file).expect("Could not read values.");

        for topic in config.topics {
            let topic_name = format!("{}.{}.{}", topic.purpose, config.service, topic.resource);
            let subject = format!("{}/subjects/{}-value/versions/latest", url, topic_name);
            let response = reqwest::blocking::get(subject).expect("Could not get response.");
            let body = response.text().expect("Could not read response.");
            let parsed: Value = utils::read_json(&body);
            let schema = parsed["schema"].as_str().unwrap();
            let parsed: Value = utils::read_json(&schema);

            let purpose = format!("{}", topic.purpose);
            let resource = format!("{}-0001.avsc", topic.resource);

            let path: PathBuf = [&config.schema_path, &purpose, &resource].iter().collect();

            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).expect("Couldn't create directory");
            };

            let mut file = OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(path)
                .expect("Couldn't open file");
            serde_json::to_writer_pretty(&mut file, &parsed).expect("Couldn't write to file");
        }
        Ok("Success".to_string())
    }
}
